import http from 'k6/http'
import { check, sleep } from 'k6'
import { Rate, Trend } from 'k6/metrics'

// Custom metrics
const errorRate = new Rate('errors')
const languageToolLatency = new Trend('languagetool_latency')

// Configuration
const API_URL = __ENV.API_URL || 'https://languagetool-autocorrect.fly.dev'
const CACHE_HIT_RATIO = 0.2 // 20% cache hits pour plus de variance

// Target: 100 users, 10 minutes
export const options = {
  stages: [
    { duration: '1m', target: 50 }, // Ramp-up phase 1
    { duration: '1m', target: 100 }, // Ramp-up phase 2
    { duration: '7m', target: 100 }, // Stable at target
    { duration: '1m', target: 0 }, // Ramp-down
  ],
  thresholds: {
    http_req_duration: ['p(95)<2000'],
    errors: ['rate<0.01'],
  },
}

// === POOL DE PHRASES FRANÇAISES (100+) ===
const frenchPhrases = [
  // Vie quotidienne
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
  // Santé et services
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
  // Travail et études
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
  // Communication
  'Je vous envoie le document par email dans la journée',
  'Merci de confirmer votre présence avant vendredi',
  "N'hésitez pas à me contacter si vous avez des questions",
  'Je vous remercie pour votre aide précieuse',
  'Veuillez trouver ci-joint le rapport mensuel',
  'Suite à notre conversation téléphonique de ce matin',
  'Je me permets de vous relancer concernant ma demande',
  'Pourriez-vous me donner plus de détails sur ce sujet',
  'Je reste à votre disposition pour tout renseignement',
  "Cordialement et dans l'attente de votre réponse",
  // E-commerce et service client
  "Ma commande n'est toujours pas arrivée depuis deux semaines",
  'Je souhaite retourner cet article car il ne me convient pas',
  'Le produit reçu ne correspond pas à la description',
  'Pouvez-vous me rembourser le montant de ma commande',
  "Je n'arrive pas à me connecter à mon compte client",
  'Le code promo ne fonctionne pas sur votre site',
  "Je voudrais modifier l'adresse de livraison",
  'Quand sera disponible cet article en rupture de stock',
  'Je souhaite annuler ma commande passée ce matin',
  'Le service client ne répond pas à mes messages',
  // Support technique
  "L'application plante à chaque fois que je l'ouvre",
  'Je ne reçois plus les notifications sur mon téléphone',
  'Comment puis-je réinitialiser mon mot de passe',
  'La synchronisation entre mes appareils ne fonctionne plus',
  "Mon abonnement premium n'est pas actif malgré le paiement",
  'Les données de mon compte ont disparu sans raison',
  'Je rencontre une erreur lors de la mise à jour',
  "L'interface est très lente depuis la dernière version",
  'Comment exporter mes données au format CSV',
  'Je voudrais supprimer définitivement mon compte',
  // Zendesk / Tickets
  'Bonjour, je rencontre un problème avec votre service',
  'Merci de bien vouloir traiter ma demande en urgence',
  "Je n'ai toujours pas reçu de réponse à mon ticket",
  'Ce problème persiste depuis plusieurs jours maintenant',
  "J'ai déjà contacté le support sans succès",
  'Pouvez-vous escalader ma demande à un superviseur',
  'Je suis très mécontent de la qualité du service',
  'Comment puis-je obtenir un remboursement complet',
  'Le délai de réponse est beaucoup trop long',
  'Je souhaite déposer une réclamation officielle',
  // Phrases plus longues et complexes
  "Après avoir longuement réfléchi à la situation, je pense qu'il serait préférable de reporter la réunion à la semaine prochaine pour permettre à tout le monde de mieux se préparer",
  'Je vous écris concernant le projet dont nous avons discuté lors de notre dernière rencontre et je voudrais vous faire part de quelques remarques importantes',
  'Suite aux dernières modifications apportées au système, plusieurs utilisateurs ont signalé des dysfonctionnements qui nécessitent une attention immédiate',
  'Nous avons le plaisir de vous informer que votre candidature a été retenue pour la prochaine étape du processus de sélection',
  "En raison des conditions météorologiques défavorables, l'événement prévu ce week-end sera reporté à une date ultérieure qui vous sera communiquée prochainement",
]

