import http from 'k6/http'
import { check, sleep } from 'k6'
import { Rate, Trend, Counter } from 'k6/metrics'

// Custom metrics
const errorRate = new Rate('errors')
const latency = new Trend('languagetool_latency')
const frenchRequests = new Counter('french_requests')
const englishRequests = new Counter('english_requests')

// Configuration
const API_URL = __ENV.API_URL || 'https://languagetool-autocorrect.fly.dev'

// Test 300 users concurrents
export const options = {
  stages: [
    { duration: '30s', target: 50 },   // Warm-up
    { duration: '30s', target: 100 },  // Montée progressive
    { duration: '30s', target: 200 },  // Approche de la cible
    { duration: '1m', target: 300 },   // Montée à 300
    { duration: '3m', target: 300 },   // HOLD: 300 users pendant 3 min
    { duration: '30s', target: 100 },  // Descente
    { duration: '30s', target: 0 },    // Cool-down
  ],
  thresholds: {
    http_req_duration: ['p(95)<2000'],  // 95% < 2s
    http_req_duration: ['p(99)<5000'],  // 99% < 5s
    errors: ['rate<0.05'],              // < 5% erreurs
    http_req_failed: ['rate<0.05'],     // < 5% failed requests
  },
}

// Phrases françaises avec erreurs typiques
const frenchTexts = [
  { text: "Je suis allé au marché pour achetter des legumes", errors: 2 },
  { text: "Il fait vraiment beau aujourd'hui, je vais me promener", errors: 0 },
  { text: "Les enfant jouent dans le jardin depuis ce matin", errors: 1 },
  { text: "Elle a preparé un exellent repas pour la famille", errors: 2 },
  { text: "Nous avons rendez-vous a la gare demain matin", errors: 1 },
  { text: "Le chat dort sur le canapé, il est tres fatigué", errors: 1 },
  { text: "Je dois finir ce travail avant la fin de la journee", errors: 1 },
  { text: "Pouvez vous me dire ou se trouve la bibliotheque", errors: 2 },
  { text: "Mon ordinateur ne fonctione plus depuis hier soir", errors: 1 },
  { text: "Le medecin m'a prescrit des medicaments pour la grippe", errors: 2 },
  { text: "Nous partons en vacance la semaine prochaine en France", errors: 1 },
  { text: "Le directeur a annoncé une reunion importante demain", errors: 1 },
  { text: "Je cherche un appartement spacieux dans le centre ville", errors: 0 },
  { text: "Les elèves preparent leurs examens de fin d'année", errors: 2 },
  { text: "Ce film a recu de nombreuses recompenses internationales", errors: 2 },
]

// Phrases anglaises avec erreurs typiques
const englishTexts = [
  { text: "I have went to the store yesterday afternoon", errors: 1 },
  { text: "The weather is really beautifull today outside", errors: 1 },
  { text: "She don't know what to do about this problem", errors: 1 },
  { text: "We was planning to visit the museum tomorrow", errors: 1 },
  { text: "The meeting has been resceduled for next week", errors: 1 },
  { text: "Can you help me understanding this concept better", errors: 1 },
  { text: "They are planing a trip to Europe next summer", errors: 1 },
  { text: "I have been working on this since three hours", errors: 1 },
  { text: "The company anounced significant changes yesterday", errors: 1 },
  { text: "She has alot of experience in software development", errors: 1 },
  { text: "We should definately discuss this matter soon", errors: 1 },
  { text: "The resturant serves excellent Italian cuisine", errors: 1 },
  { text: "I need to finsh this project before the deadline", errors: 1 },
  { text: "Please send me the documments as soon as possible", errors: 1 },
  { text: "Their going to the conference next monday morning", errors: 1 },
]

function encodeParams(params) {
  return Object.keys(params)
    .map((key) => `${encodeURIComponent(key)}=${encodeURIComponent(params[key])}`)
    .join('&')
}

