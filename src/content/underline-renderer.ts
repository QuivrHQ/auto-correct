import { LanguageToolMatch } from '../shared/types'

interface TextPosition {
  x: number
  y: number
  width: number
  height: number
}

interface VisibleRange {
  startOffset: number
  endOffset: number
}

interface TooltipCallbacks {
  onReplace: (match: LanguageToolMatch, replacement: string) => void
  onIgnore: (match: LanguageToolMatch) => void
}

export class UnderlineRenderer {
  private element: HTMLInputElement | HTMLTextAreaElement | HTMLElement
  private overlay: HTMLDivElement | null = null
  private shadowRoot: ShadowRoot | null = null
  private resizeObserver: ResizeObserver | null = null
  private tooltip: HTMLDivElement | null = null
  private currentMatches: LanguageToolMatch[] = []
  private ignoredMatches: Set<string> = new Set()
  private callbacks: TooltipCallbacks | null = null
  private boundHideTooltip: (e: Event) => void

  constructor(element: HTMLInputElement | HTMLTextAreaElement | HTMLElement) {
    this.element = element
    this.boundHideTooltip = this.handleOutsideClick.bind(this)
  }

  init(callbacks: TooltipCallbacks): void {
    this.callbacks = callbacks
    this.createOverlay()
    this.setupObservers()
  }

  private createOverlay(): void {
    this.overlay = document.createElement('div')
    this.overlay.className = 'autocorrect-overlay'
    this.overlay.style.cssText = `
      position: absolute;
      pointer-events: none;
      overflow: visible;
      z-index: 2147483646;
    `

    this.shadowRoot = this.overlay.attachShadow({ mode: 'open' })

    const style = document.createElement('style')
    style.textContent = `
      :host {
        all: initial;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      }

      /* Error underlines - now clickable */
      .error-highlight {
        position: absolute;
        background: transparent;
        cursor: pointer;
        pointer-events: auto;
        border-radius: 2px;
      }
      .error-highlight:hover {
        background: rgba(239, 68, 68, 0.1);
      }
      .error-highlight::after {
        content: '';
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        height: 2px;
        background: currentColor;
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='6' height='3' viewBox='0 0 6 3'%3E%3Cpath d='M0 3 L1.5 0 L3 3 L4.5 0 L6 3' stroke='%23EF4444' fill='none' stroke-width='1'/%3E%3C/svg%3E");
        background-repeat: repeat-x;
        background-position: bottom;
        background-size: 6px 3px;
      }
      .error-spelling::after {
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='6' height='3' viewBox='0 0 6 3'%3E%3Cpath d='M0 3 L1.5 0 L3 3 L4.5 0 L6 3' stroke='%23EF4444' fill='none' stroke-width='1'/%3E%3C/svg%3E");
      }
      .error-grammar::after {
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='6' height='3' viewBox='0 0 6 3'%3E%3Cpath d='M0 3 L1.5 0 L3 3 L4.5 0 L6 3' stroke='%23F59E0B' fill='none' stroke-width='1'/%3E%3C/svg%3E");
      }
      .error-grammar:hover {
        background: rgba(245, 158, 11, 0.1);
      }

      /* Tooltip */
      .tooltip {
        position: fixed;
        background: white;
        border-radius: 12px;
        box-shadow: 0 4px 24px rgba(0, 0, 0, 0.12), 0 0 0 1px rgba(0, 0, 0, 0.05);
        padding: 0;
        min-width: 240px;
        max-width: 320px;
        z-index: 2147483647;
        animation: tooltipIn 0.15s ease-out;
        overflow: hidden;
        pointer-events: auto;
      }
      @keyframes tooltipIn {
        from {
          opacity: 0;
          transform: translateY(-4px);
        }
        to {
          opacity: 1;
          transform: translateY(0);
        }
      }

      .tooltip-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 10px 12px;
        background: #FAFAFA;
        border-bottom: 1px solid #F0F0F0;
      }
      .tooltip-category {
        display: flex;
        align-items: center;
        gap: 8px;
        font-weight: 600;
        font-size: 13px;
        color: #374151;
      }
      .category-dot {
        width: 8px;
        height: 8px;
        border-radius: 50%;
      }
      .category-dot.spelling { background: #EF4444; }
      .category-dot.grammar { background: #F59E0B; }
      .category-dot.style { background: #3B82F6; }

      .tooltip-close {
        width: 24px;
        height: 24px;
        display: flex;
        align-items: center;
        justify-content: center;
        border: none;
        background: transparent;
        color: #9CA3AF;
        cursor: pointer;
        border-radius: 4px;
        transition: all 0.15s;
      }
      .tooltip-close:hover {
        background: #E5E7EB;
        color: #374151;
      }

      .tooltip-body {
        padding: 12px;
      }
      .tooltip-message {
        color: #4B5563;
        font-size: 13px;
        line-height: 1.5;
        margin-bottom: 12px;
      }

      .tooltip-suggestions {
        display: flex;
        flex-wrap: wrap;
        gap: 6px;
      }
      .suggestion-btn {
        padding: 7px 14px;
        background: #3B82F6;
        color: white;
        border: none;
        border-radius: 8px;
        font-size: 13px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.15s;
        pointer-events: auto;
      }
      .suggestion-btn:hover {
        background: #2563EB;
        transform: translateY(-1px);
      }
      .suggestion-btn:active {
        transform: translateY(0);
      }

      .ignore-btn {
        padding: 7px 14px;
        background: white;
        color: #6B7280;
        border: 1px solid #E5E7EB;
        border-radius: 8px;
        font-size: 13px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.15s;
        pointer-events: auto;
      }
      .ignore-btn:hover {
        background: #F9FAFB;
        border-color: #D1D5DB;
      }
    `
    this.shadowRoot.appendChild(style)

    document.body.appendChild(this.overlay)
    this.updatePosition()
  }

