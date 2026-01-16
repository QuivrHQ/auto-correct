import http from 'k6/http'
import { check, sleep } from 'k6'
import { Rate, Trend } from 'k6/metrics'

// Custom metrics
const errorRate = new Rate('errors')
const languageToolLatency = new Trend('languagetool_latency')

// Configuration
const API_URL = __ENV.API_URL || 'https://languagetool-autocorrect.fly.dev'
const CACHE_HIT_RATIO = 0.3 // 30% des requêtes utilisent des textes répétés

// Baseline: 10 users, 5 minutes
export const options = {
  stages: [
    { duration: '30s', target: 10 }, // Ramp-up
    { duration: '4m', target: 10 }, // Stable
    { duration: '30s', target: 0 }, // Ramp-down
  ],
  thresholds: {
    http_req_duration: ['p(95)<2000'], // 95% des requêtes < 2s
    errors: ['rate<0.01'], // < 1% d'erreurs
  },
}

// Pool de phrases françaises avec fautes potentielles
const frenchPhrases = [
  "Je vais au marché aujourd'hui pour acheter des légumes frais",
  'Il fait beau temps cette semaine, je pense aller me promener',
  'Le chat noir dort sur le canapé depuis ce matin',
  'Nous avons rendez-vous demain matin à neuf heures',
  'Elle a préparé un excellent repas pour toute la famille',
  'Les enfants jouent dans le jardin avec leurs amis',
  'Je dois finir ce rapport avant la fin de la journée',
  'Le train arrive à la gare dans quinze minutes',
  'Pouvez-vous me dire où se trouve la bibliothèque',
  'Il pleut beaucoup en automne dans cette région',
  'Mon ordinateur ne fonctionne plus depuis hier soir',
  "Le médecin m'a prescrit des médicaments pour la grippe",
  'Nous partons en vacances la semaine prochaine',
  'Le restaurant était complet quand nous sommes arrivés',
  'Elle étudie le français depuis trois ans maintenant',
]

// Pool de phrases anglaises avec fautes potentielles
const englishPhrases = [
  'The weather is really nice today, perfect for a walk',
  'I need to finish this project before the deadline',
  'She went to the store to buy some groceries',
  'The meeting has been rescheduled for next Monday',
  'Can you help me understand this difficult concept',
  'They are planning a trip to Europe next summer',
  "The book I'm reading is absolutely fascinating",
  'Please send me the documents as soon as possible',
  'We should discuss this matter in our next meeting',
  'The restaurant serves excellent Italian cuisine',
]

// Typos réalistes à injecter
const typoPatterns = {
  // Français
  é: ['e', 'è', 'ée'],
  è: ['e', 'é'],
  à: ['a'],
  ç: ['c'],
  œ: ['oe', 'o'],
  // Doublons/inversions
  ou: ['ouu', 'uo'],
  ai: ['aii', 'ia'],
  an: ['ann', 'na'],
  // Anglais
  th: ['hte', 'ht'],
  ie: ['ei'],
  tion: ['toin', 'tion'],
}

// Cache de textes répétés pour simuler le comportement réel
const cachedTexts = []
const MAX_CACHED = 20

function injectTypo(text) {
  // 60% de chance d'injecter une faute
  if (Math.random() > 0.6) return text

  const patterns = Object.keys(typoPatterns)
  const pattern = patterns[Math.floor(Math.random() * patterns.length)]

  if (text.toLowerCase().includes(pattern)) {
    const replacements = typoPatterns[pattern]
    const replacement = replacements[Math.floor(Math.random() * replacements.length)]
    return text.replace(new RegExp(pattern, 'i'), replacement)
  }

  return text
}

function generateText() {
  // 30% de chance d'utiliser un texte en cache (simule le cache client)
  if (cachedTexts.length > 0 && Math.random() < CACHE_HIT_RATIO) {
    return cachedTexts[Math.floor(Math.random() * cachedTexts.length)]
  }

  // 70% français, 30% anglais
  const isFrench = Math.random() < 0.7
  const phrases = isFrench ? frenchPhrases : englishPhrases
  const base = phrases[Math.floor(Math.random() * phrases.length)]

  // Injecter des typos
  let text = injectTypo(base)

  // Ajouter un suffixe unique pour éviter le cache serveur (70% du temps)
  if (Math.random() > CACHE_HIT_RATIO) {
    text += ` [${Date.now()}-${Math.random().toString(36).substr(2, 5)}]`
  }

  // Stocker dans le cache local pour réutilisation
  if (cachedTexts.length < MAX_CACHED) {
    cachedTexts.push(text)
  }

  return text
}

function getLanguage(text) {
  // Détection simple basée sur les caractères
  if (/[éèêëàâäùûüçœ]/i.test(text)) {
    return 'fr-FR'
  }
  return 'auto'
}

// Helper pour encoder les paramètres (k6 n'a pas URLSearchParams)
function encodeParams(params) {
  return Object.keys(params)
    .map((key) => `${encodeURIComponent(key)}=${encodeURIComponent(params[key])}`)
    .join('&')
}

export default function () {
  const text = generateText()
  const language = getLanguage(text)

  // preferredVariants uniquement avec language=auto
  const params_obj = {
    text: text,
    language: language,
  }
  if (language === 'auto') {
    params_obj.preferredVariants = 'fr-FR,en-US'
  }
  const payload = encodeParams(params_obj)

  const params = {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    timeout: '15s',
  }

  const startTime = Date.now()
  const response = http.post(`${API_URL}/v2/check`, payload, params)
  const duration = Date.now() - startTime

  languageToolLatency.add(duration)

  // Check status
  const statusOk = response.status === 200

  // Check response body
  let bodyOk = false
  if (statusOk && response.body) {
    try {
      const body = JSON.parse(response.body)
      bodyOk = body.matches !== undefined
    } catch (e) {
      // JSON parse error
      bodyOk = false
    }
  }

  // Check latency
  const latencyOk = response.timings.duration < 2000

  const success = check(response, {
    'status is 200': () => statusOk,
    'valid JSON response': () => bodyOk,
    'response time < 2s': () => latencyOk,
  })

  // Only count as error if status or body failed (not latency alone)
  errorRate.add(!statusOk || !bodyOk)

  // Simuler le debounce de l'extension (400ms)
  sleep(0.4 + Math.random() * 0.2) // 400-600ms
}

export function handleSummary(data) {
  const summary = {
    scenario: 'baseline',
    timestamp: new Date().toISOString(),
    metrics: {
      requests: data.metrics.http_reqs.values.count,
      rps: data.metrics.http_reqs.values.rate,
      latency: {
        avg: data.metrics.http_req_duration.values.avg,
        p50: data.metrics.http_req_duration.values['p(50)'],
        p95: data.metrics.http_req_duration.values['p(95)'],
        p99: data.metrics.http_req_duration.values['p(99)'],
      },
      errors: data.metrics.errors ? data.metrics.errors.values.rate : 0,
    },
  }

  return {
    stdout: JSON.stringify(summary, null, 2) + '\n',
    'results/baseline.json': JSON.stringify(summary, null, 2),
  }
}
