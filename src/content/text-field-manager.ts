import { Settings, LanguageToolMatch } from '../shared/types'
import { checkText } from './language-tool-client'
import { UnderlineRenderer } from './underline-renderer'
import { findRangeInDOM } from './position-map'
import { addToDictionary } from '../shared/storage'
import type { Message, MatchesResponseMessage, ApplySuggestionMessage } from '../shared/messaging'

interface ManagedField {
  element: HTMLInputElement | HTMLTextAreaElement | HTMLElement
  renderer: UnderlineRenderer
  debounceTimer: number | null
  lastText: string
  currentMatches: LanguageToolMatch[]
}

const DEBOUNCE_MS = 400
const managedFields = new WeakMap<Element, ManagedField>()

let currentSettings: Settings | null = null
let activeField: ManagedField | null = null

export function setSettings(settings: Settings): void {
  currentSettings = settings

  // If disabled, clear all underlines
  if (!settings.enabled) {
    document.querySelectorAll('.autocorrect-overlay').forEach(el => el.remove())
  }

  // Update dictionary for all managed fields
  document.querySelectorAll('input, textarea, [contenteditable]:not([contenteditable="false"])').forEach(el => {
    const field = managedFields.get(el)
    if (field) {
      field.renderer.setDictionary(settings.personalDictionary || [])
    }
  })
}

function isEditableElement(element: Element): element is HTMLInputElement | HTMLTextAreaElement | HTMLElement {
  if (element instanceof HTMLInputElement) {
    const type = element.type.toLowerCase()
    // Skip password, hidden, and other non-text inputs
    if (!['text', 'search', 'email', 'url', 'tel', ''].includes(type)) {
      return false
    }
    // Skip tiny inputs (likely search boxes, etc.)
    if (element.offsetWidth < 100) {
      return false
    }
    return true
  }
  if (element instanceof HTMLTextAreaElement) {
    return true
  }
  if (element instanceof HTMLElement && element.isContentEditable) {
    // Skip small contenteditable elements (buttons, etc.)
    if (element.offsetWidth < 100 || element.offsetHeight < 30) {
      return false
    }
    // Skip elements with role="button" or similar
    const role = element.getAttribute('role')
    if (role && ['button', 'menuitem', 'option', 'tab'].includes(role)) {
      return false
    }
    return true
  }
  return false
}

function getTextContent(element: HTMLInputElement | HTMLTextAreaElement | HTMLElement): string {
  let text: string
  if (element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement) {
    text = element.value
  } else {
    // Use innerText for contenteditable to preserve line breaks
    // innerText respects <br> and block elements as newlines
    text = (element as HTMLElement).innerText || ''
  }
  // Normalize Unicode to NFC form to ensure consistent character counting
  // This converts decomposed characters (e + combining accent) to precomposed (é)
  // which matches how LanguageTool counts characters
  return text.normalize('NFC')
}

// Get text around cursor position (for large documents, only check nearby text)
function getTextAroundCursor(element: HTMLInputElement | HTMLTextAreaElement | HTMLElement, maxChars: number = 500): { text: string; offset: number } {
  const fullText = getTextContent(element)

  // For small texts, return everything
  if (fullText.length <= maxChars) {
    return { text: fullText, offset: 0 }
  }

  // Get cursor position
  let cursorPos = 0
  if (element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement) {
    cursorPos = element.selectionStart || 0
  } else {
    const selection = window.getSelection()
    if (selection && selection.rangeCount > 0) {
      const range = selection.getRangeAt(0)
      // Calculate offset within the element
      const preRange = document.createRange()
      preRange.selectNodeContents(element)
      preRange.setEnd(range.startContainer, range.startOffset)
      cursorPos = preRange.toString().length
    }
  }

  // Find paragraph boundaries around cursor
  const paragraphStart = fullText.lastIndexOf('\n\n', cursorPos)
  const paragraphEnd = fullText.indexOf('\n\n', cursorPos)

  let start = paragraphStart === -1 ? 0 : paragraphStart + 2
  let end = paragraphEnd === -1 ? fullText.length : paragraphEnd

  // Expand to include more context if paragraph is small
  const halfMax = Math.floor(maxChars / 2)
  if (end - start < maxChars) {
    start = Math.max(0, cursorPos - halfMax)
    end = Math.min(fullText.length, cursorPos + halfMax)
  }

  // Adjust to word boundaries
  while (start > 0 && fullText[start - 1] !== ' ' && fullText[start - 1] !== '\n') {
    start--
  }
  while (end < fullText.length && fullText[end] !== ' ' && fullText[end] !== '\n') {
    end++
  }

  return {
    text: fullText.substring(start, end),
    offset: start
  }
}

