import { LanguageToolMatch } from '../shared/types'
import { buildPositionMap, getPositionFromMap } from './position-map'

interface TextPosition {
  x: number
  y: number
  width: number
  height: number
  matchIndex: number  // Index of the match this position belongs to
}

interface VisibleRange {
  startOffset: number
  endOffset: number
}

interface TooltipCallbacks {
  onReplace: (match: LanguageToolMatch, replacement: string) => void
  onIgnore: (match: LanguageToolMatch) => void
}

// Feature detection for CSS Custom Highlights API
const supportsCustomHighlights = (): boolean => {
  return typeof CSS !== 'undefined' &&
         'highlights' in CSS &&
         typeof (window as unknown as { Highlight?: typeof Highlight }).Highlight !== 'undefined'
}

export class UnderlineRenderer {
  private element: HTMLInputElement | HTMLTextAreaElement | HTMLElement
  private overlay: HTMLDivElement | null = null
  private shadowRoot: ShadowRoot | null = null
  private tooltipContainer: HTMLDivElement | null = null
  private tooltipShadowRoot: ShadowRoot | null = null
  private resizeObserver: ResizeObserver | null = null
  private tooltip: HTMLDivElement | null = null
  private currentMatches: LanguageToolMatch[] = []
  private ignoredMatches: Set<string> = new Set()
  private personalDictionary: Set<string> = new Set()
  private callbacks: TooltipCallbacks | null = null
  private boundHideTooltip: (e: Event) => void
  private useCustomHighlights: boolean = false
  private highlightStyleSheet: CSSStyleSheet | null = null

  constructor(element: HTMLInputElement | HTMLTextAreaElement | HTMLElement) {
    this.element = element
    this.boundHideTooltip = this.handleOutsideClick.bind(this)

    // Only use CSS Custom Highlights for contenteditable elements (not inputs/textareas)
    // Inputs/textareas don't support Range selection properly for highlights
    const isContentEditable = !(element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement)
    this.useCustomHighlights = isContentEditable && supportsCustomHighlights()

    if (this.useCustomHighlights) {
      console.log('[AutoCorrect] Using CSS Custom Highlights API (modern path)')
      this.setupCustomHighlightStyles()
    }
  }

  private setupCustomHighlightStyles(): void {
    // Create and inject CSS for ::highlight() pseudo-elements
    // These styles will apply to text highlighted by the CSS Custom Highlights API
    this.highlightStyleSheet = new CSSStyleSheet()
    this.highlightStyleSheet.replaceSync(`
      ::highlight(autocorrect-spelling) {
        text-decoration: underline wavy #EF4444;
        text-decoration-skip-ink: none;
        text-underline-offset: 2px;
      }
      ::highlight(autocorrect-grammar) {
        text-decoration: underline wavy #F59E0B;
        text-decoration-skip-ink: none;
        text-underline-offset: 2px;
      }
      ::highlight(autocorrect-style) {
        text-decoration: underline wavy #3B82F6;
        text-decoration-skip-ink: none;
        text-underline-offset: 2px;
      }
      @media (prefers-color-scheme: dark) {
        ::highlight(autocorrect-spelling) {
          text-decoration-color: #F87171;
        }
        ::highlight(autocorrect-grammar) {
          text-decoration-color: #FBBF24;
        }
        ::highlight(autocorrect-style) {
          text-decoration-color: #60A5FA;
        }
      }
    `)
    document.adoptedStyleSheets = [...document.adoptedStyleSheets, this.highlightStyleSheet]
  }

  init(callbacks: TooltipCallbacks): void {
    this.callbacks = callbacks
    this.createOverlay()
    this.setupObservers()
  }

