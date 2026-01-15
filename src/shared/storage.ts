import { Settings, DEFAULT_SETTINGS } from './types'

const STORAGE_KEY = 'autocorrect_settings'

export async function getSettings(): Promise<Settings> {
  const result = await chrome.storage.sync.get(STORAGE_KEY)
  return { ...DEFAULT_SETTINGS, ...result[STORAGE_KEY] }
}

export async function setSettings(settings: Partial<Settings>): Promise<Settings> {
  const current = await getSettings()
  const updated = { ...current, ...settings }
  await chrome.storage.sync.set({ [STORAGE_KEY]: updated })
  return updated
}

export function onSettingsChange(callback: (settings: Settings) => void): () => void {
  const listener = (changes: { [key: string]: chrome.storage.StorageChange }) => {
    if (changes[STORAGE_KEY]) {
      callback({ ...DEFAULT_SETTINGS, ...changes[STORAGE_KEY].newValue })
    }
  }
  chrome.storage.onChanged.addListener(listener)
  return () => chrome.storage.onChanged.removeListener(listener)
}
