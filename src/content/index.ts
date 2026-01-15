import { getSettings, onSettingsChange } from '../shared/storage'
import { init, setSettings, destroy } from './text-field-manager'

async function main(): Promise<void> {
  // Get initial settings
  const settings = await getSettings()
  setSettings(settings)

  // Initialize if enabled
  if (settings.enabled) {
    init()
  }

  // Listen for settings changes
  onSettingsChange((newSettings) => {
    setSettings(newSettings)

    if (newSettings.enabled) {
      init()
    } else {
      destroy()
    }
  })
}

// Start when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', main)
} else {
  main()
}