// === POOL DE PHRASES ANGLAISES (50+) ===
const englishPhrases = [
  // Daily life
  'The weather is really nice today, perfect for a walk in the park',
  'I need to finish this project before the deadline next week',
  'She went to the store to buy some groceries for dinner',
  'The meeting has been rescheduled for next Monday afternoon',
  'Can you help me understand this difficult concept please',
  'They are planning a trip to Europe next summer vacation',
  "The book I'm reading is absolutely fascinating and engaging",
  'Please send me the documents as soon as possible',
  'We should discuss this matter in our next team meeting',
  'The restaurant serves excellent Italian cuisine downtown',
  // Work
  'I have been working on this problem for several hours now',
  'The company announced significant organizational changes yesterday',
  'We need to find a solution to this challenge quickly',
  'The conference will take place in the main auditorium',
  'She has extensive experience in software development and design',
  'The quarterly report shows promising growth in all sectors',
  'Please review the attached document and provide your feedback',
  'We are pleased to announce the launch of our new product',
  'The team successfully completed the project ahead of schedule',
  'I would like to schedule a call to discuss the details',
  // Customer service
  "I haven't received my order yet and it's been two weeks",
  "The product I received doesn't match the description online",
  'I would like to request a refund for my recent purchase',
  'How can I track the status of my shipment please',
  'The discount code is not working on your website',
  'I need to change the delivery address for my order',
  'When will this item be back in stock again',
  'I want to cancel my subscription effective immediately',
  'The customer service wait time is unacceptably long',
  "I'm experiencing issues with my account login credentials",
  // Technical support
  'The application crashes every time I try to open it',
  "I'm not receiving push notifications on my device anymore",
  'How do I reset my password for this account',
  'The sync feature between my devices stopped working',
  'My premium subscription is not active despite payment',
  'All my account data has disappeared without any warning',
  'I keep getting an error message during the update process',
  'The interface has become very slow since the last update',
  'How can I export my data in a different format',
  'I want to permanently delete my account and all data',
  // Complex sentences
  'After careful consideration of all the available options, I believe we should proceed with the second proposal as it offers the best value',
  'I am writing to follow up on our previous conversation regarding the partnership opportunity that was discussed last month',
  'Due to unforeseen circumstances, we regret to inform you that the scheduled event has been postponed until further notice',
  'We are excited to announce that your application has been selected for the next round of interviews starting next week',
  'Following the recent system updates, several users have reported experiencing technical difficulties that require immediate attention',
]

// === FRAGMENTS POUR GÉNÉRER DES VARIATIONS ===
const frenchSubjects = [
  'Je',
  'Nous',
  'Il',
  'Elle',
  'Ils',
  'Elles',
  'On',
  'Mon collègue',
  "L'équipe",
  'Le client',
  'Ma soeur',
  'Mon frère',
  'Les utilisateurs',
  'Le système',
  "L'application",
]

const frenchVerbs = [
  'pense que',
  'crois que',
  'voudrais',
  'aimerais',
  'dois',
  'peux',
  'souhaite',
  'confirme que',
  'indique que',
  'signale que',
  'demande si',
  'espère que',
]

const frenchComplements = [
  "c'est très important",
  'ça ne fonctionne pas correctement',
  'il y a un problème',
  'tout va bien maintenant',
  "c'est urgent",
  "nous avons besoin d'aide",
  "la situation s'améliore",
  'nous attendons une réponse',
  "c'est résolu",
]

// === TYPOS RÉALISTES ÉTENDUS ===
const typoPatterns = {
  // Accents français
  é: ['e', 'è', 'ée', 'er'],
  è: ['e', 'é', 'ê'],
  ê: ['e', 'é'],
  à: ['a', 'â'],
  â: ['a', 'à'],
  ù: ['u', 'û'],
  û: ['u', 'ù'],
  ç: ['c', 'ss'],
  œ: ['oe', 'o', 'eu'],
  î: ['i', 'ï'],
  // Doublons et inversions
  ou: ['ouu', 'uo', 'oo'],
  ai: ['aii', 'ia', 'ei'],
  an: ['ann', 'na', 'en'],
  en: ['enn', 'ne', 'an'],
  on: ['onn', 'no'],
  qu: ['quu', 'cu', 'k'],
  // Anglais
  th: ['hte', 'ht', 't'],
  ie: ['ei', 'ee'],
  ea: ['ae', 'ee'],
  tion: ['toin', 'shun', 'sion'],
  ing: ['ign', 'ng', 'in'],
  ght: ['gt', 'ht'],
  // Communs
  ' le ': [' el ', ' l e'],
  ' la ': [' al ', ' l a'],
  ' de ': [' ed ', ' d e'],
  ' the ': [' teh ', ' hte '],
  ' and ': [' adn ', ' nad '],
}

