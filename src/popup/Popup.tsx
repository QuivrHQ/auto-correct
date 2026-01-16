import { useState, useEffect, useCallback, useRef } from 'react'
import { Settings, DEFAULT_SETTINGS, LanguageToolMatch } from '../shared/types'
import { getSettings, setSettings } from '../shared/storage'
import { checkConnection } from '../content/language-tool-client'
import type { GetMatchesMessage, MatchesResponseMessage, ApplySuggestionMessage } from '../shared/messaging'

type ExtendedMatch = LanguageToolMatch

// Icons as components
const FlagFR = () => (
  <svg width="20" height="14" viewBox="0 0 20 14" fill="none" className="rounded-sm overflow-hidden">
    <rect width="7" height="14" fill="#002395"/>
    <rect x="7" width="6" height="14" fill="#FFFFFF"/>
    <rect x="13" width="7" height="14" fill="#ED2939"/>
  </svg>
)

const FlagEN = () => (
  <svg width="20" height="14" viewBox="0 0 20 14" fill="none" className="rounded-sm overflow-hidden">
    <rect width="20" height="14" fill="#012169"/>
    <path d="M0 0L20 14M20 0L0 14" stroke="white" strokeWidth="2.5"/>
    <path d="M0 0L20 14M20 0L0 14" stroke="#C8102E" strokeWidth="1.5"/>
    <path d="M10 0V14M0 7H20" stroke="white" strokeWidth="4"/>
    <path d="M10 0V14M0 7H20" stroke="#C8102E" strokeWidth="2.5"/>
  </svg>
)

const FlagAuto = () => (
  <svg width="20" height="14" viewBox="0 0 20 14" fill="none" className="rounded-sm">
    <rect width="20" height="14" rx="2" fill="#E5E7EB"/>
    <text x="10" y="10" textAnchor="middle" fontSize="8" fill="#6B7280" fontWeight="500">AUTO</text>
  </svg>
)

const ChevronDown = () => (
  <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
    <path d="M3 4.5L6 7.5L9 4.5"/>
  </svg>
)

const TrashIcon = () => (
  <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round">
    <path d="M2 4h12M5.333 4V2.667a1.333 1.333 0 011.334-1.334h2.666a1.333 1.333 0 011.334 1.334V4m2 0v9.333a1.333 1.333 0 01-1.334 1.334H4.667a1.333 1.333 0 01-1.334-1.334V4h9.334z"/>
  </svg>
)

// Summary Section Component
function SummarySection({
  matches,
  score,
  onFixAll,
  isFixing
}: {
  matches: ExtendedMatch[]
  score: number
  onFixAll: () => void
  isFixing: boolean
}) {
  const spellingErrors = matches.filter(m => m.rule.category.id === 'TYPOS')
  const grammarErrors = matches.filter(m => m.rule.category.id === 'GRAMMAR')
  const styleErrors = matches.filter(m => !['TYPOS', 'GRAMMAR'].includes(m.rule.category.id))

  if (matches.length === 0) return null

  // Get the first error as main recommendation
  const mainError = matches[0]
  const mainSuggestion = mainError.replacements[0]?.value

  return (
    <div className="px-4 py-3 bg-gradient-to-r from-blue-50 to-indigo-50 border-b border-blue-100">
      {/* Score Bar */}
      <div className="flex items-center gap-3 mb-3">
        <span className="text-sm font-medium text-gray-600">Score</span>
        <div className="flex-1 bg-gray-200 rounded-full h-2.5">
          <div
            className={`h-2.5 rounded-full transition-all duration-500 ${
              score >= 80 ? 'bg-emerald-500' : score >= 50 ? 'bg-amber-500' : 'bg-red-500'
            }`}
            style={{ width: `${score}%` }}
          />
        </div>
        <span className={`text-sm font-bold ${
          score >= 80 ? 'text-emerald-600' : score >= 50 ? 'text-amber-600' : 'text-red-600'
        }`}>
          {score}/100
        </span>
      </div>

      {/* Stats */}
      <div className="flex flex-wrap gap-3 mb-3">
        {spellingErrors.length > 0 && (
          <div className="flex items-center gap-1.5 text-sm">
            <span className="w-2 h-2 rounded-full bg-red-500"></span>
            <span className="text-gray-600">
              {spellingErrors.length} orthographe
            </span>
          </div>
        )}
        {grammarErrors.length > 0 && (
          <div className="flex items-center gap-1.5 text-sm">
            <span className="w-2 h-2 rounded-full bg-orange-500"></span>
            <span className="text-gray-600">
              {grammarErrors.length} grammaire
            </span>
          </div>
        )}
        {styleErrors.length > 0 && (
          <div className="flex items-center gap-1.5 text-sm">
            <span className="w-2 h-2 rounded-full bg-blue-500"></span>
            <span className="text-gray-600">
              {styleErrors.length} style
            </span>
          </div>
        )}
      </div>

      {/* Main Recommendation */}
      {mainSuggestion && (
        <div className="mb-3 p-2 bg-white/60 rounded-lg">
          <p className="text-xs text-gray-500 mb-1">Recommandation principale:</p>
          <p className="text-sm text-gray-700">
            <span className="text-red-500 line-through">{mainError.errorText}</span>
            {' → '}
            <span className="text-emerald-600 font-medium">{mainSuggestion}</span>
          </p>
        </div>
      )}

      {/* Fix All Button */}
      <button
        onClick={onFixAll}
        disabled={isFixing}
        className="w-full py-2 bg-blue-500 hover:bg-blue-600 disabled:bg-blue-300 text-white rounded-lg font-medium transition-colors flex items-center justify-center gap-2"
      >
        {isFixing ? (
          <>
            <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
            Correction en cours...
          </>
        ) : (
          <>
            <svg className="w-4 h-4" fill="none" stroke="currentColor" strokeWidth="2" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
            </svg>
            Tout corriger ({matches.length})
          </>
        )}
      </button>
    </div>
  )
}

