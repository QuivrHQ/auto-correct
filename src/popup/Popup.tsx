import { useState, useEffect, useCallback } from 'react'
import { Settings, DEFAULT_SETTINGS } from '../shared/types'
import { getSettings, setSettings, removeFromDictionary, onSettingsChange } from '../shared/storage'
import { checkConnection } from '../content/language-tool-client'

// ════════════════════════════════════════════════════════════════════════════
// ICONS
// ════════════════════════════════════════════════════════════════════════════

const CheckIcon = () => (
  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <polyline points="20 6 9 17 4 12"/>
  </svg>
)

const ChevronIcon = ({ direction = 'down' }: { direction?: 'down' | 'up' }) => (
  <svg
    width="12"
    height="12"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    style={{
      transform: direction === 'up' ? 'rotate(180deg)' : undefined,
      transition: 'transform 200ms ease'
    }}
  >
    <polyline points="6 9 12 15 18 9"/>
  </svg>
)

const RefreshIcon = () => (
  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
    <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
    <path d="M21 3v5h-5"/>
  </svg>
)

const XIcon = () => (
  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
    <line x1="18" y1="6" x2="6" y2="18"/>
    <line x1="6" y1="6" x2="18" y2="18"/>
  </svg>
)

const BookIcon = () => (
  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
    <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
    <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
  </svg>
)

// Language flags
const FlagFR = () => (
  <svg width="20" height="14" viewBox="0 0 20 14" fill="none" className="rounded-sm overflow-hidden" style={{ boxShadow: '0 0 0 1px rgba(0,0,0,0.08)' }}>
    <rect width="7" height="14" fill="#002395"/>
    <rect x="7" width="6" height="14" fill="#FFFFFF"/>
    <rect x="13" width="7" height="14" fill="#ED2939"/>
  </svg>
)

const FlagEN = () => (
  <svg width="20" height="14" viewBox="0 0 20 14" fill="none" className="rounded-sm overflow-hidden" style={{ boxShadow: '0 0 0 1px rgba(0,0,0,0.08)' }}>
    <rect width="20" height="14" fill="#012169"/>
    <path d="M0 0L20 14M20 0L0 14" stroke="white" strokeWidth="2.5"/>
    <path d="M0 0L20 14M20 0L0 14" stroke="#C8102E" strokeWidth="1.5"/>
    <path d="M10 0V14M0 7H20" stroke="white" strokeWidth="4"/>
    <path d="M10 0V14M0 7H20" stroke="#C8102E" strokeWidth="2.5"/>
  </svg>
)

const FlagAuto = () => (
  <div
    className="flex items-center justify-center rounded-sm text-[9px] font-semibold tracking-wide"
    style={{
      width: 20,
      height: 14,
      background: 'var(--ac-border)',
      color: 'var(--ac-ink-muted)',
      boxShadow: '0 0 0 1px rgba(0,0,0,0.08)'
    }}
  >
    AUTO
  </div>
)

// ════════════════════════════════════════════════════════════════════════════
// DICTIONARY SECTION COMPONENT
// ════════════════════════════════════════════════════════════════════════════

