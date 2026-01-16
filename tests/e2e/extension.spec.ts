import { test, expect } from './fixtures'

const TEST_PAGE = 'http://localhost:8889/test/test-page.html'
const DEBOUNCE_WAIT = 600 // Wait for debounce + API call

test.describe('AutoCorrect Extension', () => {
  test.beforeEach(async ({ context }) => {
    // Wait a bit for extension to initialize
    await new Promise(r => setTimeout(r, 1000))
  })

  test.describe('Error Detection', () => {
    test('detects spelling errors in input field', async ({ context }) => {
      const page = await context.newPage()
      await page.goto(TEST_PAGE)

      // Find the test input
      const input = page.locator('#test-input')
      await input.clear()
      await input.fill('Je fais des erreures')

      // Wait for debounce and API response
      await page.waitForTimeout(DEBOUNCE_WAIT)

      // Check for underline overlay
      const overlay = page.locator('.autocorrect-overlay')
      await expect(overlay).toBeVisible()

      // Check that underlines are rendered
      const underlines = page.locator('.autocorrect-underline')
      await expect(underlines).toHaveCount(1)
    })

    test('detects spelling errors in textarea', async ({ context }) => {
      const page = await context.newPage()
      await page.goto(TEST_PAGE)

      // Find a textarea
      const textarea = page.locator('textarea').first()
      await textarea.clear()
      await textarea.fill('Ceci est un testttt avec des fotes')

      // Wait for debounce and API response
      await page.waitForTimeout(DEBOUNCE_WAIT)

      // Check for underlines
      const underlines = page.locator('.autocorrect-underline')
      const count = await underlines.count()
      expect(count).toBeGreaterThan(0)
    })

    test('detects errors in contenteditable', async ({ context }) => {
      const page = await context.newPage()
      await page.goto(TEST_PAGE)

      // Find contenteditable element
      const editable = page.locator('[contenteditable="true"]').first()
      await editable.click()
      await editable.fill('Je mange des pomm')

      // Wait for debounce and API response
      await page.waitForTimeout(DEBOUNCE_WAIT)

      // Check for underlines
      const underlines = page.locator('.autocorrect-underline')
      const count = await underlines.count()
      expect(count).toBeGreaterThan(0)
    })
  })

  test.describe('Suggestion Application', () => {
    test('applies suggestion when clicked in input', async ({ context }) => {
      const page = await context.newPage()
      await page.goto(TEST_PAGE)

      // Type text with error
      const input = page.locator('#test-input')
      await input.clear()
      await input.fill('aujourdui')

      // Wait for detection
      await page.waitForTimeout(DEBOUNCE_WAIT)

      // Click on the underline to show tooltip
      const underline = page.locator('.autocorrect-underline').first()
      await underline.click()

      // Wait for tooltip
      await page.waitForTimeout(200)

      // Find and click the suggestion button
      const suggestionBtn = page.locator('.autocorrect-suggestion-btn').first()
      await expect(suggestionBtn).toBeVisible()
      await suggestionBtn.click()

      // Wait for replacement
      await page.waitForTimeout(200)

      // Verify text was replaced
      const value = await input.inputValue()
      expect(value).toBe("aujourd'hui")
    })

    test('applies suggestion when clicked in textarea', async ({ context }) => {
      const page = await context.newPage()
      await page.goto(TEST_PAGE)

      // Type text with error
      const textarea = page.locator('textarea').first()
      await textarea.clear()
      await textarea.fill('Je vais au marche')

      // Wait for detection
      await page.waitForTimeout(DEBOUNCE_WAIT)

      // Click on the underline
      const underline = page.locator('.autocorrect-underline').first()
      await underline.click()

      // Wait for tooltip
      await page.waitForTimeout(200)

      // Click the suggestion
      const suggestionBtn = page.locator('.autocorrect-suggestion-btn').first()
      if (await suggestionBtn.isVisible()) {
        await suggestionBtn.click()

        // Wait for replacement
        await page.waitForTimeout(200)

        // Verify text was replaced
        const value = await textarea.inputValue()
        expect(value).toContain('marché')
      }
    })
  })

  test.describe('UI Behavior', () => {
    test('shows tooltip on underline click', async ({ context }) => {
      const page = await context.newPage()
      await page.goto(TEST_PAGE)

      // Type text with error
      const input = page.locator('#test-input')
      await input.clear()
      await input.fill('erreures')

      // Wait for detection
      await page.waitForTimeout(DEBOUNCE_WAIT)

      // Click on the underline
      const underline = page.locator('.autocorrect-underline').first()
      await underline.click()

      // Check tooltip is visible
      const tooltip = page.locator('.autocorrect-tooltip')
      await expect(tooltip).toBeVisible()
    })

    test('hides tooltip when clicking elsewhere', async ({ context }) => {
      const page = await context.newPage()
      await page.goto(TEST_PAGE)

      // Type text with error
      const input = page.locator('#test-input')
      await input.clear()
      await input.fill('erreures')

      // Wait for detection
      await page.waitForTimeout(DEBOUNCE_WAIT)

      // Click on the underline to show tooltip
      const underline = page.locator('.autocorrect-underline').first()
      await underline.click()

      // Verify tooltip is shown
      const tooltip = page.locator('.autocorrect-tooltip')
      await expect(tooltip).toBeVisible()

      // Click elsewhere
      await page.locator('body').click({ position: { x: 10, y: 10 } })

      // Wait a bit
      await page.waitForTimeout(200)

      // Tooltip should be hidden
      await expect(tooltip).not.toBeVisible()
    })

    test('removes underline after applying suggestion', async ({ context }) => {
      const page = await context.newPage()
      await page.goto(TEST_PAGE)

      // Type text with error
      const input = page.locator('#test-input')
      await input.clear()
      await input.fill('testttt')

      // Wait for detection
      await page.waitForTimeout(DEBOUNCE_WAIT)

      // Get initial underline count
      const underlines = page.locator('.autocorrect-underline')
      const initialCount = await underlines.count()
      expect(initialCount).toBe(1)

      // Click underline and apply suggestion
      await underlines.first().click()
      await page.waitForTimeout(200)

      const suggestionBtn = page.locator('.autocorrect-suggestion-btn').first()
      if (await suggestionBtn.isVisible()) {
        await suggestionBtn.click()

        // Wait for re-analysis
        await page.waitForTimeout(DEBOUNCE_WAIT)

        // Underline should be gone
        const newCount = await underlines.count()
        expect(newCount).toBe(0)
      }
    })
  })

  test.describe('Ignore Functionality', () => {
    test('ignores error when ignore button clicked', async ({ context }) => {
      const page = await context.newPage()
      await page.goto(TEST_PAGE)

      // Type text with error
      const input = page.locator('#test-input')
      await input.clear()
      await input.fill('erreures')

      // Wait for detection
      await page.waitForTimeout(DEBOUNCE_WAIT)

      // Click underline
      const underline = page.locator('.autocorrect-underline').first()
      await underline.click()
      await page.waitForTimeout(200)

      // Click ignore button
      const ignoreBtn = page.locator('.autocorrect-ignore-btn')
      if (await ignoreBtn.isVisible()) {
        await ignoreBtn.click()

        // Wait a bit
        await page.waitForTimeout(200)

        // Underline should be gone but text unchanged
        const underlines = page.locator('.autocorrect-underline')
        const count = await underlines.count()
        expect(count).toBe(0)

        const value = await input.inputValue()
        expect(value).toBe('erreures')
      }
    })
  })

  test.describe('Multi-line Contenteditable Replacement', () => {
    test('replaces error on second line of contenteditable with br tags', async ({ context }) => {
      const page = await context.newPage()
      await page.goto(TEST_PAGE)

      // Find the multi-line contenteditable (Zendesk-style)
      const editable = page.locator('#test-ce-multiline')
      await editable.click()

      // Wait for detection
      await page.waitForTimeout(DEBOUNCE_WAIT)

      // There should be underlines for "erreures" and "testttt"
      const underlines = page.locator('.autocorrect-underline')
      const count = await underlines.count()
      expect(count).toBeGreaterThanOrEqual(1)

      // Click on the first underline (erreures on line 2)
      await underlines.first().click()
      await page.waitForTimeout(200)

      // Click the suggestion
      const suggestionBtn = page.locator('.autocorrect-suggestion-btn').first()
      if (await suggestionBtn.isVisible()) {
        await suggestionBtn.click()
        await page.waitForTimeout(200)

        // Verify the text was replaced correctly
        const content = await editable.innerText()
        // "erreures" should be replaced with "erreurs"
        expect(content).toContain('erreurs')
        expect(content).not.toContain('erreures')

        // First line should still be intact
        expect(content).toContain('Première ligne sans erreur')
      }
    })

    test('replaces error on third line of contenteditable with br tags', async ({ context }) => {
      const page = await context.newPage()
      await page.goto(TEST_PAGE)

      // Clear and set specific content
      const editable = page.locator('#test-ce-multiline')
      await editable.click()
      await editable.evaluate((el: HTMLElement) => {
        el.innerHTML = 'Line 1 ok<br>Line 2 ok<br>Line 3 testttt error'
      })

      // Wait for detection
      await page.waitForTimeout(DEBOUNCE_WAIT)

      // Click on the underline for "testttt"
      const underlines = page.locator('.autocorrect-underline')
      const count = await underlines.count()
      expect(count).toBeGreaterThanOrEqual(1)

      await underlines.first().click()
      await page.waitForTimeout(200)

      const suggestionBtn = page.locator('.autocorrect-suggestion-btn').first()
      if (await suggestionBtn.isVisible()) {
        await suggestionBtn.click()
        await page.waitForTimeout(200)

        const content = await editable.innerText()
        // Verify line 3 was fixed and lines 1-2 are intact
        expect(content).toContain('Line 1 ok')
        expect(content).toContain('Line 2 ok')
        expect(content).not.toContain('testttt')
      }
    })

    test('replaces error in Gmail-style nested divs', async ({ context }) => {
      const page = await context.newPage()
      await page.goto(TEST_PAGE)

      // Find the Gmail-style contenteditable
      const editable = page.locator('#test-ce-gmail')
      await editable.click()

      // Wait for detection
      await page.waitForTimeout(DEBOUNCE_WAIT)

      // There should be an underline for "erreures"
      const underlines = page.locator('.autocorrect-underline')
      const count = await underlines.count()
      expect(count).toBeGreaterThanOrEqual(1)

      // Click on the underline
      await underlines.first().click()
      await page.waitForTimeout(200)

      const suggestionBtn = page.locator('.autocorrect-suggestion-btn').first()
      if (await suggestionBtn.isVisible()) {
        await suggestionBtn.click()
        await page.waitForTimeout(200)

        const content = await editable.innerText()
        // Verify the fix worked
        expect(content).toContain('erreurs')
        expect(content).not.toContain('erreures')

        // Other lines should be intact
        expect(content).toContain('Bonjour')
        expect(content).toContain('Je vous écris')
      }
    })

    test('maintains correct text structure after replacement', async ({ context }) => {
      const page = await context.newPage()
      await page.goto(TEST_PAGE)

      const editable = page.locator('#test-ce-multiline')
      await editable.click()
      await editable.evaluate((el: HTMLElement) => {
        el.innerHTML = 'Hello<br>This has erreures<br>Goodbye'
      })

      await page.waitForTimeout(DEBOUNCE_WAIT)

      const underlines = page.locator('.autocorrect-underline')
      if (await underlines.count() > 0) {
        await underlines.first().click()
        await page.waitForTimeout(200)

        const suggestionBtn = page.locator('.autocorrect-suggestion-btn').first()
        if (await suggestionBtn.isVisible()) {
          await suggestionBtn.click()
          await page.waitForTimeout(200)

          const content = await editable.innerText()

          // Verify all three lines exist
          expect(content).toContain('Hello')
          expect(content).toContain('Goodbye')
          // The middle line should have been corrected
          expect(content).toContain('erreurs')
        }
      }
    })

    test('handles multiple errors on different lines', async ({ context }) => {
      const page = await context.newPage()
      await page.goto(TEST_PAGE)

      const editable = page.locator('#test-ce-simple')
      await editable.click()
      await editable.evaluate((el: HTMLElement) => {
        el.innerHTML = 'First erreures<br>Second testttt<br>Third fote'
      })

      await page.waitForTimeout(DEBOUNCE_WAIT)

      // Should have multiple underlines
      const underlines = page.locator('.autocorrect-underline')
      const initialCount = await underlines.count()
      expect(initialCount).toBeGreaterThanOrEqual(2)

      // Fix first error
      await underlines.first().click()
      await page.waitForTimeout(200)

      const suggestionBtn = page.locator('.autocorrect-suggestion-btn').first()
      if (await suggestionBtn.isVisible()) {
        await suggestionBtn.click()
        await page.waitForTimeout(DEBOUNCE_WAIT)

        // After fixing, there should be fewer underlines
        const newCount = await underlines.count()
        expect(newCount).toBeLessThan(initialCount)
      }
    })
  })
})