export function Popup() {
  const [settings, setLocalSettings] = useState<Settings>(DEFAULT_SETTINGS)
  const [connected, setConnected] = useState<boolean | null>(null)
  const [checking, setChecking] = useState(false)
  const [languageOpen, setLanguageOpen] = useState(false)
  const [matches, setMatches] = useState<ExtendedMatch[]>([])
  const [dismissedMatches, setDismissedMatches] = useState<Set<number>>(new Set())
  const [loading, setLoading] = useState(true)
  const [hasActiveField, setHasActiveField] = useState(false)
  const [textLength, setTextLength] = useState(0)
  const [isFixingAll, setIsFixingAll] = useState(false)
  const pollInterval = useRef<number | null>(null)

  // Fetch matches from content script
  const fetchMatches = useCallback(() => {
    chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
      if (tabs[0]?.id) {
        const message: GetMatchesMessage = { type: 'GET_MATCHES' }
        chrome.tabs.sendMessage(tabs[0].id, message, (response: MatchesResponseMessage | undefined) => {
          if (chrome.runtime.lastError) {
            // Content script not loaded on this page
            setLoading(false)
            setHasActiveField(false)
            return
          }
          if (response) {
            setMatches(response.matches)
            setTextLength(response.textLength)
            setHasActiveField(response.fieldInfo !== null)
            setLoading(false)
          }
        })
      } else {
        setLoading(false)
      }
    })
  }, [])

  useEffect(() => {
    getSettings().then(setLocalSettings)
    checkConnectionStatus()
    fetchMatches()

    // Poll for updates every 2 seconds
    pollInterval.current = window.setInterval(fetchMatches, 2000)

    return () => {
      if (pollInterval.current) {
        clearInterval(pollInterval.current)
      }
    }
  }, [fetchMatches])

  const checkConnectionStatus = useCallback(async () => {
    setChecking(true)
    const currentSettings = await getSettings()
    const isConnected = await checkConnection(currentSettings.apiUrl)
    setConnected(isConnected)
    setChecking(false)
  }, [])

  async function handleToggle() {
    const newEnabled = !settings.enabled
    const updated = await setSettings({ enabled: newEnabled })
    setLocalSettings(updated)
  }

  async function handleLanguageChange(language: 'auto' | 'fr' | 'en') {
    const updated = await setSettings({ language })
    setLocalSettings(updated)
    setLanguageOpen(false)
  }

  function dismissMatch(index: number) {
    setDismissedMatches(prev => new Set([...prev, index]))
  }

  function applySuggestion(matchIndex: number, suggestion: string): Promise<void> {
    return new Promise((resolve) => {
      chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
        if (tabs[0]?.id) {
          const message: ApplySuggestionMessage = {
            type: 'APPLY_SUGGESTION',
            matchIndex,
            replacement: suggestion,
          }
          chrome.tabs.sendMessage(tabs[0].id, message, () => {
            dismissMatch(matchIndex)
            // Refetch matches after applying
            setTimeout(() => {
              fetchMatches()
              resolve()
            }, 100)
          })
        } else {
          resolve()
        }
      })
    })
  }

  async function fixAllErrors() {
    setIsFixingAll(true)
    try {
      // Apply suggestions one by one (from the last to avoid index shifting)
      const matchesToFix = [...visibleMatches].reverse()
      for (const match of matchesToFix) {
        if (match.replacements.length > 0) {
          const actualIndex = matches.indexOf(match)
          await applySuggestion(actualIndex, match.replacements[0].value)
          await new Promise(r => setTimeout(r, 150)) // Small delay between corrections
        }
      }
    } finally {
      setIsFixingAll(false)
      // Final refresh
      setTimeout(fetchMatches, 200)
    }
  }

  const visibleMatches = matches.filter((_, i) => !dismissedMatches.has(i))
  const grammarCount = visibleMatches.filter(m => m.rule.category.id === 'GRAMMAR' || m.rule.category.id === 'STYLE').length
  const spellingCount = visibleMatches.filter(m => m.rule.category.id === 'TYPOS').length

  const languageOptions = [
    { value: 'auto' as const, label: 'Auto', flag: <FlagAuto /> },
    { value: 'fr' as const, label: 'Français', flag: <FlagFR /> },
    { value: 'en' as const, label: 'English', flag: <FlagEN /> },
  ]

  const currentLang = languageOptions.find(l => l.value === settings.language) || languageOptions[0]

  // Calculate score based on text length and error count
  const score = textLength > 0
    ? Math.max(0, Math.round(100 - (visibleMatches.length / Math.max(textLength / 50, 1)) * 10))
    : 100

  function getCategoryColor(categoryId: string): string {
    if (categoryId === 'TYPOS') return '#EF4444'
    if (categoryId === 'GRAMMAR') return '#F59E0B'
    return '#3B82F6' // Style and others
  }

  function getCategoryName(categoryId: string): string {
    if (categoryId === 'TYPOS') return 'Orthographe'
    if (categoryId === 'GRAMMAR') return 'Grammaire'
    return 'Style'
  }

  return (
    <div className="w-[380px] bg-white font-sans antialiased">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-gray-100">
        <div className="flex items-center gap-2">
          {/* Logo */}
          <div className="w-8 h-8 bg-gradient-to-br from-blue-500 to-blue-600 rounded-lg flex items-center justify-center shadow-sm">
            <span className="text-white font-bold text-sm tracking-tight">AC</span>
          </div>
          <span className="font-semibold text-gray-800 tracking-tight">AutoCorrect</span>
        </div>

        {/* Language Selector */}
        <div className="relative">
          <button
            onClick={() => setLanguageOpen(!languageOpen)}
            className="flex items-center gap-2 px-2 py-1.5 rounded-lg hover:bg-gray-50 transition-colors"
          >
            {currentLang.flag}
            <span className="text-sm text-gray-600">{currentLang.label}</span>
            <ChevronDown />
          </button>

          {languageOpen && (
            <>
              <div className="fixed inset-0 z-10" onClick={() => setLanguageOpen(false)} />
              <div className="absolute right-0 top-full mt-1 bg-white rounded-lg shadow-lg border border-gray-100 py-1 z-20 min-w-[140px]">
                {languageOptions.map(option => (
                  <button
                    key={option.value}
                    onClick={() => handleLanguageChange(option.value)}
                    className={`w-full flex items-center gap-2 px-3 py-2 hover:bg-gray-50 transition-colors ${
                      settings.language === option.value ? 'bg-blue-50' : ''
                    }`}
                  >
                    {option.flag}
                    <span className="text-sm text-gray-700">{option.label}</span>
                    {settings.language === option.value && (
                      <svg className="w-4 h-4 ml-auto text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                        <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd"/>
                      </svg>
                    )}
                  </button>
                ))}
              </div>
            </>
          )}
        </div>
      </div>

      {/* Stats & Toggle Section */}
      <div className="px-4 py-3 border-b border-gray-100">
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center gap-2">
            <h2 className="text-lg font-semibold text-gray-800">Suggestions</h2>
            <span className="px-2 py-0.5 bg-gray-100 text-gray-600 text-xs font-medium rounded-full">
              {visibleMatches.length}
            </span>
          </div>

          {/* Score Circle */}
          <div className="relative w-11 h-11">
            <svg className="w-full h-full -rotate-90" viewBox="0 0 36 36">
              <circle
                cx="18" cy="18" r="15"
                fill="none"
                stroke="#E5E7EB"
                strokeWidth="3"
              />
              <circle
                cx="18" cy="18" r="15"
                fill="none"
                stroke={score >= 80 ? '#10B981' : score >= 50 ? '#F59E0B' : '#EF4444'}
                strokeWidth="3"
                strokeDasharray={`${score * 0.94} 100`}
                strokeLinecap="round"
                className="transition-all duration-500"
              />
            </svg>
            <span className="absolute inset-0 flex items-center justify-center text-xs font-bold text-gray-700">
              {score}
            </span>
          </div>
        </div>

        {/* Real-time Toggle */}
        <div className="flex items-center justify-between py-2">
          <div className="flex items-center gap-2">
            <svg width="18" height="18" viewBox="0 0 18 18" fill="none" stroke="#6B7280" strokeWidth="1.5">
              <circle cx="9" cy="9" r="3"/>
              <circle cx="9" cy="9" r="7" strokeDasharray="2 2"/>
            </svg>
            <span className="text-sm text-gray-600">Correction en temps réel</span>
          </div>
          <button
            onClick={handleToggle}
            className={`relative w-11 h-6 rounded-full transition-all duration-300 ${
              settings.enabled
                ? 'bg-gradient-to-r from-blue-500 to-blue-600 shadow-inner'
                : 'bg-gray-200'
            }`}
            aria-label={settings.enabled ? 'Désactiver' : 'Activer'}
          >
            <span
              className={`absolute top-0.5 w-5 h-5 bg-white rounded-full shadow-md transition-all duration-300 ${
                settings.enabled ? 'left-[22px]' : 'left-0.5'
              }`}
            >
              {settings.enabled && (
                <svg className="w-full h-full p-1 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd"/>
                </svg>
              )}
            </span>
          </button>
        </div>
      </div>

      {/* Summary Section */}
      {!loading && hasActiveField && (
        <SummarySection
          matches={visibleMatches}
          score={score}
          onFixAll={fixAllErrors}
          isFixing={isFixingAll}
        />
      )}

      {/* Error Cards */}
      <div className="max-h-80 overflow-y-auto">
        {/* Loading State - Skeleton Cards */}
        {loading && (
          <div className="p-3 space-y-3">
            {[1, 2, 3].map((i) => (
              <div key={i} className="bg-white rounded-xl border border-gray-100 overflow-hidden animate-pulse">
                <div className="flex items-center gap-2 px-3 py-2 bg-gray-50/50">
                  <div className="w-2.5 h-2.5 rounded-full bg-gray-200" />
                  <div className="h-4 w-20 bg-gray-200 rounded" />
                </div>
                <div className="px-3 py-2">
                  <div className="h-5 w-24 bg-gray-200 rounded mb-2" />
                  <div className="h-4 w-full bg-gray-100 rounded mb-3" />
                  <div className="flex gap-1.5">
                    <div className="h-8 w-16 bg-gray-200 rounded-lg" />
                    <div className="h-8 w-20 bg-gray-200 rounded-lg" />
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}

        {/* No Active Field */}
        {!loading && !hasActiveField && (
          <div className="py-8 text-center px-4">
            <div className="w-12 h-12 mx-auto mb-3 bg-gray-100 rounded-full flex items-center justify-center">
              <svg className="w-6 h-6 text-gray-400" fill="none" stroke="currentColor" strokeWidth="1.5" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" d="M15.042 21.672L13.684 16.6m0 0l-2.51 2.225.569-9.47 5.227 7.917-3.286-.672zM12 2.25V4.5m5.834.166l-1.591 1.591M20.25 10.5H18M7.757 14.743l-1.59 1.591M6 10.5H3.75m4.007-4.243l-1.59-1.591" />
              </svg>
            </div>
            <p className="text-sm text-gray-600 font-medium">Aucun champ de texte actif</p>
            <p className="text-xs text-gray-400 mt-1">Cliquez sur un champ de texte pour l'analyser</p>
          </div>
        )}

        {!loading && hasActiveField && visibleMatches.map((match, index) => {
          const actualIndex = matches.indexOf(match)
          const categoryColor = getCategoryColor(match.rule.category.id)
          const categoryName = getCategoryName(match.rule.category.id)

          return (
            <div
              key={actualIndex}
              className="mx-3 mb-3 bg-white rounded-xl border border-gray-100 shadow-sm overflow-hidden hover:shadow-md transition-shadow"
            >
              {/* Category Header */}
              <div className="flex items-center justify-between px-3 py-2 bg-gray-50/50">
                <div className="flex items-center gap-2">
                  <span
                    className="w-2.5 h-2.5 rounded-full"
                    style={{ backgroundColor: categoryColor }}
                  />
                  <span className="text-sm font-medium text-gray-700">{categoryName}</span>
                </div>
                <button
                  onClick={() => dismissMatch(actualIndex)}
                  className="p-1 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded transition-colors"
                  title="Ignorer"
                >
                  <TrashIcon />
                </button>
              </div>

              {/* Error Content */}
              <div className="px-3 py-2">
                {/* Error Word */}
                <div className="mb-2">
                  <span
                    className="text-base font-medium"
                    style={{
                      textDecoration: 'underline wavy',
                      textDecorationColor: categoryColor,
                      textUnderlineOffset: '3px'
                    }}
                  >
                    {match.errorText || match.shortMessage}
                  </span>
                </div>

                {/* Context */}
                {match.context?.text && (
                  <p className="text-sm text-gray-500 mb-3 line-clamp-2">
                    {match.context.text.split(match.errorText || '').map((part, i, arr) => (
                      <span key={i}>
                        {part}
                        {i < arr.length - 1 && (
                          <span className="font-medium" style={{ color: categoryColor }}>
                            {match.errorText}
                          </span>
                        )}
                      </span>
                    ))}
                  </p>
                )}

                {/* Suggestions */}
                <div className="flex flex-wrap gap-1.5 mb-2">
                  {match.replacements.slice(0, 3).map((replacement, i) => (
                    <button
                      key={i}
                      onClick={() => applySuggestion(actualIndex, replacement.value)}
                      className="px-3 py-1.5 bg-blue-500 hover:bg-blue-600 text-white text-sm font-medium rounded-lg transition-colors shadow-sm hover:shadow"
                    >
                      {replacement.value}
                    </button>
                  ))}
                </div>

                {/* Explanation */}
                <p className="text-xs text-gray-500">{match.message}</p>
              </div>
            </div>
          )
        })}

        {!loading && hasActiveField && visibleMatches.length === 0 && (
          <div className="py-8 text-center">
            <div className="w-12 h-12 mx-auto mb-3 bg-green-100 rounded-full flex items-center justify-center">
              <svg className="w-6 h-6 text-green-500" fill="none" stroke="currentColor" strokeWidth="2" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7"/>
              </svg>
            </div>
            <p className="text-sm text-gray-500">Aucune erreur détectée</p>
            <p className="text-xs text-gray-400 mt-1">Votre texte est parfait !</p>
          </div>
        )}
      </div>

      {/* Connection Status Footer */}
      <div className="flex items-center gap-2 px-4 py-3 border-t border-gray-100 bg-gray-50/50">
        <span
          className={`w-2 h-2 rounded-full transition-colors ${
            checking
              ? 'bg-yellow-400 animate-pulse'
              : connected === true
              ? 'bg-emerald-500'
              : connected === false
              ? 'bg-red-500'
              : 'bg-gray-300'
          }`}
        />
        <span className="text-xs text-gray-500">
          {checking
            ? 'Connexion...'
            : connected === true
            ? 'Serveur connecté'
            : connected === false
            ? 'Serveur déconnecté'
            : 'Status inconnu'}
        </span>
        <button
          onClick={checkConnectionStatus}
          disabled={checking}
          className="ml-auto text-xs text-gray-400 hover:text-blue-500 disabled:opacity-50 transition-colors"
        >
          Actualiser
        </button>
      </div>
    </div>
  )
}