  private setupObservers(): void {
    this.resizeObserver = new ResizeObserver(() => {
      this.updatePosition()
    })
    this.resizeObserver.observe(this.element)

    this.element.addEventListener('scroll', () => {
      this.updatePosition()
      this.hideTooltip()
    })
    window.addEventListener('scroll', () => {
      this.updatePosition()
      this.hideTooltip()
    }, true)
    window.addEventListener('resize', () => {
      this.updatePosition()
      this.hideTooltip()
    })

    // Hide tooltip on input
    this.element.addEventListener('input', () => {
      this.hideTooltip()
    })
  }

  private updatePosition(): void {
    if (!this.overlay) return

    const rect = this.element.getBoundingClientRect()
    const scrollX = window.scrollX
    const scrollY = window.scrollY

    this.overlay.style.left = `${rect.left + scrollX}px`
    this.overlay.style.top = `${rect.top + scrollY}px`
    this.overlay.style.width = `${rect.width}px`
    this.overlay.style.height = `${rect.height}px`
  }

  render(matches: LanguageToolMatch[], text: string): void {
    if (!this.shadowRoot) return

    this.currentMatches = matches
    this.hideTooltip()

    // Clear existing underlines (keep style)
    const existingUnderlines = this.shadowRoot.querySelectorAll('.error-highlight')
    existingUnderlines.forEach(el => el.remove())

    console.log('[AutoCorrect] Rendering', matches.length, 'matches')
    if (matches.length === 0) return

    // For optimization: only render errors in visible range + buffer
    const visibleRange = this.getVisibleTextRange(text)
    const visibleMatches = matches.filter(match => {
      const matchEnd = match.offset + match.length
      // Include errors that overlap with visible range (with 500 char buffer)
      const bufferStart = Math.max(0, visibleRange.startOffset - 500)
      const bufferEnd = visibleRange.endOffset + 500
      return matchEnd > bufferStart && match.offset < bufferEnd
    })

    console.log('[AutoCorrect] Visible matches:', visibleMatches.length, 'range:', visibleRange)
    const positions = this.calculatePositions(visibleMatches, text)
    console.log('[AutoCorrect] Positions calculated:', positions)

    let renderedCount = 0
    positions.forEach((pos, i) => {
      const match = visibleMatches[i]
      const originalIndex = matches.indexOf(match)

      // Skip ignored matches
      const matchKey = `${match.offset}-${match.length}-${match.rule.id}`
      if (this.ignoredMatches.has(matchKey)) {
        console.log('[AutoCorrect] Skipping ignored match:', matchKey)
        return
      }

      // Skip if position is outside visible area
      if (pos.y < -50 || pos.y > this.element.clientHeight + 50) {
        console.log('[AutoCorrect] Skipping out-of-bounds match:', pos.y, 'element height:', this.element.clientHeight)
        return
      }

      const underline = document.createElement('span')
      underline.className = `error-highlight ${this.getErrorClass(match)}`
      underline.style.left = `${pos.x}px`
      underline.style.top = `${pos.y}px`
      underline.style.width = `${pos.width}px`
      underline.style.height = `${pos.height}px`
      underline.dataset.matchIndex = String(originalIndex)

      underline.addEventListener('click', (e) => {
        e.preventDefault()
        e.stopPropagation()
        this.showTooltip(match, e.clientX, e.clientY, pos)
      })

      this.shadowRoot!.appendChild(underline)
      renderedCount++
    })
    console.log('[AutoCorrect] Rendered', renderedCount, 'underlines')
  }

