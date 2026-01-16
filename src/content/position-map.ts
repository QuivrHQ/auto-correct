/**
 * Position mapping utility for contenteditable elements
 *
 * This module handles the critical task of mapping between:
 * - innerText offsets (what LanguageTool returns)
 * - DOM positions (text node + offset)
 *
 * The challenge is that innerText adds implicit newlines (\n) for:
 * - <br> elements
 * - Block element boundaries (div, p, etc.)
 * - CSS display:block elements
 *
 * We must accurately track these to map offsets correctly.
 */

export interface DOMPosition {
  node: Text
  offset: number
}

export interface TextNodeInfo {
  node: Text
  start: number  // Start offset in plaintext
  end: number    // End offset in plaintext
}

interface PositionMapEntry {
  node: Text
  nodeOffset: number  // Offset within the text node
}

/**
 * Build a complete position map by comparing DOM text nodes against innerText.
 *
 * This approach is more reliable than trying to predict where innerText inserts
 * newlines. Instead, we:
 * 1. Get the actual innerText (which has all newlines already)
 * 2. Walk text nodes in document order (same order innerText uses)
 * 3. Compare characters to detect where innerText inserted virtual newlines
 *
 * This automatically handles all edge cases because we match against the
 * actual innerText output rather than trying to replicate the algorithm.
 */
function buildCompletePositionMap(element: HTMLElement): Map<number, PositionMapEntry> {
  const map = new Map<number, PositionMapEntry>()
  const innerText = element.innerText || ''

  if (innerText.length === 0) {
    return map
  }

  // Use TreeWalker to get all text nodes in document order
  // This is the same order that innerText concatenates them
  const walker = document.createTreeWalker(
    element,
    NodeFilter.SHOW_TEXT,
    {
      acceptNode: (node) => {
        // Skip text nodes in hidden elements
        const parent = node.parentElement
        if (parent) {
          const tagName = parent.tagName.toUpperCase()
          if (tagName === 'SCRIPT' || tagName === 'STYLE' ||
              tagName === 'NOSCRIPT' || tagName === 'TEMPLATE') {
            return NodeFilter.FILTER_REJECT
          }
          try {
            const style = window.getComputedStyle(parent)
            if (style.display === 'none' || style.visibility === 'hidden') {
              return NodeFilter.FILTER_REJECT
            }
          } catch {
            // Ignore style errors
          }
        }
        return NodeFilter.FILTER_ACCEPT
      }
    }
  )

  let innerTextIndex = 0
  let node: Text | null

  while ((node = walker.nextNode() as Text | null)) {
    const nodeText = node.textContent || ''

    for (let i = 0; i < nodeText.length; i++) {
      // Skip any newlines in innerText that don't correspond to this text node
      // These are "virtual" newlines inserted by innerText for block boundaries
      while (innerTextIndex < innerText.length &&
             innerText[innerTextIndex] === '\n' &&
             nodeText[i] !== '\n') {
        innerTextIndex++ // Skip the virtual newline (it maps to no DOM position)
      }

      // Safety check: make sure characters match
      if (innerTextIndex < innerText.length) {
        // Map this character position
        map.set(innerTextIndex, { node, nodeOffset: i })
        innerTextIndex++
      }
    }
  }

  return map
}

/**
 * Find the text node and offset for a given plaintext offset
 */
export function findPositionInDOM(
  element: HTMLElement,
  targetOffset: number
): DOMPosition | null {
  const innerText = element.innerText || ''

  // Validate offset
  if (targetOffset < 0 || targetOffset > innerText.length) {
    return null
  }

  // Build the position map
  const map = buildCompletePositionMap(element)

  // Look up the target offset
  const entry = map.get(targetOffset)
  if (entry) {
    return {
      node: entry.node,
      offset: entry.nodeOffset
    }
  }

  // If exact offset not found, it might be at a newline position
  // Find the closest preceding position
  let closestOffset = -1
  for (const offset of map.keys()) {
    if (offset <= targetOffset && offset > closestOffset) {
      closestOffset = offset
    }
  }

  if (closestOffset >= 0) {
    const entry = map.get(closestOffset)!
    const diff = targetOffset - closestOffset
    // Return position at end of the text node + any offset difference
    const nodeLength = entry.node.textContent?.length || 0
    return {
      node: entry.node,
      offset: Math.min(entry.nodeOffset + diff, nodeLength)
    }
  }

  return null
}

/**
 * Find DOM range for a given plaintext offset and length
 */
export function findRangeInDOM(
  element: HTMLElement,
  offset: number,
  length: number
): Range | null {
  const startPos = findPositionInDOM(element, offset)
  if (!startPos) {
    console.warn('[AutoCorrect] Could not find start position for offset:', offset)
    return null
  }

  const endPos = findPositionInDOM(element, offset + length)
  if (!endPos) {
    // Try end of selection (last character position)
    const endPosAlt = findPositionInDOM(element, offset + length - 1)
    if (!endPosAlt) {
      console.warn('[AutoCorrect] Could not find end position for offset:', offset + length)
      return null
    }
    // Adjust to be after the last character
    const range = document.createRange()
    range.setStart(startPos.node, startPos.offset)
    range.setEnd(endPosAlt.node, Math.min(endPosAlt.offset + 1, endPosAlt.node.length))
    return range
  }

  const range = document.createRange()
  range.setStart(startPos.node, startPos.offset)
  range.setEnd(endPos.node, endPos.offset)
  return range
}

// Legacy exports for compatibility
export function buildPositionMap(element: HTMLElement): Map<number, DOMPosition> {
  const map = new Map<number, DOMPosition>()
  const innerText = element.innerText || ''

  for (let i = 0; i < innerText.length; i++) {
    const pos = findPositionInDOM(element, i)
    if (pos) {
      map.set(i, pos)
    }
  }

  return map
}

export function getPositionFromMap(
  map: Map<number, DOMPosition>,
  offset: number
): DOMPosition | null {
  return map.get(offset) || null
}
