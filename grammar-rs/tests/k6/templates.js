// Templates and word dictionaries for generating varied test texts

// ============================================================================
// ENGLISH TEMPLATES
// ============================================================================

export const EN_TEMPLATES = [
  "I {verb} {article} {noun} yesterday.",
  "The {adjective} {noun} {verb} in the {location}.",
  "She {verb} to {verb} {adverb}.",
  "{article} {adjective} {noun} {verb} {adverb}.",
  "They {verb} {article} {noun} every day.",
  "We should {verb} the {adjective} {noun}.",
  "The {noun} was {adjective} and {adjective}.",
  "I think {article} {noun} {verb} too {adverb}.",
  "He {verb} {article} {adjective} {noun} in {location}.",
  "Can you {verb} the {noun} {adverb}?",
  "The {adjective} {noun} will {verb} tomorrow.",
  "{article} {noun} {verb} because of the {adjective} {noun}.",
  "She {verb} {article} {noun} while I {verb}.",
  "We need to {verb} {article} {adjective} {noun}.",
  "The {location} has {article} {adjective} {noun}.",
  "They want to {verb} {article} {noun} {adverb}.",
  "I {verb} when the {noun} {verb}.",
  "{article} {adjective} {noun} {verb} last week.",
  "He always {verb} {article} {noun} in the morning.",
  "The {noun} should {verb} more {adverb}.",
  "Everyone {verb} {article} {adjective} {noun}.",
  "My {noun} {verb} {adverb} every {noun}.",
  "We {verb} {article} {adjective} {noun} together.",
  "The {adjective} {location} has many {noun}.",
  "I will {verb} {article} {noun} next {noun}.",
];

export const EN_WORDS = {
  verb: [
    "run", "walk", "jump", "think", "see", "eat", "go", "make", "take", "give",
    "find", "tell", "work", "call", "try", "ask", "need", "feel", "become", "leave",
    "put", "mean", "keep", "let", "begin", "seem", "help", "talk", "turn", "start",
    "show", "hear", "play", "move", "like", "live", "believe", "hold", "bring", "happen",
    "write", "provide", "sit", "stand", "lose", "pay", "meet", "include", "continue", "set",
  ],
  noun: [
    "dog", "cat", "house", "car", "book", "tree", "friend", "family", "city", "country",
    "day", "night", "week", "month", "year", "time", "place", "thing", "person", "child",
    "world", "life", "way", "man", "woman", "system", "program", "question", "number", "group",
    "fact", "hand", "part", "eye", "case", "point", "week", "company", "problem", "service",
    "room", "area", "idea", "body", "music", "door", "water", "history", "power", "money",
  ],
  adjective: [
    "big", "small", "red", "quick", "lazy", "beautiful", "old", "new", "good", "bad",
    "long", "short", "high", "low", "great", "little", "own", "other", "right", "wrong",
    "large", "small", "next", "early", "young", "important", "few", "public", "able", "free",
    "human", "local", "sure", "clear", "recent", "simple", "real", "open", "strong", "difficult",
  ],
  adverb: [
    "quickly", "slowly", "carefully", "well", "badly", "often", "always", "never", "sometimes", "usually",
    "really", "very", "too", "quite", "almost", "already", "also", "still", "just", "only",
    "even", "perhaps", "probably", "certainly", "clearly", "definitely", "exactly", "finally", "generally", "normally",
  ],
  article: [
    "a", "an", "the",
  ],
  location: [
    "park", "school", "office", "store", "city", "country", "building", "street", "room", "house",
    "garden", "forest", "mountain", "beach", "river", "lake", "station", "airport", "hospital", "library",
  ],
};

// ============================================================================
// FRENCH TEMPLATES
// ============================================================================

