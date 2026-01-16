import { describe, it, expect, beforeEach, afterEach } from 'vitest'
import {
  findPositionInDOM,
  findRangeInDOM,
  buildPositionMap,
  getPositionFromMap,
  type DOMPosition
} from '../../src/content/position-map'

describe('position-map', () => {
  let container: HTMLDivElement

  beforeEach(() => {
    container = document.createElement('div')
    container.setAttribute('contenteditable', 'true')
    document.body.appendChild(container)
  })

  afterEach(() => {
    document.body.removeChild(container)
  })

  describe('findPositionInDOM', () => {
    describe('single-line text', () => {
      it('finds position at start of text', () => {
        container.innerHTML = 'Hello World'
        const pos = findPositionInDOM(container, 0)

        expect(pos).not.toBeNull()
        expect(pos!.node.textContent).toBe('Hello World')
        expect(pos!.offset).toBe(0)
      })

      it('finds position in middle of text', () => {
        container.innerHTML = 'Hello World'
        const pos = findPositionInDOM(container, 6)

        expect(pos).not.toBeNull()
        expect(pos!.offset).toBe(6) // 'W' in World
      })

      it('finds position at end of text', () => {
        container.innerHTML = 'Hello World'
        const pos = findPositionInDOM(container, 10)

        expect(pos).not.toBeNull()
        expect(pos!.offset).toBe(10) // 'd' in World
      })

      it('returns null for offset beyond text length', () => {
        container.innerHTML = 'Hello'
        const pos = findPositionInDOM(container, 100)

        expect(pos).toBeNull()
      })

      it('returns null for negative offset', () => {
        container.innerHTML = 'Hello'
        const pos = findPositionInDOM(container, -1)

        expect(pos).toBeNull()
      })
    })

    describe('multi-line with <br> elements', () => {
      it('handles single <br> between text', () => {
        container.innerHTML = 'Line 1<br>Line 2'
        const innerText = container.innerText

        // Note: happy-dom may not insert \n for <br>, test the position finding
        // In real browser, innerText would be "Line 1\nLine 2"
        // Find 'Line 2' regardless of newline handling
        const line2Start = innerText.indexOf('Line 2')
        const pos = findPositionInDOM(container, line2Start)

        expect(pos).not.toBeNull()
        expect(pos!.node.textContent).toBe('Line 2')
        expect(pos!.offset).toBe(0)
      })

      it('handles multiple <br> elements', () => {
        container.innerHTML = 'A<br>B<br>C'
        const innerText = container.innerText

        // Find 'C' position - test position mapping works regardless of newline handling
        const cIndex = innerText.indexOf('C')
        const pos = findPositionInDOM(container, cIndex)

        expect(pos).not.toBeNull()
        expect(pos!.node.textContent).toBe('C')
        expect(pos!.offset).toBe(0)
      })

      it('handles consecutive <br> elements', () => {
        container.innerHTML = 'A<br><br>B'
        const innerText = container.innerText

        // Find 'B' position
        const bIndex = innerText.indexOf('B')
        const pos = findPositionInDOM(container, bIndex)

        expect(pos).not.toBeNull()
        expect(pos!.node.textContent).toBe('B')
        expect(pos!.offset).toBe(0)
      })
    })

    describe('multi-line with block elements', () => {
      it('handles text in nested divs', () => {
        container.innerHTML = '<div>Line 1</div><div>Line 2</div>'
        const innerText = container.innerText

        // innerText adds newlines between block elements
        expect(innerText).toContain('Line 1')
        expect(innerText).toContain('Line 2')

        // Find 'Line 2' position
        const line2Start = innerText.indexOf('Line 2')
        const pos = findPositionInDOM(container, line2Start)

        expect(pos).not.toBeNull()
        expect(pos!.node.textContent).toBe('Line 2')
        expect(pos!.offset).toBe(0)
      })

      it('handles text in nested paragraphs', () => {
        container.innerHTML = '<p>Para 1</p><p>Para 2</p>'
        const innerText = container.innerText

        // Find 'Para 2' position
        const para2Start = innerText.indexOf('Para 2')
        const pos = findPositionInDOM(container, para2Start)

        expect(pos).not.toBeNull()
        expect(pos!.node.textContent).toBe('Para 2')
        expect(pos!.offset).toBe(0)
      })

      it('handles mixed block elements', () => {
        container.innerHTML = '<div>First</div><p>Second</p><div>Third</div>'
        const innerText = container.innerText

        // Find 'Third' position
        const thirdStart = innerText.indexOf('Third')
        const pos = findPositionInDOM(container, thirdStart)

        expect(pos).not.toBeNull()
        expect(pos!.node.textContent).toBe('Third')
        expect(pos!.offset).toBe(0)
      })

      it('handles deeply nested elements', () => {
        container.innerHTML = '<div><div><div>Deep text</div></div></div>'
        const innerText = container.innerText

        const pos = findPositionInDOM(container, 0)

        expect(pos).not.toBeNull()
        expect(pos!.node.textContent).toBe('Deep text')
        expect(pos!.offset).toBe(0)
      })
    })

    describe('complex structures (Zendesk-like)', () => {
      it('handles div with br and text nodes', () => {
        // This mimics Zendesk's contenteditable structure
        container.innerHTML = '<div>First line<br>Second line<br>Third line</div>'
        const innerText = container.innerText

        // Find position in second line
        const secondLineStart = innerText.indexOf('Second')
        const pos = findPositionInDOM(container, secondLineStart)

        expect(pos).not.toBeNull()
        expect(pos!.node.textContent).toBe('Second line')
        expect(pos!.offset).toBe(0)

        // Find position in third line
        const thirdLineStart = innerText.indexOf('Third')
        const pos3 = findPositionInDOM(container, thirdLineStart)

        expect(pos3).not.toBeNull()
        expect(pos3!.node.textContent).toBe('Third line')
        expect(pos3!.offset).toBe(0)
      })

      it('handles multiple divs each with content', () => {
        container.innerHTML = `<div>Hello there</div><div>How are you</div><div>I am fine</div>`
        const innerText = container.innerText

        // Find 'How' position
        const howStart = innerText.indexOf('How')
        const pos = findPositionInDOM(container, howStart)

        expect(pos).not.toBeNull()
        expect(pos!.node.textContent?.trim()).toBe('How are you')
        // Offset may vary based on whitespace handling
        expect(pos!.offset).toBeGreaterThanOrEqual(0)
      })

      it('correctly maps offset within second line', () => {
        container.innerHTML = '<div>First<br>Second word here</div>'
        const innerText = container.innerText

        // Find 'word' position (inside second line)
        const wordStart = innerText.indexOf('word')
        const pos = findPositionInDOM(container, wordStart)

        expect(pos).not.toBeNull()
        expect(pos!.node.textContent).toBe('Second word here')
        // 'word' starts at position 7 within "Second word here"
        expect(pos!.offset).toBe(7)
      })

      it('handles error word replacement scenario', () => {
        // Simulate: "Bonjour\nJ'ai un erreures ici"
        container.innerHTML = '<div>Bonjour<br>J\'ai un erreures ici</div>'
        const innerText = container.innerText

        // Find 'erreures' position
        const errorStart = innerText.indexOf('erreures')
        const errorLength = 'erreures'.length

        const startPos = findPositionInDOM(container, errorStart)
        const endPos = findPositionInDOM(container, errorStart + errorLength)

        expect(startPos).not.toBeNull()
        expect(endPos).not.toBeNull()

        // Verify the text at that position
        const textNode = startPos!.node
        const extractedText = textNode.textContent!.substring(startPos!.offset, startPos!.offset + errorLength)
        expect(extractedText).toBe('erreures')
      })
    })

    describe('hidden and special elements', () => {
      it('skips script tags', () => {
        container.innerHTML = 'Visible<script>hidden script</script> text'
        const innerText = container.innerText

        // Script content should not appear in innerText
        expect(innerText).not.toContain('hidden script')

        const pos = findPositionInDOM(container, 0)
        expect(pos).not.toBeNull()
        expect(pos!.node.textContent).toBe('Visible')
      })

      it('skips style tags', () => {
        container.innerHTML = 'Visible<style>.class { color: red; }</style> text'
        const innerText = container.innerText

        expect(innerText).not.toContain('color')
      })

      it('handles empty container', () => {
        container.innerHTML = ''
        const pos = findPositionInDOM(container, 0)

        expect(pos).toBeNull()
      })

      it('handles container with only whitespace', () => {
        container.innerHTML = '   '
        const pos = findPositionInDOM(container, 0)

        expect(pos).not.toBeNull()
      })
    })

    describe('inline elements', () => {
      it('handles span elements', () => {
        container.innerHTML = 'Hello <span>World</span>!'
        const innerText = container.innerText

        const worldStart = innerText.indexOf('World')
        const pos = findPositionInDOM(container, worldStart)

        expect(pos).not.toBeNull()
        expect(pos!.node.textContent).toBe('World')
        expect(pos!.offset).toBe(0)
      })

      it('handles bold and italic', () => {
        container.innerHTML = 'Normal <b>bold</b> <i>italic</i> text'
        const innerText = container.innerText

        const boldStart = innerText.indexOf('bold')
        const pos = findPositionInDOM(container, boldStart)

        expect(pos).not.toBeNull()
        expect(pos!.node.textContent).toBe('bold')
      })

      it('handles links', () => {
        container.innerHTML = 'Click <a href="#">here</a> please'
        const innerText = container.innerText

        const hereStart = innerText.indexOf('here')
        const pos = findPositionInDOM(container, hereStart)

        expect(pos).not.toBeNull()
        expect(pos!.node.textContent).toBe('here')
      })
    })
  })

  describe('findRangeInDOM', () => {
    it('creates range for single word', () => {
      container.innerHTML = 'Hello World'
      const range = findRangeInDOM(container, 0, 5) // "Hello"

      expect(range).not.toBeNull()
      expect(range!.toString()).toBe('Hello')
    })

    it('creates range for word in middle', () => {
      container.innerHTML = 'Hello World Test'
      const range = findRangeInDOM(container, 6, 5) // "World"

      expect(range).not.toBeNull()
      expect(range!.toString()).toBe('World')
    })

    it('creates range spanning multiple text nodes', () => {
      container.innerHTML = 'Hello <b>World</b>'
      // Range for "o Wo" spanning from "Hello" to "World"
      const range = findRangeInDOM(container, 4, 4)

      expect(range).not.toBeNull()
      expect(range!.toString()).toBe('o Wo')
    })

    it('creates range on second line', () => {
      container.innerHTML = '<div>Line 1<br>erreures here</div>'
      const innerText = container.innerText

      const errorStart = innerText.indexOf('erreures')
      const range = findRangeInDOM(container, errorStart, 8)

      expect(range).not.toBeNull()
      expect(range!.toString()).toBe('erreures')
    })

    it('creates range on third line', () => {
      container.innerHTML = '<div>Line 1<br>Line 2<br>erreures here</div>'
      const innerText = container.innerText

      const errorStart = innerText.indexOf('erreures')
      const range = findRangeInDOM(container, errorStart, 8)

      expect(range).not.toBeNull()
      expect(range!.toString()).toBe('erreures')
    })

    it('returns null for invalid offset', () => {
      container.innerHTML = 'Short'
      const range = findRangeInDOM(container, 100, 5)

      expect(range).toBeNull()
    })

    it('handles range at very end of text', () => {
      container.innerHTML = 'Test'
      const range = findRangeInDOM(container, 0, 4)

      expect(range).not.toBeNull()
      expect(range!.toString()).toBe('Test')
    })
  })

  describe('buildPositionMap', () => {
    it('builds complete map for simple text', () => {
      container.innerHTML = 'Hello'
      const map = buildPositionMap(container)

      expect(map.size).toBe(5)

      for (let i = 0; i < 5; i++) {
        const pos = map.get(i)
        expect(pos).not.toBeUndefined()
        expect(pos!.offset).toBe(i)
      }
    })

    it('builds map for multi-line content', () => {
      container.innerHTML = 'A<br>B'
      const innerText = container.innerText
      const map = buildPositionMap(container)

      // Should have entries for A, newline position (maybe), and B
      expect(map.size).toBeGreaterThanOrEqual(2)

      // Position 0 should be 'A'
      const posA = map.get(0)
      expect(posA).not.toBeUndefined()
      expect(posA!.node.textContent).toBe('A')

      // Find position for 'B'
      const bIndex = innerText.indexOf('B')
      const posB = map.get(bIndex)
      expect(posB).not.toBeUndefined()
      expect(posB!.node.textContent).toBe('B')
    })
  })

  describe('getPositionFromMap', () => {
    it('retrieves position from map', () => {
      container.innerHTML = 'Hello'
      const map = buildPositionMap(container)

      const pos = getPositionFromMap(map, 2)
      expect(pos).not.toBeNull()
      expect(pos!.offset).toBe(2)
    })

    it('returns null for missing offset', () => {
      container.innerHTML = 'Hello'
      const map = buildPositionMap(container)

      const pos = getPositionFromMap(map, 100)
      expect(pos).toBeNull()
    })
  })

  describe('regression tests', () => {
    it('correctly positions replacement on line 2 (original bug)', () => {
      // This is the exact scenario that was failing in Zendesk
      container.innerHTML = '<div>Première ligne sans erreur<br>Deuxième ligne avec erreures</div>'
      const innerText = container.innerText

      // Find the error word
      const errorStart = innerText.indexOf('erreures')
      expect(errorStart).toBeGreaterThan(0)

      const range = findRangeInDOM(container, errorStart, 8)
      expect(range).not.toBeNull()
      expect(range!.toString()).toBe('erreures')
    })

    it('handles Zendesk-style divs with br elements', () => {
      container.innerHTML = `<div>Bonjour,<br><br>Je vous écris pour signaler un problème.<br>Il y a des erreures dans le texte.</div>`
      const innerText = container.innerText

      const errorStart = innerText.indexOf('erreures')
      const range = findRangeInDOM(container, errorStart, 8)

      expect(range).not.toBeNull()
      expect(range!.toString()).toBe('erreures')
    })

    it('handles Gmail-style structure', () => {
      container.innerHTML = `<div dir="ltr"><div>Hello</div><div><br></div><div>This has an erreures</div></div>`
      const innerText = container.innerText

      const errorStart = innerText.indexOf('erreures')
      const range = findRangeInDOM(container, errorStart, 8)

      expect(range).not.toBeNull()
      expect(range!.toString()).toBe('erreures')
    })

    it('handles Notion-style nested blocks', () => {
      container.innerHTML = `<div data-block="1"><div>Block 1 content</div></div><div data-block="2"><div>Block 2 with erreures</div></div>`
      const innerText = container.innerText

      const errorStart = innerText.indexOf('erreures')
      const range = findRangeInDOM(container, errorStart, 8)

      expect(range).not.toBeNull()
      expect(range!.toString()).toBe('erreures')
    })

    it('maintains correct positions after multiple operations', () => {
      container.innerHTML = '<div>Line A<br>Line B<br>Line C with error</div>'

      // Check each line independently
      const innerText = container.innerText

      const lineAPos = findPositionInDOM(container, innerText.indexOf('Line A'))
      expect(lineAPos).not.toBeNull()

      const lineBPos = findPositionInDOM(container, innerText.indexOf('Line B'))
      expect(lineBPos).not.toBeNull()

      const lineCPos = findPositionInDOM(container, innerText.indexOf('Line C'))
      expect(lineCPos).not.toBeNull()

      // Each should be in its own text node
      expect(lineAPos!.node).not.toBe(lineBPos!.node)
      expect(lineBPos!.node).not.toBe(lineCPos!.node)
    })
  })
})