  private getVisibleTextRange(text: string): VisibleRange {
    const element = this.element

    // For single-line inputs, the entire text is "visible"
    if (element instanceof HTMLInputElement) {
      return { startOffset: 0, endOffset: text.length }
    }

    // For textareas, calculate based on scroll position
    if (element instanceof HTMLTextAreaElement) {
      const styles = window.getComputedStyle(element)
      const lineHeight = parseFloat(styles.lineHeight) || parseFloat(styles.fontSize) * 1.2
      const scrollTop = element.scrollTop
      const clientHeight = element.clientHeight

      // Calculate visible line range with buffer
      const firstVisibleLine = Math.max(0, Math.floor(scrollTop / lineHeight) - 2)
      const lastVisibleLine = Math.ceil((scrollTop + clientHeight) / lineHeight) + 2

      // Calculate character offsets
      const lines = text.split('\n')
      let startOffset = 0
      let endOffset = text.length

      // Sum lengths of lines before visible area
      for (let i = 0; i < firstVisibleLine && i < lines.length; i++) {
        startOffset += lines[i].length + 1 // +1 for newline
      }

      // Sum lengths up to end of visible area
      let currentOffset = 0
      for (let i = 0; i <= lastVisibleLine && i < lines.length; i++) {
        currentOffset += lines[i].length + 1
      }
      endOffset = Math.min(currentOffset, text.length)

      return { startOffset, endOffset }
    }

    // For contenteditable, return full range (harder to calculate)
    return { startOffset: 0, endOffset: text.length }
  }

  private getErrorClass(match: LanguageToolMatch): string {
    const category = match.rule.category.id.toUpperCase()
    if (category.includes('TYPO') || category.includes('SPELL')) {
      return 'error-spelling'
    }
    if (category.includes('GRAMMAR')) {
      return 'error-grammar'
    }
    return 'error-grammar' // Default to grammar style
  }

  private getCategoryInfo(match: LanguageToolMatch): { name: string; class: string } {
    const category = match.rule.category.id.toUpperCase()
    if (category.includes('TYPO') || category.includes('SPELL')) {
      return { name: 'Orthographe', class: 'spelling' }
    }
    if (category.includes('GRAMMAR')) {
      return { name: 'Grammaire', class: 'grammar' }
    }
    return { name: 'Style', class: 'style' }
  }