function setTextContent(
  element: HTMLInputElement | HTMLTextAreaElement | HTMLElement,
  offset: number,
  length: number,
  replacement: string
): void {
  // Get the full text to verify the match
  const fullText = getTextContent(element)
  const matchedText = fullText.substring(offset, offset + length)

  console.log('[AutoCorrect] setTextContent called:', {
    offset,
    length,
    replacement,
    elementType: element.tagName,
    matchedText: `"${matchedText}"`,
    contextBefore: `"${fullText.substring(Math.max(0, offset - 5), offset)}"`,
    contextAfter: `"${fullText.substring(offset + length, offset + length + 5)}"`
  })

  // Warn if the matched text doesn't look right (potential offset issue)
  if (matchedText.length !== length) {
    console.warn('[AutoCorrect] WARNING: Matched text length mismatch!',
      { expected: length, actual: matchedText.length, matchedText: `"${matchedText}"` })
  }

  if (element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement) {
    const text = element.value
    console.log('[AutoCorrect] Input/Textarea replacement:', { textLength: text.length, beforeOffset: offset })
    const before = text.substring(0, offset)
    const after = text.substring(offset + length)
    element.value = before + replacement + after

    // Move cursor to end of replacement
    const newCursorPos = offset + replacement.length
    element.setSelectionRange(newCursorPos, newCursorPos)

    // Trigger input event to re-analyze
    element.dispatchEvent(new Event('input', { bubbles: true }))
    console.log('[AutoCorrect] Replacement done for input/textarea')
  } else {
    // For contenteditable elements (including CKEditor, etc.)
    console.log('[AutoCorrect] Contenteditable replacement at offset:', offset, 'length:', length)

    // Use findRangeInDOM for reliable DOM position finding (TreeWalker-based like LanguageTool)
    const range = findRangeInDOM(element as HTMLElement, offset, length)
    if (!range) {
      console.warn('[AutoCorrect] Could not create range for replacement')
      return
    }

    // Verify the selection matches expected text
    const rangeText = range.toString()
    console.log('[AutoCorrect] Range created:', {
      rangeText: `"${rangeText}"`,
      expectedText: `"${matchedText}"`,
      matches: rangeText === matchedText
    })

    if (rangeText !== matchedText) {
      console.warn('[AutoCorrect] WARNING: Range text mismatch! Expected:', `"${matchedText}"`, 'Got:', `"${rangeText}"`)
    }

    // Check if this is CKEditor
    const isCKEditor = element.classList.contains('ck-editor__editable') ||
                       element.classList.contains('ck-content')

    if (isCKEditor) {
      // For CKEditor: use clipboard paste simulation
      console.log('[AutoCorrect] CKEditor detected, using paste simulation')

      const selection = window.getSelection()
      if (selection) {
        selection.removeAllRanges()
        selection.addRange(range)

        // Use clipboard API to paste the replacement text
        setTimeout(async () => {
          try {
            // Re-set selection
            selection.removeAllRanges()
            selection.addRange(range)

            // Write to clipboard and trigger paste
            await navigator.clipboard.writeText(replacement)
            console.log('[AutoCorrect] Clipboard written, triggering paste')

            // Create and dispatch paste event
            const pasteEvent = new ClipboardEvent('paste', {
              bubbles: true,
              cancelable: true,
              clipboardData: new DataTransfer()
            })
            pasteEvent.clipboardData?.setData('text/plain', replacement)

            const pasteHandled = element.dispatchEvent(pasteEvent)
            console.log('[AutoCorrect] Paste event dispatched, handled:', pasteHandled)

            // If paste didn't work, try execCommand as fallback
            if (!pasteHandled || pasteEvent.defaultPrevented) {
              document.execCommand('insertText', false, replacement)
            }
          } catch (err) {
            console.error('[AutoCorrect] Paste simulation failed:', err)
            // Fallback to execCommand
            document.execCommand('insertText', false, replacement)
          }
        }, 10)
      }
    } else {
      // For regular contenteditable: focus then use selection + execCommand
      element.focus()

      setTimeout(() => {
        try {
          const selection = window.getSelection()
          if (selection) {
            selection.removeAllRanges()
            selection.addRange(range)

            const selectedText = selection.toString()
            console.log('[AutoCorrect] Selection set:', {
              selectedText: `"${selectedText}"`,
              expectedText: `"${matchedText}"`,
              matches: selectedText === matchedText
            })

            // Use insertText which is supported by modern browsers
            const success = document.execCommand('insertText', false, replacement)

            if (success) {
              console.log('[AutoCorrect] Replacement done via execCommand')
            } else {
              // Fallback: try delete + insertText
              console.log('[AutoCorrect] execCommand failed, trying delete + insertText')
              document.execCommand('delete', false)
              document.execCommand('insertText', false, replacement)
            }
          }
        } catch (err) {
          console.error('[AutoCorrect] Error during replacement:', err)
        }
      }, 10)
    }

    // Trigger input event to notify the editor
    element.dispatchEvent(new InputEvent('input', { bubbles: true, inputType: 'insertText', data: replacement }))
  }
}