  setDictionary(words: string[]): void {
    this.personalDictionary = new Set(words.map(w => w.toLowerCase()))
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
      @import url('https://fonts.googleapis.com/css2?family=DM+Sans:wght@400;500;600&display=swap');

      :host {
        all: initial;
        font-family: 'DM Sans', -apple-system, BlinkMacSystemFont, sans-serif;
        /* Light mode - warm editorial palette */
        --ac-cream: #FAF8F5;
        --ac-paper: #FFFFFF;
        --ac-ink: #1A1612;
        --ac-ink-soft: #4A4540;
        --ac-ink-muted: #8A857D;
        --ac-border: #E8E4DE;
        --ac-border-soft: #F2EFEA;
        --ac-coral: #E85D4C;
        --ac-coral-soft: #FEF2F0;
        --ac-amber: #D4940A;
        --ac-amber-soft: #FDF8EC;
        --ac-indigo: #5B6AD0;
        --ac-indigo-soft: #F3F4FC;
      }

      @media (prefers-color-scheme: dark) {
        :host {
          --ac-cream: #1A1612;
          --ac-paper: #242019;
          --ac-ink: #FAF8F5;
          --ac-ink-soft: #C9C4BC;
          --ac-ink-muted: #7A756D;
          --ac-border: #3A352D;
          --ac-border-soft: #2A2620;
          --ac-coral: #F08070;
          --ac-coral-soft: #2D201E;
          --ac-amber: #E8A820;
          --ac-amber-soft: #2A2418;
          --ac-indigo: #7B8AE0;
          --ac-indigo-soft: #1E2030;
        }
      }

      /* Error underlines - now clickable */
      .error-highlight {
        position: absolute;
        background: transparent;
        cursor: pointer;
        pointer-events: auto;
        border-radius: 2px;
        animation: highlightFadeIn 0.2s ease-out;
      }
      @keyframes highlightFadeIn {
        from { opacity: 0; }
        to { opacity: 1; }
      }
      .error-highlight:hover {
        background: rgba(232, 93, 76, 0.08);
      }
      .error-highlight::after {
        content: '';
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        height: 3px;
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='8' height='4' viewBox='0 0 8 4'%3E%3Cpath d='M0 3 Q2 0 4 3 Q6 6 8 3' stroke='%23E85D4C' fill='none' stroke-width='1.5'/%3E%3C/svg%3E");
        background-repeat: repeat-x;
        background-position: bottom;
        background-size: 8px 4px;
      }
      .error-spelling::after {
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='8' height='4' viewBox='0 0 8 4'%3E%3Cpath d='M0 3 Q2 0 4 3 Q6 6 8 3' stroke='%23E85D4C' fill='none' stroke-width='1.5'/%3E%3C/svg%3E");
      }
      @media (prefers-color-scheme: dark) {
        .error-spelling::after {
          background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='8' height='4' viewBox='0 0 8 4'%3E%3Cpath d='M0 3 Q2 0 4 3 Q6 6 8 3' stroke='%23F08070' fill='none' stroke-width='1.5'/%3E%3C/svg%3E");
        }
      }
      .error-grammar::after {
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='8' height='4' viewBox='0 0 8 4'%3E%3Cpath d='M0 3 Q2 0 4 3 Q6 6 8 3' stroke='%23D4940A' fill='none' stroke-width='1.5'/%3E%3C/svg%3E");
      }
      @media (prefers-color-scheme: dark) {
        .error-grammar::after {
          background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='8' height='4' viewBox='0 0 8 4'%3E%3Cpath d='M0 3 Q2 0 4 3 Q6 6 8 3' stroke='%23E8A820' fill='none' stroke-width='1.5'/%3E%3C/svg%3E");
        }
      }
      .error-grammar:hover {
        background: rgba(212, 148, 10, 0.08);
      }

      /* Tooltip - refined editorial design */
      .tooltip {
        position: fixed;
        background: var(--ac-paper);
        border-radius: 16px;
        box-shadow: 0 8px 32px rgba(26, 22, 18, 0.12), 0 0 0 1px var(--ac-border-soft);
        padding: 0;
        min-width: 260px;
        max-width: 340px;
        z-index: 2147483647;
        animation: tooltipIn 0.2s cubic-bezier(0.22, 1, 0.36, 1);
        overflow: hidden;
        pointer-events: auto;
      }
      @media (prefers-color-scheme: dark) {
        .tooltip {
          box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4), 0 0 0 1px var(--ac-border);
        }
      }
      @keyframes tooltipIn {
        from {
          opacity: 0;
          transform: translateY(-8px) scale(0.96);
        }
        to {
          opacity: 1;
          transform: translateY(0) scale(1);
        }
      }