  private showTooltip(match: LanguageToolMatch, clickX: number, clickY: number, pos: TextPosition): void {
    this.hideTooltip()

    if (!this.shadowRoot) return

    const categoryInfo = this.getCategoryInfo(match)

    this.tooltip = document.createElement('div')
    this.tooltip.className = 'tooltip'

    // Position the tooltip below the click, or above if no space
    const viewportHeight = window.innerHeight
    const tooltipHeight = 150 // Approximate height
    const spaceBelow = viewportHeight - clickY - 20

    let top = clickY + 10
    if (spaceBelow < tooltipHeight && clickY > tooltipHeight) {
      top = clickY - tooltipHeight - 10
    }

    this.tooltip.style.left = `${Math.min(clickX - 20, window.innerWidth - 340)}px`
    this.tooltip.style.top = `${top}px`

    // Build tooltip HTML
    this.tooltip.innerHTML = `
      <div class="tooltip-header">
        <div class="tooltip-category">
          <span class="category-dot ${categoryInfo.class}"></span>
          <span>${categoryInfo.name}</span>
        </div>
        <button class="tooltip-close" aria-label="Fermer">
          <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M1 1L13 13M1 13L13 1"/>
          </svg>
        </button>
      </div>
      <div class="tooltip-body">
        <p class="tooltip-message">${match.message}</p>
        <div class="tooltip-suggestions">
          ${match.replacements.slice(0, 3).map(r =>
            `<button class="suggestion-btn" data-replacement="${this.escapeHtml(r.value)}">${this.escapeHtml(r.value)}</button>`
          ).join('')}
          <button class="ignore-btn">Ignorer</button>
        </div>
      </div>
    `

    // Add event listeners
    this.tooltip.querySelector('.tooltip-close')?.addEventListener('click', () => {
      this.hideTooltip()
    })

    this.tooltip.querySelectorAll('.suggestion-btn').forEach(btn => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation()
        const replacement = (e.target as HTMLElement).dataset.replacement || ''
        console.log('[AutoCorrect] Suggestion clicked:', replacement)
        this.callbacks?.onReplace(match, replacement)
        this.hideTooltip()
      })
    })

    this.tooltip.querySelector('.ignore-btn')?.addEventListener('click', () => {
      const matchKey = `${match.offset}-${match.length}-${match.rule.id}`
      this.ignoredMatches.add(matchKey)
      this.callbacks?.onIgnore(match)
      this.hideTooltip()
      // Re-render to hide the ignored error
      this.render(this.currentMatches, this.getElementText())
    })

    this.shadowRoot.appendChild(this.tooltip)

    // Add click outside listener (use bubble phase, not capture)
    setTimeout(() => {
      document.addEventListener('click', this.boundHideTooltip, false)
    }, 10)
  }

  private handleOutsideClick(e: Event): void {
    // Use composedPath() to correctly detect clicks inside Shadow DOM
    const path = e.composedPath()
    if (this.tooltip && !path.includes(this.tooltip)) {
      this.hideTooltip()
    }
  }

  private hideTooltip(): void {
    if (this.tooltip) {
      this.tooltip.remove()
      this.tooltip = null
      document.removeEventListener('click', this.boundHideTooltip, false)
    }
  }

  private escapeHtml(text: string): string {
    const div = document.createElement('div')
    div.textContent = text
    return div.innerHTML
  }

  private getElementText(): string {
    if (this.element instanceof HTMLInputElement || this.element instanceof HTMLTextAreaElement) {
      return this.element.value
    }
    return this.element.textContent || ''
  }

  private calculatePositions(matches: LanguageToolMatch[], text: string): TextPosition[] {
    const isInput = this.element instanceof HTMLInputElement
    const isTextarea = this.element instanceof HTMLTextAreaElement

    if (isInput || isTextarea) {
      return this.calculateInputPositions(matches, text)
    }

    return this.calculateContentEditablePositions(matches, text)
  }

  private calculateInputPositions(matches: LanguageToolMatch[], text: string): TextPosition[] {
    const element = this.element as HTMLInputElement | HTMLTextAreaElement
    const styles = window.getComputedStyle(element)

    // Create a mirror div that exactly matches the input's text rendering
    const mirror = document.createElement('div')
    mirror.style.cssText = `
      position: absolute;
      top: -9999px;
      left: -9999px;
      visibility: hidden;
      white-space: pre-wrap;
      word-wrap: break-word;
      overflow-wrap: break-word;
      font-family: ${styles.fontFamily};
      font-size: ${styles.fontSize};
      font-weight: ${styles.fontWeight};
      font-style: ${styles.fontStyle};
      letter-spacing: ${styles.letterSpacing};
      word-spacing: ${styles.wordSpacing};
      line-height: ${styles.lineHeight};
      text-transform: ${styles.textTransform};
      padding: ${styles.padding};
      border: ${styles.borderWidth} solid transparent;
      box-sizing: border-box;
      width: ${element.offsetWidth}px;
    `
    document.body.appendChild(mirror)

    const positions: TextPosition[] = []
    const paddingLeft = parseFloat(styles.paddingLeft) || 0
    const paddingTop = parseFloat(styles.paddingTop) || 0
    const borderLeft = parseFloat(styles.borderLeftWidth) || 0
    const borderTop = parseFloat(styles.borderTopWidth) || 0
    const lineHeight = parseFloat(styles.lineHeight) || parseFloat(styles.fontSize) * 1.2

    const scrollLeft = element.scrollLeft || 0
    const scrollTop = element.scrollTop || 0

    matches.forEach(match => {
      const beforeText = text.substring(0, match.offset)
      const errorText = text.substring(match.offset, match.offset + match.length)

      // Clear mirror and rebuild with spans for precise measurement
      mirror.innerHTML = ''

      // For multi-line handling, split by newlines
      const lines = beforeText.split('\n')
      const currentLine = lines.length - 1
      const lineText = lines[currentLine]

      // Create spans to measure text positions
      const preSpan = document.createElement('span')
      preSpan.textContent = lineText
      mirror.appendChild(preSpan)

      const errorSpan = document.createElement('span')
      errorSpan.textContent = errorText
      mirror.appendChild(errorSpan)

      // Get measurements using getBoundingClientRect for accuracy
      const preRect = preSpan.getBoundingClientRect()
      const errorRect = errorSpan.getBoundingClientRect()
      const mirrorRect = mirror.getBoundingClientRect()

      // Calculate x position: distance from mirror left to error span start
      const x = errorRect.left - mirrorRect.left - scrollLeft
      const width = errorRect.width || errorSpan.offsetWidth || 10

      // Calculate y position based on line number
      const y = paddingTop + borderTop + currentLine * lineHeight - scrollTop

      positions.push({
        x: Math.max(0, x),
        y: y,
        width: Math.max(width, 10),
        height: lineHeight,
      })
    })

    document.body.removeChild(mirror)
    return positions
  }

  private calculateContentEditablePositions(matches: LanguageToolMatch[], text: string): TextPosition[] {
    const positions: TextPosition[] = []
    const element = this.element
    const elementRect = element.getBoundingClientRect()

    // Use the actual text content for matching (textContent is more reliable than innerText for DOM traversal)
    const actualText = element.textContent || ''

    matches.forEach(match => {
      try {
        // Find the error text in the actual DOM text content
        const errorText = text.substring(match.offset, match.offset + match.length)

        // Search for this error text near the expected position in actualText
        // Allow some flexibility in case innerText and textContent differ slightly
        let searchStart = Math.max(0, match.offset - 50)
        let searchEnd = Math.min(actualText.length, match.offset + match.length + 50)
        let searchRegion = actualText.substring(searchStart, searchEnd)

        let foundIndex = searchRegion.indexOf(errorText)
        if (foundIndex === -1) {
          // Try case-insensitive search
          foundIndex = searchRegion.toLowerCase().indexOf(errorText.toLowerCase())
        }

        if (foundIndex !== -1) {
          const actualOffset = searchStart + foundIndex

          const range = document.createRange()
          const textNode = this.findTextNodeByOffset(element, actualOffset, match.length)

          if (textNode) {
            range.setStart(textNode.node, textNode.offset)
            const endOffset = Math.min(textNode.offset + match.length, textNode.node.textContent?.length || 0)
            range.setEnd(textNode.node, endOffset)

            const rects = range.getClientRects()
            if (rects.length > 0) {
              const rect = rects[0]
              positions.push({
                x: rect.left - elementRect.left + element.scrollLeft,
                y: rect.top - elementRect.top + element.scrollTop,
                width: Math.max(rect.width, 10),
                height: rect.height,
              })
            }
          }
        }
      } catch (e) {
        console.log('[AutoCorrect] Position calculation error:', e)
      }
    })

    return positions
  }

  private findTextNodeByOffset(element: Element, offset: number, length: number): { node: Text; offset: number } | null {
    const walker = document.createTreeWalker(element, NodeFilter.SHOW_TEXT)
    let currentOffset = 0

    while (walker.nextNode()) {
      const node = walker.currentNode as Text
      const nodeText = node.textContent || ''
      const nodeLength = nodeText.length

      if (currentOffset + nodeLength > offset) {
        const nodeOffset = offset - currentOffset

        // Check if the match spans multiple nodes
        if (nodeOffset + length <= nodeLength) {
          return { node, offset: nodeOffset }
        } else {
          // Match spans multiple nodes, just return start position
          return { node, offset: nodeOffset }
        }
      }

      currentOffset += nodeLength
    }

    return null
  }

  destroy(): void {
    this.hideTooltip()
    if (this.resizeObserver) {
      this.resizeObserver.disconnect()
    }
    if (this.overlay) {
      this.overlay.remove()
    }
  }
}
