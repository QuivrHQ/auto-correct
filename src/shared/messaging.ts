import { LanguageToolMatch } from './types'

// Message types for communication between popup and content script
export interface GetMatchesMessage {
  type: 'GET_MATCHES'
}

export interface MatchesResponseMessage {
  type: 'MATCHES_RESPONSE'
  matches: LanguageToolMatch[]
  textLength: number
  fieldInfo: {
    tagName: string
    hasContent: boolean
  } | null
}

export interface ApplySuggestionMessage {
  type: 'APPLY_SUGGESTION'
  matchIndex: number
  replacement: string
}

export interface SuggestionAppliedMessage {
  type: 'SUGGESTION_APPLIED'
  success: boolean
}

export type Message =
  | GetMatchesMessage
  | MatchesResponseMessage
  | ApplySuggestionMessage
  | SuggestionAppliedMessage