// Homophones français courants (erreurs fréquentes)
const homophoneErrors = {
  'ce ': ['se '],
  'se ': ['ce '],
  ' a ': [' à '],
  ' à ': [' a '],
  'et ': ['est '],
  'est ': ['et '],
  'son ': ['sont '],
  'sont ': ['son '],
  'ou ': ['où '],
  'où ': ['ou '],
  'ces ': ['ses ', "c'est "],
  'ses ': ['ces ', "s'est "],
}

const cachedTexts = []
const MAX_CACHED = 100

function injectTypo(text) {
  // 70% de chance d'injecter une faute
  if (Math.random() > 0.7) return text

  let modified = text

  // 50% chance d'ajouter une erreur d'homophone
  if (Math.random() < 0.5) {
    const homophones = Object.keys(homophoneErrors)
    for (const homo of homophones) {
      if (modified.includes(homo) && Math.random() < 0.3) {
        const replacements = homophoneErrors[homo]
        modified = modified.replace(
          homo,
          replacements[Math.floor(Math.random() * replacements.length)]
        )
        break
      }
    }
  }

  // Ajouter une typo standard
  const patterns = Object.keys(typoPatterns)
  const shuffled = patterns.sort(() => Math.random() - 0.5)

  for (const pattern of shuffled.slice(0, 3)) {
    if (modified.toLowerCase().includes(pattern) && Math.random() < 0.4) {
      const replacements = typoPatterns[pattern]
      const replacement = replacements[Math.floor(Math.random() * replacements.length)]
      modified = modified.replace(new RegExp(pattern, 'i'), replacement)
      break
    }
  }

  return modified
}

function generateComposedPhrase() {
  // Générer une phrase composée aléatoirement
  const subject = frenchSubjects[Math.floor(Math.random() * frenchSubjects.length)]
  const verb = frenchVerbs[Math.floor(Math.random() * frenchVerbs.length)]
  const complement = frenchComplements[Math.floor(Math.random() * frenchComplements.length)]
  return `${subject} ${verb} ${complement}.`
}

function generateText() {
  // 20% de chance d'utiliser un texte en cache
  if (cachedTexts.length > 0 && Math.random() < CACHE_HIT_RATIO) {
    return cachedTexts[Math.floor(Math.random() * cachedTexts.length)]
  }

  let text
  const rand = Math.random()

  if (rand < 0.6) {
    // 60% : phrase française du pool
    text = frenchPhrases[Math.floor(Math.random() * frenchPhrases.length)]
  } else if (rand < 0.8) {
    // 20% : phrase anglaise du pool
    text = englishPhrases[Math.floor(Math.random() * englishPhrases.length)]
  } else {
    // 20% : phrase composée dynamiquement
    text = generateComposedPhrase()
  }

  // Injecter des typos
  text = injectTypo(text)

  // Ajouter un suffixe unique pour éviter le cache serveur (80% du temps)
  if (Math.random() > CACHE_HIT_RATIO) {
    const uniqueId = `${Date.now()}-${Math.random().toString(36).substr(2, 8)}`
    text += ` [${uniqueId}]`
  }

  // Stocker dans le cache local
  if (cachedTexts.length < MAX_CACHED) {
    cachedTexts.push(text)
  } else if (Math.random() < 0.1) {
    // Remplacer aléatoirement un élément du cache
    cachedTexts[Math.floor(Math.random() * MAX_CACHED)] = text
  }

  return text
}

function getLanguage(text) {
  if (/[éèêëàâäùûüçœîï]/i.test(text)) {
    return 'fr-FR'
  }
  return 'auto'
}

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

  const statusOk = response.status === 200
  let bodyOk = false
  if (statusOk && response.body) {
    try {
      const body = JSON.parse(response.body)
      bodyOk = body.matches !== undefined
    } catch (e) {
      bodyOk = false
    }
  }

  check(response, {
    'status is 200': () => statusOk,
    'valid JSON response': () => bodyOk,
    'response time < 2s': () => response.timings.duration < 2000,
  })

  errorRate.add(!statusOk || !bodyOk)

  // Debounce simulation (300-600ms pour plus de variance)
  sleep(0.3 + Math.random() * 0.3)
}

export function handleSummary(data) {
  const summary = {
    scenario: 'target',
    users: 100,
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
    thresholds: {
      latency_p95_ok: data.metrics.http_req_duration.values['p(95)'] < 2000,
      error_rate_ok: (data.metrics.errors ? data.metrics.errors.values.rate : 0) < 0.01,
    },
  }

  return {
    stdout: JSON.stringify(summary, null, 2) + '\n',
    'results/target.json': JSON.stringify(summary, null, 2),
  }
}
