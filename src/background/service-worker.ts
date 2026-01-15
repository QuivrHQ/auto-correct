import { getSettings, setSettings, onSettingsChange } from '../shared/storage'
import { MessageType, Settings } from '../shared/types'

// Handle messages from popup and content scripts
chrome.runtime.onMessage.addListener((message: MessageType, _sender, sendResponse) => {
  if (message.type === 'GET_SETTINGS') {
    getSettings().then(sendResponse)
    return true // Async response
  }

  if (message.type === 'SET_SETTINGS') {
    setSettings(message.settings).then(sendResponse)
    return true
  }

  return false
})

// Broadcast settings changes to all tabs
onSettingsChange((settings: Settings) => {
  chrome.tabs.query({}, (tabs) => {
    tabs.forEach(tab => {
      if (tab.id) {
        chrome.tabs.sendMessage(tab.id, {
          type: 'SETTINGS_CHANGED',
          settings,
        }).catch(() => {
          // Ignore errors for tabs without content script
        })
      }
    })
  })
})

// Set up extension icon badge to show status
async function updateBadge(): Promise<void> {
  const settings = await getSettings()
  if (settings.enabled) {
    chrome.action.setBadgeText({ text: '' })
  } else {
    chrome.action.setBadgeText({ text: 'OFF' })
    chrome.action.setBadgeBackgroundColor({ color: '#6B7280' })
  }
}

// Update badge on startup and settings change
updateBadge()
onSettingsChange(updateBadge)
