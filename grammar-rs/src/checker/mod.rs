mod spell;
mod rules;
mod data;
mod confusion;
mod pattern_rules;
mod replace_rules;
mod style_checker;
mod coherency_checker;
mod diacritics_checker;
mod contraction_checker;
mod context_checker;
mod pos_pattern_checker;
mod uncountable_checker;
mod compound_checker;

pub use spell::SpellChecker;
pub use rules::{
    RuleChecker, Rule,
    // Universal rules
    DoubleSpaceRule, RepeatedWordRule, UppercaseSentenceStartRule,
    RepeatedPunctuationRule, MissingSpaceAfterPunctRule,
    // English rules
    AAnRule, SubjectVerbAgreementRule, ItsItsRule, YourYoureRule,
    TheirTheyreThereRule, ImprovedAAnRule, CommaSpliceRule,
    // Advanced English rules (Phase 4)
    LessFewerRule, WhoWhomRule, GoodWellRule, DoubleNegativeRule,
    // Style rules
    PassiveVoiceRule, WordinessRule, SentenceFragmentRule, TypographicQuotesRule,
    // Advanced style rules (Phase 6)
    SentenceLengthRule, ClicheRule, RedundancyRule,
    // French rules
    FrenchPunctuationRule, FrenchAAccentRule, FrenchOuAccentRule, FrenchCeSeRule,
    FrenchSubjectVerbRule, FrenchAdjectiveNounRule,
    // Advanced French rules (Phase 5)
    FrenchConditionnelSiRule, FrenchToutAccordRule,
};
pub use confusion::{EnglishConfusionRule, FrenchConfusionRule};
pub use data::{EN_CONFUSION_DATA, FR_CONFUSION_DATA};
pub use data::{EN_PATTERN_RULES, FR_PATTERN_RULES};
pub use data::{EN_REPLACE_RULES, FR_REPLACE_RULES};
pub use data::{
    PatternTestExample, EN_PATTERN_TEST_EXAMPLES, FR_PATTERN_TEST_EXAMPLES,
    get_en_incorrect_examples, get_en_correct_examples, get_en_examples_for_rule,
    get_fr_incorrect_examples, get_fr_correct_examples, get_fr_examples_for_rule,
};
pub use data::{
    StyleRule, StyleCategory, EN_STYLE_RULES,
    get_en_wordiness_rules, get_en_redundancy_rules, get_en_style_phrases,
};
pub use data::{
    CoherencyPair, EN_COHERENCY_PAIRS, EN_COHERENCY_LOOKUP,
    get_en_coherency_pair, get_en_coherency_variants,
};
pub use data::{
    get_en_diacritics, EN_DIACRITICS_RULES,
};
pub use data::{
    is_en_common_word, is_fr_common_word,
    EN_COMMON_WORDS, FR_COMMON_WORDS,
};
// Contraction data (EN)
pub use data::{
    ContractionRule, EN_CONTRACTION_RULES, get_en_contraction,
};
// Determiner data (EN)
pub use data::{
    requires_en_a, requires_en_an, EN_DET_A_WORDS, EN_DET_AN_WORDS,
};
// Context-sensitive word data (EN)
pub use data::{
    ContextRule, EN_CONTEXT_RULES,
};
// Synonym data
pub use data::{
    SynonymEntry, EN_SYNONYM_RULES, FR_SYNONYM_RULES,
    get_en_synonyms, get_fr_synonyms,
};
// Antipattern data (rule exceptions)
pub use data::{
    Antipattern, AntipatternToken,
    EN_ANTIPATTERNS, FR_ANTIPATTERNS,
    get_en_antipatterns, get_fr_antipatterns,
};
// POS pattern rules (require POS tagging)
pub use data::{
    EN_POS_PATTERN_RULES, FR_POS_PATTERN_RULES,
    create_en_pos_pattern_checker, create_fr_pos_pattern_checker,
};
// POS-tagged words for dictionary
pub use data::{
    PosTaggedWord, EN_ADDED_WORDS, get_en_added_word,
};
pub use pattern_rules::{AhoPatternRuleChecker, PatternRule, PatternRuleChecker};
pub use replace_rules::ReplaceRuleChecker;
pub use style_checker::StyleChecker;
pub use coherency_checker::CoherencyChecker;
pub use diacritics_checker::DiacriticsChecker;
pub use contraction_checker::ContractionChecker;
pub use context_checker::ContextChecker;
pub use pos_pattern_checker::{PosPatternChecker, PosPatternRule, PosPatternElement};
pub use uncountable_checker::UncountableNounChecker;
pub use compound_checker::CompoundWordChecker;