function debounce(fn: () => void, ms: number, field: ManagedField): void {
  if (field.debounceTimer !== null) {
    clearTimeout(field.debounceTimer)
  }
  field.debounceTimer = window.setTimeout(() => {
    field.debounceTimer = null
    fn()
  }, ms)
}

async function handleInput(field: ManagedField): Promise<void> {
  if (!currentSettings?.enabled || !currentSettings?.apiUrl) {
    console.log('[AutoCorrect] Disabled or no API URL')
    return
  }

  const fullText = getTextContent(field.element)

  // Don't re-check if text hasn't changed
  if (fullText === field.lastText) {
    console.log('[AutoCorrect] Text unchanged, skipping')
    return
  }
  field.lastText = fullText

  if (fullText.trim().length < 3) {
    field.currentMatches = []
    field.renderer.render([], fullText)
    return
  }

  // For large documents, only check text around cursor
  const { text: textToCheck, offset: textOffset } = getTextAroundCursor(field.element, 500)
  console.log('[AutoCorrect] Checking text:', textToCheck.substring(0, 50), '... (', textToCheck.length, 'of', fullText.length, 'chars, offset:', textOffset, ')')

  console.log('[AutoCorrect] Calling API...')
  const matches = await checkText(textToCheck, currentSettings.language, currentSettings.apiUrl)

  // Adjust match offsets to account for the text offset
  const adjustedMatches = matches.map(match => ({
    ...match,
    offset: match.offset + textOffset
  }))

  console.log('[AutoCorrect] Got', adjustedMatches.length, 'matches')
  field.currentMatches = adjustedMatches
  field.renderer.render(adjustedMatches, fullText)
}

function attachToField(element: HTMLInputElement | HTMLTextAreaElement | HTMLElement): void {
  if (managedFields.has(element)) {
    return
  }

  console.log('[AutoCorrect] Attaching to field:', element.tagName, element.className?.substring?.(0, 50))
  const renderer = new UnderlineRenderer(element)

  const field: ManagedField = {
    element,
    renderer,
    debounceTimer: null,
    lastText: '',
    currentMatches: [],
  }

  // Initialize renderer with callbacks
  renderer.init({
    onReplace: (match: LanguageToolMatch, replacement: string) => {
      setTextContent(element, match.offset, match.length, replacement)
    },
    onIgnore: (match: LanguageToolMatch) => {
      // Add word to personal dictionary
      const text = getTextContent(element)
      const matchedText = text.substring(match.offset, match.offset + match.length)
      addToDictionary(matchedText)
    },
  })

  // Initialize dictionary from current settings
  if (currentSettings?.personalDictionary) {
    renderer.setDictionary(currentSettings.personalDictionary)
  }

  managedFields.set(element, field)

  // Listen for input events
  element.addEventListener('input', () => {
    debounce(() => handleInput(field), DEBOUNCE_MS, field)
  })

  element.addEventListener('focus', () => {
    // Track active field for popup communication
    activeField = field

    // Check on focus if there's existing content
    const text = getTextContent(element)
    if (text.trim().length >= 3) {
      debounce(() => handleInput(field), DEBOUNCE_MS, field)
    }
  })

  element.addEventListener('blur', () => {
    // Keep activeField reference even on blur for popup queries
    // Only clear if another field gains focus
  })

  // Initial check if focused
  if (document.activeElement === element) {
    activeField = field
    // Only check content for the focused element
    const text = getTextContent(element)
    if (text.trim().length >= 3) {
      setTimeout(() => handleInput(field), 500)
    }
  }
  // Don't auto-check unfocused elements - wait for user to focus them
}