function DictionarySection({
  words,
  onRemoveWord
}: {
  words: string[]
  onRemoveWord: (word: string) => void
}) {
  const [expanded, setExpanded] = useState(false)

  return (
    <div
      style={{
        background: 'var(--ac-paper)',
        borderRadius: 12,
        border: '1px solid var(--ac-border-soft)',
        overflow: 'hidden',
      }}
    >
      {/* Header - clickable to expand/collapse */}
      <button
        onClick={() => setExpanded(!expanded)}
        className="w-full flex items-center justify-between px-4 py-3 transition-colors"
        style={{
          background: expanded ? 'var(--ac-border-soft)' : 'transparent',
        }}
        onMouseEnter={(e) => {
          if (!expanded) e.currentTarget.style.background = 'var(--ac-border-soft)'
        }}
        onMouseLeave={(e) => {
          if (!expanded) e.currentTarget.style.background = 'transparent'
        }}
      >
        <div className="flex items-center gap-3">
          <div
            className="w-8 h-8 rounded-lg flex items-center justify-center"
            style={{ background: 'var(--ac-indigo-soft)', color: 'var(--ac-indigo)' }}
          >
            <BookIcon />
          </div>
          <div className="text-left">
            <p className="text-sm font-medium" style={{ color: 'var(--ac-ink)' }}>
              Dictionnaire personnel
            </p>
            <p className="text-xs" style={{ color: 'var(--ac-ink-muted)' }}>
              {words.length} {words.length === 1 ? 'mot' : 'mots'}
            </p>
          </div>
        </div>
        <ChevronIcon direction={expanded ? 'up' : 'down'} />
      </button>

      {/* Expanded content */}
      {expanded && (
        <div
          style={{
            borderTop: '1px solid var(--ac-border-soft)',
            maxHeight: 200,
            overflowY: 'auto',
          }}
        >
          {words.length === 0 ? (
            <div className="px-4 py-6 text-center">
              <p className="text-sm" style={{ color: 'var(--ac-ink-muted)' }}>
                Aucun mot dans le dictionnaire
              </p>
              <p className="text-xs mt-1" style={{ color: 'var(--ac-ink-muted)' }}>
                Ajoutez des mots via le menu contextuel des erreurs
              </p>
            </div>
          ) : (
            <div className="py-1">
              {words.map((word) => (
                <div
                  key={word}
                  className="flex items-center justify-between px-4 py-2 group transition-colors"
                  style={{ background: 'transparent' }}
                  onMouseEnter={(e) => {
                    e.currentTarget.style.background = 'var(--ac-border-soft)'
                  }}
                  onMouseLeave={(e) => {
                    e.currentTarget.style.background = 'transparent'
                  }}
                >
                  <span className="text-sm" style={{ color: 'var(--ac-ink)' }}>
                    {word}
                  </span>
                  <button
                    onClick={() => onRemoveWord(word)}
                    className="p-1 rounded-md opacity-0 group-hover:opacity-100 transition-opacity"
                    style={{ color: 'var(--ac-ink-muted)' }}
                    onMouseEnter={(e) => {
                      e.currentTarget.style.background = 'var(--ac-coral-soft)'
                      e.currentTarget.style.color = 'var(--ac-coral)'
                    }}
                    onMouseLeave={(e) => {
                      e.currentTarget.style.background = 'transparent'
                      e.currentTarget.style.color = 'var(--ac-ink-muted)'
                    }}
                    title="Supprimer du dictionnaire"
                  >
                    <XIcon />
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  )
}

// ════════════════════════════════════════════════════════════════════════════
// MAIN POPUP COMPONENT
// ════════════════════════════════════════════════════════════════════════════

export function Popup() {
  const [settings, setLocalSettings] = useState<Settings>(DEFAULT_SETTINGS)
  const [connected, setConnected] = useState<boolean | null>(null)
  const [checking, setChecking] = useState(false)
  const [languageOpen, setLanguageOpen] = useState(false)

  useEffect(() => {
    getSettings().then(setLocalSettings)
    checkConnectionStatus()

    // Listen for settings changes (e.g., from tooltip adding words)
    const unsubscribe = onSettingsChange(setLocalSettings)
    return unsubscribe
  }, [])

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

  async function handleRemoveWord(word: string) {
    await removeFromDictionary(word)
    const updated = await getSettings()
    setLocalSettings(updated)
  }

  const languageOptions = [
    { value: 'auto' as const, label: 'Auto', flag: <FlagAuto /> },
    { value: 'fr' as const, label: 'Francais', flag: <FlagFR /> },
    { value: 'en' as const, label: 'English', flag: <FlagEN /> },
  ]

  const currentLang = languageOptions.find(l => l.value === settings.language) || languageOptions[0]

  // ════════════════════════════════════════════════════════════════════════════
  // RENDER
  // ════════════════════════════════════════════════════════════════════════════

  return (
    <div
      className="w-[380px] min-h-[200px] flex flex-col"
      style={{ background: 'var(--ac-cream)' }}
    >
      {/* ══════════════════════════════════════════════════════════════════════
          HEADER
          ══════════════════════════════════════════════════════════════════════ */}
      <header
        className="flex items-center justify-between px-4 py-3"
        style={{
          borderBottom: '1px solid var(--ac-border-soft)',
          animation: 'slideDown 0.3s ease-out'
        }}
      >
        <div className="flex items-center gap-2.5">
          {/* Logo mark */}
          <div
            className="w-8 h-8 rounded-lg flex items-center justify-center"
            style={{
              background: 'linear-gradient(135deg, var(--ac-ink) 0%, #3A3530 100%)',
              boxShadow: 'var(--shadow-sm)'
            }}
          >
            <span className="text-sm font-display font-bold" style={{ color: 'var(--ac-cream)' }}>
              Ac
            </span>
          </div>
          <div>
            <h1 className="font-display text-base font-semibold leading-none" style={{ color: 'var(--ac-ink)' }}>
              AutoCorrect
            </h1>
            <p className="text-[10px] mt-0.5" style={{ color: 'var(--ac-ink-muted)' }}>
              Assistant d'ecriture
            </p>
          </div>
        </div>
      </header>

      {/* ══════════════════════════════════════════════════════════════════════
          MAIN CONTENT - SETTINGS
          ══════════════════════════════════════════════════════════════════════ */}
      <div className="flex-1 overflow-y-auto px-4 py-4 space-y-4">

        {/* ─────────────────────────────────────────────────────────────────────
            ENABLE/DISABLE TOGGLE
            ───────────────────────────────────────────────────────────────────── */}
        <div
          className="flex items-center justify-between px-4 py-3 rounded-xl"
          style={{
            background: 'var(--ac-paper)',
            border: '1px solid var(--ac-border-soft)',
          }}
        >
          <div className="flex items-center gap-3">
            <div
              className="w-2.5 h-2.5 rounded-full transition-colors"
              style={{
                background: settings.enabled ? 'var(--ac-sage)' : 'var(--ac-border)',
              }}
            />
            <span className="text-sm font-medium" style={{ color: 'var(--ac-ink)' }}>
              {settings.enabled ? 'Active' : 'Desactive'}
            </span>
          </div>

          <button
            onClick={handleToggle}
            className="relative w-12 h-7 rounded-full transition-colors"
            style={{
              background: settings.enabled
                ? 'var(--ac-ink)'
                : 'var(--ac-border)',
            }}
          >
            <span
              className="absolute top-1 w-5 h-5 rounded-full transition-all"
              style={{
                background: 'var(--ac-paper)',
                left: settings.enabled ? '26px' : '4px',
                boxShadow: 'var(--shadow-sm)',
              }}
            />
          </button>
        </div>

        {/* ─────────────────────────────────────────────────────────────────────
            LANGUAGE SELECTION
            ───────────────────────────────────────────────────────────────────── */}
        <div
          className="px-4 py-3 rounded-xl"
          style={{
            background: 'var(--ac-paper)',
            border: '1px solid var(--ac-border-soft)',
          }}
        >
          <label className="text-xs font-medium mb-2 block" style={{ color: 'var(--ac-ink-muted)' }}>
            Langue
          </label>
          <div className="relative">
            <button
              onClick={() => setLanguageOpen(!languageOpen)}
              className="w-full flex items-center justify-between px-3 py-2.5 rounded-lg transition-colors"
              style={{
                background: 'var(--ac-border-soft)',
                border: '1px solid var(--ac-border)',
              }}
            >
              <div className="flex items-center gap-2.5">
                {currentLang.flag}
                <span className="text-sm" style={{ color: 'var(--ac-ink)' }}>{currentLang.label}</span>
              </div>
              <ChevronIcon direction={languageOpen ? 'up' : 'down'} />
            </button>

            {languageOpen && (
              <>
                <div className="fixed inset-0 z-10" onClick={() => setLanguageOpen(false)} />
                <div
                  className="absolute left-0 right-0 top-full mt-1 py-1 z-20 rounded-xl overflow-hidden"
                  style={{
                    background: 'var(--ac-paper)',
                    boxShadow: 'var(--shadow-lg)',
                    border: '1px solid var(--ac-border-soft)',
                    animation: 'scaleIn 0.15s ease-out'
                  }}
                >
                  {languageOptions.map(option => (
                    <button
                      key={option.value}
                      onClick={() => handleLanguageChange(option.value)}
                      className="w-full flex items-center gap-2.5 px-3 py-2.5 transition-colors"
                      style={{
                        background: settings.language === option.value ? 'var(--ac-border-soft)' : 'transparent',
                      }}
                      onMouseEnter={(e) => {
                        e.currentTarget.style.background = 'var(--ac-border-soft)'
                      }}
                      onMouseLeave={(e) => {
                        e.currentTarget.style.background = settings.language === option.value ? 'var(--ac-border-soft)' : 'transparent'
                      }}
                    >
                      {option.flag}
                      <span className="text-sm flex-1 text-left" style={{ color: 'var(--ac-ink)' }}>{option.label}</span>
                      {settings.language === option.value && (
                        <CheckIcon />
                      )}
                    </button>
                  ))}
                </div>
              </>
            )}
          </div>
        </div>

        {/* ─────────────────────────────────────────────────────────────────────
            PERSONAL DICTIONARY
            ───────────────────────────────────────────────────────────────────── */}
        <DictionarySection
          words={settings.personalDictionary}
          onRemoveWord={handleRemoveWord}
        />

      </div>

      {/* ══════════════════════════════════════════════════════════════════════
          FOOTER - CONNECTION STATUS
          ══════════════════════════════════════════════════════════════════════ */}
      <footer
        className="flex items-center justify-between px-4 py-2.5"
        style={{
          borderTop: '1px solid var(--ac-border-soft)',
          background: 'var(--ac-paper)',
        }}
      >
        <div className="flex items-center gap-2">
          <span
            className="w-2 h-2 rounded-full transition-colors"
            style={{
              background: checking
                ? 'var(--ac-amber)'
                : connected === true
                ? 'var(--ac-sage)'
                : connected === false
                ? 'var(--ac-coral)'
                : 'var(--ac-border)',
              animation: checking ? 'pulse 1s ease-in-out infinite' : undefined,
            }}
          />
          <span className="text-xs" style={{ color: 'var(--ac-ink-muted)' }}>
            {checking
              ? 'Connexion...'
              : connected === true
              ? 'Connecte'
              : connected === false
              ? 'Deconnecte'
              : 'Inconnu'}
          </span>
        </div>

        <button
          onClick={checkConnectionStatus}
          disabled={checking}
          className="flex items-center gap-1.5 text-xs transition-colors"
          style={{
            color: 'var(--ac-ink-muted)',
            opacity: checking ? 0.5 : 1,
          }}
          onMouseEnter={(e) => {
            if (!checking) e.currentTarget.style.color = 'var(--ac-indigo)'
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.color = 'var(--ac-ink-muted)'
          }}
        >
          <RefreshIcon />
          Actualiser
        </button>
      </footer>
    </div>
  )
}
