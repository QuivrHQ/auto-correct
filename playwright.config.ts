import { defineConfig } from '@playwright/test'
import path from 'path'

export default defineConfig({
  testDir: './tests/e2e',
  timeout: 30000,
  retries: 0,
  use: {
    headless: false, // Extensions require headed mode
    viewport: { width: 1280, height: 720 },
    actionTimeout: 10000,
  },
  projects: [
    {
      name: 'chromium',
      use: {
        browserName: 'chromium',
      },
    },
  ],
  // Run local server for test page
  webServer: {
    command: 'python3 -m http.server 8889 --directory .',
    port: 8889,
    reuseExistingServer: true,
  },
})
