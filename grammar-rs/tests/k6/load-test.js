import http from 'k6/http';
import { check, sleep } from 'k6';
import { Rate, Trend, Counter } from 'k6/metrics';
import { getRandomText } from './corpus.js';

// ============================================================================
// CUSTOM METRICS
// ============================================================================

const errorRate = new Rate('errors');
const matchCount = new Trend('match_count');
const requestsPerTier = new Counter('requests_by_tier');

// ============================================================================
// TEST CONFIGURATION
// ============================================================================

export const options = {
  // Test rapide: 4 min par service = 8 min total
  scenarios: {
    // === GRAMMAR-RS (4 min) ===
    grammarrs: {
      executor: 'ramping-vus',
      startVUs: 0,
      stages: [
        { duration: '30s', target: 5 },   // Warm-up
        { duration: '1m', target: 10 },   // Charge normale
        { duration: '1m', target: 20 },   // Charge élevée
        { duration: '1m', target: 50 },   // Stress test
        { duration: '30s', target: 0 },   // Ramp-down
      ],
      tags: { service: 'grammar-rs' },
      env: { BASE_URL: 'https://grammar-rs-autocorrect.fly.dev' },
    },

    // === LANGUAGETOOL (4 min, démarre après grammar-rs) ===
    languagetool: {
      executor: 'ramping-vus',
      startTime: '4m',
      startVUs: 0,
      stages: [
        { duration: '30s', target: 5 },   // Warm-up
        { duration: '1m', target: 10 },   // Charge normale
        { duration: '1m', target: 20 },   // Charge élevée
        { duration: '1m', target: 50 },   // Stress test
        { duration: '30s', target: 0 },   // Ramp-down
      ],
      tags: { service: 'languagetool' },
      env: { BASE_URL: 'https://languagetool-autocorrect.fly.dev' },
    },
  },

  // Thresholds (critères de succès/échec)
  thresholds: {
    // Latence globale
    'http_req_duration': ['p(95)<1000', 'p(99)<2000'],

    // Latence par service (ajustées pour être réalistes)
    'http_req_duration{service:grammar-rs}': ['p(95)<500', 'p(99)<1000'],
    'http_req_duration{service:languagetool}': ['p(95)<1500', 'p(99)<3000'],

    // Error rate
    'errors': ['rate<0.10'], // <10% errors (permissif pour stress test)

    // HTTP success rate
    'http_req_failed': ['rate<0.10'],

    // Checks
    'checks': ['rate>0.85'], // >85% de checks passés
  },
};

// ============================================================================
// MAIN TEST FUNCTION
// ============================================================================

export default function() {
  const baseUrl = __ENV.BASE_URL;

  // Sélectionner texte aléatoire (75% medium, 20% long, 5% short)
  const rand = Math.random();
  let tier;
  if (rand < 0.05) {
    tier = 'short';
  } else if (rand < 0.25) {
    tier = 'long';
  } else {
    tier = 'medium';
  }

  const sample = getRandomText(tier);

  // Préparer la requête
  const params = {
    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
    tags: {
      text_tier: tier,
      text_length: sample.text.length,
    },
    timeout: '30s',
  };

  const payload = `text=${encodeURIComponent(sample.text)}&language=${sample.lang}`;

  // Envoyer la requête
  const response = http.post(`${baseUrl}/v2/check`, payload, params);

  // Vérifications
  const success = check(response, {
    'status is 200': (r) => r.status === 200,
    'response time < 5s': (r) => r.timings.duration < 5000,
    'has matches field': (r) => {
      try {
        const body = JSON.parse(r.body);
        return body.hasOwnProperty('matches');
      } catch {
        return false;
      }
    },
    'has language object': (r) => {
      try {
        const body = JSON.parse(r.body);
        return body.hasOwnProperty('language');
      } catch {
        return false;
      }
    },
  });

  // Métriques custom
  errorRate.add(!success);
  requestsPerTier.add(1, { tier: tier });

  if (success && response.body) {
    try {
      const body = JSON.parse(response.body);
      const numMatches = body.matches?.length || 0;
      matchCount.add(numMatches, { tier: tier });
    } catch (e) {
      console.error('Failed to parse response body:', e);
    }
  }

  // Throttle: petite pause entre requêtes (éviter de spammer)
  // Plus court sous stress (50 VUs), plus long en warm-up (5 VUs)
  const pauseMs = 100 + Math.random() * 200; // 100-300ms
  sleep(pauseMs / 1000);
}

// ============================================================================
// LIFECYCLE HOOKS
// ============================================================================

export function setup() {
  console.log('\n🔬 K6 Load Test: grammar-rs vs LanguageTool');
  console.log('═'.repeat(70));
  console.log(`Duration: 8 minutes (4 min per service)`);
  console.log(`Load profile: 5 → 10 → 20 → 50 VUs`);
  console.log(`Text tiers: short (~100w) 5%, medium (~500w) 75%, long (~1500w) 20%`);
  console.log('═'.repeat(70) + '\n');
}

export function teardown(data) {
  console.log('\n✅ Test completed!');
  console.log('Check the summary above for detailed metrics.\n');
}