function detachFromField(element: Element): void {
  const field = managedFields.get(element)
  if (field) {
    if (field.debounceTimer !== null) {
      clearTimeout(field.debounceTimer)
    }
    field.renderer.destroy()
    managedFields.delete(element)
  }
}

function scanForFields(): void {
  // Find all editable elements
  const inputs = document.querySelectorAll('input[type="text"], input[type="search"], input[type="email"], input[type="url"], input[type="tel"], input:not([type])')
  const textareas = document.querySelectorAll('textarea')
  // Match any contenteditable value (true, "", plaintext-only, etc.)
  const contentEditables = document.querySelectorAll('[contenteditable]:not([contenteditable="false"])')

  console.log('[AutoCorrect] Scan found:', inputs.length, 'inputs,', textareas.length, 'textareas,', contentEditables.length, 'contenteditables')

  inputs.forEach(el => {
    if (isEditableElement(el)) {
      attachToField(el)
    }
  })

  textareas.forEach(el => {
    attachToField(el as HTMLTextAreaElement)
  })

  contentEditables.forEach(el => {
    if (el instanceof HTMLElement) {
      attachToField(el)
    }
  })
}

export function init(): void {
  // Initial scan
  scanForFields()

  // Watch for new elements AND attribute changes (for editors like CKEditor)
  const observer = new MutationObserver((mutations) => {
    mutations.forEach(mutation => {
      // Handle attribute changes (contenteditable added dynamically)
      if (mutation.type === 'attributes' && mutation.attributeName === 'contenteditable') {
        const target = mutation.target as Element
        if (target instanceof HTMLElement && isEditableElement(target)) {
          console.log('[AutoCorrect] Contenteditable attribute changed on:', target.tagName, target.className?.substring?.(0, 50))
          attachToField(target)
        }
      }

      mutation.addedNodes.forEach(node => {
        if (node instanceof Element) {
          if (isEditableElement(node)) {
            attachToField(node)
          }
          // Also check children - use broader selector
          node.querySelectorAll('input, textarea, [contenteditable]:not([contenteditable="false"])').forEach(el => {
            if (isEditableElement(el)) {
              attachToField(el)
            }
          })
        }
      })

      mutation.removedNodes.forEach(node => {
        if (node instanceof Element) {
          detachFromField(node)
          node.querySelectorAll('input, textarea, [contenteditable]').forEach(detachFromField)
        }
      })
    })
  })

  observer.observe(document.body, {
    childList: true,
    subtree: true,
    attributes: true,
    attributeFilter: ['contenteditable'],
  })

  // Also re-scan periodically for editors that load late (like CKEditor)
  setTimeout(scanForFields, 2000)
  setTimeout(scanForFields, 5000)

  // Listen for messages from popup
  setupMessageListener()
}

function setupMessageListener(): void {
  chrome.runtime.onMessage.addListener((message: Message, _sender, sendResponse) => {
    if (message.type === 'GET_MATCHES') {
      const response: MatchesResponseMessage = {
        type: 'MATCHES_RESPONSE',
        matches: activeField?.currentMatches || [],
        textLength: activeField ? getTextContent(activeField.element).length : 0,
        fieldInfo: activeField ? {
          tagName: activeField.element.tagName.toLowerCase(),
          hasContent: getTextContent(activeField.element).trim().length > 0,
        } : null,
      }
      sendResponse(response)
      return true
    }

    if (message.type === 'APPLY_SUGGESTION') {
      const suggestionMessage = message as ApplySuggestionMessage
      if (activeField && activeField.currentMatches[suggestionMessage.matchIndex]) {
        const match = activeField.currentMatches[suggestionMessage.matchIndex]
        setTextContent(activeField.element, match.offset, match.length, suggestionMessage.replacement)
        sendResponse({ type: 'SUGGESTION_APPLIED', success: true })
      } else {
        sendResponse({ type: 'SUGGESTION_APPLIED', success: false })
      }
      return true
    }

    return false
  })
}

export function destroy(): void {
  document.querySelectorAll('input, textarea, [contenteditable="true"]').forEach(detachFromField)
}
