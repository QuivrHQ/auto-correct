//! Auto-generated data from LanguageTool
//!
//! This module contains data imported from LanguageTool:
//! - Confusion pairs from confusion_sets.txt
//! - Pattern rules from grammar.xml
//! - Replace rules from replace.txt
//! - Test examples for validation
//! - Style rules (wordiness/redundancy)
//! - Coherency pairs (spelling variants)
//! - Diacritics rules
//! - Common words (language detection)
//! - Contraction rules
//! - Determiner rules (a/an)
//! - Context-sensitive word rules
//! - Synonyms

// ═══════════════════════════════════════════════════════════════════════════════
// Confusion data
// ═══════════════════════════════════════════════════════════════════════════════
pub mod en_confusion;
pub mod fr_confusion;
pub use en_confusion::{get_en_confusions, EN_CONFUSION_DATA};
pub use fr_confusion::{get_fr_confusions, FR_CONFUSION_DATA};

// ═══════════════════════════════════════════════════════════════════════════════
// Pattern rules
// ═══════════════════════════════════════════════════════════════════════════════
pub mod en_patterns;
pub mod fr_patterns;
pub use en_patterns::EN_PATTERN_RULES;
pub use fr_patterns::FR_PATTERN_RULES;

// ═══════════════════════════════════════════════════════════════════════════════
// Replace rules
// ═══════════════════════════════════════════════════════════════════════════════
pub mod en_replace;
pub mod fr_replace;
pub use en_replace::{get_en_replacement, EN_REPLACE_RULES};
pub use fr_replace::{get_fr_replacement, FR_REPLACE_RULES};

// ═══════════════════════════════════════════════════════════════════════════════
// Test examples (for validation)
// ═══════════════════════════════════════════════════════════════════════════════
pub mod en_pattern_tests;
pub mod fr_pattern_tests;
pub use en_pattern_tests::PatternTestExample;
pub use en_pattern_tests::{
    get_en_correct_examples, get_en_examples_for_rule, get_en_incorrect_examples,
    EN_PATTERN_TEST_EXAMPLES,
};
pub use fr_pattern_tests::{
    get_fr_correct_examples, get_fr_examples_for_rule, get_fr_incorrect_examples,
    FR_PATTERN_TEST_EXAMPLES,
};

// ═══════════════════════════════════════════════════════════════════════════════
// Style rules (wordiness/redundancy)
// ═══════════════════════════════════════════════════════════════════════════════
pub mod en_style;
pub mod fr_style;
pub use en_style::{
    get_en_redundancy_rules, get_en_style_phrases, get_en_wordiness_rules, StyleCategory,
    StyleRule, EN_STYLE_RULES,
};
pub use fr_style::{
    get_fr_redundancy_rules, get_fr_style_phrases, get_fr_wordiness_rules, FR_STYLE_RULES,
};

// ═══════════════════════════════════════════════════════════════════════════════
// Coherency pairs (spelling variants)
// ═══════════════════════════════════════════════════════════════════════════════
pub mod en_coherency;
pub use en_coherency::{
    get_en_coherency_pair, get_en_coherency_variants, CoherencyPair, EN_COHERENCY_LOOKUP,
    EN_COHERENCY_PAIRS,
};

// ═══════════════════════════════════════════════════════════════════════════════
// Diacritics rules
// ═══════════════════════════════════════════════════════════════════════════════
pub mod en_diacritics;
pub use en_diacritics::{get_en_diacritics, EN_DIACRITICS_RULES};

// ═══════════════════════════════════════════════════════════════════════════════
// Common words (language detection)
// ═══════════════════════════════════════════════════════════════════════════════
pub mod en_common_words;
pub mod fr_common_words;
pub use en_common_words::{is_en_common_word, EN_COMMON_WORDS};
pub use fr_common_words::{is_fr_common_word, FR_COMMON_WORDS};

// ═══════════════════════════════════════════════════════════════════════════════
// Contraction rules (EN)
// ═══════════════════════════════════════════════════════════════════════════════
pub mod en_contractions;
pub use en_contractions::{get_en_contraction, ContractionRule, EN_CONTRACTION_RULES};

// ═══════════════════════════════════════════════════════════════════════════════
// Determiner rules (a/an) (EN)
// ═══════════════════════════════════════════════════════════════════════════════
pub mod en_determiners;
pub use en_determiners::{requires_en_a, requires_en_an, EN_DET_A_WORDS, EN_DET_AN_WORDS};

