export interface LanguageToolMatch {
  offset: number
  length: number
  message: string
  shortMessage: string
  replacements: { value: string }[]
  rule: {
    id: string
    category: {
      id: string
      name: string
    }
  }
  context?: {
    text: string
    offset: number
    length: number
  }
  errorText?: string
}

export interface LanguageToolResponse {
  matches: LanguageToolMatch[]
  language: {
    code: string
    name: string
    detectedLanguage: {
      code: string
      name: string
      confidence: number
    }
  }
}

export interface Settings {
  enabled: boolean
  language: 'auto' | 'fr' | 'en'
  apiUrl: string
  personalDictionary: string[]
}

export const DEFAULT_SETTINGS: Settings = {
  enabled: true,
  language: 'auto',
  apiUrl: 'https://languagetool-autocorrect.fly.dev',
  personalDictionary: []
}

export type MessageType =
  | { type: 'GET_SETTINGS' }
  | { type: 'SET_SETTINGS'; settings: Partial<Settings> }
  | { type: 'SETTINGS_CHANGED'; settings: Settings }
  | { type: 'CHECK_CONNECTION' }
  | { type: 'CONNECTION_STATUS'; connected: boolean }
