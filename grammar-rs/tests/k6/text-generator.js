// Text generator for K6 load tests - creates varied, realistic texts

import {
  EN_TEMPLATES, EN_WORDS, FR_TEMPLATES, FR_WORDS,
  injectRandomErrors
} from './templates.js';

// ============================================================================
// CORPUS GENERATION
// ============================================================================

/**
 * Generate a corpus of varied texts for load testing
 * @param {number} size - Number of unique texts to generate
 * @returns {Array} Array of {text, lang, expectedErrors} objects
 */
export function generateVariedCorpus(size) {
  const corpus = [];

  for (let i = 0; i < size; i++) {
    const lang = Math.random() < 0.8 ? 'en' : 'fr'; // 80% EN, 20% FR
    const wordCount = chooseWordCount();
    const hasErrors = Math.random() < 0.7; // 70% with errors

    let text = generateFromTemplate(lang, wordCount);

    if (hasErrors) {
      text = injectRandomErrors(text, 0.15); // 15% of words have errors
    }

    corpus.push({
      text,
      lang,
      expectedErrors: hasErrors ? Math.floor(wordCount * 0.15) : 0,
    });
  }

  return corpus;
}

/**
 * Generate completely unique texts (no repetition)
 * @param {number} count - Number of unique texts needed
 * @returns {Array} Array of {text, lang} objects
 */
export function generateUniqueTexts(count) {
  const texts = [];
  const used = new Set();

  let attempts = 0;
  const maxAttempts = count * 2; // Safety limit

  while (texts.length < count && attempts < maxAttempts) {
    attempts++;

    const lang = Math.random() < 0.8 ? 'en' : 'fr';
    const wordCount = chooseWordCount();
    const hasErrors = Math.random() < 0.7;

    let text = generateFromTemplate(lang, wordCount);

    if (hasErrors) {
      text = injectRandomErrors(text, 0.15);
    }

    const hash = simpleHash(text);

    if (!used.has(hash)) {
      used.add(hash);
      texts.push({ text, lang });
    }
  }

  if (texts.length < count) {
    console.warn(`Only generated ${texts.length} unique texts (wanted ${count})`);
  }

  return texts;
}

// ============================================================================
// TEXT GENERATION HELPERS
// ============================================================================

/**
 * Choose word count based on realistic distribution
 * - 5% short (50-100 words)
 * - 75% medium (200-500 words)
 * - 20% long (800-1500 words)
 */
function chooseWordCount() {
  const rand = Math.random();
  if (rand < 0.05) {
    return 50 + Math.floor(Math.random() * 50);   // short: 50-100
  } else if (rand < 0.80) {
    return 200 + Math.floor(Math.random() * 300); // medium: 200-500
  } else {
    return 800 + Math.floor(Math.random() * 700); // long: 800-1500
  }
}

/**
 * Generate text from templates
 */
function generateFromTemplate(lang, wordCount) {
  const templates = lang === 'en' ? EN_TEMPLATES : FR_TEMPLATES;
  const words = lang === 'en' ? EN_WORDS : FR_WORDS;

  let sentences = [];
  let currentWords = 0;

  // Generate sentences until we reach target word count
  while (currentWords < wordCount) {
    const template = templates[Math.floor(Math.random() * templates.length)];
    let sentence = fillTemplate(template, words);

    sentences.push(sentence);
    currentWords += sentence.split(' ').length;
  }

  let text = sentences.join(' ');

  // Trim to exact word count
  const allWords = text.split(' ');
  if (allWords.length > wordCount) {
    text = allWords.slice(0, wordCount).join(' ');
  }

  // Add some variation with numbers, dates, etc.
  if (Math.random() < 0.3) {
    text = addVariation(text);
  }

  return text;
}

/**
 * Fill template with random words
 */
function fillTemplate(template, words) {
  let result = template;

  // Replace all {placeholder} with random words from that category
  for (const [key, values] of Object.entries(words)) {
    const regex = new RegExp(`\\{${key}\\}`, 'g');
    result = result.replace(regex, () =>
      values[Math.floor(Math.random() * values.length)]
    );
  }

  // Capitalize first letter
  result = result.charAt(0).toUpperCase() + result.slice(1);

  return result;
}

/**
 * Add variation: numbers, dates, proper nouns, etc.
 */
function addVariation(text) {
  const variations = [
    // Add a number
    (t) => t.replace(/the/i, `the ${Math.floor(Math.random() * 100)}`),

    // Add a year
    (t) => t + ` in ${2020 + Math.floor(Math.random() * 5)}.`,

    // Add a proper noun
    (t) => t.replace(/person/i, ['John', 'Mary', 'Alice', 'Bob'][Math.floor(Math.random() * 4)]),

    // Add a company name
    (t) => t.replace(/company/i, ['Google', 'Microsoft', 'Amazon', 'Apple'][Math.floor(Math.random() * 4)]),
  ];

  const variation = variations[Math.floor(Math.random() * variations.length)];
  return variation(text);
}

/**
 * Simple hash function for deduplication
 */
function simpleHash(text) {
  let hash = 0;
  for (let i = 0; i < text.length; i++) {
    const char = text.charCodeAt(i);
    hash = ((hash << 5) - hash) + char;
    hash = hash & hash; // Convert to 32-bit integer
  }
  return hash;
}

// ============================================================================
// EXPORT HELPERS
// ============================================================================

/**
 * Get a random text from the corpus
 */
export function getRandomText(corpus) {
  return corpus[Math.floor(Math.random() * corpus.length)];
}

/**
 * Get text by index (for round-robin)
 */
export function getTextByIndex(corpus, index) {
  return corpus[index % corpus.length];
}
