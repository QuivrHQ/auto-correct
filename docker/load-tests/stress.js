import http from 'k6/http'
import { check, sleep } from 'k6'
import { Rate, Trend } from 'k6/metrics'

// Custom metrics
const errorRate = new Rate('errors')
const languageToolLatency = new Trend('languagetool_latency')

// Configuration
const API_URL = __ENV.API_URL || 'https://languagetool-autocorrect.fly.dev'
const CACHE_HIT_RATIO = 0.3

// Stress test: ramp up to 500 users to find breaking point
export const options = {
  stages: [
    { duration: '30s', target: 50 }, // Warm-up
    { duration: '1m', target: 100 }, // Target load
    { duration: '1m', target: 200 }, // Beyond target
    { duration: '1m', target: 300 }, // Stress level 1
    { duration: '1m', target: 400 }, // Stress level 2
    { duration: '1m', target: 500 }, // Maximum stress
    { duration: '30s', target: 0 }, // Recovery
  ],
  thresholds: {
    // Plus permissif pour un stress test - on veut trouver les limites
    http_req_duration: ['p(95)<5000'], // 95% des requêtes < 5s
    errors: ['rate<0.10'], // < 10% d'erreurs acceptable en stress
  },
}

// Pool de phrases françaises
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
  'Le directeur a annoncé une réunion importante demain',
  'Je cherche un appartement dans le centre-ville',
  'La voiture est en panne, il faut appeler le garagiste',
  "Les élèves préparent leurs examens de fin d'année",
  'Ce film a reçu de nombreuses récompenses internationales',
  'Le musée est ouvert tous les jours sauf le lundi',
  "Nous devons réserver les billets avant qu'il soit trop tard",
  'La température va baisser considérablement cette nuit',
  'Il travaille dans une entreprise de technologie depuis cinq ans',
  'Les scientifiques ont fait une découverte remarquable',
  "L'équipe a remporté le championnat pour la troisième fois",
  "Je préfère voyager en train plutôt qu'en avion",
  'Le nouveau produit sera disponible à partir de demain',
  'Nous avons besoin de plus de temps pour terminer ce projet',
  'La conférence internationale aura lieu le mois prochain',
]

// Pool de phrases anglaises
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
  'I have been working on this problem for hours',
  'The company announced significant changes yesterday',
  'We need to find a solution to this challenge quickly',
  'The conference will take place in the main auditorium',
  'She has experience in software development and design',
]

// Typos réalistes
const typoPatterns = {
  é: ['e', 'è', 'ée'],
  è: ['e', 'é'],
  à: ['a'],
  ç: ['c'],
  ou: ['ouu', 'uo'],
  ai: ['aii', 'ia'],
  th: ['hte', 'ht'],
  ie: ['ei'],
}

const cachedTexts = []
const MAX_CACHED = 100

function injectTypo(text) {
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
  if (cachedTexts.length > 0 && Math.random() < CACHE_HIT_RATIO) {
    return cachedTexts[Math.floor(Math.random() * cachedTexts.length)]
  }

  const isFrench = Math.random() < 0.7
  const phrases = isFrench ? frenchPhrases : englishPhrases
  const base = phrases[Math.floor(Math.random() * phrases.length)]
  let text = injectTypo(base)

  if (Math.random() > CACHE_HIT_RATIO) {
    text += ` [${Date.now()}-${Math.random().toString(36).substr(2, 5)}]`
  }

  if (cachedTexts.length < MAX_CACHED) {
    cachedTexts.push(text)
  }

  return text
}

function getLanguage(text) {
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
    timeout: '30s', // Plus long pour le stress test
  }

  const startTime = Date.now()
  const response = http.post(`${API_URL}/v2/check`, payload, params)
  const duration = Date.now() - startTime

  languageToolLatency.add(duration)

  const success = check(response, {
    'status is 200': (r) => r.status === 200,
    'response has matches': (r) => {
      try {
        const body = JSON.parse(r.body)
        return body.matches !== undefined
      } catch {
        return false
      }
    },
  })

  errorRate.add(!success)

  // Debounce plus court en stress pour maximiser la charge
  sleep(0.2 + Math.random() * 0.2)
}

export function handleSummary(data) {
  const summary = {
    scenario: 'stress',
    maxUsers: 500,
    timestamp: new Date().toISOString(),
    metrics: {
      requests: data.metrics.http_reqs.values.count,
      rps: data.metrics.http_reqs.values.rate,
      latency: {
        avg: data.metrics.http_req_duration.values.avg,
        min: data.metrics.http_req_duration.values.min,
        max: data.metrics.http_req_duration.values.max,
        p50: data.metrics.http_req_duration.values['p(50)'],
        p95: data.metrics.http_req_duration.values['p(95)'],
        p99: data.metrics.http_req_duration.values['p(99)'],
      },
      errors: data.metrics.errors ? data.metrics.errors.values.rate : 0,
      errorCount: data.metrics.http_req_failed ? data.metrics.http_req_failed.values.passes : 0,
    },
    analysis: {
      breaking_point_estimate:
        data.metrics.http_req_duration.values['p(95)'] > 3000
          ? 'Server showing stress at current load'
          : 'Server handling load well',
      recommendation:
        data.metrics.errors && data.metrics.errors.values.rate > 0.05
          ? 'Consider scaling up or horizontal scaling'
          : 'Current capacity appears sufficient',
    },
  }

  return {
    stdout: JSON.stringify(summary, null, 2) + '\n',
    'results/stress.json': JSON.stringify(summary, null, 2),
  }
}