      .tooltip-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 12px 14px;
        border-bottom: 1px solid var(--ac-border-soft);
      }

      .category-badge {
        display: inline-flex;
        align-items: center;
        gap: 6px;
        padding: 4px 10px;
        border-radius: 20px;
        font-weight: 500;
        font-size: 12px;
      }
      .category-badge.spelling {
        background: var(--ac-coral-soft);
        color: var(--ac-coral);
      }
      .category-badge.grammar {
        background: var(--ac-amber-soft);
        color: var(--ac-amber);
      }
      .category-badge.style {
        background: var(--ac-indigo-soft);
        color: var(--ac-indigo);
      }
      .category-badge .dot {
        width: 6px;
        height: 6px;
        border-radius: 50%;
        background: currentColor;
      }

      .tooltip-close {
        width: 28px;
        height: 28px;
        display: flex;
        align-items: center;
        justify-content: center;
        border: none;
        background: transparent;
        color: var(--ac-ink-muted);
        cursor: pointer;
        border-radius: 8px;
        transition: all 0.15s ease;
      }
      .tooltip-close:hover {
        background: var(--ac-border-soft);
        color: var(--ac-ink-soft);
      }

      .tooltip-body {
        padding: 14px;
      }
      .tooltip-message {
        color: var(--ac-ink-soft);
        font-size: 13px;
        line-height: 1.55;
        margin-bottom: 14px;
      }

      .tooltip-suggestions {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
        margin-bottom: 12px;
      }
      .suggestion-btn {
        padding: 8px 16px;
        background: var(--ac-ink);
        color: var(--ac-paper);
        border: none;
        border-radius: 10px;
        font-size: 13px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.15s ease;
        pointer-events: auto;
      }
      .suggestion-btn:hover {
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(26, 22, 18, 0.15);
      }
      .suggestion-btn:active {
        transform: translateY(0);
      }
      .suggestion-btn.secondary {
        background: transparent;
        color: var(--ac-ink);
        border: 1px solid var(--ac-border);
      }
      .suggestion-btn.secondary:hover {
        background: var(--ac-border-soft);
        box-shadow: none;
        transform: none;
      }

      .tooltip-actions {
        display: flex;
        gap: 8px;
        padding-top: 12px;
        border-top: 1px solid var(--ac-border-soft);
      }
      .action-btn {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 6px;
        padding: 8px 12px;
        background: transparent;
        color: var(--ac-ink-muted);
        border: 1px solid var(--ac-border);
        border-radius: 8px;
        font-size: 12px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.15s ease;
        pointer-events: auto;
      }
      .action-btn:hover {
        background: var(--ac-border-soft);
        color: var(--ac-ink-soft);
        border-color: var(--ac-border);
      }
      .action-btn.dictionary:hover {
        background: var(--ac-indigo-soft);
        color: var(--ac-indigo);
        border-color: var(--ac-indigo-soft);
      }
      .action-btn svg {
        width: 14px;
        height: 14px;
      }
    `
    this.shadowRoot.appendChild(style)

    document.body.appendChild(this.overlay)
    this.updatePosition()

    // Create a separate tooltip container using a custom HTML element
    // This follows LanguageTool's approach: custom elements avoid browser default styles
    // and when appended directly to document.body with position:fixed, they escape
    // any stacking context issues from parent elements
    this.tooltipContainer = document.createElement('autocorrect-tooltip-portal') as HTMLDivElement
    this.tooltipContainer.style.cssText = `
      display: block !important;
      position: fixed !important;
      top: 0 !important;
      left: 0 !important;
      width: 0 !important;
      height: 0 !important;
      overflow: visible !important;
      z-index: 2147483647 !important;
      pointer-events: none !important;
    `
    this.tooltipShadowRoot = this.tooltipContainer.attachShadow({ mode: 'open' })

    // Add tooltip styles to the tooltip shadow root
    const tooltipStyle = document.createElement('style')
    tooltipStyle.textContent = this.getTooltipStyles()
    this.tooltipShadowRoot.appendChild(tooltipStyle)

    document.body.appendChild(this.tooltipContainer)
  }

  private getTooltipStyles(): string {
    return `
      @import url('https://fonts.googleapis.com/css2?family=DM+Sans:wght@400;500;600&display=swap');

      :host {
        all: initial;
        font-family: 'DM Sans', -apple-system, BlinkMacSystemFont, sans-serif;
        /* Light mode - warm editorial palette */
        --ac-cream: #FAF8F5;
        --ac-paper: #FFFFFF;
        --ac-ink: #1A1612;
        --ac-ink-soft: #4A4540;
        --ac-ink-muted: #8A857D;
        --ac-border: #E8E4DE;
        --ac-border-soft: #F2EFEA;
        --ac-coral: #E85D4C;
        --ac-coral-soft: #FEF2F0;
        --ac-amber: #D4940A;
        --ac-amber-soft: #FDF8EC;
        --ac-indigo: #5B6AD0;
        --ac-indigo-soft: #F3F4FC;
      }

      @media (prefers-color-scheme: dark) {
        :host {
          --ac-cream: #1A1612;
          --ac-paper: #242019;
          --ac-ink: #FAF8F5;
          --ac-ink-soft: #C9C4BC;
          --ac-ink-muted: #7A756D;
          --ac-border: #3A352D;
          --ac-border-soft: #2A2620;
          --ac-coral: #F08070;
          --ac-coral-soft: #2D201E;
          --ac-amber: #E8A820;
          --ac-amber-soft: #2A2418;
          --ac-indigo: #7B8AE0;
          --ac-indigo-soft: #1E2030;
        }
      }

      /* Tooltip - refined editorial design */
      .tooltip {
        position: fixed;
        background: var(--ac-paper);
        border-radius: 16px;
        box-shadow: 0 8px 32px rgba(26, 22, 18, 0.12), 0 0 0 1px var(--ac-border-soft);
        padding: 0;
        min-width: 260px;
        max-width: 340px;
        z-index: 2147483647;
        animation: tooltipIn 0.2s cubic-bezier(0.22, 1, 0.36, 1);
        overflow: hidden;
        pointer-events: auto;
      }
      @media (prefers-color-scheme: dark) {
        .tooltip {
          box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4), 0 0 0 1px var(--ac-border);
        }
      }
      @keyframes tooltipIn {
        from {
          opacity: 0;
          transform: translateY(-8px) scale(0.96);
        }
        to {
          opacity: 1;
          transform: translateY(0) scale(1);
        }
      }

      .tooltip-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 12px 14px;
        border-bottom: 1px solid var(--ac-border-soft);
      }

      .category-badge {
        display: inline-flex;
        align-items: center;
        gap: 6px;
        padding: 4px 10px;
        border-radius: 20px;
        font-weight: 500;
        font-size: 12px;
      }
      .category-badge.spelling {
        background: var(--ac-coral-soft);
        color: var(--ac-coral);
      }
      .category-badge.grammar {
        background: var(--ac-amber-soft);
        color: var(--ac-amber);
      }
      .category-badge.style {
        background: var(--ac-indigo-soft);
        color: var(--ac-indigo);
      }
      .category-badge .dot {
        width: 6px;
        height: 6px;
        border-radius: 50%;
        background: currentColor;
      }

      .tooltip-close {
        width: 28px;
        height: 28px;
        display: flex;
        align-items: center;
        justify-content: center;
        border: none;
        background: transparent;
        color: var(--ac-ink-muted);
        cursor: pointer;
        border-radius: 8px;
        transition: all 0.15s ease;
      }
      .tooltip-close:hover {
        background: var(--ac-border-soft);
        color: var(--ac-ink-soft);
      }

      .tooltip-body {
        padding: 14px;
      }
      .tooltip-message {
        color: var(--ac-ink-soft);
        font-size: 13px;
        line-height: 1.55;
        margin-bottom: 14px;
      }

      .tooltip-suggestions {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
        margin-bottom: 12px;
      }
      .suggestion-btn {
        padding: 8px 16px;
        background: var(--ac-ink);
        color: var(--ac-paper);
        border: none;
        border-radius: 10px;
        font-size: 13px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.15s ease;
        pointer-events: auto;
      }
      .suggestion-btn:hover {
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(26, 22, 18, 0.15);
      }
      .suggestion-btn:active {
        transform: translateY(0);
      }
      .suggestion-btn.secondary {
        background: transparent;
        color: var(--ac-ink);
        border: 1px solid var(--ac-border);
      }
      .suggestion-btn.secondary:hover {
        background: var(--ac-border-soft);
        box-shadow: none;
        transform: none;
      }

      .tooltip-actions {
        display: flex;
        gap: 8px;
        padding-top: 12px;
        border-top: 1px solid var(--ac-border-soft);
      }
      .action-btn {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 6px;
        padding: 8px 12px;
        background: transparent;
        color: var(--ac-ink-muted);
        border: 1px solid var(--ac-border);
        border-radius: 8px;
        font-size: 12px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.15s ease;
        pointer-events: auto;
      }
      .action-btn:hover {
        background: var(--ac-border-soft);
        color: var(--ac-ink-soft);
        border-color: var(--ac-border);
      }
      .action-btn.dictionary:hover {
        background: var(--ac-indigo-soft);
        color: var(--ac-indigo);
        border-color: var(--ac-indigo-soft);
      }
      .action-btn svg {
        width: 14px;
        height: 14px;
      }
    `
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
    this.currentMatches = matches
    this.hideTooltip()

    // Filter ignored matches and dictionary words
    const activeMatches = matches.filter(match => {
      const matchKey = `${match.offset}-${match.length}-${match.rule.id}`
      if (this.ignoredMatches.has(matchKey)) return false

      // Filter out words in personal dictionary
      const matchedText = text.substring(match.offset, match.offset + match.length).toLowerCase()
      if (this.personalDictionary.has(matchedText)) return false

      return true
    })

    console.log('[AutoCorrect] Rendering', activeMatches.length, 'matches (useCustomHighlights:', this.useCustomHighlights, ')')

    if (activeMatches.length === 0) {
      this.clearHighlights()
      return
    }

    // Use CSS Custom Highlights for contenteditable if supported
    if (this.useCustomHighlights) {
      this.renderWithCustomHighlights(activeMatches, text)
      return
    }

    // Fallback: overlay-based rendering for inputs/textareas or unsupported browsers
    if (!this.shadowRoot) return

    // Clear existing underlines (keep style)
    const existingUnderlines = this.shadowRoot.querySelectorAll('.error-highlight')
    existingUnderlines.forEach(el => el.remove())

    // For optimization: only render errors in visible range + buffer
    const visibleRange = this.getVisibleTextRange(text)
    const visibleMatches = activeMatches.filter(match => {
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
    positions.forEach((pos) => {
      const match = visibleMatches[pos.matchIndex]
      if (!match) return  // Safety check

      // Store the index in the full matches array for reference
      const originalIndex = this.currentMatches.indexOf(match)

      // Skip if position is outside visible area
      if (pos.y < -50 || pos.y > this.element.clientHeight + 50) {
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
    console.log('[AutoCorrect] Rendered', renderedCount, 'underlines (multi-rect support enabled)')
  }

  private renderWithCustomHighlights(matches: LanguageToolMatch[], text: string): void {
    // Clear existing highlights first
    this.clearHighlights()

    const element = this.element as HTMLElement
    const positionMap = buildPositionMap(element)

    // Create separate Highlight objects for each error type
    const spellingRanges: Range[] = []
    const grammarRanges: Range[] = []
    const styleRanges: Range[] = []

    matches.forEach(match => {
      try {
        const startPos = getPositionFromMap(positionMap, match.offset)
        if (!startPos) return

        const endPos = getPositionFromMap(positionMap, match.offset + match.length - 1)
        if (!endPos) return

        const range = document.createRange()
        range.setStart(startPos.node, startPos.offset)
        range.setEnd(endPos.node, Math.min(endPos.offset + 1, endPos.node.length))

        // Categorize by error type
        const category = match.rule.category.id.toUpperCase()
        if (category.includes('TYPO') || category.includes('SPELL')) {
          spellingRanges.push(range)
        } else if (category.includes('GRAMMAR')) {
          grammarRanges.push(range)
        } else {
          styleRanges.push(range)
        }
      } catch (e) {
        console.log('[AutoCorrect] CSS Highlights range error:', e)
      }
    })

    // Register highlights with the CSS Custom Highlights API
    const cssHighlights = (CSS as unknown as { highlights: Map<string, Highlight> }).highlights
    const HighlightClass = (window as unknown as { Highlight: typeof Highlight }).Highlight

    if (spellingRanges.length > 0) {
      cssHighlights.set('autocorrect-spelling', new HighlightClass(...spellingRanges))
    }
    if (grammarRanges.length > 0) {
      cssHighlights.set('autocorrect-grammar', new HighlightClass(...grammarRanges))
    }
    if (styleRanges.length > 0) {
      cssHighlights.set('autocorrect-style', new HighlightClass(...styleRanges))
    }

    console.log('[AutoCorrect] CSS Custom Highlights rendered:', {
      spelling: spellingRanges.length,
      grammar: grammarRanges.length,
      style: styleRanges.length
    })

    // Still need overlay for click handlers (tooltip interaction)
    // Render invisible click targets on the overlay
    this.renderClickTargets(matches, text)
  }

  private renderClickTargets(matches: LanguageToolMatch[], text: string): void {
    if (!this.shadowRoot) return

    // Clear existing click targets
    const existingTargets = this.shadowRoot.querySelectorAll('.click-target')
    existingTargets.forEach(el => el.remove())

    const positions = this.calculatePositions(matches, text)

    positions.forEach((pos) => {
      const match = matches[pos.matchIndex]
      if (!match) return

      const originalIndex = this.currentMatches.indexOf(match)

      // Create an invisible click target for tooltip interaction
      const target = document.createElement('span')
      target.className = 'click-target'
      target.style.cssText = `
        position: absolute;
        left: ${pos.x}px;
        top: ${pos.y}px;
        width: ${pos.width}px;
        height: ${pos.height}px;
        cursor: pointer;
        pointer-events: auto;
        background: transparent;
      `
      target.dataset.matchIndex = String(originalIndex)

      target.addEventListener('click', (e) => {
        e.preventDefault()
        e.stopPropagation()
        this.showTooltip(match, e.clientX, e.clientY, pos)
      })

      this.shadowRoot!.appendChild(target)
    })
  }

  private clearHighlights(): void {
    // Clear CSS Custom Highlights
    if (this.useCustomHighlights) {
      const cssHighlights = (CSS as unknown as { highlights: Map<string, Highlight> }).highlights
      cssHighlights.delete('autocorrect-spelling')
      cssHighlights.delete('autocorrect-grammar')
      cssHighlights.delete('autocorrect-style')
    }

    // Clear overlay underlines
    if (this.shadowRoot) {
      const existingUnderlines = this.shadowRoot.querySelectorAll('.error-highlight, .click-target')
      existingUnderlines.forEach(el => el.remove())
    }
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

    if (!this.tooltipShadowRoot) return

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

    // Get the matched text for dictionary functionality
    const matchedText = this.getElementText().substring(match.offset, match.offset + match.length)

    // Build tooltip HTML with new editorial design
    this.tooltip.innerHTML = `
      <div class="tooltip-header">
        <div class="category-badge ${categoryInfo.class}">
          <span class="dot"></span>
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
          ${match.replacements.slice(0, 3).map((r, i) =>
            `<button class="suggestion-btn${i > 0 ? ' secondary' : ''}" data-replacement="${this.escapeHtml(r.value)}">${this.escapeHtml(r.value)}</button>`
          ).join('')}
        </div>
        <div class="tooltip-actions">
          <button class="action-btn ignore-btn">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M18.36 6.64A9 9 0 1 1 5.64 6.64 9 9 0 0 1 18.36 6.64Z"/>
              <path d="M6 6l12 12"/>
            </svg>
            Ignorer
          </button>
          <button class="action-btn dictionary dictionary-btn">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
            </svg>
            Dictionnaire
          </button>
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

    this.tooltip.querySelector('.dictionary-btn')?.addEventListener('click', () => {
      // Add to personal dictionary (the callback will persist it)
      this.personalDictionary.add(matchedText.toLowerCase())
      this.callbacks?.onIgnore(match)
      this.hideTooltip()
      // Re-render to hide errors for this word
      this.render(this.currentMatches, this.getElementText())
    })

    this.tooltipShadowRoot.appendChild(this.tooltip)

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
    return (this.element as HTMLElement).innerText || ''
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
    const paddingTop = parseFloat(styles.paddingTop) || 0
    const borderTop = parseFloat(styles.borderTopWidth) || 0
    const lineHeight = parseFloat(styles.lineHeight) || parseFloat(styles.fontSize) * 1.2

    const scrollLeft = element.scrollLeft || 0
    const scrollTop = element.scrollTop || 0

    matches.forEach((match, matchIndex) => {
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
        matchIndex,
      })
    })

    document.body.removeChild(mirror)
    return positions
  }

  private calculateContentEditablePositions(matches: LanguageToolMatch[], text: string): TextPosition[] {
    const positions: TextPosition[] = []
    const element = this.element
    const elementRect = element.getBoundingClientRect()

    // Build position map once for all matches
    // This correctly maps innerText offsets (which include virtual \n for <br> and blocks)
    // to actual DOM text node positions
    const positionMap = buildPositionMap(element as HTMLElement)

    matches.forEach((match, matchIndex) => {
      try {
        const startPos = getPositionFromMap(positionMap, match.offset)
        if (!startPos) {
          console.log('[AutoCorrect] Could not find start position for offset:', match.offset)
          return
        }

        const endPos = getPositionFromMap(positionMap, match.offset + match.length - 1)
        if (!endPos) {
          console.log('[AutoCorrect] Could not find end position for offset:', match.offset + match.length - 1)
          return
        }

        const range = document.createRange()
        range.setStart(startPos.node, startPos.offset)
        // End offset is +1 because setEnd is exclusive
        range.setEnd(endPos.node, Math.min(endPos.offset + 1, endPos.node.length))

        const rects = range.getClientRects()
        // Multi-rect support: loop through all rects to handle word-wrapped errors
        // Each rect represents a separate line fragment of the same error
        Array.from(rects).forEach(rect => {
          positions.push({
            x: rect.left - elementRect.left + element.scrollLeft,
            y: rect.top - elementRect.top + element.scrollTop,
            width: Math.max(rect.width, 10),
            height: rect.height,
            matchIndex,
          })
        })
      } catch (e) {
        console.log('[AutoCorrect] Position calculation error:', e)
      }
    })

    return positions
  }

  destroy(): void {
    this.hideTooltip()
    this.clearHighlights()

    if (this.resizeObserver) {
      this.resizeObserver.disconnect()
    }
    if (this.overlay) {
      this.overlay.remove()
    }
    if (this.tooltipContainer) {
      this.tooltipContainer.remove()
      this.tooltipContainer = null
      this.tooltipShadowRoot = null
    }

    // Remove the CSS Custom Highlights stylesheet
    if (this.highlightStyleSheet) {
      const index = document.adoptedStyleSheets.indexOf(this.highlightStyleSheet)
      if (index !== -1) {
        document.adoptedStyleSheets = document.adoptedStyleSheets.filter((_, i) => i !== index)
      }
      this.highlightStyleSheet = null
    }
  }
}