export default function () {
  // 70% français, 30% anglais (comme le use case réel)
  const isFrench = Math.random() < 0.7
  const texts = isFrench ? frenchTexts : englishTexts
  const sample = texts[Math.floor(Math.random() * texts.length)]

  // Ajouter un timestamp pour éviter le cache (simule des textes uniques)
  const text = Math.random() < 0.3
    ? sample.text  // 30% cache hit
    : `${sample.text} [${Date.now().toString(36)}]`  // 70% unique

  const language = isFrench ? 'fr' : 'en'

  if (isFrench) {
    frenchRequests.add(1)
  } else {
    englishRequests.add(1)
  }

  const payload = encodeParams({
    text: text,
    language: language,
  })

  const params = {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    timeout: '30s',
  }

  const start = Date.now()
  const response = http.post(`${API_URL}/v2/check`, payload, params)
  const duration = Date.now() - start

  latency.add(duration)

  const success = check(response, {
    'status is 200': (r) => r.status === 200,
    'has matches array': (r) => {
      try {
        return JSON.parse(r.body).matches !== undefined
      } catch {
        return false
      }
    },
    'response time < 3s': (r) => r.timings.duration < 3000,
  })

  errorRate.add(!success)

  // Simule un debounce utilisateur réaliste (300-500ms)
  sleep(0.3 + Math.random() * 0.2)
}

export function handleSummary(data) {
  const p95 = data.metrics.http_req_duration.values['p(95)']
  const p99 = data.metrics.http_req_duration.values['p(99)']
  const errorPct = data.metrics.errors ? data.metrics.errors.values.rate * 100 : 0
  const rps = data.metrics.http_reqs.values.rate

  const passed = p95 < 2000 && errorPct < 5

  const summary = {
    test: '300 Users Load Test',
    target: '300 concurrent users',
    timestamp: new Date().toISOString(),
    result: passed ? 'PASSED' : 'FAILED',
    metrics: {
      total_requests: Math.round(data.metrics.http_reqs.values.count),
      requests_per_second: Math.round(rps * 10) / 10,
      french_requests: data.metrics.french_requests ? data.metrics.french_requests.values.count : 'N/A',
      english_requests: data.metrics.english_requests ? data.metrics.english_requests.values.count : 'N/A',
      latency_ms: {
        avg: Math.round(data.metrics.http_req_duration.values.avg),
        p50: Math.round(data.metrics.http_req_duration.values['p(50)']),
        p95: Math.round(p95),
        p99: Math.round(p99),
        max: Math.round(data.metrics.http_req_duration.values.max),
      },
      errors: {
        rate: `${Math.round(errorPct * 100) / 100}%`,
        count: data.metrics.http_req_failed ? data.metrics.http_req_failed.values.passes : 0,
      },
    },
    thresholds: {
      'p95 < 2000ms': p95 < 2000 ? 'PASS' : 'FAIL',
      'p99 < 5000ms': p99 < 5000 ? 'PASS' : 'FAIL',
      'errors < 5%': errorPct < 5 ? 'PASS' : 'FAIL',
    },
    capacity: {
      estimated_users_per_machine: Math.round(300 / (p95 > 1500 ? 2 : 1)),
      recommendation: p95 < 1000
        ? 'Excellent - can likely handle more load'
        : p95 < 2000
          ? 'Good - at target capacity'
          : 'Consider adding more machines',
    },
  }

  const output = `
================================================================================
                        300 USERS LOAD TEST RESULTS
================================================================================

Result: ${summary.result}
Time:   ${summary.timestamp}

PERFORMANCE
-----------
Total Requests:     ${summary.metrics.total_requests.toLocaleString()}
Requests/sec:       ${summary.metrics.requests_per_second}
French Requests:    ${summary.metrics.french_requests}
English Requests:   ${summary.metrics.english_requests}

LATENCY (ms)
------------
Average:  ${summary.metrics.latency_ms.avg}
p50:      ${summary.metrics.latency_ms.p50}
p95:      ${summary.metrics.latency_ms.p95}
p99:      ${summary.metrics.latency_ms.p99}
Max:      ${summary.metrics.latency_ms.max}

ERRORS
------
Rate:   ${summary.metrics.errors.rate}
Count:  ${summary.metrics.errors.count}

THRESHOLDS
----------
p95 < 2000ms:   ${summary.thresholds['p95 < 2000ms']}
p99 < 5000ms:   ${summary.thresholds['p99 < 5000ms']}
Errors < 5%:    ${summary.thresholds['errors < 5%']}

CAPACITY ANALYSIS
-----------------
${summary.capacity.recommendation}

================================================================================
`

  return {
    stdout: output,
    'results/300users.json': JSON.stringify(summary, null, 2),
  }
}
