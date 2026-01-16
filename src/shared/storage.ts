import { Settings, DEFAULT_SETTINGS } from './types'

const STORAGE_KEY = 'autocorrect_settings'

export async function getSettings(): Promise<Settings> {
  const result = await chrome.storage.sync.get(STORAGE_KEY)
  const stored = result[STORAGE_KEY] as Partial<Settings> | undefined
  return { ...DEFAULT_SETTINGS, ...stored }
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
      const newValue = changes[STORAGE_KEY].newValue as Partial<Settings> | undefined
      callback({ ...DEFAULT_SETTINGS, ...newValue })
    }
  }
  chrome.storage.onChanged.addListener(listener)
  return () => chrome.storage.onChanged.removeListener(listener)
}

export async function addToDictionary(word: string): Promise<void> {
  const settings = await getSettings()
  const normalized = word.toLowerCase().trim()
  if (!settings.personalDictionary.includes(normalized)) {
    await setSettings({
      personalDictionary: [...settings.personalDictionary, normalized]
    })
  }
}

export async function removeFromDictionary(word: string): Promise<void> {
  const settings = await getSettings()
  const normalized = word.toLowerCase().trim()
  await setSettings({
    personalDictionary: settings.personalDictionary.filter(w => w !== normalized)
  })
}