export const FR_TEMPLATES = [
  "Je {verbe} {article} {nom}.",
  "Le {adjectif} {nom} {verbe} dans {lieu}.",
  "Elle {verbe} {adverbe}.",
  "{article} {adjectif} {nom} {verbe} aujourd'hui.",
  "Ils {verbe} {article} {nom} chaque jour.",
  "Nous devons {verbe} le {adjectif} {nom}.",
  "Le {nom} était {adjectif} et {adjectif}.",
  "Je pense que {article} {nom} {verbe} trop {adverbe}.",
  "Il {verbe} {article} {adjectif} {nom} à {lieu}.",
  "Peux-tu {verbe} le {nom} {adverbe}?",
  "Le {adjectif} {nom} va {verbe} demain.",
  "{article} {nom} {verbe} à cause du {adjectif} {nom}.",
  "Elle {verbe} {article} {nom} pendant que je {verbe}.",
  "Nous avons besoin de {verbe} {article} {adjectif} {nom}.",
  "Le {lieu} a {article} {adjectif} {nom}.",
  "Ils veulent {verbe} {article} {nom} {adverbe}.",
  "Je {verbe} quand le {nom} {verbe}.",
  "{article} {adjectif} {nom} {verbe} la semaine dernière.",
  "Il {verbe} toujours {article} {nom} le matin.",
  "Le {nom} devrait {verbe} plus {adverbe}.",
];

export const FR_WORDS = {
  verbe: [
    "aller", "venir", "faire", "dire", "pouvoir", "voir", "savoir", "vouloir", "devoir", "prendre",
    "donner", "trouver", "parler", "aimer", "passer", "mettre", "demander", "laisser", "suivre", "vivre",
    "porter", "croire", "apparaître", "perdre", "rester", "tenir", "sembler", "tomber", "entendre", "montrer",
  ],
  nom: [
    "chien", "chat", "maison", "voiture", "livre", "arbre", "ami", "famille", "ville", "pays",
    "jour", "nuit", "semaine", "mois", "année", "temps", "endroit", "chose", "personne", "enfant",
    "monde", "vie", "chemin", "homme", "femme", "système", "programme", "question", "nombre", "groupe",
  ],
  adjectif: [
    "grand", "petit", "rouge", "rapide", "paresseux", "beau", "vieux", "nouveau", "bon", "mauvais",
    "long", "court", "haut", "bas", "autre", "dernier", "jeune", "important", "public", "libre",
  ],
  adverbe: [
    "rapidement", "lentement", "bien", "mal", "souvent", "toujours", "jamais", "parfois", "très", "trop",
    "aussi", "encore", "déjà", "seulement", "peut-être", "probablement", "certainement", "clairement", "exactement", "finalement",
  ],
  article: [
    "le", "la", "l'", "un", "une", "des",
  ],
  lieu: [
    "parc", "école", "bureau", "magasin", "ville", "pays", "bâtiment", "rue", "chambre", "maison",
    "jardin", "forêt", "montagne", "plage", "rivière", "lac", "gare", "aéroport", "hôpital", "bibliothèque",
  ],
};

// ============================================================================
// ERROR INJECTION FUNCTIONS
// ============================================================================

// Common English grammar errors to inject
export const EN_ERROR_PATTERNS = [
  { from: /\btheir\b/gi, to: 'there', type: 'confusion' },
  { from: /\bthere\b/gi, to: 'their', type: 'confusion' },
  { from: /\byour\b/gi, to: "you're", type: 'confusion' },
  { from: /\byou're\b/gi, to: 'your', type: 'confusion' },
  { from: /\bits\b/gi, to: "it's", type: 'confusion' },
  { from: /\bit's\b/gi, to: 'its', type: 'confusion' },
  { from: /\bthan\b/gi, to: 'then', type: 'confusion' },
  { from: /\bthen\b/gi, to: 'than', type: 'confusion' },
  { from: /\baffect\b/gi, to: 'effect', type: 'confusion' },
  { from: /\beffect\b/gi, to: 'affect', type: 'confusion' },
];

// Function to randomly inject errors
export function injectRandomErrors(text, errorRate = 0.2) {
  const words = text.split(' ');
  const numErrors = Math.floor(words.length * errorRate);

  for (let i = 0; i < numErrors; i++) {
    const idx = Math.floor(Math.random() * words.length);
    words[idx] = applyRandomError(words[idx]);
  }

  return words.join(' ');
}

function applyRandomError(word) {
  const errorFuncs = [
    (w) => w.replace(/their/gi, 'there'),
    (w) => w.replace(/your/gi, "you're"),
    (w) => w.replace(/its/gi, "it's"),
    (w) => w + (w.endsWith('s') ? '' : 's'), // Plural error
    (w) => w.replace(/ed$/i, 'ing'), // Tense error
    (w) => w.replace(/y$/i, 'ies'), // Plural -y error
    (w) => w.replace(/\ba\s+([aeiou])/gi, 'an $1'), // a/an swap
  ];

  const func = errorFuncs[Math.floor(Math.random() * errorFuncs.length)];
  return func(word) || word;
}
