import { test as base, chromium, BrowserContext } from '@playwright/test'
import path from 'path'

// Path to the built extension
const extensionPath = path.join(__dirname, '../../dist')

// Custom test fixture that loads the extension
export const test = base.extend<{
  context: BrowserContext
  extensionId: string
}>({
  // eslint-disable-next-line no-empty-pattern
  context: async ({}, use) => {
    const context = await chromium.launchPersistentContext('', {
      headless: false,
      args: [
        `--disable-extensions-except=${extensionPath}`,
        `--load-extension=${extensionPath}`,
        '--no-first-run',
        '--disable-default-apps',
      ],
    })
    await use(context)
    await context.close()
  },
  extensionId: async ({ context }, use) => {
    // Get the extension ID from the service worker
    let [background] = context.serviceWorkers()
    if (!background) {
      background = await context.waitForEvent('serviceworker')
    }
    const extensionId = background.url().split('/')[2]
    await use(extensionId)
  },
})

export const expect = base.expect