// ═══════════════════════════════════════════════════════════════════════════════
// Context-sensitive word rules (EN)
// ═══════════════════════════════════════════════════════════════════════════════
pub mod en_context_words;
pub use en_context_words::{ContextRule, EN_CONTEXT_RULES};

// ═══════════════════════════════════════════════════════════════════════════════
// Synonyms
// ═══════════════════════════════════════════════════════════════════════════════
pub mod en_synonyms;
pub mod fr_synonyms;
pub use en_synonyms::{get_en_synonyms, SynonymEntry, EN_SYNONYM_RULES};
pub use fr_synonyms::{get_fr_synonyms, FR_SYNONYM_RULES};
// Note: SynonymEntry is re-exported from en_synonyms only to avoid conflict

// ═══════════════════════════════════════════════════════════════════════════════
// POS pattern rules (require POS tagging)
// ═══════════════════════════════════════════════════════════════════════════════
pub mod en_pos_patterns;
pub mod fr_pos_patterns;
pub use en_pos_patterns::{create_en_pos_pattern_checker, EN_POS_PATTERN_RULES};
pub use fr_pos_patterns::{create_fr_pos_pattern_checker, FR_POS_PATTERN_RULES};
pub mod en_confusion_extended;
pub mod en_uncountable;
pub mod en_partlycountable;
pub mod en_proper_nouns;
pub mod en_compounds;
pub mod en_multiwords;
pub mod en_spelling;
pub mod en_ignore;
pub mod fr_compounds;
pub mod fr_multiwords;
pub mod fr_hyphenated;
pub mod fr_spelling;
pub mod fr_ignore;
pub mod en_word_definitions;
pub mod en_prohibit;
pub mod en_us_gb;
pub mod en_confusion_l2_de;
pub mod en_confusion_l2_es;
pub mod en_confusion_l2_fr;
pub mod en_confusion_l2_nl;
pub mod en_added;
pub mod en_numbers;

pub use en_word_definitions::{WordDefinition, EN_WORD_DEFINITIONS, get_en_word_definition};

pub use en_prohibit::{EN_PROHIBIT, is_en_prohibit};

pub use en_us_gb::{UsGbMapping, EN_US_GB_MAPPINGS, us_to_gb, gb_to_us, is_us_spelling, is_gb_spelling};

pub use en_confusion_l2_de::{L2ConfusionPair as L2ConfusionPairDe, EN_L2_DE_CONFUSION_PAIRS, get_en_l2_de_confusion};

pub use en_confusion_l2_es::{L2ConfusionPair as L2ConfusionPairEs, EN_L2_ES_CONFUSION_PAIRS, get_en_l2_es_confusion};

pub use en_confusion_l2_fr::{L2ConfusionPair as L2ConfusionPairFr, EN_L2_FR_CONFUSION_PAIRS, get_en_l2_fr_confusion};

pub use en_confusion_l2_nl::{L2ConfusionPair as L2ConfusionPairNl, EN_L2_NL_CONFUSION_PAIRS, get_en_l2_nl_confusion};

pub use en_added::{PosTaggedWord, EN_ADDED_WORDS, get_en_added_word};

// ═══════════════════════════════════════════════════════════════════════════════
// Spelling skip lists (for spell checker)
// ═══════════════════════════════════════════════════════════════════════════════
pub use en_ignore::{EN_IGNORE, is_en_ignore};
pub use en_proper_nouns::{EN_PROPER_NOUNS, is_en_proper_noun};
pub use en_spelling::{EN_SPELLING, is_en_spelling};
pub use fr_ignore::{FR_IGNORE, is_fr_ignore};
pub use fr_spelling::{FR_SPELLING, is_fr_spelling};

// ═══════════════════════════════════════════════════════════════════════════════
// Antipatterns (exceptions to pattern rules)
// ═══════════════════════════════════════════════════════════════════════════════
pub mod en_antipatterns;
pub mod fr_antipatterns;
// Re-export types from en_antipatterns only, use FR_ANTIPATTERNS data directly
pub use en_antipatterns::{Antipattern, AntipatternToken, EN_ANTIPATTERNS, get_en_antipatterns};
pub use fr_antipatterns::{FR_ANTIPATTERNS, get_fr_antipatterns};
