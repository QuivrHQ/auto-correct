//! Synchronise les règles LanguageTool vers grammar-rs
//!
//! Usage:
//!   cargo run --bin sync-lt              # Clone/update LT automatiquement
//!   cargo run --bin sync-lt -- --path ./lt  # Utilise un path local
//!
//! Portable sur Mac et Linux.

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

const LT_REPO: &str = "https://github.com/languagetool-org/languagetool.git";
const CACHE_DIR: &str = ".cache/languagetool";

// ═══════════════════════════════════════════════════════════════════════════════
// Data structures
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
struct PatternToken {
    text: Option<String>,
    regexp: Option<String>,
    postag: Option<String>,
    postag_regexp: bool,
    inflected: bool,
    case_sensitive: bool,
    negation: bool,
    min: u32,
    max: u32,
}

impl Default for PatternToken {
    fn default() -> Self {
        PatternToken {
            text: None,
            regexp: None,
            postag: None,
            postag_regexp: false,
            inflected: false,
            case_sensitive: false,
            negation: false,
            min: 1,
            max: 1,
        }
    }
}

#[derive(Debug, Clone)]
struct Example {
    text: String,
    is_correct: bool,
    correction: Option<String>,
}

#[derive(Debug, Clone)]
struct XmlRule {
    id: String,
    name: String,
    pattern: Vec<PatternToken>,
    message: String,
    suggestions: Vec<String>,
    examples: Vec<Example>,
    category: String,
}

#[derive(Debug, Clone)]
struct ConfusionPair {
    word1: String,
    word2: String,
    factor: u64,
    bidirectional: bool,
}

#[derive(Debug, Clone)]
struct ReplaceRule {
    wrong: String,
    correct: String,
}

#[derive(Debug, Clone)]
struct StyleRule {
    phrase: String,
    suggestions: Vec<String>,
    category: StyleCategory,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum StyleCategory {
    Wordiness,
    Redundancy,
}

#[derive(Debug, Clone)]
struct CoherencyPair {
    variants: Vec<String>,
}

#[derive(Debug, Clone)]
struct DiacriticsRule {
    without: String,
    with: String,
}

#[derive(Debug, Clone)]
struct ContractionRule {
    without: String,
    with: Vec<String>,
}

#[derive(Debug, Clone)]
struct ContextRule {
    word1: String,
    word2: String,
    match1: String,
    match2: String,
    context1_regex: String,
    context2_regex: String,
    explanation1: Option<String>,
    explanation2: Option<String>,
}

#[derive(Debug, Clone)]
struct SynonymRule {
    word: String,
    pos_tag: Option<String>,
    synonyms: Vec<String>,
}

#[derive(Debug, Clone)]
struct PosPatternRule {
    id: String,
    pattern: Vec<PosPatternElement>,
    message: String,
    suggestions: Vec<String>,
}

#[derive(Debug, Clone)]
struct PosPatternElement {
    text: Option<String>,
    pos_pattern: Option<String>,
    negation: bool,
}

#[derive(Debug, Clone)]
struct CompoundRule {
    word: String,
    allow_no_hyphen: bool,   // + at end
    hyphen_only: bool,       // * at end
    lowercase_joined: bool,  // ? at end
    allow_both: bool,        // $ at end
}

#[derive(Debug, Clone)]
struct MultiwordEntry {
    phrase: String,
    pos_tag: String,
}

#[derive(Debug, Clone)]
struct WordDefinition {
    word: String,
    definition: String,
}

#[derive(Debug, Clone)]
struct UsGbMapping {
    us_word: String,
    gb_word: String,
}

#[derive(Debug, Clone)]
struct L2ConfusionPair {
    word1: String,
    word2: String,
    factor: u64,
    native_language: String,
}

#[derive(Debug, Clone)]
struct PosTaggedWord {
    word: String,
    base_form: String,
    pos_tag: String,
}

#[derive(Debug, Clone)]
struct AntipatternToken {
    text: Option<String>,
    regexp: Option<String>,
    inflected: bool,
    negation: bool,
    postag: Option<String>,
    skip: Option<i32>,
}

impl Default for AntipatternToken {
    fn default() -> Self {
        AntipatternToken {
            text: None,
            regexp: None,
            inflected: false,
            negation: false,
            postag: None,
            skip: None,
        }
    }
}

#[derive(Debug, Clone)]
struct Antipattern {
    rule_id: String,
    tokens: Vec<AntipatternToken>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Complex pattern structures (for JSON serialization)
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ComplexPatternToken {
    text: Option<String>,
    regexp: Option<String>,
    postag: Option<String>,
    postag_regexp: bool,
    inflected: bool,
    case_sensitive: bool,
    negation: bool,
    min: u32,
    max: u32,
    skip: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ComplexAntipattern {
    tokens: Vec<ComplexPatternToken>,
}

/// Unification group for gender/number agreement checking
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct UnificationGroup {
    /// Features to unify (e.g., "gender", "number")
    features: Vec<String>,
    /// Token indices in the pattern that must have the same feature values
    token_indices: Vec<usize>,
    /// If true, tokens must NOT have the same features (negate="yes")
    negate: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ComplexSuggestion {
    /// Static text parts and match references
    parts: Vec<SuggestionPart>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
enum SuggestionPart {
    Literal { text: String },
    MatchRef {
        /// Token index (1-indexed as in LanguageTool)
        index: usize,
        /// Regex pattern to match on token text
        regexp_match: Option<String>,
        /// Replacement for regex match
        regexp_replace: Option<String>,
        /// POS tag pattern (with postag_regexp="yes")
        postag: Option<String>,
        /// POS tag replacement pattern (e.g., "$1 f s")
        postag_replace: Option<String>,
        /// Case conversion: "startlower", "startupper", "alllower", "allupper", "preserve"
        case_conversion: Option<String>,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ComplexRule {
    id: String,
    name: String,
    category: String,
    pattern: Vec<ComplexPatternToken>,
    antipatterns: Vec<ComplexAntipattern>,
    message: String,
    /// Static suggestions (simple text)
    suggestions: Vec<String>,
    /// Dynamic suggestions with <match> references
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    dynamic_suggestions: Vec<ComplexSuggestion>,
    /// Unification groups for gender/number agreement
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    unification_groups: Vec<UnificationGroup>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    examples: Vec<ComplexExample>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ComplexExample {
    text: String,
    is_correct: bool,
    correction: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Disambiguation structures
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq)]
enum DisambigAction {
    Replace,
    Add,
    Remove,
    IgnoreSpelling,
    Filter,
    FilterAll,
    Unify,
    Immunize,
}

#[derive(Debug, Clone)]
struct DisambigWd {
    lemma: Option<String>,
    pos: Option<String>,
}

#[derive(Debug, Clone)]
struct DisambigRule {
    id: String,
    pattern: Vec<PatternToken>,
    marker_indices: Vec<usize>,  // Which tokens are inside <marker>
    action: DisambigAction,
    wd: Option<DisambigWd>,
    postag: Option<String>,
}

#[derive(Debug, Default)]
struct SyncStats {
    grammar_rules: usize,
    simple_patterns: usize,
    pos_pattern_rules: usize,
    confusion_pairs: usize,
    confusion_extended_pairs: usize,
    replace_rules: usize,
    test_examples: usize,
    wordiness_rules: usize,
    redundancy_rules: usize,
    coherency_pairs: usize,
    diacritics_rules: usize,
    common_words: usize,
    contraction_rules: usize,
    det_a_words: usize,
    det_an_words: usize,
    context_rules: usize,
    synonym_rules: usize,
    uncountable_words: usize,
    partlycountable_words: usize,
    specific_case_words: usize,
    compound_rules: usize,
    multiword_entries: usize,
    hyphenated_words: usize,
    spelling_words: usize,
    ignore_words: usize,
    // Phase 4
    word_definitions: usize,
    prohibit_words: usize,
    us_gb_mappings: usize,
    confusion_l2_de: usize,
    confusion_l2_es: usize,
    confusion_l2_fr: usize,
    confusion_l2_nl: usize,
    added_words: usize,
    numbers_words: usize,
    // Phase 5
    antipatterns: usize,
    // Phase 6: Disambiguation
    disambig_skip: usize,
    disambig_skip_regex: usize,
    disambig_pos: usize,
    // Phase 7: N-gram confusion words
    ngram_confusion_words: usize,
    // Phase 8: Complex patterns
    complex_patterns: usize,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Main
// ═══════════════════════════════════════════════════════════════════════════════

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    // Check for help
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return Ok(());
    }

    // Check for N-gram extraction mode
    if args.iter().any(|a| a == "--extract-ngrams") {
        let ngram_path = if let Some(idx) = args.iter().position(|a| a == "--ngram-path") {
            PathBuf::from(args.get(idx + 1).expect("--ngram-path requires a value"))
        } else {
            // Default path: data/ngrams/ngrams-en-20150817/
            PathBuf::from("data/ngrams")
        };

        let output_path = if let Some(idx) = args.iter().position(|a| a == "--output") {
            PathBuf::from(args.get(idx + 1).expect("--output requires a value"))
        } else {
            PathBuf::from("data/ngrams")
        };

        let lang = if let Some(idx) = args.iter().position(|a| a == "--language") {
            args.get(idx + 1).expect("--language requires a value").clone()
        } else {
            "en".to_string()
        };

        return extract_ngrams(&ngram_path, &output_path, &lang);
    }

    // Parse arguments
    let lt_path = if let Some(idx) = args.iter().position(|a| a == "--path") {
        PathBuf::from(args.get(idx + 1).expect("--path requires a value"))
    } else {
        ensure_languagetool_cloned()?
    };

    println!("Using LanguageTool from: {}", lt_path.display());

    let languages = vec!["en", "fr"];
    let mut total_stats = SyncStats::default();

    for lang in &languages {
        println!("\nSyncing {} rules...", lang.to_uppercase());
        let stats = sync_language(&lt_path, lang)?;
        total_stats.grammar_rules += stats.grammar_rules;
        total_stats.simple_patterns += stats.simple_patterns;
        total_stats.pos_pattern_rules += stats.pos_pattern_rules;
        total_stats.confusion_pairs += stats.confusion_pairs;
        total_stats.confusion_extended_pairs += stats.confusion_extended_pairs;
        total_stats.replace_rules += stats.replace_rules;
        total_stats.test_examples += stats.test_examples;
        total_stats.wordiness_rules += stats.wordiness_rules;
        total_stats.redundancy_rules += stats.redundancy_rules;
        total_stats.coherency_pairs += stats.coherency_pairs;
        total_stats.diacritics_rules += stats.diacritics_rules;
        total_stats.common_words += stats.common_words;
        total_stats.contraction_rules += stats.contraction_rules;
        total_stats.det_a_words += stats.det_a_words;
        total_stats.det_an_words += stats.det_an_words;
        total_stats.context_rules += stats.context_rules;
        total_stats.synonym_rules += stats.synonym_rules;
        total_stats.uncountable_words += stats.uncountable_words;
        total_stats.partlycountable_words += stats.partlycountable_words;
        total_stats.specific_case_words += stats.specific_case_words;
        total_stats.compound_rules += stats.compound_rules;
        total_stats.multiword_entries += stats.multiword_entries;
        total_stats.hyphenated_words += stats.hyphenated_words;
        total_stats.spelling_words += stats.spelling_words;
        total_stats.ignore_words += stats.ignore_words;
        // Phase 4
        total_stats.word_definitions += stats.word_definitions;
        total_stats.prohibit_words += stats.prohibit_words;
        total_stats.us_gb_mappings += stats.us_gb_mappings;
        total_stats.confusion_l2_de += stats.confusion_l2_de;
        total_stats.confusion_l2_es += stats.confusion_l2_es;
        total_stats.confusion_l2_fr += stats.confusion_l2_fr;
        total_stats.confusion_l2_nl += stats.confusion_l2_nl;
        total_stats.added_words += stats.added_words;
        total_stats.numbers_words += stats.numbers_words;
        // Phase 5
        total_stats.antipatterns += stats.antipatterns;
        // Phase 6: Disambiguation
        total_stats.disambig_skip += stats.disambig_skip;
        total_stats.disambig_skip_regex += stats.disambig_skip_regex;
        total_stats.disambig_pos += stats.disambig_pos;
        // Phase 7: N-gram
        total_stats.ngram_confusion_words += stats.ngram_confusion_words;
        // Phase 8
        total_stats.complex_patterns += stats.complex_patterns;
    }

    println!("\n{}", "=".repeat(70));
    println!("Synchronisation complete!");
    println!(
        "  Patterns: {} | POS patterns: {} | Confusion: {} (+{} extended) | Replace: {}",
        total_stats.simple_patterns, total_stats.pos_pattern_rules,
        total_stats.confusion_pairs, total_stats.confusion_extended_pairs,
        total_stats.replace_rules
    );
    println!(
        "  Wordiness: {} | Redundancy: {} | Coherency: {}",
        total_stats.wordiness_rules, total_stats.redundancy_rules, total_stats.coherency_pairs
    );
    println!(
        "  Diacritics: {} | Common words: {} | Test examples: {}",
        total_stats.diacritics_rules,
        total_stats.common_words,
        total_stats.test_examples
    );
    println!(
        "  Contractions: {} | Determiners: {} (a: {}, an: {})",
        total_stats.contraction_rules,
        total_stats.det_a_words + total_stats.det_an_words,
        total_stats.det_a_words,
        total_stats.det_an_words
    );
    println!(
        "  Context rules: {} | Synonyms: {}",
        total_stats.context_rules, total_stats.synonym_rules
    );
    println!(
        "  Uncountable: {} | Partly countable: {} | Proper nouns: {}",
        total_stats.uncountable_words,
        total_stats.partlycountable_words,
        total_stats.specific_case_words
    );
    println!(
        "  Compounds: {} | Multiwords: {} | Hyphenated: {}",
        total_stats.compound_rules,
        total_stats.multiword_entries,
        total_stats.hyphenated_words
    );
    println!(
        "  Spelling: {} | Ignore: {}",
        total_stats.spelling_words, total_stats.ignore_words
    );
    // Phase 4 stats
    println!(
        "  Word definitions: {} | US/GB mappings: {} | Prohibit: {}",
        total_stats.word_definitions, total_stats.us_gb_mappings, total_stats.prohibit_words
    );
    let total_l2 = total_stats.confusion_l2_de + total_stats.confusion_l2_es
        + total_stats.confusion_l2_fr + total_stats.confusion_l2_nl;
    println!(
        "  L2 confusion: {} (DE:{} ES:{} FR:{} NL:{})",
        total_l2, total_stats.confusion_l2_de, total_stats.confusion_l2_es,
        total_stats.confusion_l2_fr, total_stats.confusion_l2_nl
    );
    println!(
        "  Added words: {} | Numbers: {}",
        total_stats.added_words, total_stats.numbers_words
    );
    // Phase 5 stats
    println!(
        "  Antipatterns: {}",
        total_stats.antipatterns
    );
    // Phase 6: Disambiguation stats
    println!(
        "  Disambiguation skip: {} (+{} regex) | POS rules: {}",
        total_stats.disambig_skip,
        total_stats.disambig_skip_regex,
        total_stats.disambig_pos
    );
    // Phase 7: N-gram stats
    println!(
        "  N-gram confusion words: {}",
        total_stats.ngram_confusion_words
    );
    // Phase 8 stats
    println!(
        "  Complex patterns: {}",
        total_stats.complex_patterns
    );
    println!("{}", "=".repeat(70));

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// Git operations
// ═══════════════════════════════════════════════════════════════════════════════

fn ensure_languagetool_cloned() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let project_root = find_project_root()?;
    let cache_path = project_root.join(CACHE_DIR);

    if cache_path.join(".git").exists() {
        println!("Updating LanguageTool...");
        let status = Command::new("git")
            .args(["pull", "--depth", "1"])
            .current_dir(&cache_path)
            .status()?;

        if !status.success() {
            eprintln!("Warning: git pull failed, using existing version");
        }
    } else {
        println!("Cloning LanguageTool (sparse checkout, rules only)...");

        // Create cache directory
        fs::create_dir_all(cache_path.parent().unwrap())?;

        // Clone with sparse checkout
        let status = Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                "--filter=blob:none",
                "--sparse",
                LT_REPO,
                cache_path.to_str().unwrap(),
            ])
            .status()?;

        if !status.success() {
            return Err("Failed to clone LanguageTool".into());
        }

        // Configure sparse checkout for rules and resources
        let status = Command::new("git")
            .args([
                "sparse-checkout",
                "set",
                "languagetool-language-modules/en/src/main/resources/org/languagetool/rules/en",
                "languagetool-language-modules/fr/src/main/resources/org/languagetool/rules/fr",
                "languagetool-language-modules/en/src/main/resources/org/languagetool/resource/en",
                "languagetool-language-modules/fr/src/main/resources/org/languagetool/resource/fr",
            ])
            .current_dir(&cache_path)
            .status()?;

        if !status.success() {
            return Err("Failed to configure sparse checkout".into());
        }
    }

    Ok(cache_path)
}

fn find_project_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let current = std::env::current_dir()?;
    let mut path = current.as_path();

    while !path.join("Cargo.toml").exists() {
        path = path
            .parent()
            .ok_or("Could not find project root (Cargo.toml)")?;
    }

    Ok(path.to_path_buf())
}

// ═══════════════════════════════════════════════════════════════════════════════
// Language synchronization
// ═══════════════════════════════════════════════════════════════════════════════

fn sync_language(lt_path: &Path, lang: &str) -> Result<SyncStats, Box<dyn std::error::Error>> {
    let rules_path = lt_path
        .join("languagetool-language-modules")
        .join(lang)
        .join("src/main/resources/org/languagetool/rules")
        .join(lang);

    let output_dir = find_project_root()?.join("src/checker/data");
    fs::create_dir_all(&output_dir)?;

    let mut stats = SyncStats::default();

    // 1. Sync grammar.xml + style.xml -> patterns + test examples
    let grammar_path = rules_path.join("grammar.xml");
    let style_xml_path = rules_path.join("style.xml");

    let mut all_rules = Vec::new();
    let mut grammar_count = 0;
    let mut style_count = 0;

    if grammar_path.exists() {
        let rules = parse_grammar_xml(&grammar_path)?;
        grammar_count = rules.len();
        all_rules.extend(rules);
    }

    // Also parse style.xml for additional pattern rules (pleonasms, style issues)
    if style_xml_path.exists() {
        let rules = parse_grammar_xml(&style_xml_path)?;
        style_count = rules.len();
        all_rules.extend(rules);
    }

    if !all_rules.is_empty() {
        stats.grammar_rules = grammar_count + style_count;
        let simple_rules = filter_simple_rules(all_rules.clone());
        stats.simple_patterns = simple_rules.len();

        // Extract POS pattern rules
        let pos_rules = filter_pos_pattern_rules(&all_rules, lang);
        stats.pos_pattern_rules = pos_rules.len();

        // Count and generate test examples
        let test_examples = extract_test_examples(&simple_rules);
        stats.test_examples = test_examples.len();

        println!(
            "   grammar.xml: {} rules, style.xml: {} rules -> {} simple patterns, {} POS patterns, {} test examples",
            grammar_count, style_count, stats.simple_patterns, stats.pos_pattern_rules, stats.test_examples
        );

        if !simple_rules.is_empty() {
            let code = generate_patterns_file(&simple_rules, lang);
            let output_path = output_dir.join(format!("{}_patterns.rs", lang));
            fs::write(&output_path, code)?;

            // Generate test examples file
            if !test_examples.is_empty() {
                let test_code = generate_test_examples_file(&test_examples, lang);
                let test_output_path = output_dir.join(format!("{}_pattern_tests.rs", lang));
                fs::write(&test_output_path, test_code)?;
            }
        }

        // Generate POS pattern rules file
        if !pos_rules.is_empty() {
            let code = generate_pos_patterns_file(&pos_rules, lang);
            let output_path = output_dir.join(format!("{}_pos_patterns.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 2. Sync confusion_sets.txt + custom rules -> confusion
    // Check both rules/ and resource/ folders (EN is in rules/, FR is in resource/)
    let confusion_path = rules_path.join("confusion_sets.txt");
    let resource_confusion_path = lt_path
        .join("languagetool-language-modules")
        .join(lang)
        .join("src/main/resources/org/languagetool/resource")
        .join(lang)
        .join("confusion_sets.txt");
    let custom_confusion_path = output_dir.join(format!("{}_custom_confusion.txt", lang));

    let actual_confusion_path = if confusion_path.exists() {
        Some(confusion_path)
    } else if resource_confusion_path.exists() {
        Some(resource_confusion_path)
    } else {
        None
    };

    let mut all_confusion_pairs = Vec::new();
    let mut lt_confusion_count = 0;
    let mut custom_confusion_count = 0;

    if let Some(path) = actual_confusion_path {
        let pairs = parse_confusion_sets(&path)?;
        lt_confusion_count = pairs.len();
        all_confusion_pairs.extend(pairs);
    }

    // Load custom confusion pairs if they exist
    if custom_confusion_path.exists() {
        let pairs = parse_confusion_sets(&custom_confusion_path)?;
        custom_confusion_count = pairs.len();
        all_confusion_pairs.extend(pairs);
    }

    if !all_confusion_pairs.is_empty() {
        stats.confusion_pairs = all_confusion_pairs.len();
        if custom_confusion_count > 0 {
            println!("   confusion_sets.txt: {} + custom: {} = {} pairs", lt_confusion_count, custom_confusion_count, stats.confusion_pairs);
        } else {
            println!("   confusion_sets.txt: {} pairs", stats.confusion_pairs);
        }

        let code = generate_confusion_file(&all_confusion_pairs, lang);
        let output_path = output_dir.join(format!("{}_confusion.rs", lang));
        fs::write(&output_path, code)?;
    }

    // 3. Sync replace.txt + custom rules -> replace
    let replace_path = rules_path.join("replace.txt");
    let custom_replace_path = output_dir.join(format!("{}_custom_replace.txt", lang));

    let mut all_replace_rules = Vec::new();
    let mut lt_count = 0;
    let mut custom_count = 0;

    if replace_path.exists() {
        let rules = parse_replace_txt(&replace_path)?;
        lt_count = rules.len();
        all_replace_rules.extend(rules);
    }

    // Load custom replace rules if they exist
    if custom_replace_path.exists() {
        let rules = parse_replace_txt(&custom_replace_path)?;
        custom_count = rules.len();
        all_replace_rules.extend(rules);
    }

    if !all_replace_rules.is_empty() {
        // Sort and deduplicate
        all_replace_rules.sort_by(|a, b| a.wrong.cmp(&b.wrong));
        all_replace_rules.dedup_by(|a, b| a.wrong == b.wrong);

        stats.replace_rules = all_replace_rules.len();
        if custom_count > 0 {
            println!("   replace.txt: {} + custom: {} = {} replacements", lt_count, custom_count, stats.replace_rules);
        } else {
            println!("   replace.txt: {} replacements", stats.replace_rules);
        }

        let code = generate_replace_file(&all_replace_rules, lang);
        let output_path = output_dir.join(format!("{}_replace.rs", lang));
        fs::write(&output_path, code)?;
    }

    // 4. Sync wordiness.txt + redundancies.txt + custom redundancies -> style
    let mut style_rules = Vec::new();
    let mut lt_wordiness_count = 0;
    let mut lt_redundancy_count = 0;
    let mut custom_redundancy_count = 0;

    let wordiness_path = rules_path.join("wordiness.txt");
    if wordiness_path.exists() {
        let rules = parse_style_file(&wordiness_path, StyleCategory::Wordiness)?;
        lt_wordiness_count = rules.len();
        style_rules.extend(rules);
    }

    let redundancies_path = rules_path.join("redundancies.txt");
    if redundancies_path.exists() {
        let rules = parse_style_file(&redundancies_path, StyleCategory::Redundancy)?;
        lt_redundancy_count = rules.len();
        style_rules.extend(rules);
    }

    // Load custom redundancies (pleonasms) if they exist
    let custom_redundancies_path = output_dir.join(format!("{}_custom_redundancies.txt", lang));
    if custom_redundancies_path.exists() {
        let rules = parse_custom_redundancies(&custom_redundancies_path)?;
        custom_redundancy_count = rules.len();
        style_rules.extend(rules);
    }

    stats.wordiness_rules = lt_wordiness_count;
    stats.redundancy_rules = lt_redundancy_count + custom_redundancy_count;

    if lt_wordiness_count > 0 || lt_redundancy_count > 0 || custom_redundancy_count > 0 {
        if custom_redundancy_count > 0 {
            println!(
                "   wordiness: {} | redundancy: {} + custom: {} = {} style rules",
                lt_wordiness_count, lt_redundancy_count, custom_redundancy_count,
                lt_wordiness_count + lt_redundancy_count + custom_redundancy_count
            );
        } else if lt_wordiness_count > 0 || lt_redundancy_count > 0 {
            println!("   wordiness.txt: {} rules", lt_wordiness_count);
            println!("   redundancies.txt: {} rules", lt_redundancy_count);
        }
    }

    if !style_rules.is_empty() {
        let code = generate_style_file(&style_rules, lang);
        let output_path = output_dir.join(format!("{}_style.rs", lang));
        fs::write(&output_path, code)?;
    }

    // 5. Sync coherency.txt -> coherency pairs
    let coherency_path = rules_path.join("coherency.txt");
    if coherency_path.exists() {
        let pairs = parse_coherency_file(&coherency_path)?;
        stats.coherency_pairs = pairs.len();
        println!("   coherency.txt: {} pairs", stats.coherency_pairs);

        if !pairs.is_empty() {
            let code = generate_coherency_file(&pairs, lang);
            let output_path = output_dir.join(format!("{}_coherency.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 6. Sync diacritics.txt -> diacritics rules
    let diacritics_path = rules_path.join("diacritics.txt");
    if diacritics_path.exists() {
        let rules = parse_diacritics_file(&diacritics_path)?;
        stats.diacritics_rules = rules.len();
        println!("   diacritics.txt: {} rules", stats.diacritics_rules);

        if !rules.is_empty() {
            let code = generate_diacritics_file(&rules, lang);
            let output_path = output_dir.join(format!("{}_diacritics.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 7. Sync common_words.txt -> language detection
    let resource_path = lt_path
        .join("languagetool-language-modules")
        .join(lang)
        .join("src/main/resources/org/languagetool/resource")
        .join(lang);
    let common_words_path = resource_path.join("common_words.txt");
    if common_words_path.exists() {
        let words = parse_common_words_file(&common_words_path)?;
        stats.common_words = words.len();
        println!("   common_words.txt: {} words", stats.common_words);

        if !words.is_empty() {
            let code = generate_common_words_file(&words, lang);
            let output_path = output_dir.join(format!("{}_common_words.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 8. Sync contractions.txt -> contraction rules (EN only)
    let contractions_path = rules_path.join("contractions.txt");
    if contractions_path.exists() {
        let rules = parse_contractions_txt(&contractions_path)?;
        stats.contraction_rules = rules.len();
        println!("   contractions.txt: {} rules", stats.contraction_rules);

        if !rules.is_empty() {
            let code = generate_contractions_file(&rules, lang);
            let output_path = output_dir.join(format!("{}_contractions.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 9. Sync det_a.txt + det_an.txt -> determiners (EN only)
    let det_a_path = rules_path.join("det_a.txt");
    let det_an_path = rules_path.join("det_an.txt");
    if det_a_path.exists() || det_an_path.exists() {
        let a_words = if det_a_path.exists() {
            parse_determiner_txt(&det_a_path)?
        } else {
            Vec::new()
        };
        let an_words = if det_an_path.exists() {
            parse_determiner_txt(&det_an_path)?
        } else {
            Vec::new()
        };

        stats.det_a_words = a_words.len();
        stats.det_an_words = an_words.len();
        println!("   det_a.txt + det_an.txt: {} words (a: {}, an: {})",
            stats.det_a_words + stats.det_an_words,
            stats.det_a_words,
            stats.det_an_words
        );

        if !a_words.is_empty() || !an_words.is_empty() {
            let code = generate_determiners_file(&a_words, &an_words, lang);
            let output_path = output_dir.join(format!("{}_determiners.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 10. Sync wrongWordInContext.txt -> context rules (EN only)
    let context_path = rules_path.join("wrongWordInContext.txt");
    if context_path.exists() {
        let rules = parse_context_words_txt(&context_path)?;
        stats.context_rules = rules.len();
        println!("   wrongWordInContext.txt: {} rules", stats.context_rules);

        if !rules.is_empty() {
            let code = generate_context_words_file(&rules, lang);
            let output_path = output_dir.join(format!("{}_context_words.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 11. Sync synonyms.txt -> synonym rules (FR mainly)
    let synonyms_path = rules_path.join("synonyms.txt");
    if synonyms_path.exists() {
        let rules = parse_synonyms_txt(&synonyms_path)?;
        stats.synonym_rules = rules.len();
        println!("   synonyms.txt: {} rules", stats.synonym_rules);

        if !rules.is_empty() {
            let code = generate_synonyms_file(&rules, lang);
            let output_path = output_dir.join(format!("{}_synonyms.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 12. Sync confusion_sets_extended.txt -> extended confusion pairs (EN only)
    let confusion_extended_path = resource_path.join("confusion_sets_extended.txt");
    if confusion_extended_path.exists() {
        let pairs = parse_confusion_extended(&confusion_extended_path)?;
        stats.confusion_extended_pairs = pairs.len();
        println!("   confusion_sets_extended.txt: {} pairs", stats.confusion_extended_pairs);

        if !pairs.is_empty() {
            let code = generate_confusion_extended_file(&pairs, lang);
            let output_path = output_dir.join(format!("{}_confusion_extended.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 13. Sync uncountable.txt -> uncountable nouns (EN only)
    let uncountable_path = resource_path.join("uncountable.txt");
    if uncountable_path.exists() {
        let words = parse_word_list(&uncountable_path)?;
        stats.uncountable_words = words.len();
        println!("   uncountable.txt: {} words", stats.uncountable_words);

        if !words.is_empty() {
            let code = generate_word_list_file(&words, lang, "uncountable", "Uncountable nouns (cannot be pluralized)");
            let output_path = output_dir.join(format!("{}_uncountable.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 14. Sync partlycountable.txt -> partly countable nouns (EN only)
    let partlycountable_path = resource_path.join("partlycountable.txt");
    if partlycountable_path.exists() {
        let words = parse_word_list(&partlycountable_path)?;
        stats.partlycountable_words = words.len();
        println!("   partlycountable.txt: {} words", stats.partlycountable_words);

        if !words.is_empty() {
            let code = generate_word_list_file(&words, lang, "partlycountable", "Partly countable nouns (can be both countable and uncountable)");
            let output_path = output_dir.join(format!("{}_partlycountable.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 15. Sync specific_case.txt -> proper nouns with specific casing (EN mainly)
    let specific_case_path = resource_path.join("specific_case.txt");
    if specific_case_path.exists() {
        let words = parse_specific_case(&specific_case_path)?;
        stats.specific_case_words = words.len();
        println!("   specific_case.txt: {} proper nouns", stats.specific_case_words);

        if !words.is_empty() {
            let code = generate_specific_case_file(&words, lang);
            let output_path = output_dir.join(format!("{}_proper_nouns.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 16. Sync compounds.txt -> compound word rules (EN + FR)
    let compounds_path = resource_path.join("compounds.txt");
    if compounds_path.exists() {
        let rules = parse_compounds_txt(&compounds_path)?;
        stats.compound_rules = rules.len();
        println!("   compounds.txt: {} rules", stats.compound_rules);

        if !rules.is_empty() {
            let code = generate_compounds_file(&rules, lang);
            let output_path = output_dir.join(format!("{}_compounds.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 17. Sync multiwords.txt -> multiword expressions (EN + FR)
    let multiwords_path = resource_path.join("multiwords.txt");
    if multiwords_path.exists() {
        let entries = parse_multiwords_txt(&multiwords_path)?;
        stats.multiword_entries = entries.len();
        println!("   multiwords.txt: {} entries", stats.multiword_entries);

        if !entries.is_empty() {
            let code = generate_multiwords_file(&entries, lang);
            let output_path = output_dir.join(format!("{}_multiwords.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 18. Sync hyphenated_words.txt -> hyphenated words (FR mainly)
    let hyphenated_path = resource_path.join("hyphenated_words.txt");
    if hyphenated_path.exists() {
        let words = parse_word_list(&hyphenated_path)?;
        stats.hyphenated_words = words.len();
        println!("   hyphenated_words.txt: {} words", stats.hyphenated_words);

        if !words.is_empty() {
            let code = generate_word_list_file(&words, lang, "hyphenated", "Hyphenated words (correct hyphenation)");
            let output_path = output_dir.join(format!("{}_hyphenated.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 19. Sync spelling.txt + ignore.txt -> spell check lists
    let hunspell_path = resource_path.join("hunspell");
    let spelling_path = hunspell_path.join("spelling.txt");
    let ignore_path = hunspell_path.join("ignore.txt");

    if spelling_path.exists() {
        let words = parse_word_list(&spelling_path)?;
        stats.spelling_words = words.len();
        println!("   spelling.txt: {} words", stats.spelling_words);

        if !words.is_empty() {
            let code = generate_word_list_file(&words, lang, "spelling", "Spelling additions (valid words to add to spell checker)");
            let output_path = output_dir.join(format!("{}_spelling.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    if ignore_path.exists() {
        let words = parse_word_list(&ignore_path)?;
        stats.ignore_words = words.len();
        println!("   ignore.txt: {} words", stats.ignore_words);

        if !words.is_empty() {
            let code = generate_word_list_file(&words, lang, "ignore", "Ignored words (should not trigger spell check errors)");
            let output_path = output_dir.join(format!("{}_ignore.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Phase 4: Additional LanguageTool resources
    // ═══════════════════════════════════════════════════════════════════════════════

    // 20. Sync word_definitions.txt -> word definitions for disambiguation (EN only)
    let word_definitions_path = resource_path.join("word_definitions.txt");
    if word_definitions_path.exists() {
        let definitions = parse_word_definitions(&word_definitions_path)?;
        stats.word_definitions = definitions.len();
        println!("   word_definitions.txt: {} definitions", stats.word_definitions);

        if !definitions.is_empty() {
            let code = generate_word_definitions_file(&definitions, lang);
            let output_path = output_dir.join(format!("{}_word_definitions.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 21. Sync prohibit.txt -> words that should be marked as errors
    let prohibit_path = hunspell_path.join("prohibit.txt");
    if prohibit_path.exists() {
        let words = parse_word_list(&prohibit_path)?;
        stats.prohibit_words = words.len();
        println!("   prohibit.txt: {} words", stats.prohibit_words);

        if !words.is_empty() {
            let code = generate_word_list_file(&words, lang, "prohibit", "Prohibited words (should be marked as spelling errors)");
            let output_path = output_dir.join(format!("{}_prohibit.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 22. Sync en-US-GB.txt -> US/UK spelling mappings (EN only)
    let us_gb_path = resource_path.join("en-US-GB.txt");
    if us_gb_path.exists() {
        let mappings = parse_us_gb_mappings(&us_gb_path)?;
        stats.us_gb_mappings = mappings.len();
        println!("   en-US-GB.txt: {} mappings", stats.us_gb_mappings);

        if !mappings.is_empty() {
            let code = generate_us_gb_file(&mappings, lang);
            let output_path = output_dir.join(format!("{}_us_gb.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 23. Sync confusion_sets_l2_*.txt -> L2 learner confusion sets (EN only)
    if lang == "en" {
        // German native speakers
        let l2_de_path = resource_path.join("confusion_sets_l2_de.txt");
        if l2_de_path.exists() {
            let pairs = parse_confusion_l2(&l2_de_path, "de")?;
            stats.confusion_l2_de = pairs.len();
            println!("   confusion_sets_l2_de.txt: {} pairs", stats.confusion_l2_de);

            if !pairs.is_empty() {
                let code = generate_confusion_l2_file(&pairs, lang, "de");
                let output_path = output_dir.join(format!("{}_confusion_l2_de.rs", lang));
                fs::write(&output_path, code)?;
            }
        }

        // Spanish native speakers
        let l2_es_path = resource_path.join("confusion_sets_l2_es.txt");
        if l2_es_path.exists() {
            let pairs = parse_confusion_l2(&l2_es_path, "es")?;
            stats.confusion_l2_es = pairs.len();
            println!("   confusion_sets_l2_es.txt: {} pairs", stats.confusion_l2_es);

            if !pairs.is_empty() {
                let code = generate_confusion_l2_file(&pairs, lang, "es");
                let output_path = output_dir.join(format!("{}_confusion_l2_es.rs", lang));
                fs::write(&output_path, code)?;
            }
        }

        // French native speakers
        let l2_fr_path = resource_path.join("confusion_sets_l2_fr.txt");
        if l2_fr_path.exists() {
            let pairs = parse_confusion_l2(&l2_fr_path, "fr")?;
            stats.confusion_l2_fr = pairs.len();
            println!("   confusion_sets_l2_fr.txt: {} pairs", stats.confusion_l2_fr);

            if !pairs.is_empty() {
                let code = generate_confusion_l2_file(&pairs, lang, "fr");
                let output_path = output_dir.join(format!("{}_confusion_l2_fr.rs", lang));
                fs::write(&output_path, code)?;
            }
        }

        // Dutch native speakers
        let l2_nl_path = resource_path.join("confusion_sets_l2_nl.txt");
        if l2_nl_path.exists() {
            let pairs = parse_confusion_l2(&l2_nl_path, "nl")?;
            stats.confusion_l2_nl = pairs.len();
            println!("   confusion_sets_l2_nl.txt: {} pairs", stats.confusion_l2_nl);

            if !pairs.is_empty() {
                let code = generate_confusion_l2_file(&pairs, lang, "nl");
                let output_path = output_dir.join(format!("{}_confusion_l2_nl.rs", lang));
                fs::write(&output_path, code)?;
            }
        }
    }

    // 24. Sync added.txt -> POS-tagged words to add to dictionary
    let added_path = resource_path.join("added.txt");
    if added_path.exists() {
        let words = parse_pos_tagged_words(&added_path)?;
        stats.added_words = words.len();
        println!("   added.txt: {} POS-tagged words", stats.added_words);

        if !words.is_empty() {
            let code = generate_pos_tagged_words_file(&words, lang);
            let output_path = output_dir.join(format!("{}_added.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // 25. Sync numbers.txt -> number words (EN only)
    let numbers_path = resource_path.join("numbers.txt");
    if numbers_path.exists() {
        let words = parse_word_list(&numbers_path)?;
        stats.numbers_words = words.len();
        println!("   numbers.txt: {} number words", stats.numbers_words);

        if !words.is_empty() {
            let code = generate_word_list_file(&words, lang, "numbers", "Number words (one, two, three, etc.)");
            let output_path = output_dir.join(format!("{}_numbers.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Phase 5: Antipatterns from grammar.xml
    // ═══════════════════════════════════════════════════════════════════════════════

    // 26. Sync antipatterns from grammar.xml -> exceptions to pattern rules
    if grammar_path.exists() {
        let antipatterns = parse_antipatterns(&grammar_path)?;
        stats.antipatterns = antipatterns.len();
        println!("   antipatterns: {} extracted from grammar.xml", stats.antipatterns);

        if !antipatterns.is_empty() {
            let code = generate_antipatterns_file(&antipatterns, lang);
            let output_path = output_dir.join(format!("{}_antipatterns.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Phase 6: Disambiguation rules
    // ═══════════════════════════════════════════════════════════════════════════════

    // 27. Sync disambiguation.xml -> ignore_spelling patterns + POS rules
    let disambig_path = resource_path.join("disambiguation.xml");
    if disambig_path.exists() {
        let rules = parse_disambiguation_xml(&disambig_path)?;
        println!("   disambiguation.xml: {} rules parsed", rules.len());

        // Extract ignore_spelling patterns (single-token)
        let (skip_words, skip_regex) = extract_ignore_spelling_patterns(&rules);
        stats.disambig_skip = skip_words.len();
        stats.disambig_skip_regex = skip_regex.len();
        println!("      ignore_spelling: {} words + {} regex patterns", stats.disambig_skip, stats.disambig_skip_regex);

        if !skip_words.is_empty() || !skip_regex.is_empty() {
            let code = generate_disambig_skip_file(&skip_words, &skip_regex, lang);
            let output_path = output_dir.join(format!("{}_disambig_skip.rs", lang));
            fs::write(&output_path, code)?;
        }

        // Extract single-token POS rules (replace/add)
        let pos_rules = extract_single_token_pos_rules(&rules);
        stats.disambig_pos = pos_rules.len();
        println!("      single-token POS: {} rules", stats.disambig_pos);

        if !pos_rules.is_empty() {
            let code = generate_disambig_pos_file(&pos_rules, lang);
            let output_path = output_dir.join(format!("{}_disambig_pos.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Phase 7: N-gram confusion words extraction
    // ═══════════════════════════════════════════════════════════════════════════════

    // 28. Extract unique words from all confusion pairs for N-gram filtering
    if lang == "en" {
        let mut confusion_words = std::collections::HashSet::new();

        // Add words from main confusion_sets.txt
        let confusion_path = rules_path.join("confusion_sets.txt");
        if confusion_path.exists() {
            if let Ok(pairs) = parse_confusion_sets(&confusion_path) {
                for pair in &pairs {
                    confusion_words.insert(pair.word1.to_lowercase());
                    confusion_words.insert(pair.word2.to_lowercase());
                }
            }
        }

        // Add words from confusion_sets_extended.txt
        let extended_path = resource_path.join("confusion_sets_extended.txt");
        if extended_path.exists() {
            if let Ok(pairs) = parse_confusion_extended(&extended_path) {
                for pair in &pairs {
                    confusion_words.insert(pair.word1.to_lowercase());
                    confusion_words.insert(pair.word2.to_lowercase());
                }
            }
        }

        stats.ngram_confusion_words = confusion_words.len();
        println!("   N-gram confusion words: {} unique words extracted", stats.ngram_confusion_words);

        // Generate confusion words file for N-gram filtering
        if !confusion_words.is_empty() {
            let mut words: Vec<_> = confusion_words.into_iter().collect();
            words.sort();
            let code = generate_ngram_words_file(&words, lang);
            let output_path = output_dir.join(format!("{}_ngram_words.rs", lang));
            fs::write(&output_path, code)?;
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Phase 8: Complex pattern rules (regex, postag_regexp, skip, etc.)
    // ═══════════════════════════════════════════════════════════════════════════════

    // 29. Extract complex patterns that can't be compiled to simple Rust code
    let mut all_complex_rules = Vec::new();

    // Extract from grammar.xml
    if grammar_path.exists() {
        let grammar_rules = extract_complex_rules(&grammar_path)?;
        println!("   complex patterns: {} extracted from grammar.xml", grammar_rules.len());
        all_complex_rules.extend(grammar_rules);
    }

    // Extract from style.xml (same format as grammar.xml)
    let style_xml_path = rules_path.join("style.xml");
    if style_xml_path.exists() {
        let style_rules = extract_complex_rules(&style_xml_path)?;
        println!("   complex patterns: {} extracted from style.xml", style_rules.len());
        all_complex_rules.extend(style_rules);
    }

    stats.complex_patterns = all_complex_rules.len();

    if !all_complex_rules.is_empty() {
        // Serialize to JSON for runtime loading
        let json = serde_json::to_string_pretty(&all_complex_rules)?;
        let output_path = output_dir.join(format!("{}_complex_patterns.json", lang));
        fs::write(&output_path, json)?;
    }

    // Update mod.rs
    update_data_mod(&output_dir, lang)?;

    Ok(stats)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: grammar.xml
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_grammar_xml(path: &Path) -> Result<Vec<XmlRule>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let mut reader = Reader::from_str(&content);
    reader.trim_text(true);

    let mut rules = Vec::new();
    let mut buf = Vec::new();

    let mut current_rule: Option<XmlRule> = None;
    let mut current_pattern: Vec<PatternToken> = Vec::new();
    let mut current_token: Option<PatternToken> = None;
    let mut current_example: Option<Example> = None;
    let mut in_message = false;
    let mut in_suggestion = false;
    let mut in_token = false;
    let mut in_example = false;
    let mut current_category = String::new();
    let mut text_buffer = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                match name.as_str() {
                    "category" => {
                        current_category = get_attr(e, "name").unwrap_or_default();
                    }
                    "rule" => {
                        let id = get_attr(e, "id").unwrap_or_default();
                        let rule_name = get_attr(e, "name").unwrap_or_default();
                        current_rule = Some(XmlRule {
                            id,
                            name: rule_name,
                            pattern: Vec::new(),
                            message: String::new(),
                            suggestions: Vec::new(),
                            examples: Vec::new(),
                            category: current_category.clone(),
                        });
                        current_pattern.clear();
                    }
                    "pattern" => {
                        current_pattern.clear();
                    }
                    "token" => {
                        let mut token = PatternToken::default();
                        token.inflected = get_attr(e, "inflected")
                            .map(|v| v == "yes")
                            .unwrap_or(false);
                        token.case_sensitive = get_attr(e, "case_sensitive")
                            .map(|v| v == "yes")
                            .unwrap_or(false);
                        token.negation =
                            get_attr(e, "negate").map(|v| v == "yes").unwrap_or(false);
                        token.postag = get_attr(e, "postag");
                        token.postag_regexp = get_attr(e, "postag_regexp")
                            .map(|v| v == "yes")
                            .unwrap_or(false);
                        token.regexp = get_attr(e, "regexp");
                        token.min = get_attr(e, "min")
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(1);
                        token.max = get_attr(e, "max")
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(1);
                        current_token = Some(token);
                        in_token = true;
                        text_buffer.clear();
                    }
                    "message" => {
                        in_message = true;
                        text_buffer.clear();
                    }
                    "suggestion" => {
                        in_suggestion = true;
                        text_buffer.clear();
                    }
                    "example" => {
                        let correction = get_attr(e, "correction");
                        let is_correct = get_attr(e, "type")
                            .map(|v| v != "incorrect")
                            .unwrap_or(true);
                        current_example = Some(Example {
                            text: String::new(),
                            is_correct,
                            correction,
                        });
                        in_example = true;
                        text_buffer.clear();
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                match name.as_str() {
                    "rule" => {
                        if let Some(mut rule) = current_rule.take() {
                            rule.pattern = current_pattern.clone();
                            rules.push(rule);
                        }
                    }
                    "token" => {
                        if let Some(mut token) = current_token.take() {
                            if !text_buffer.is_empty() {
                                token.text = Some(text_buffer.clone());
                            }
                            current_pattern.push(token);
                        }
                        in_token = false;
                    }
                    "message" => {
                        if let Some(ref mut rule) = current_rule {
                            rule.message = text_buffer.trim().to_string();
                        }
                        in_message = false;
                    }
                    "suggestion" => {
                        if let Some(ref mut rule) = current_rule {
                            rule.suggestions.push(text_buffer.trim().to_string());
                        }
                        in_suggestion = false;
                    }
                    "example" => {
                        if let Some(mut example) = current_example.take() {
                            example.text = text_buffer.trim().to_string();
                            if let Some(ref mut rule) = current_rule {
                                rule.examples.push(example);
                            }
                        }
                        in_example = false;
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                if in_token || in_message || in_suggestion || in_example {
                    text_buffer.push_str(&text);
                }
            }
            Ok(Event::Empty(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if name == "token" {
                    let mut token = PatternToken::default();
                    token.inflected = get_attr(e, "inflected")
                        .map(|v| v == "yes")
                        .unwrap_or(false);
                    token.case_sensitive = get_attr(e, "case_sensitive")
                        .map(|v| v == "yes")
                        .unwrap_or(false);
                    token.negation = get_attr(e, "negate").map(|v| v == "yes").unwrap_or(false);
                    token.postag = get_attr(e, "postag");
                    token.postag_regexp = get_attr(e, "postag_regexp")
                        .map(|v| v == "yes")
                        .unwrap_or(false);
                    token.regexp = get_attr(e, "regexp");
                    token.min = get_attr(e, "min")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(1);
                    token.max = get_attr(e, "max")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(1);
                    // Empty token with just attributes (e.g., postag matching)
                    current_pattern.push(token);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("Error parsing XML at position {}: {:?}", reader.buffer_position(), e);
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(rules)
}

fn get_attr(e: &BytesStart, name: &str) -> Option<String> {
    e.attributes()
        .filter_map(|a| a.ok())
        .find(|a| a.key.as_ref() == name.as_bytes())
        .map(|a| String::from_utf8_lossy(&a.value).to_string())
}

fn filter_simple_rules(rules: Vec<XmlRule>) -> Vec<XmlRule> {
    rules
        .into_iter()
        .filter(|r| {
            is_simple_pattern(&r.pattern)
                && !r.message.is_empty()
                && !r.id.is_empty()  // Must have an ID
                && !r.suggestions.is_empty()  // Must have a suggestion
                && is_simple_suggestion(&r.suggestions)  // Suggestion must be simple
                && is_simple_message(&r.message)  // Message must be simple
        })
        .collect()
}

fn is_simple_suggestion(suggestions: &[String]) -> bool {
    suggestions.iter().all(|s| {
        // Reject suggestions with regex backreferences or special syntax
        !s.contains('\\')
            && !s.contains("$1")
            && !s.contains("$2")
            && !s.contains("<match")
            && !s.contains("<suggestion")
    })
}

fn is_simple_message(message: &str) -> bool {
    // Reject messages with regex backreferences or special syntax
    !message.contains("\\1")
        && !message.contains("\\2")
        && !message.contains("$1")
        && !message.contains("$2")
        && !message.contains("<match")
}

fn is_simple_pattern(tokens: &[PatternToken]) -> bool {
    // Only keep rules with 2-6 tokens, all with simple text (no regex, no postag)
    tokens.len() >= 2
        && tokens.len() <= 6
        && tokens.iter().all(|t| {
            if let Some(text) = &t.text {
                // Reject patterns with regex characters
                let has_regex = text.contains('|')
                    || text.contains('*')
                    || text.contains('+')
                    || text.contains('?')
                    || text.contains('[')
                    || text.contains('(');
                !has_regex
                    && t.postag.is_none()
                    && t.regexp.is_none()
                    && !t.negation
                    && t.min == 1
                    && t.max == 1
            } else {
                false
            }
        })
}

// ═══════════════════════════════════════════════════════════════════════════════
// Filter: POS pattern rules
// ═══════════════════════════════════════════════════════════════════════════════

fn filter_pos_pattern_rules(rules: &[XmlRule], lang: &str) -> Vec<PosPatternRule> {
    rules
        .iter()
        .filter_map(|r| {
            // Must have a valid ID, message, and suggestion
            if r.id.is_empty() || r.message.is_empty() || r.suggestions.is_empty() {
                return None;
            }

            // Must have at least one token with postag
            let has_postag = r.pattern.iter().any(|t| t.postag.is_some());
            if !has_postag {
                return None;
            }

            // Check if the pattern is compatible with our POS system
            if !is_pos_pattern_compatible(&r.pattern, lang) {
                return None;
            }

            // Check if suggestions are simple
            if !is_simple_suggestion(&r.suggestions) {
                return None;
            }

            // Check if message is simple
            if !is_simple_message(&r.message) {
                return None;
            }

            // Convert to PosPatternRule
            let pattern: Vec<PosPatternElement> = r
                .pattern
                .iter()
                .filter_map(|t| {
                    // Skip tokens with unsupported features
                    if t.min != 1 || t.max != 1 || t.regexp.is_some() {
                        // Allow min=0 tokens by skipping them (optional tokens)
                        if t.min == 0 {
                            return None;
                        }
                        return None;
                    }

                    // Get text (case-insensitive) if present
                    let text = t.text.as_ref().map(|s| s.to_lowercase());

                    // Skip tokens with regex in text
                    if let Some(ref txt) = text {
                        if txt.contains('|')
                            || txt.contains('*')
                            || txt.contains('+')
                            || txt.contains('?')
                            || txt.contains('[')
                            || txt.contains('(')
                        {
                            return None;
                        }
                    }

                    Some(PosPatternElement {
                        text,
                        pos_pattern: t.postag.clone(),
                        negation: t.negation,
                    })
                })
                .collect();

            // Need at least 2 elements after filtering
            if pattern.len() < 2 {
                return None;
            }

            // Need at least one token with POS pattern
            if !pattern.iter().any(|e| e.pos_pattern.is_some()) {
                return None;
            }

            Some(PosPatternRule {
                id: r.id.clone(),
                pattern,
                message: r.message.clone(),
                suggestions: r.suggestions.clone(),
            })
        })
        .collect()
}

fn is_pos_pattern_compatible(tokens: &[PatternToken], lang: &str) -> bool {
    // Must have 2-8 tokens
    if tokens.len() < 2 || tokens.len() > 8 {
        return false;
    }

    // All tokens must be compatible
    tokens.iter().all(|t| is_token_pos_compatible(t, lang))
}

fn is_token_pos_compatible(token: &PatternToken, lang: &str) -> bool {
    // Skip optional tokens (they'll be filtered out)
    if token.min == 0 {
        return true;
    }

    // Must have min=1, max=1 (no repetition)
    if token.min != 1 || token.max != 1 {
        return false;
    }

    // No inflected forms (requires morphological analysis)
    if token.inflected {
        return false;
    }

    // Check postag if present
    if let Some(ref postag) = token.postag {
        // Check if we support this postag
        if token.postag_regexp {
            // Regex postag - check if it's a supported pattern
            if !is_supported_postag_pattern(postag, lang) {
                return false;
            }
        } else {
            // Exact postag - check if we have a mapping
            if !is_supported_postag(postag, lang) {
                return false;
            }
        }
    }

    // Check text if present
    if let Some(ref text) = token.text {
        // No regex in text
        if token.regexp.is_some() {
            return false;
        }
        // No regex patterns in text
        if text.contains('|')
            || text.contains('*')
            || text.contains('+')
            || text.contains('?')
            || text.contains('[')
            || text.contains('(')
        {
            return false;
        }
    }

    true
}

fn is_supported_postag_pattern(pattern: &str, lang: &str) -> bool {
    match lang {
        "en" => {
            // Penn Treebank patterns
            // Supported: NN.*, VB.*, JJ.*, RB.*, exact tags
            pattern.starts_with("NN")
                || pattern.starts_with("VB")
                || pattern.starts_with("JJ")
                || pattern.starts_with("RB")
                || pattern.starts_with("DT")
                || pattern.starts_with("IN")
                || pattern.starts_with("CC")
                || pattern.starts_with("MD")
                || pattern.starts_with("PRP")
                || pattern.starts_with("WP")
                || pattern.starts_with("WDT")
                || pattern.starts_with("WRB")
                || pattern.starts_with("CD")
                || pattern.starts_with("TO")
                || pattern.starts_with("RP")
                || pattern.starts_with("SENT")
                || pattern == ".*" // any tag
        }
        "fr" => {
            // French tagset patterns
            // Supported: V.*, N.*, A, D.*, R.*, P.*, etc.
            pattern.starts_with('V')
                || pattern.starts_with('N')
                || pattern.starts_with('A')
                || pattern.starts_with('D')
                || pattern.starts_with('R')
                || pattern.starts_with('P')
                || pattern.starts_with('C')
                || pattern.starts_with('J')
                || pattern.starts_with('Z')
                || pattern.starts_with("SENT")
                || pattern == ".*"
        }
        _ => false,
    }
}

fn is_supported_postag(tag: &str, lang: &str) -> bool {
    match lang {
        "en" => {
            // Penn Treebank exact tags
            matches!(
                tag,
                "CC" | "CD"
                    | "DT"
                    | "EX"
                    | "FW"
                    | "IN"
                    | "JJ"
                    | "JJR"
                    | "JJS"
                    | "NN"
                    | "NNS"
                    | "NNP"
                    | "NNPS"
                    | "MD"
                    | "PDT"
                    | "POS"
                    | "PRP"
                    | "PRP$"
                    | "RB"
                    | "RBR"
                    | "RBS"
                    | "RP"
                    | "TO"
                    | "UH"
                    | "VB"
                    | "VBD"
                    | "VBG"
                    | "VBN"
                    | "VBP"
                    | "VBZ"
                    | "WDT"
                    | "WP"
                    | "WP$"
                    | "WRB"
                    | "SENT_START"
                    | "SENT_END"
            )
        }
        "fr" => {
            // French tags are more complex with spaces
            // Accept any tag that starts with known categories
            tag.starts_with('V')
                || tag.starts_with('N')
                || tag.starts_with('A')
                || tag.starts_with('D')
                || tag.starts_with('R')
                || tag.starts_with('P')
                || tag.starts_with('C')
                || tag.starts_with('J')
                || tag.starts_with('Z')
                || tag.starts_with("SENT")
                || tag == "UNKNOWN"
        }
        _ => false,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: confusion_sets.txt
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_confusion_sets(path: &Path) -> Result<Vec<ConfusionPair>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut pairs = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Remove inline comments
        let line = if let Some(idx) = line.find('#') {
            line[..idx].trim()
        } else {
            line
        };

        // Parse format: word1 -> word2; factor   OR   word1; word2; factor
        let bidirectional = !line.contains("->");

        let parts: Vec<&str> = if bidirectional {
            line.split(';').map(|s| s.trim()).collect()
        } else {
            // word1 -> word2; factor
            let arrow_split: Vec<&str> = line.split("->").collect();
            if arrow_split.len() != 2 {
                continue;
            }
            let word1 = arrow_split[0].trim();
            let rest: Vec<&str> = arrow_split[1].split(';').map(|s| s.trim()).collect();
            if rest.len() < 2 {
                continue;
            }
            vec![word1, rest[0], rest[1]]
        };

        if parts.len() < 3 {
            continue;
        }

        let word1 = parts[0].to_lowercase();
        let word2 = parts[1].to_lowercase();
        let factor: u64 = parts[2].parse().unwrap_or(1000);

        // Skip pairs with empty words
        if word1.is_empty() || word2.is_empty() {
            continue;
        }

        pairs.push(ConfusionPair {
            word1,
            word2,
            factor,
            bidirectional,
        });
    }

    Ok(pairs)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: replace.txt
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_replace_txt(path: &Path) -> Result<Vec<ReplaceRule>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut rules = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Remove inline comments
        let line = if let Some(idx) = line.find('#') {
            line[..idx].trim()
        } else {
            line
        };

        // Format: wrong=correct or wrong=correct\texplanation
        if let Some(eq_idx) = line.find('=') {
            let wrong = line[..eq_idx].trim().to_lowercase();
            let correct_part = line[eq_idx + 1..].trim();

            // Handle tab-separated explanation (ignore it, just take the correction)
            let correct = if let Some(tab_idx) = correct_part.find('\t') {
                correct_part[..tab_idx].trim().to_lowercase()
            } else {
                correct_part.to_lowercase()
            };

            // Skip entries with regex patterns in wrong part
            let has_regex = wrong.contains('|')
                || wrong.contains('*')
                || wrong.contains('+')
                || wrong.contains('?')
                || wrong.contains('[')
                || wrong.contains('(');

            if !wrong.is_empty() && !correct.is_empty() && !has_regex {
                rules.push(ReplaceRule { wrong, correct });
            }
        }
    }

    Ok(rules)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: wordiness.txt and redundancies.txt
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_style_file(path: &Path, category: StyleCategory) -> Result<Vec<StyleRule>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut rules = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Format: phrase=suggestion or phrase=suggestion1|suggestion2
        if let Some(eq_idx) = line.find('=') {
            let phrase = line[..eq_idx].trim().to_lowercase();
            let suggestions_part = line[eq_idx + 1..].trim();

            // Skip entries with special syntax we can't handle
            if phrase.contains('|') || phrase.contains('[') || phrase.contains('(') {
                continue;
            }

            // Parse suggestions (can be pipe-separated)
            let suggestions: Vec<String> = suggestions_part
                .split('|')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty() && s != "[OMIT]")
                .collect();

            // Skip if no usable suggestions (only [OMIT] or empty)
            // But keep rules where [OMIT] means "remove the phrase"
            let has_omit = suggestions_part.contains("[OMIT]");

            if !phrase.is_empty() && (!suggestions.is_empty() || has_omit) {
                rules.push(StyleRule {
                    phrase,
                    suggestions: if suggestions.is_empty() && has_omit {
                        vec!["".to_string()] // Empty suggestion means "remove"
                    } else {
                        suggestions
                    },
                    category,
                });
            }
        }
    }

    Ok(rules)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: custom redundancies (pléonasmes)
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_custom_redundancies(path: &Path) -> Result<Vec<StyleRule>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut rules = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Format: phrase=suggestion1|suggestion2 or just phrase (for removal)
        if let Some(eq_idx) = line.find('=') {
            let phrase = line[..eq_idx].trim().to_lowercase();
            let suggestions_part = line[eq_idx + 1..].trim();

            // Parse suggestions (can be pipe-separated)
            let suggestions: Vec<String> = suggestions_part
                .split('|')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            if !phrase.is_empty() {
                rules.push(StyleRule {
                    phrase,
                    suggestions: if suggestions.is_empty() {
                        vec!["".to_string()] // Empty suggestion means "remove"
                    } else {
                        suggestions
                    },
                    category: StyleCategory::Redundancy,
                });
            }
        } else {
            // Just phrase without = means remove the phrase
            let phrase = line.to_lowercase();
            if !phrase.is_empty() {
                rules.push(StyleRule {
                    phrase,
                    suggestions: vec!["".to_string()],
                    category: StyleCategory::Redundancy,
                });
            }
        }
    }

    Ok(rules)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: patterns
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_patterns_file(rules: &[XmlRule], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    // Header
    output.push_str(&format!(
        "//! Auto-generated by sync-lt from LanguageTool\n\
         //! Source: {}/grammar.xml\n\
         //! Synced: {}\n\
         //! Total rules: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\n\
         use crate::checker::PatternRule;\n\n",
        lang,
        timestamp,
        rules.len()
    ));

    // Generate the array
    output.push_str(&format!(
        "pub const {}_PATTERN_RULES: &[PatternRule] = &[\n",
        lang.to_uppercase()
    ));

    for rule in rules {
        let pattern: Vec<_> = rule
            .pattern
            .iter()
            .filter_map(|t| t.text.as_ref())
            .map(|s| format!("\"{}\"", escape_string(&s.to_lowercase())))
            .collect();

        let suggestion = rule
            .suggestions
            .first()
            .map(|s| escape_string(s))
            .unwrap_or_default();

        let message = escape_string(&rule.message.replace('\n', " "));

        output.push_str(&format!(
            "    PatternRule {{\n\
             \t\tid: \"{}\",\n\
             \t\tpattern: &[{}],\n\
             \t\tsuggestion: \"{}\",\n\
             \t\tmessage: \"{}\",\n\
             \t}},\n",
            escape_string(&rule.id),
            pattern.join(", "),
            suggestion,
            message,
        ));
    }

    output.push_str("];\n");
    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: POS patterns
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_pos_patterns_file(rules: &[PosPatternRule], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    // Header
    output.push_str(&format!(
        "//! Auto-generated POS pattern rules for {} from LanguageTool\n\
         //! Source: {}/grammar.xml\n\
         //! Synced: {}\n\
         //! Total rules: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! These rules use POS (Part-of-Speech) tagging to match patterns.\n\
         //! They require the PosPatternChecker to be enabled.\n\n\
         use crate::checker::pos_pattern_checker::{{PosPatternRule, PosPatternElement}};\n\n",
        lang.to_uppercase(),
        lang,
        timestamp,
        rules.len()
    ));

    // Generate static pattern arrays for each rule
    for (idx, rule) in rules.iter().enumerate() {
        output.push_str(&format!(
            "static PATTERN_{}: &[PosPatternElement] = &[\n",
            idx
        ));
        for elem in &rule.pattern {
            let text = match &elem.text {
                Some(t) => format!("Some(\"{}\")", escape_string(t)),
                None => "None".to_string(),
            };
            let pos = match &elem.pos_pattern {
                Some(p) => format!("Some(\"{}\")", escape_string(p)),
                None => "None".to_string(),
            };
            output.push_str(&format!(
                "    PosPatternElement {{ text: {}, pos_pattern: {}, negation: {} }},\n",
                text, pos, elem.negation
            ));
        }
        output.push_str("];\n\n");
    }

    // Generate the main array
    output.push_str(&format!(
        "/// POS pattern rules for {} (requires POS tagging)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!(
        "pub static {}_POS_PATTERN_RULES: &[PosPatternRule] = &[\n",
        lang.to_uppercase()
    ));

    for (idx, rule) in rules.iter().enumerate() {
        let suggestion = rule
            .suggestions
            .first()
            .map(|s| escape_string(s))
            .unwrap_or_default();
        let message = escape_string(&rule.message.replace('\n', " "));
        let suggestions = if rule.suggestions.is_empty() {
            "&[]".to_string()
        } else {
            format!("&[\"{}\"]", suggestion)
        };

        output.push_str(&format!(
            "    PosPatternRule {{\n\
             \t\tid: \"{}\",\n\
             \t\tpattern: PATTERN_{},\n\
             \t\tmessage: \"{}\",\n\
             \t\tsuggestions: {},\n\
             \t}},\n",
            escape_string(&rule.id),
            idx,
            message,
            suggestions,
        ));
    }

    output.push_str("];\n\n");

    // Generate helper to create a checker
    output.push_str(&format!(
        "/// Create a PosPatternChecker with {} rules\n\
         pub fn create_{}_pos_pattern_checker() -> crate::checker::pos_pattern_checker::PosPatternChecker {{\n\
         \tcrate::checker::pos_pattern_checker::PosPatternChecker::with_rules({}_POS_PATTERN_RULES)\n\
         }}\n",
        lang.to_uppercase(),
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: confusion
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_confusion_file(pairs: &[ConfusionPair], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated confusion pairs for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool confusion_sets.txt\n\
         //! License: LGPL 2.1+\n\n",
        lang.to_uppercase(),
        timestamp
    ));

    // Group pairs by word1
    let mut grouped: HashMap<String, Vec<(String, u64)>> = HashMap::new();

    for pair in pairs {
        grouped
            .entry(pair.word1.clone())
            .or_default()
            .push((pair.word2.clone(), pair.factor));

        // For bidirectional pairs, add reverse
        if pair.bidirectional {
            grouped
                .entry(pair.word2.clone())
                .or_default()
                .push((pair.word1.clone(), pair.factor));
        }
    }

    // Sort for deterministic output
    let mut keys: Vec<_> = grouped.keys().cloned().collect();
    keys.sort();

    // Generate as a static slice of tuples for zero-cost lookup
    output.push_str(&format!(
        "/// Confusion pairs for {}\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!("/// Total unique words: {}\n", keys.len()));
    output.push_str("/// Format: (source_word, &[(target_word, factor)])\n");
    output.push_str(&format!(
        "pub const {}_CONFUSION_DATA: &[(&str, &[(&str, u64)])] = &[\n",
        lang.to_uppercase()
    ));

    for key in &keys {
        let targets = grouped.get(key).unwrap();
        output.push_str(&format!("    (\"{}\", &[", key));
        for (i, (target, factor)) in targets.iter().enumerate() {
            if i > 0 {
                output.push_str(", ");
            }
            output.push_str(&format!("(\"{}\", {})", target, factor));
        }
        output.push_str("]),\n");
    }

    output.push_str("];\n\n");

    // Generate a binary search lookup function
    output.push_str("/// Check if word might be confused with another word (binary search)\n");
    output.push_str(&format!(
        "pub fn get_{}_confusions(word: &str) -> Option<&'static [(&'static str, u64)]> {{\n",
        lang.to_lowercase()
    ));
    output.push_str(&format!(
        "    {}_CONFUSION_DATA\n",
        lang.to_uppercase()
    ));
    output.push_str("        .binary_search_by_key(&word, |(w, _)| *w)\n");
    output.push_str("        .ok()\n");
    output.push_str(&format!(
        "        .map(|idx| {}_CONFUSION_DATA[idx].1)\n",
        lang.to_uppercase()
    ));
    output.push_str("}\n\n");

    // Generate stats as a regular comment
    let total_pairs: usize = grouped.values().map(|v| v.len()).sum();
    output.push_str(&format!(
        "// Statistics: {} unique words, {} total confusion mappings\n",
        keys.len(),
        total_pairs
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: replace
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_replace_file(rules: &[ReplaceRule], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated replace rules for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total rules: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool replace.txt\n\
         //! License: LGPL 2.1+\n\n",
        lang.to_uppercase(),
        timestamp,
        rules.len()
    ));

    // Sort for binary search
    let mut sorted_rules: Vec<_> = rules.to_vec();
    sorted_rules.sort_by(|a, b| a.wrong.cmp(&b.wrong));

    // Generate as a static slice
    output.push_str(&format!(
        "/// Replace rules for {}\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!("/// Total rules: {}\n", sorted_rules.len()));
    output.push_str("/// Format: (wrong, correct)\n");
    output.push_str(&format!(
        "pub const {}_REPLACE_RULES: &[(&str, &str)] = &[\n",
        lang.to_uppercase()
    ));

    for rule in &sorted_rules {
        output.push_str(&format!(
            "    (\"{}\", \"{}\"),\n",
            escape_string(&rule.wrong),
            escape_string(&rule.correct)
        ));
    }

    output.push_str("];\n\n");

    // Generate lookup function
    output.push_str("/// Get replacement for a potentially incorrect word (binary search)\n");
    output.push_str(&format!(
        "pub fn get_{}_replacement(word: &str) -> Option<&'static str> {{\n",
        lang.to_lowercase()
    ));
    output.push_str(&format!(
        "    {}_REPLACE_RULES\n",
        lang.to_uppercase()
    ));
    output.push_str("        .binary_search_by_key(&word, |(w, _)| *w)\n");
    output.push_str("        .ok()\n");
    output.push_str(&format!(
        "        .map(|idx| {}_REPLACE_RULES[idx].1)\n",
        lang.to_uppercase()
    ));
    output.push_str("}\n");

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: coherency.txt
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_coherency_file(path: &Path) -> Result<Vec<CoherencyPair>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut pairs = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Format: variant1;variant2 or variant1;variant2;variant3...
        let variants: Vec<String> = line
            .split(';')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // Need at least 2 variants
        if variants.len() >= 2 {
            pairs.push(CoherencyPair { variants });
        }
    }

    Ok(pairs)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: coherency
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_coherency_file(pairs: &[CoherencyPair], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    // Count total unique words
    let total_words: usize = pairs.iter().map(|p| p.variants.len()).sum();

    output.push_str(&format!(
        "//! Auto-generated coherency pairs for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total pairs: {} ({} unique words)\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool coherency.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! These are spelling variants (UK/US, hyphenation, etc.) that should\n\
         //! be consistent within a document.\n\n",
        lang.to_uppercase(),
        timestamp,
        pairs.len(),
        total_words
    ));

    // Generate a struct to hold pair info
    output.push_str("/// A coherency pair - multiple valid spellings that should be consistent\n");
    output.push_str("#[derive(Debug, Clone)]\n");
    output.push_str("pub struct CoherencyPair {\n");
    output.push_str("    /// The pair ID (index in the array)\n");
    output.push_str("    pub id: usize,\n");
    output.push_str("    /// All valid variant spellings\n");
    output.push_str("    pub variants: &'static [&'static str],\n");
    output.push_str("}\n\n");

    // Generate the pairs array
    output.push_str(&format!(
        "/// Coherency pairs for {} (spelling variants)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!(
        "pub const {}_COHERENCY_PAIRS: &[CoherencyPair] = &[\n",
        lang.to_uppercase()
    ));

    for (idx, pair) in pairs.iter().enumerate() {
        let variants: Vec<_> = pair.variants.iter()
            .map(|v| format!("\"{}\"", escape_string(v)))
            .collect();

        output.push_str(&format!(
            "    CoherencyPair {{ id: {}, variants: &[{}] }},\n",
            idx,
            variants.join(", "),
        ));
    }

    output.push_str("];\n\n");

    // Build a word -> pair_id lookup map
    // This is more efficient for runtime lookup
    output.push_str(&format!(
        "/// Lookup table: word -> pair index\n\
         /// Sorted for binary search\n\
         pub const {}_COHERENCY_LOOKUP: &[(&str, usize)] = &[\n",
        lang.to_uppercase()
    ));

    // Build and sort the lookup entries
    let mut lookups: Vec<(String, usize)> = Vec::new();
    for (idx, pair) in pairs.iter().enumerate() {
        for variant in &pair.variants {
            lookups.push((variant.clone(), idx));
        }
    }
    lookups.sort_by(|a, b| a.0.cmp(&b.0));

    for (word, idx) in &lookups {
        output.push_str(&format!(
            "    (\"{}\", {}),\n",
            escape_string(word),
            idx
        ));
    }

    output.push_str("];\n\n");

    // Generate lookup function
    output.push_str(&format!(
        "/// Find the coherency pair for a word (binary search)\n\
         /// Returns the pair ID if the word is part of a coherency pair\n\
         pub fn get_{}_coherency_pair(word: &str) -> Option<usize> {{\n\
         \t{}_COHERENCY_LOOKUP\n\
         \t\t.binary_search_by_key(&word, |(w, _)| *w)\n\
         \t\t.ok()\n\
         \t\t.map(|idx| {}_COHERENCY_LOOKUP[idx].1)\n\
         }}\n\n",
        lang.to_lowercase(),
        lang.to_uppercase(),
        lang.to_uppercase()
    ));

    // Generate helper to get variants for a pair
    output.push_str(&format!(
        "/// Get all variants for a pair ID\n\
         pub fn get_{}_coherency_variants(pair_id: usize) -> Option<&'static [&'static str]> {{\n\
         \t{}_COHERENCY_PAIRS.get(pair_id).map(|p| p.variants)\n\
         }}\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: common_words.txt (for language detection)
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_common_words_file(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let word = line.trim().to_lowercase();

        // Skip comments and empty lines
        if word.is_empty() || word.starts_with('#') {
            continue;
        }

        // Only keep words (skip numbers, dates, etc.)
        // A word must contain at least one letter
        if word.chars().any(|c| c.is_alphabetic()) {
            words.push(word);
        }
    }

    // Sort for binary search
    words.sort();
    words.dedup();

    Ok(words)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: common_words
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_common_words_file(words: &[String], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated common words for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total words: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool common_words.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! These words are used for language detection.\n\n",
        lang.to_uppercase(),
        timestamp,
        words.len()
    ));

    // Generate as a static slice (sorted for binary search)
    output.push_str(&format!(
        "/// Common words for {} (sorted for binary search)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!("/// Total words: {}\n", words.len()));
    output.push_str(&format!(
        "pub const {}_COMMON_WORDS: &[&str] = &[\n",
        lang.to_uppercase()
    ));

    for word in words {
        output.push_str(&format!("    \"{}\",\n", escape_string(word)));
    }

    output.push_str("];\n\n");

    // Generate lookup function
    output.push_str(&format!(
        "/// Check if a word is a common {} word (binary search)\n\
         pub fn is_{}_common_word(word: &str) -> bool {{\n\
         \t{}_COMMON_WORDS.binary_search(&word).is_ok()\n\
         }}\n",
        lang.to_uppercase(),
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: diacritics.txt
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_diacritics_file(path: &Path) -> Result<Vec<DiacriticsRule>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut rules = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Format: without=with (e.g., cafe=café)
        if let Some(eq_idx) = line.find('=') {
            let without = line[..eq_idx].trim().to_string();
            let with = line[eq_idx + 1..].trim().to_string();

            // Skip empty entries
            if !without.is_empty() && !with.is_empty() {
                rules.push(DiacriticsRule { without, with });
            }
        }
    }

    Ok(rules)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: diacritics
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_diacritics_file(rules: &[DiacriticsRule], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated diacritics rules for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total rules: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool diacritics.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! These rules suggest proper diacritics for words borrowed from other languages.\n\n",
        lang.to_uppercase(),
        timestamp,
        rules.len()
    ));

    // Sort for binary search
    let mut sorted_rules: Vec<_> = rules.to_vec();
    sorted_rules.sort_by(|a, b| a.without.cmp(&b.without));

    // Generate as a static slice
    output.push_str(&format!(
        "/// Diacritics rules for {} (word without diacritics -> word with diacritics)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!("/// Total rules: {}\n", sorted_rules.len()));
    output.push_str("/// Format: (without_diacritics, with_diacritics)\n");
    output.push_str(&format!(
        "pub const {}_DIACRITICS_RULES: &[(&str, &str)] = &[\n",
        lang.to_uppercase()
    ));

    for rule in &sorted_rules {
        output.push_str(&format!(
            "    (\"{}\", \"{}\"),\n",
            escape_string(&rule.without),
            escape_string(&rule.with)
        ));
    }

    output.push_str("];\n\n");

    // Generate lookup function
    output.push_str("/// Get the diacritics version of a word (binary search)\n");
    output.push_str(&format!(
        "pub fn get_{}_diacritics(word: &str) -> Option<&'static str> {{\n",
        lang.to_lowercase()
    ));
    output.push_str(&format!(
        "    {}_DIACRITICS_RULES\n",
        lang.to_uppercase()
    ));
    output.push_str("        .binary_search_by_key(&word, |(w, _)| *w)\n");
    output.push_str("        .ok()\n");
    output.push_str(&format!(
        "        .map(|idx| {}_DIACRITICS_RULES[idx].1)\n",
        lang.to_uppercase()
    ));
    output.push_str("}\n");

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: test examples
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
struct TestExample {
    rule_id: String,
    text: String,
    is_correct: bool,
    correction: Option<String>,
}

fn extract_test_examples(rules: &[XmlRule]) -> Vec<TestExample> {
    let mut examples = Vec::new();

    for rule in rules {
        for ex in &rule.examples {
            // Skip empty examples
            if ex.text.trim().is_empty() {
                continue;
            }

            // Clean up the text (remove <marker> tags if present)
            let clean_text = ex
                .text
                .replace("<marker>", "")
                .replace("</marker>", "")
                .trim()
                .to_string();

            // Skip if still empty after cleanup
            if clean_text.is_empty() {
                continue;
            }

            // Determine if this is a correct or incorrect example
            // In LanguageTool: examples without correction attr are correct
            // Examples with correction attr are incorrect (they contain the error)
            let is_correct = ex.correction.is_none();

            examples.push(TestExample {
                rule_id: rule.id.clone(),
                text: clean_text,
                is_correct,
                correction: ex.correction.clone(),
            });
        }
    }

    examples
}

fn generate_test_examples_file(examples: &[TestExample], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    let incorrect_count = examples.iter().filter(|e| !e.is_correct).count();
    let correct_count = examples.iter().filter(|e| e.is_correct).count();

    // Header
    output.push_str(&format!(
        "//! Auto-generated test examples for {} pattern rules from LanguageTool\n\
         //! Synced: {}\n\
         //! Total examples: {} ({} incorrect, {} correct)\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! These examples are extracted from LanguageTool's grammar.xml\n\
         //! - Incorrect examples: should trigger the rule\n\
         //! - Correct examples: should NOT trigger the rule\n\n",
        lang.to_uppercase(),
        timestamp,
        examples.len(),
        incorrect_count,
        correct_count
    ));

    // Generate test data structure
    output.push_str("/// A test example for pattern rule validation\n");
    output.push_str("#[derive(Debug, Clone)]\n");
    output.push_str("pub struct PatternTestExample {\n");
    output.push_str("    pub rule_id: &'static str,\n");
    output.push_str("    pub text: &'static str,\n");
    output.push_str("    pub is_correct: bool,\n");
    output.push_str("    pub correction: Option<&'static str>,\n");
    output.push_str("}\n\n");

    // Generate the array
    output.push_str(&format!(
        "/// Test examples for {} pattern rules\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!(
        "pub const {}_PATTERN_TEST_EXAMPLES: &[PatternTestExample] = &[\n",
        lang.to_uppercase()
    ));

    for ex in examples {
        let correction = match &ex.correction {
            Some(c) => format!("Some(\"{}\")", escape_string(c)),
            None => "None".to_string(),
        };

        output.push_str(&format!(
            "    PatternTestExample {{\n\
             \t\trule_id: \"{}\",\n\
             \t\ttext: \"{}\",\n\
             \t\tis_correct: {},\n\
             \t\tcorrection: {},\n\
             \t}},\n",
            escape_string(&ex.rule_id),
            escape_string(&ex.text),
            ex.is_correct,
            correction,
        ));
    }

    output.push_str("];\n\n");

    // Generate helper functions
    output.push_str(&format!(
        "/// Get all incorrect examples (should trigger rules)\n\
         pub fn get_{}_incorrect_examples() -> Vec<&'static PatternTestExample> {{\n\
         \t{}_PATTERN_TEST_EXAMPLES.iter().filter(|e| !e.is_correct).collect()\n\
         }}\n\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output.push_str(&format!(
        "/// Get all correct examples (should NOT trigger rules)\n\
         pub fn get_{}_correct_examples() -> Vec<&'static PatternTestExample> {{\n\
         \t{}_PATTERN_TEST_EXAMPLES.iter().filter(|e| e.is_correct).collect()\n\
         }}\n\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output.push_str(&format!(
        "/// Get examples for a specific rule\n\
         pub fn get_{}_examples_for_rule(rule_id: &str) -> Vec<&'static PatternTestExample> {{\n\
         \t{}_PATTERN_TEST_EXAMPLES.iter().filter(|e| e.rule_id == rule_id).collect()\n\
         }}\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: style rules (wordiness + redundancies)
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_style_file(rules: &[StyleRule], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    let wordiness_count = rules.iter().filter(|r| r.category == StyleCategory::Wordiness).count();
    let redundancy_count = rules.iter().filter(|r| r.category == StyleCategory::Redundancy).count();

    output.push_str(&format!(
        "//! Auto-generated style rules for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total rules: {} ({} wordiness, {} redundancy)\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool wordiness.txt and redundancies.txt\n\
         //! License: LGPL 2.1+\n\n",
        lang.to_uppercase(),
        timestamp,
        rules.len(),
        wordiness_count,
        redundancy_count
    ));

    // Sort by phrase for deterministic output and efficient lookup
    let mut sorted_rules: Vec<_> = rules.to_vec();
    sorted_rules.sort_by(|a, b| a.phrase.cmp(&b.phrase));

    // For non-English languages, import types from en_style
    if lang.to_lowercase() != "en" {
        output.push_str("use super::en_style::{StyleCategory, StyleRule};\n\n");
    } else {
        // Generate the category enum
        output.push_str("/// Style rule category\n");
        output.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
        output.push_str("pub enum StyleCategory {\n");
        output.push_str("    /// Wordy phrases that can be simplified\n");
        output.push_str("    Wordiness,\n");
        output.push_str("    /// Redundant/pleonastic phrases\n");
        output.push_str("    Redundancy,\n");
        output.push_str("}\n\n");

        // Generate the rule struct
        output.push_str("/// A style rule for detecting wordy or redundant phrases\n");
        output.push_str("#[derive(Debug, Clone)]\n");
        output.push_str("pub struct StyleRule {\n");
        output.push_str("    pub phrase: &'static str,\n");
        output.push_str("    pub suggestions: &'static [&'static str],\n");
        output.push_str("    pub category: StyleCategory,\n");
        output.push_str("}\n\n");
    }

    // Generate the array
    output.push_str(&format!(
        "/// Style rules for {} (wordiness + redundancy)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!(
        "pub const {}_STYLE_RULES: &[StyleRule] = &[\n",
        lang.to_uppercase()
    ));

    for rule in &sorted_rules {
        let suggestions: Vec<_> = rule.suggestions.iter()
            .map(|s| format!("\"{}\"", escape_string(s)))
            .collect();

        let category = match rule.category {
            StyleCategory::Wordiness => "StyleCategory::Wordiness",
            StyleCategory::Redundancy => "StyleCategory::Redundancy",
        };

        output.push_str(&format!(
            "    StyleRule {{ phrase: \"{}\", suggestions: &[{}], category: {} }},\n",
            escape_string(&rule.phrase),
            suggestions.join(", "),
            category,
        ));
    }

    output.push_str("];\n\n");

    // Generate helper functions
    output.push_str(&format!(
        "/// Get all wordiness rules\n\
         pub fn get_{}_wordiness_rules() -> impl Iterator<Item = &'static StyleRule> {{\n\
         \t{}_STYLE_RULES.iter().filter(|r| r.category == StyleCategory::Wordiness)\n\
         }}\n\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output.push_str(&format!(
        "/// Get all redundancy rules\n\
         pub fn get_{}_redundancy_rules() -> impl Iterator<Item = &'static StyleRule> {{\n\
         \t{}_STYLE_RULES.iter().filter(|r| r.category == StyleCategory::Redundancy)\n\
         }}\n\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    // Generate a list of all phrases for Aho-Corasick
    output.push_str(&format!(
        "/// Get all style phrases (for Aho-Corasick building)\n\
         pub fn get_{}_style_phrases() -> impl Iterator<Item = &'static str> {{\n\
         \t{}_STYLE_RULES.iter().map(|r| r.phrase)\n\
         }}\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Update data/mod.rs
// ═══════════════════════════════════════════════════════════════════════════════

fn update_data_mod(output_dir: &Path, lang: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mod_path = output_dir.join("mod.rs");

    // Read existing content or create new
    let mut content = if mod_path.exists() {
        fs::read_to_string(&mod_path)?
    } else {
        "//! Auto-generated confusion data from LanguageTool\n\
         //!\n\
         //! This module contains data imported from LanguageTool.\n\
         //! The data is used for detecting commonly confused words,\n\
         //! pattern-based rules, and simple replacements.\n\n"
            .to_string()
    };

    // Add modules if not present
    let patterns_mod = format!("pub mod {}_patterns;", lang);
    let pos_patterns_mod = format!("pub mod {}_pos_patterns;", lang);
    let confusion_mod = format!("pub mod {}_confusion;", lang);
    let replace_mod = format!("pub mod {}_replace;", lang);
    let pattern_tests_mod = format!("pub mod {}_pattern_tests;", lang);
    let style_mod = format!("pub mod {}_style;", lang);
    let coherency_mod = format!("pub mod {}_coherency;", lang);
    let diacritics_mod = format!("pub mod {}_diacritics;", lang);
    let common_words_mod = format!("pub mod {}_common_words;", lang);
    let contractions_mod = format!("pub mod {}_contractions;", lang);
    let determiners_mod = format!("pub mod {}_determiners;", lang);
    let context_words_mod = format!("pub mod {}_context_words;", lang);
    let synonyms_mod = format!("pub mod {}_synonyms;", lang);
    let confusion_extended_mod = format!("pub mod {}_confusion_extended;", lang);
    let uncountable_mod = format!("pub mod {}_uncountable;", lang);
    let partlycountable_mod = format!("pub mod {}_partlycountable;", lang);
    let proper_nouns_mod = format!("pub mod {}_proper_nouns;", lang);
    let compounds_mod = format!("pub mod {}_compounds;", lang);
    let multiwords_mod = format!("pub mod {}_multiwords;", lang);
    let hyphenated_mod = format!("pub mod {}_hyphenated;", lang);
    let spelling_mod = format!("pub mod {}_spelling;", lang);
    let ignore_mod = format!("pub mod {}_ignore;", lang);
    // Phase 4 modules
    let word_definitions_mod = format!("pub mod {}_word_definitions;", lang);
    let prohibit_mod = format!("pub mod {}_prohibit;", lang);
    let us_gb_mod = format!("pub mod {}_us_gb;", lang);
    let l2_de_mod = format!("pub mod {}_confusion_l2_de;", lang);
    let l2_es_mod = format!("pub mod {}_confusion_l2_es;", lang);
    let l2_fr_mod = format!("pub mod {}_confusion_l2_fr;", lang);
    let l2_nl_mod = format!("pub mod {}_confusion_l2_nl;", lang);
    let added_mod = format!("pub mod {}_added;", lang);
    let numbers_mod = format!("pub mod {}_numbers;", lang);

    // Check if files exist
    let patterns_exists = output_dir.join(format!("{}_patterns.rs", lang)).exists();
    let pos_patterns_exists = output_dir.join(format!("{}_pos_patterns.rs", lang)).exists();
    let confusion_exists = output_dir.join(format!("{}_confusion.rs", lang)).exists();
    let replace_exists = output_dir.join(format!("{}_replace.rs", lang)).exists();
    let pattern_tests_exists = output_dir.join(format!("{}_pattern_tests.rs", lang)).exists();
    let style_exists = output_dir.join(format!("{}_style.rs", lang)).exists();
    let coherency_exists = output_dir.join(format!("{}_coherency.rs", lang)).exists();
    let diacritics_exists = output_dir.join(format!("{}_diacritics.rs", lang)).exists();
    let common_words_exists = output_dir.join(format!("{}_common_words.rs", lang)).exists();
    let contractions_exists = output_dir.join(format!("{}_contractions.rs", lang)).exists();
    let determiners_exists = output_dir.join(format!("{}_determiners.rs", lang)).exists();
    let context_words_exists = output_dir.join(format!("{}_context_words.rs", lang)).exists();
    let synonyms_exists = output_dir.join(format!("{}_synonyms.rs", lang)).exists();
    let confusion_extended_exists = output_dir.join(format!("{}_confusion_extended.rs", lang)).exists();
    let uncountable_exists = output_dir.join(format!("{}_uncountable.rs", lang)).exists();
    let partlycountable_exists = output_dir.join(format!("{}_partlycountable.rs", lang)).exists();
    let proper_nouns_exists = output_dir.join(format!("{}_proper_nouns.rs", lang)).exists();
    let compounds_exists = output_dir.join(format!("{}_compounds.rs", lang)).exists();
    let multiwords_exists = output_dir.join(format!("{}_multiwords.rs", lang)).exists();
    let hyphenated_exists = output_dir.join(format!("{}_hyphenated.rs", lang)).exists();
    let spelling_exists = output_dir.join(format!("{}_spelling.rs", lang)).exists();
    let ignore_exists = output_dir.join(format!("{}_ignore.rs", lang)).exists();
    // Phase 4 file checks
    let word_definitions_exists = output_dir.join(format!("{}_word_definitions.rs", lang)).exists();
    let prohibit_exists = output_dir.join(format!("{}_prohibit.rs", lang)).exists();
    let us_gb_exists = output_dir.join(format!("{}_us_gb.rs", lang)).exists();
    let l2_de_exists = output_dir.join(format!("{}_confusion_l2_de.rs", lang)).exists();
    let l2_es_exists = output_dir.join(format!("{}_confusion_l2_es.rs", lang)).exists();
    let l2_fr_exists = output_dir.join(format!("{}_confusion_l2_fr.rs", lang)).exists();
    let l2_nl_exists = output_dir.join(format!("{}_confusion_l2_nl.rs", lang)).exists();
    let added_exists = output_dir.join(format!("{}_added.rs", lang)).exists();
    let numbers_exists = output_dir.join(format!("{}_numbers.rs", lang)).exists();

    if patterns_exists && !content.contains(&patterns_mod) {
        content.push_str(&format!("{}\n", patterns_mod));
    }

    if pos_patterns_exists && !content.contains(&pos_patterns_mod) {
        content.push_str(&format!("{}\n", pos_patterns_mod));
    }

    if confusion_exists && !content.contains(&confusion_mod) {
        content.push_str(&format!("{}\n", confusion_mod));
    }

    if replace_exists && !content.contains(&replace_mod) {
        content.push_str(&format!("{}\n", replace_mod));
    }

    if pattern_tests_exists && !content.contains(&pattern_tests_mod) {
        content.push_str(&format!("{}\n", pattern_tests_mod));
    }

    if style_exists && !content.contains(&style_mod) {
        content.push_str(&format!("{}\n", style_mod));
    }

    if coherency_exists && !content.contains(&coherency_mod) {
        content.push_str(&format!("{}\n", coherency_mod));
    }

    if diacritics_exists && !content.contains(&diacritics_mod) {
        content.push_str(&format!("{}\n", diacritics_mod));
    }

    if common_words_exists && !content.contains(&common_words_mod) {
        content.push_str(&format!("{}\n", common_words_mod));
    }

    if contractions_exists && !content.contains(&contractions_mod) {
        content.push_str(&format!("{}\n", contractions_mod));
    }

    if determiners_exists && !content.contains(&determiners_mod) {
        content.push_str(&format!("{}\n", determiners_mod));
    }

    if context_words_exists && !content.contains(&context_words_mod) {
        content.push_str(&format!("{}\n", context_words_mod));
    }

    if synonyms_exists && !content.contains(&synonyms_mod) {
        content.push_str(&format!("{}\n", synonyms_mod));
    }

    if confusion_extended_exists && !content.contains(&confusion_extended_mod) {
        content.push_str(&format!("{}\n", confusion_extended_mod));
    }

    if uncountable_exists && !content.contains(&uncountable_mod) {
        content.push_str(&format!("{}\n", uncountable_mod));
    }

    if partlycountable_exists && !content.contains(&partlycountable_mod) {
        content.push_str(&format!("{}\n", partlycountable_mod));
    }

    if proper_nouns_exists && !content.contains(&proper_nouns_mod) {
        content.push_str(&format!("{}\n", proper_nouns_mod));
    }

    if compounds_exists && !content.contains(&compounds_mod) {
        content.push_str(&format!("{}\n", compounds_mod));
    }

    if multiwords_exists && !content.contains(&multiwords_mod) {
        content.push_str(&format!("{}\n", multiwords_mod));
    }

    if hyphenated_exists && !content.contains(&hyphenated_mod) {
        content.push_str(&format!("{}\n", hyphenated_mod));
    }

    if spelling_exists && !content.contains(&spelling_mod) {
        content.push_str(&format!("{}\n", spelling_mod));
    }

    if ignore_exists && !content.contains(&ignore_mod) {
        content.push_str(&format!("{}\n", ignore_mod));
    }

    // Phase 4 modules
    if word_definitions_exists && !content.contains(&word_definitions_mod) {
        content.push_str(&format!("{}\n", word_definitions_mod));
    }

    if prohibit_exists && !content.contains(&prohibit_mod) {
        content.push_str(&format!("{}\n", prohibit_mod));
    }

    if us_gb_exists && !content.contains(&us_gb_mod) {
        content.push_str(&format!("{}\n", us_gb_mod));
    }

    if l2_de_exists && !content.contains(&l2_de_mod) {
        content.push_str(&format!("{}\n", l2_de_mod));
    }

    if l2_es_exists && !content.contains(&l2_es_mod) {
        content.push_str(&format!("{}\n", l2_es_mod));
    }

    if l2_fr_exists && !content.contains(&l2_fr_mod) {
        content.push_str(&format!("{}\n", l2_fr_mod));
    }

    if l2_nl_exists && !content.contains(&l2_nl_mod) {
        content.push_str(&format!("{}\n", l2_nl_mod));
    }

    if added_exists && !content.contains(&added_mod) {
        content.push_str(&format!("{}\n", added_mod));
    }

    if numbers_exists && !content.contains(&numbers_mod) {
        content.push_str(&format!("{}\n", numbers_mod));
    }

    // Phase 5 modules
    let antipatterns_mod = format!("pub mod {}_antipatterns;", lang);
    let antipatterns_exists = output_dir.join(format!("{}_antipatterns.rs", lang)).exists();

    if antipatterns_exists && !content.contains(&antipatterns_mod) {
        content.push_str(&format!("{}\n", antipatterns_mod));
    }

    // Phase 6 modules (disambiguation)
    let disambig_skip_mod = format!("pub mod {}_disambig_skip;", lang);
    let disambig_pos_mod = format!("pub mod {}_disambig_pos;", lang);
    let disambig_skip_exists = output_dir.join(format!("{}_disambig_skip.rs", lang)).exists();
    let disambig_pos_exists = output_dir.join(format!("{}_disambig_pos.rs", lang)).exists();

    if disambig_skip_exists && !content.contains(&disambig_skip_mod) {
        content.push_str(&format!("{}\n", disambig_skip_mod));
    }

    if disambig_pos_exists && !content.contains(&disambig_pos_mod) {
        content.push_str(&format!("{}\n", disambig_pos_mod));
    }

    // Phase 7 modules (N-gram)
    let ngram_words_mod = format!("pub mod {}_ngram_words;", lang);
    let ngram_words_exists = output_dir.join(format!("{}_ngram_words.rs", lang)).exists();

    if ngram_words_exists && !content.contains(&ngram_words_mod) {
        content.push_str(&format!("{}\n", ngram_words_mod));
    }

    // Add re-exports if not present
    if patterns_exists {
        let pattern_export = format!(
            "pub use {}_patterns::{}_PATTERN_RULES;",
            lang,
            lang.to_uppercase()
        );
        if !content.contains(&pattern_export) {
            content.push_str(&format!("\n{}\n", pattern_export));
        }
    }

    if pos_patterns_exists {
        let pos_pattern_export = format!(
            "pub use {}_pos_patterns::{{{}_POS_PATTERN_RULES, create_{}_pos_pattern_checker}};",
            lang,
            lang.to_uppercase(),
            lang.to_lowercase()
        );
        if !content.contains(&format!("{}_pos_patterns::", lang)) {
            content.push_str(&format!("\n{}\n", pos_pattern_export));
        }
    }

    if confusion_exists {
        let confusion_export = format!(
            "pub use {}_confusion::{{get_{}_confusions, {}_CONFUSION_DATA}};",
            lang,
            lang,
            lang.to_uppercase()
        );
        if !content.contains(&format!("{}_confusion::", lang)) {
            content.push_str(&format!("\n{}\n", confusion_export));
        }
    }

    if replace_exists {
        let replace_export = format!(
            "pub use {}_replace::{{get_{}_replacement, {}_REPLACE_RULES}};",
            lang,
            lang,
            lang.to_uppercase()
        );
        if !content.contains(&format!("{}_replace::", lang)) {
            content.push_str(&format!("\n{}\n", replace_export));
        }
    }

    if pattern_tests_exists {
        let pattern_tests_export = format!(
            "pub use {}_pattern_tests::{{PatternTestExample, {}_PATTERN_TEST_EXAMPLES, get_{}_incorrect_examples, get_{}_correct_examples, get_{}_examples_for_rule}};",
            lang,
            lang.to_uppercase(),
            lang,
            lang,
            lang
        );
        if !content.contains(&format!("{}_pattern_tests::", lang)) {
            content.push_str(&format!("\n{}\n", pattern_tests_export));
        }
    }

    if style_exists {
        let style_export = format!(
            "pub use {}_style::{{StyleRule, StyleCategory, {}_STYLE_RULES, get_{}_wordiness_rules, get_{}_redundancy_rules, get_{}_style_phrases}};",
            lang,
            lang.to_uppercase(),
            lang,
            lang,
            lang
        );
        if !content.contains(&format!("{}_style::", lang)) {
            content.push_str(&format!("\n{}\n", style_export));
        }
    }

    if coherency_exists {
        let coherency_export = format!(
            "pub use {}_coherency::{{CoherencyPair, {}_COHERENCY_PAIRS, {}_COHERENCY_LOOKUP, get_{}_coherency_pair, get_{}_coherency_variants}};",
            lang,
            lang.to_uppercase(),
            lang.to_uppercase(),
            lang,
            lang
        );
        if !content.contains(&format!("{}_coherency::", lang)) {
            content.push_str(&format!("\n{}\n", coherency_export));
        }
    }

    if diacritics_exists {
        let diacritics_export = format!(
            "pub use {}_diacritics::{{get_{}_diacritics, {}_DIACRITICS_RULES}};",
            lang,
            lang,
            lang.to_uppercase()
        );
        if !content.contains(&format!("{}_diacritics::", lang)) {
            content.push_str(&format!("\n{}\n", diacritics_export));
        }
    }

    if common_words_exists {
        let common_words_export = format!(
            "pub use {}_common_words::{{is_{}_common_word, {}_COMMON_WORDS}};",
            lang,
            lang,
            lang.to_uppercase()
        );
        if !content.contains(&format!("{}_common_words::", lang)) {
            content.push_str(&format!("\n{}\n", common_words_export));
        }
    }

    if contractions_exists {
        let contractions_export = format!(
            "pub use {}_contractions::{{ContractionRule, {}_CONTRACTION_RULES, get_{}_contraction}};",
            lang,
            lang.to_uppercase(),
            lang
        );
        if !content.contains(&format!("{}_contractions::", lang)) {
            content.push_str(&format!("\n{}\n", contractions_export));
        }
    }

    if determiners_exists {
        let determiners_export = format!(
            "pub use {}_determiners::{{requires_{}_a, requires_{}_an, {}_DET_A_WORDS, {}_DET_AN_WORDS}};",
            lang,
            lang,
            lang,
            lang.to_uppercase(),
            lang.to_uppercase()
        );
        if !content.contains(&format!("{}_determiners::", lang)) {
            content.push_str(&format!("\n{}\n", determiners_export));
        }
    }

    if context_words_exists {
        let context_words_export = format!(
            "pub use {}_context_words::{{ContextRule, {}_CONTEXT_RULES}};",
            lang,
            lang.to_uppercase()
        );
        if !content.contains(&format!("{}_context_words::", lang)) {
            content.push_str(&format!("\n{}\n", context_words_export));
        }
    }

    if synonyms_exists {
        let synonyms_export = format!(
            "pub use {}_synonyms::{{SynonymEntry, {}_SYNONYM_RULES, get_{}_synonyms}};",
            lang,
            lang.to_uppercase(),
            lang
        );
        if !content.contains(&format!("{}_synonyms::", lang)) {
            content.push_str(&format!("\n{}\n", synonyms_export));
        }
    }

    // Phase 4 re-exports
    if word_definitions_exists {
        let export = format!(
            "pub use {}_word_definitions::{{WordDefinition, {}_WORD_DEFINITIONS, get_{}_word_definition}};",
            lang,
            lang.to_uppercase(),
            lang
        );
        if !content.contains(&format!("{}_word_definitions::", lang)) {
            content.push_str(&format!("\n{}\n", export));
        }
    }

    if us_gb_exists {
        let export = format!(
            "pub use {}_us_gb::{{UsGbMapping, {}_US_GB_MAPPINGS, us_to_gb, gb_to_us, is_us_spelling, is_gb_spelling}};",
            lang,
            lang.to_uppercase()
        );
        if !content.contains(&format!("{}_us_gb::", lang)) {
            content.push_str(&format!("\n{}\n", export));
        }
    }

    if l2_de_exists {
        let export = format!(
            "pub use {}_confusion_l2_de::{{L2ConfusionPair as L2ConfusionPairDe, {}_L2_DE_CONFUSION_PAIRS, get_{}_l2_de_confusion}};",
            lang,
            lang.to_uppercase(),
            lang
        );
        if !content.contains(&format!("{}_confusion_l2_de::", lang)) {
            content.push_str(&format!("\n{}\n", export));
        }
    }

    if l2_es_exists {
        let export = format!(
            "pub use {}_confusion_l2_es::{{L2ConfusionPair as L2ConfusionPairEs, {}_L2_ES_CONFUSION_PAIRS, get_{}_l2_es_confusion}};",
            lang,
            lang.to_uppercase(),
            lang
        );
        if !content.contains(&format!("{}_confusion_l2_es::", lang)) {
            content.push_str(&format!("\n{}\n", export));
        }
    }

    if l2_fr_exists {
        let export = format!(
            "pub use {}_confusion_l2_fr::{{L2ConfusionPair as L2ConfusionPairFr, {}_L2_FR_CONFUSION_PAIRS, get_{}_l2_fr_confusion}};",
            lang,
            lang.to_uppercase(),
            lang
        );
        if !content.contains(&format!("{}_confusion_l2_fr::", lang)) {
            content.push_str(&format!("\n{}\n", export));
        }
    }

    if l2_nl_exists {
        let export = format!(
            "pub use {}_confusion_l2_nl::{{L2ConfusionPair as L2ConfusionPairNl, {}_L2_NL_CONFUSION_PAIRS, get_{}_l2_nl_confusion}};",
            lang,
            lang.to_uppercase(),
            lang
        );
        if !content.contains(&format!("{}_confusion_l2_nl::", lang)) {
            content.push_str(&format!("\n{}\n", export));
        }
    }

    if added_exists {
        let export = format!(
            "pub use {}_added::{{PosTaggedWord, {}_ADDED_WORDS, get_{}_added_word}};",
            lang,
            lang.to_uppercase(),
            lang
        );
        if !content.contains(&format!("{}_added::", lang)) {
            content.push_str(&format!("\n{}\n", export));
        }
    }

    // Phase 5: Antipatterns re-export
    if antipatterns_exists {
        let export = format!(
            "pub use {}_antipatterns::{{Antipattern, AntipatternToken, {}_ANTIPATTERNS, get_{}_antipatterns}};",
            lang,
            lang.to_uppercase(),
            lang
        );
        if !content.contains(&format!("{}_antipatterns::", lang)) {
            content.push_str(&format!("\n{}\n", export));
        }
    }

    // Phase 6: Disambiguation re-exports
    if disambig_skip_exists {
        let export = format!(
            "pub use {}_disambig_skip::{{{}_DISAMBIG_SKIP, {}_DISAMBIG_SKIP_REGEX}};",
            lang,
            lang.to_uppercase(),
            lang.to_uppercase()
        );
        if !content.contains(&format!("{}_disambig_skip::", lang)) {
            content.push_str(&format!("\n{}\n", export));
        }
    }

    if disambig_pos_exists {
        let export = if lang == "en" {
            format!(
                "pub use {}_disambig_pos::{{DisambigPosEntry, {}_DISAMBIG_POS}};",
                lang,
                lang.to_uppercase()
            )
        } else {
            format!(
                "pub use {}_disambig_pos::{}_DISAMBIG_POS;",
                lang,
                lang.to_uppercase()
            )
        };
        if !content.contains(&format!("{}_disambig_pos::", lang)) {
            content.push_str(&format!("\n{}\n", export));
        }
    }

    // Phase 7: N-gram words re-export
    if ngram_words_exists {
        let export = format!(
            "pub use {}_ngram_words::{{{}_NGRAM_WORDS, is_{}_ngram_word}};",
            lang,
            lang.to_uppercase(),
            lang
        );
        if !content.contains(&format!("{}_ngram_words::", lang)) {
            content.push_str(&format!("\n{}\n", export));
        }
    }

    fs::write(&mod_path, content)?;

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: contractions.txt
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_contractions_txt(path: &Path) -> Result<Vec<ContractionRule>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut rules = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Format: without=with or without=with1|with2
        if let Some(eq_idx) = line.find('=') {
            let without = line[..eq_idx].trim().to_string();
            let with_part = line[eq_idx + 1..].trim();

            // Parse multiple contractions (pipe-separated)
            let with: Vec<String> = with_part
                .split('|')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            if !without.is_empty() && !with.is_empty() {
                rules.push(ContractionRule { without, with });
            }
        }
    }

    Ok(rules)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: contractions
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_contractions_file(rules: &[ContractionRule], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated contraction rules for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total rules: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool contractions.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! These rules map words without apostrophes to their contracted forms.\n\n",
        lang.to_uppercase(),
        timestamp,
        rules.len()
    ));

    // Generate the rule struct
    output.push_str("/// A contraction rule mapping a word to its contracted form(s)\n");
    output.push_str("#[derive(Debug, Clone)]\n");
    output.push_str("pub struct ContractionRule {\n");
    output.push_str("    /// The word without apostrophe (e.g., \"dont\")\n");
    output.push_str("    pub without: &'static str,\n");
    output.push_str("    /// The contracted form(s) (e.g., [\"don't\"])\n");
    output.push_str("    pub with: &'static [&'static str],\n");
    output.push_str("}\n\n");

    // Sort for binary search
    let mut sorted_rules: Vec<_> = rules.to_vec();
    sorted_rules.sort_by(|a, b| a.without.cmp(&b.without));

    // Generate the array
    output.push_str(&format!(
        "/// Contraction rules for {} (sorted for binary search)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!(
        "pub const {}_CONTRACTION_RULES: &[ContractionRule] = &[\n",
        lang.to_uppercase()
    ));

    for rule in &sorted_rules {
        let with: Vec<_> = rule.with.iter()
            .map(|s| format!("\"{}\"", escape_string(s)))
            .collect();

        output.push_str(&format!(
            "    ContractionRule {{ without: \"{}\", with: &[{}] }},\n",
            escape_string(&rule.without),
            with.join(", "),
        ));
    }

    output.push_str("];\n\n");

    // Generate lookup function
    output.push_str(&format!(
        "/// Get contraction(s) for a word without apostrophe (binary search)\n\
         pub fn get_{}_contraction(word: &str) -> Option<&'static [&'static str]> {{\n\
         \t{}_CONTRACTION_RULES\n\
         \t\t.binary_search_by_key(&word, |r| r.without)\n\
         \t\t.ok()\n\
         \t\t.map(|idx| {}_CONTRACTION_RULES[idx].with)\n\
         }}\n",
        lang.to_lowercase(),
        lang.to_uppercase(),
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: det_a.txt / det_an.txt
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_determiner_txt(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Handle case-sensitive marker (lines starting with *)
        let word = if let Some(stripped) = line.strip_prefix('*') {
            // Keep exact case for words marked with *
            stripped.to_string()
        } else {
            // Store lowercase for case-insensitive matching
            line.to_lowercase()
        };

        if !word.is_empty() {
            words.push(word);
        }
    }

    // Sort for binary search
    words.sort();
    words.dedup();

    Ok(words)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: determiners
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_determiners_file(a_words: &[String], an_words: &[String], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated determiner rules for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total words: {} (a: {}, an: {})\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool det_a.txt and det_an.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! These lists contain words that require specific determiners (a/an)\n\
         //! despite starting with vowels/consonants.\n\n",
        lang.to_uppercase(),
        timestamp,
        a_words.len() + an_words.len(),
        a_words.len(),
        an_words.len()
    ));

    // Generate the det_a array
    output.push_str(&format!(
        "/// Words that require 'a' as determiner (despite starting with a vowel)\n\
         /// Examples: European, university, one-time\n\
         pub const {}_DET_A_WORDS: &[&str] = &[\n",
        lang.to_uppercase()
    ));

    for word in a_words {
        output.push_str(&format!("    \"{}\",\n", escape_string(word)));
    }
    output.push_str("];\n\n");

    // Generate the det_an array
    output.push_str(&format!(
        "/// Words that require 'an' as determiner (despite starting with a consonant)\n\
         /// Examples: hour, honest, MBA\n\
         pub const {}_DET_AN_WORDS: &[&str] = &[\n",
        lang.to_uppercase()
    ));

    for word in an_words {
        output.push_str(&format!("    \"{}\",\n", escape_string(word)));
    }
    output.push_str("];\n\n");

    // Generate lookup functions
    output.push_str(&format!(
        "/// Check if a word requires 'a' as determiner (binary search)\n\
         pub fn requires_{}_a(word: &str) -> bool {{\n\
         \tlet lower = word.to_lowercase();\n\
         \t{}_DET_A_WORDS.binary_search(&lower.as_str()).is_ok()\n\
         }}\n\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output.push_str(&format!(
        "/// Check if a word requires 'an' as determiner (binary search)\n\
         pub fn requires_{}_an(word: &str) -> bool {{\n\
         \tlet lower = word.to_lowercase();\n\
         \t{}_DET_AN_WORDS.binary_search(&lower.as_str()).is_ok()\n\
         }}\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: wrongWordInContext.txt
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_context_words_txt(path: &Path) -> Result<Vec<ContextRule>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut rules = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Format: word1\tword2\tmatch1\tmatch2\tcontext1_regex\tcontext2_regex\t[explanation1\texplanation2]
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 6 {
            continue;
        }

        let rule = ContextRule {
            word1: parts[0].to_string(),
            word2: parts[1].to_string(),
            match1: parts[2].to_string(),
            match2: parts[3].to_string(),
            context1_regex: parts[4].to_string(),
            context2_regex: parts[5].to_string(),
            explanation1: parts.get(6).map(|s| s.to_string()),
            explanation2: parts.get(7).map(|s| s.to_string()),
        };

        rules.push(rule);
    }

    Ok(rules)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: context words
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_context_words_file(rules: &[ContextRule], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated context-sensitive word rules for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total rules: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool wrongWordInContext.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! These rules detect commonly confused words based on surrounding context.\n\
         //! For example: \"affect\" vs \"effect\" depending on whether verbs or nouns are nearby.\n\n",
        lang.to_uppercase(),
        timestamp,
        rules.len()
    ));

    // Generate the rule struct
    output.push_str("/// A context-sensitive word rule\n");
    output.push_str("#[derive(Debug, Clone)]\n");
    output.push_str("pub struct ContextRule {\n");
    output.push_str("    /// First word in the confused pair\n");
    output.push_str("    pub word1: &'static str,\n");
    output.push_str("    /// Second word in the confused pair\n");
    output.push_str("    pub word2: &'static str,\n");
    output.push_str("    /// Pattern to match in word1 (for partial matching)\n");
    output.push_str("    pub match1: &'static str,\n");
    output.push_str("    /// Pattern to match in word2 (for partial matching)\n");
    output.push_str("    pub match2: &'static str,\n");
    output.push_str("    /// Regex pattern for context where word1 is correct\n");
    output.push_str("    pub context1_regex: &'static str,\n");
    output.push_str("    /// Regex pattern for context where word2 is correct\n");
    output.push_str("    pub context2_regex: &'static str,\n");
    output.push_str("    /// Explanation for word1 usage\n");
    output.push_str("    pub explanation1: Option<&'static str>,\n");
    output.push_str("    /// Explanation for word2 usage\n");
    output.push_str("    pub explanation2: Option<&'static str>,\n");
    output.push_str("}\n\n");

    // Generate the array
    output.push_str(&format!(
        "/// Context-sensitive word rules for {}\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!(
        "pub const {}_CONTEXT_RULES: &[ContextRule] = &[\n",
        lang.to_uppercase()
    ));

    for rule in rules {
        let exp1 = match &rule.explanation1 {
            Some(e) => format!("Some(\"{}\")", escape_string(e)),
            None => "None".to_string(),
        };
        let exp2 = match &rule.explanation2 {
            Some(e) => format!("Some(\"{}\")", escape_string(e)),
            None => "None".to_string(),
        };

        output.push_str(&format!(
            "    ContextRule {{\n\
             \t\tword1: \"{}\",\n\
             \t\tword2: \"{}\",\n\
             \t\tmatch1: \"{}\",\n\
             \t\tmatch2: \"{}\",\n\
             \t\tcontext1_regex: \"{}\",\n\
             \t\tcontext2_regex: \"{}\",\n\
             \t\texplanation1: {},\n\
             \t\texplanation2: {},\n\
             \t}},\n",
            escape_string(&rule.word1),
            escape_string(&rule.word2),
            escape_string(&rule.match1),
            escape_string(&rule.match2),
            escape_string(&rule.context1_regex),
            escape_string(&rule.context2_regex),
            exp1,
            exp2,
        ));
    }

    output.push_str("];\n");

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: synonyms.txt
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_synonyms_txt(path: &Path) -> Result<Vec<SynonymRule>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut rules = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Format: word/POS=synonym1;synonym2;... or word/POS.*=synonym1;synonym2;...
        if let Some(eq_idx) = line.find('=') {
            let word_part = line[..eq_idx].trim();
            let synonyms_part = line[eq_idx + 1..].trim();

            // Parse word and POS tag
            let (word, pos_tag) = if let Some(slash_idx) = word_part.find('/') {
                let w = word_part[..slash_idx].trim().to_string();
                let p = word_part[slash_idx + 1..].trim().to_string();
                (w, Some(p))
            } else {
                (word_part.to_string(), None)
            };

            // Parse synonyms (semicolon-separated)
            let synonyms: Vec<String> = synonyms_part
                .split(';')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            if !word.is_empty() && !synonyms.is_empty() {
                rules.push(SynonymRule {
                    word,
                    pos_tag,
                    synonyms,
                });
            }
        }
    }

    Ok(rules)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: synonyms
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_synonyms_file(rules: &[SynonymRule], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated synonym rules for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total rules: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool synonyms.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! These rules provide synonyms for style suggestions.\n\n",
        lang.to_uppercase(),
        timestamp,
        rules.len()
    ));

    // Generate the rule struct
    output.push_str("/// A synonym rule mapping a word to its synonyms\n");
    output.push_str("#[derive(Debug, Clone)]\n");
    output.push_str("pub struct SynonymEntry {\n");
    output.push_str("    /// The original word\n");
    output.push_str("    pub word: &'static str,\n");
    output.push_str("    /// Optional POS tag pattern (e.g., \"A\" for adverb, \"V.*\" for verbs)\n");
    output.push_str("    pub pos_tag: Option<&'static str>,\n");
    output.push_str("    /// List of synonyms\n");
    output.push_str("    pub synonyms: &'static [&'static str],\n");
    output.push_str("}\n\n");

    // Sort for binary search
    let mut sorted_rules: Vec<_> = rules.to_vec();
    sorted_rules.sort_by(|a, b| a.word.cmp(&b.word));

    // Generate the array
    output.push_str(&format!(
        "/// Synonym rules for {} (sorted by word for binary search)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!(
        "pub const {}_SYNONYM_RULES: &[SynonymEntry] = &[\n",
        lang.to_uppercase()
    ));

    for rule in &sorted_rules {
        let pos = match &rule.pos_tag {
            Some(p) => format!("Some(\"{}\")", escape_string(p)),
            None => "None".to_string(),
        };
        let syns: Vec<_> = rule.synonyms.iter()
            .map(|s| format!("\"{}\"", escape_string(s)))
            .collect();

        output.push_str(&format!(
            "    SynonymEntry {{ word: \"{}\", pos_tag: {}, synonyms: &[{}] }},\n",
            escape_string(&rule.word),
            pos,
            syns.join(", "),
        ));
    }

    output.push_str("];\n\n");

    // Generate lookup function
    output.push_str(&format!(
        "/// Get synonyms for a word (binary search)\n\
         pub fn get_{}_synonyms(word: &str) -> Option<&'static [&'static str]> {{\n\
         \t{}_SYNONYM_RULES\n\
         \t\t.binary_search_by_key(&word, |r| r.word)\n\
         \t\t.ok()\n\
         \t\t.map(|idx| {}_SYNONYM_RULES[idx].synonyms)\n\
         }}\n",
        lang.to_lowercase(),
        lang.to_uppercase(),
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: confusion_sets_extended.txt
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_confusion_extended(path: &Path) -> Result<Vec<ConfusionPair>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut pairs = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Remove inline comments
        let line = if let Some(idx) = line.find('#') {
            line[..idx].trim()
        } else {
            line
        };

        // Parse format: word1 -> word2; factor   OR   word1; word2; factor
        let bidirectional = !line.contains("->");

        let parts: Vec<&str> = if bidirectional {
            line.split(';').map(|s| s.trim()).collect()
        } else {
            // word1 -> word2; factor
            let arrow_split: Vec<&str> = line.split("->").collect();
            if arrow_split.len() != 2 {
                continue;
            }
            let word1 = arrow_split[0].trim();
            let rest: Vec<&str> = arrow_split[1].split(';').map(|s| s.trim()).collect();
            if rest.len() < 2 {
                continue;
            }
            vec![word1, rest[0], rest[1]]
        };

        if parts.len() < 3 {
            continue;
        }

        let word1 = parts[0].to_lowercase();
        let word2 = parts[1].to_lowercase();
        let factor: u64 = parts[2].parse().unwrap_or(1000);

        // Skip pairs with empty words
        if word1.is_empty() || word2.is_empty() {
            continue;
        }

        // Deduplicate - keep only the first (highest precision) entry for each pair
        let key = if word1 < word2 {
            format!("{}:{}", word1, word2)
        } else {
            format!("{}:{}", word2, word1)
        };

        if seen.contains(&key) {
            continue;
        }
        seen.insert(key);

        pairs.push(ConfusionPair {
            word1,
            word2,
            factor,
            bidirectional,
        });
    }

    Ok(pairs)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: confusion_sets_extended
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_confusion_extended_file(pairs: &[ConfusionPair], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated extended confusion data for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total pairs: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool confusion_sets_extended.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! Extended confusion pairs for detecting commonly confused words.\n\
         //! Higher precision than basic confusion sets.\n\n",
        lang.to_uppercase(),
        timestamp,
        pairs.len()
    ));

    // Define the ConfusionPair struct
    output.push_str("/// A confusion pair for detecting commonly confused words\n");
    output.push_str("#[derive(Debug, Clone, Copy)]\n");
    output.push_str("pub struct ConfusionPair {\n");
    output.push_str("    /// First word in the confusion pair\n");
    output.push_str("    pub word1: &'static str,\n");
    output.push_str("    /// Second word in the confusion pair\n");
    output.push_str("    pub word2: &'static str,\n");
    output.push_str("    /// Confusion factor (higher = more confident)\n");
    output.push_str("    pub factor: u64,\n");
    output.push_str("    /// Whether the confusion is bidirectional\n");
    output.push_str("    pub bidirectional: bool,\n");
    output.push_str("}\n\n");

    // Build sorted pairs for binary search
    let mut sorted_pairs = pairs.to_vec();
    sorted_pairs.sort_by(|a, b| a.word1.cmp(&b.word1).then(a.word2.cmp(&b.word2)));

    output.push_str(&format!(
        "/// Extended confusion pairs for {} (sorted for binary search)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!("/// Total pairs: {}\n", pairs.len()));
    output.push_str(&format!(
        "pub const {}_CONFUSION_EXTENDED: &[ConfusionPair] = &[\n",
        lang.to_uppercase()
    ));

    for pair in &sorted_pairs {
        output.push_str(&format!(
            "    ConfusionPair {{ word1: \"{}\", word2: \"{}\", factor: {}, bidirectional: {} }},\n",
            escape_string(&pair.word1),
            escape_string(&pair.word2),
            pair.factor,
            pair.bidirectional
        ));
    }

    output.push_str("];\n\n");

    // Build lookup map
    output.push_str("use std::collections::HashMap;\n");
    output.push_str("use std::sync::LazyLock;\n\n");

    output.push_str(&format!(
        "/// Lookup map for extended confusions by word\n\
         pub static {}_CONFUSION_EXTENDED_LOOKUP: LazyLock<HashMap<&'static str, Vec<&'static ConfusionPair>>> = LazyLock::new(|| {{\n\
         \tlet mut map: HashMap<&'static str, Vec<&'static ConfusionPair>> = HashMap::new();\n\
         \tfor pair in {}_CONFUSION_EXTENDED {{\n\
         \t\tmap.entry(pair.word1).or_default().push(pair);\n\
         \t\tif pair.bidirectional {{\n\
         \t\t\tmap.entry(pair.word2).or_default().push(pair);\n\
         \t\t}}\n\
         \t}}\n\
         \tmap\n\
         }});\n\n",
        lang.to_uppercase(),
        lang.to_uppercase()
    ));

    // Generate lookup function
    output.push_str(&format!(
        "/// Get extended confusions for a word\n\
         pub fn get_{}_confusion_extended(word: &str) -> Option<&Vec<&'static ConfusionPair>> {{\n\
         \t{}_CONFUSION_EXTENDED_LOOKUP.get(word)\n\
         }}\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: generic word list (uncountable.txt, partlycountable.txt, hyphenated_words.txt, etc.)
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_word_list(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let word = line.trim();

        // Skip comments and empty lines
        if word.is_empty() || word.starts_with('#') {
            continue;
        }

        // Only keep words (skip if purely numeric)
        if word.chars().any(|c| c.is_alphabetic()) {
            words.push(word.to_string());
        }
    }

    // Sort and deduplicate
    words.sort();
    words.dedup();

    Ok(words)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: generic word list
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_word_list_file(words: &[String], lang: &str, name: &str, description: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();
    let const_name = format!("{}_{}", lang.to_uppercase(), name.to_uppercase());
    let fn_name = format!("is_{}_{}", lang.to_lowercase(), name.to_lowercase());

    output.push_str(&format!(
        "//! Auto-generated {} word list for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total words: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool {}.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! {}\n\n",
        name,
        lang.to_uppercase(),
        timestamp,
        words.len(),
        name,
        description
    ));

    output.push_str(&format!(
        "/// {} words for {} (sorted for binary search)\n",
        name.replace('_', " ").to_uppercase(),
        lang.to_uppercase()
    ));
    output.push_str(&format!("/// Total words: {}\n", words.len()));
    output.push_str(&format!("pub const {}: &[&str] = &[\n", const_name));

    for word in words {
        output.push_str(&format!("    \"{}\",\n", escape_string(word)));
    }

    output.push_str("];\n\n");

    // Generate lookup function
    output.push_str(&format!(
        "/// Check if a word is in the {} list (binary search)\n\
         pub fn {}(word: &str) -> bool {{\n\
         \t{}.binary_search(&word).is_ok()\n\
         }}\n",
        name,
        fn_name,
        const_name
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: specific_case.txt (proper nouns with specific casing)
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_specific_case(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let word = line.trim();

        // Skip comments and empty lines
        if word.is_empty() || word.starts_with('#') {
            continue;
        }

        // Keep the original casing for proper nouns
        words.push(word.to_string());
    }

    // Sort (case-sensitive for proper nouns)
    words.sort();
    words.dedup();

    Ok(words)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: specific_case (proper nouns)
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_specific_case_file(words: &[String], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated proper nouns for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total entries: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool specific_case.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! Proper nouns with specific casing that should not trigger\n\
         //! capitalization errors.\n\n",
        lang.to_uppercase(),
        timestamp,
        words.len()
    ));

    output.push_str("use std::collections::HashSet;\n");
    output.push_str("use std::sync::LazyLock;\n\n");

    output.push_str(&format!(
        "/// Proper nouns for {} (original casing preserved)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!("/// Total entries: {}\n", words.len()));
    output.push_str(&format!(
        "pub const {}_PROPER_NOUNS: &[&str] = &[\n",
        lang.to_uppercase()
    ));

    for word in words {
        output.push_str(&format!("    \"{}\",\n", escape_string(word)));
    }

    output.push_str("];\n\n");

    // Build lowercase lookup set
    output.push_str(&format!(
        "/// Lookup set for proper nouns (lowercase for case-insensitive matching)\n\
         pub static {}_PROPER_NOUNS_LOWER: LazyLock<HashSet<String>> = LazyLock::new(|| {{\n\
         \t{}_PROPER_NOUNS.iter().map(|s| s.to_lowercase()).collect()\n\
         }});\n\n",
        lang.to_uppercase(),
        lang.to_uppercase()
    ));

    // Generate lookup function
    output.push_str(&format!(
        "/// Check if a phrase is a proper noun (case-insensitive)\n\
         pub fn is_{}_proper_noun(phrase: &str) -> bool {{\n\
         \t{}_PROPER_NOUNS_LOWER.contains(&phrase.to_lowercase())\n\
         }}\n\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    // Generate correct casing lookup
    output.push_str(&format!(
        "/// Get the correct casing for a proper noun\n\
         pub fn get_{}_proper_noun_casing(phrase: &str) -> Option<&'static str> {{\n\
         \tlet lower = phrase.to_lowercase();\n\
         \t{}_PROPER_NOUNS.iter().find(|s| s.to_lowercase() == lower).copied()\n\
         }}\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: compounds.txt
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_compounds_txt(path: &Path) -> Result<Vec<CompoundRule>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut rules = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse modifiers at end: +, *, ?, $
        let mut word = line.to_string();
        let allow_no_hyphen = word.ends_with('+');
        let hyphen_only = word.ends_with('*');
        let lowercase_joined = word.ends_with('?');
        let allow_both = word.ends_with('$');

        // Remove modifier
        if allow_no_hyphen || hyphen_only || lowercase_joined || allow_both {
            word.pop();
        }

        // Skip if no hyphen (invalid entry)
        if !word.contains('-') {
            continue;
        }

        rules.push(CompoundRule {
            word: word.to_lowercase(),
            allow_no_hyphen,
            hyphen_only,
            lowercase_joined,
            allow_both,
        });
    }

    // Sort and deduplicate
    rules.sort_by(|a, b| a.word.cmp(&b.word));
    rules.dedup_by(|a, b| a.word == b.word);

    Ok(rules)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: compounds
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_compounds_file(rules: &[CompoundRule], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated compound word rules for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total rules: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool compounds.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! Compound word rules for detecting words that should be hyphenated\n\
         //! or joined together.\n\n",
        lang.to_uppercase(),
        timestamp,
        rules.len()
    ));

    // Define the rule struct
    output.push_str("/// A compound word rule\n");
    output.push_str("#[derive(Debug, Clone, Copy)]\n");
    output.push_str("pub struct CompoundRule {\n");
    output.push_str("    /// The hyphenated form of the compound\n");
    output.push_str("    pub word: &'static str,\n");
    output.push_str("    /// Allow non-hyphenated form (e.g., \"backfire\" for \"back-fire\")\n");
    output.push_str("    pub allow_no_hyphen: bool,\n");
    output.push_str("    /// Only suggest hyphenated form\n");
    output.push_str("    pub hyphen_only: bool,\n");
    output.push_str("    /// Suggest lowercase joined form\n");
    output.push_str("    pub lowercase_joined: bool,\n");
    output.push_str("    /// Allow both hyphenated and non-hyphenated\n");
    output.push_str("    pub allow_both: bool,\n");
    output.push_str("}\n\n");

    output.push_str(&format!(
        "/// Compound rules for {} (sorted for binary search)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!("/// Total rules: {}\n", rules.len()));
    output.push_str(&format!(
        "pub const {}_COMPOUND_RULES: &[CompoundRule] = &[\n",
        lang.to_uppercase()
    ));

    for rule in rules {
        output.push_str(&format!(
            "    CompoundRule {{ word: \"{}\", allow_no_hyphen: {}, hyphen_only: {}, lowercase_joined: {}, allow_both: {} }},\n",
            escape_string(&rule.word),
            rule.allow_no_hyphen,
            rule.hyphen_only,
            rule.lowercase_joined,
            rule.allow_both
        ));
    }

    output.push_str("];\n\n");

    // Build lookup map by non-hyphenated form
    output.push_str("use std::collections::HashMap;\n");
    output.push_str("use std::sync::LazyLock;\n\n");

    output.push_str(&format!(
        "/// Lookup map for compounds by non-hyphenated form\n\
         pub static {}_COMPOUND_LOOKUP: LazyLock<HashMap<String, &'static CompoundRule>> = LazyLock::new(|| {{\n\
         \tlet mut map = HashMap::new();\n\
         \tfor rule in {}_COMPOUND_RULES {{\n\
         \t\t// Add lookup by joined form (without hyphen)\n\
         \t\tlet joined: String = rule.word.chars().filter(|c| *c != '-').collect();\n\
         \t\tmap.insert(joined, rule);\n\
         \t}}\n\
         \tmap\n\
         }});\n\n",
        lang.to_uppercase(),
        lang.to_uppercase()
    ));

    // Generate lookup function
    output.push_str(&format!(
        "/// Get compound rule for a word (looks up by non-hyphenated form)\n\
         pub fn get_{}_compound(word: &str) -> Option<&'static CompoundRule> {{\n\
         \t{}_COMPOUND_LOOKUP.get(&word.to_lowercase()).copied()\n\
         }}\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser: multiwords.txt
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_multiwords_txt(path: &Path) -> Result<Vec<MultiwordEntry>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Remove inline comments
        let line = if let Some(idx) = line.find('#') {
            line[..idx].trim()
        } else {
            line
        };

        // Format: phrase\tPOS_TAG
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 2 {
            continue;
        }

        let phrase = parts[0].trim();
        let pos_tag = parts[1].trim();

        if phrase.is_empty() || pos_tag.is_empty() {
            continue;
        }

        entries.push(MultiwordEntry {
            phrase: phrase.to_string(),
            pos_tag: pos_tag.to_string(),
        });
    }

    // Sort by phrase
    entries.sort_by(|a, b| a.phrase.to_lowercase().cmp(&b.phrase.to_lowercase()));

    Ok(entries)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Generator: multiwords
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_multiwords_file(entries: &[MultiwordEntry], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated multiword expressions for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total entries: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool multiwords.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! Multiword expressions with their POS tags for disambiguation.\n\n",
        lang.to_uppercase(),
        timestamp,
        entries.len()
    ));

    // Define the entry struct
    output.push_str("/// A multiword expression entry\n");
    output.push_str("#[derive(Debug, Clone, Copy)]\n");
    output.push_str("pub struct MultiwordEntry {\n");
    output.push_str("    /// The multiword phrase\n");
    output.push_str("    pub phrase: &'static str,\n");
    output.push_str("    /// The POS tag for this phrase\n");
    output.push_str("    pub pos_tag: &'static str,\n");
    output.push_str("}\n\n");

    output.push_str(&format!(
        "/// Multiword entries for {} (sorted for binary search)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!("/// Total entries: {}\n", entries.len()));
    output.push_str(&format!(
        "pub const {}_MULTIWORDS: &[MultiwordEntry] = &[\n",
        lang.to_uppercase()
    ));

    for entry in entries {
        output.push_str(&format!(
            "    MultiwordEntry {{ phrase: \"{}\", pos_tag: \"{}\" }},\n",
            escape_string(&entry.phrase),
            escape_string(&entry.pos_tag)
        ));
    }

    output.push_str("];\n\n");

    // Build lookup map
    output.push_str("use std::collections::HashMap;\n");
    output.push_str("use std::sync::LazyLock;\n\n");

    output.push_str(&format!(
        "/// Lookup map for multiwords by lowercase phrase\n\
         pub static {}_MULTIWORD_LOOKUP: LazyLock<HashMap<String, &'static MultiwordEntry>> = LazyLock::new(|| {{\n\
         \tlet mut map = HashMap::new();\n\
         \tfor entry in {}_MULTIWORDS {{\n\
         \t\tmap.insert(entry.phrase.to_lowercase(), entry);\n\
         \t}}\n\
         \tmap\n\
         }});\n\n",
        lang.to_uppercase(),
        lang.to_uppercase()
    ));

    // Generate lookup function
    output.push_str(&format!(
        "/// Get multiword entry for a phrase (case-insensitive)\n\
         pub fn get_{}_multiword(phrase: &str) -> Option<&'static MultiwordEntry> {{\n\
         \t{}_MULTIWORD_LOOKUP.get(&phrase.to_lowercase()).copied()\n\
         }}\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 4: Parser - word_definitions.txt
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_word_definitions(path: &Path) -> Result<Vec<WordDefinition>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut definitions = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Format: word\tdefinition
        let parts: Vec<&str> = line.splitn(2, '\t').collect();
        if parts.len() < 2 {
            continue;
        }

        let word = parts[0].trim();
        let definition = parts[1].trim();

        if word.is_empty() || definition.is_empty() {
            continue;
        }

        definitions.push(WordDefinition {
            word: word.to_string(),
            definition: definition.to_string(),
        });
    }

    // Sort by word
    definitions.sort_by(|a, b| a.word.to_lowercase().cmp(&b.word.to_lowercase()));

    Ok(definitions)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 4: Generator - word_definitions
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_word_definitions_file(definitions: &[WordDefinition], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated word definitions for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total definitions: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool word_definitions.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! Word definitions used for semantic disambiguation.\n\n",
        lang.to_uppercase(),
        timestamp,
        definitions.len()
    ));

    // Define the entry struct
    output.push_str("/// A word definition entry\n");
    output.push_str("#[derive(Debug, Clone, Copy)]\n");
    output.push_str("pub struct WordDefinition {\n");
    output.push_str("    /// The word\n");
    output.push_str("    pub word: &'static str,\n");
    output.push_str("    /// Short definition (< 40 chars)\n");
    output.push_str("    pub definition: &'static str,\n");
    output.push_str("}\n\n");

    output.push_str(&format!(
        "/// Word definitions for {} (sorted by word)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!("/// Total definitions: {}\n", definitions.len()));
    output.push_str(&format!(
        "pub const {}_WORD_DEFINITIONS: &[WordDefinition] = &[\n",
        lang.to_uppercase()
    ));

    for def in definitions {
        output.push_str(&format!(
            "    WordDefinition {{ word: \"{}\", definition: \"{}\" }},\n",
            escape_string(&def.word),
            escape_string(&def.definition)
        ));
    }

    output.push_str("];\n\n");

    // Build lookup map
    output.push_str("use std::collections::HashMap;\n");
    output.push_str("use std::sync::LazyLock;\n\n");

    output.push_str(&format!(
        "/// Lookup map for word definitions\n\
         pub static {}_WORD_DEFINITION_LOOKUP: LazyLock<HashMap<String, &'static WordDefinition>> = LazyLock::new(|| {{\n\
         \tlet mut map = HashMap::new();\n\
         \tfor def in {}_WORD_DEFINITIONS {{\n\
         \t\tmap.insert(def.word.to_lowercase(), def);\n\
         \t}}\n\
         \tmap\n\
         }});\n\n",
        lang.to_uppercase(),
        lang.to_uppercase()
    ));

    // Generate lookup function
    output.push_str(&format!(
        "/// Get definition for a word (case-insensitive)\n\
         pub fn get_{}_word_definition(word: &str) -> Option<&'static str> {{\n\
         \t{}_WORD_DEFINITION_LOOKUP.get(&word.to_lowercase()).map(|d| d.definition)\n\
         }}\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 4: Parser - en-US-GB.txt
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_us_gb_mappings(path: &Path) -> Result<Vec<UsGbMapping>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut mappings = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Format: us_word;gb_word
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() < 2 {
            continue;
        }

        let us_word = parts[0].trim();
        let gb_word = parts[1].trim();

        if us_word.is_empty() || gb_word.is_empty() {
            continue;
        }

        mappings.push(UsGbMapping {
            us_word: us_word.to_string(),
            gb_word: gb_word.to_string(),
        });
    }

    // Sort by US word
    mappings.sort_by(|a, b| a.us_word.to_lowercase().cmp(&b.us_word.to_lowercase()));

    Ok(mappings)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 4: Generator - US/GB mappings
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_us_gb_file(mappings: &[UsGbMapping], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated US/GB spelling mappings for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total mappings: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool en-US-GB.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! Mappings between US and British English spellings.\n\n",
        lang.to_uppercase(),
        timestamp,
        mappings.len()
    ));

    // Define the entry struct
    output.push_str("/// A US/GB spelling mapping\n");
    output.push_str("#[derive(Debug, Clone, Copy)]\n");
    output.push_str("pub struct UsGbMapping {\n");
    output.push_str("    /// US spelling\n");
    output.push_str("    pub us_word: &'static str,\n");
    output.push_str("    /// British spelling\n");
    output.push_str("    pub gb_word: &'static str,\n");
    output.push_str("}\n\n");

    output.push_str(&format!(
        "/// US/GB spelling mappings (sorted by US spelling)\n"
    ));
    output.push_str(&format!("/// Total mappings: {}\n", mappings.len()));
    output.push_str(&format!(
        "pub const {}_US_GB_MAPPINGS: &[UsGbMapping] = &[\n",
        lang.to_uppercase()
    ));

    for mapping in mappings {
        output.push_str(&format!(
            "    UsGbMapping {{ us_word: \"{}\", gb_word: \"{}\" }},\n",
            escape_string(&mapping.us_word),
            escape_string(&mapping.gb_word)
        ));
    }

    output.push_str("];\n\n");

    // Build lookup maps (both directions)
    output.push_str("use std::collections::HashMap;\n");
    output.push_str("use std::sync::LazyLock;\n\n");

    // US -> GB lookup
    output.push_str(&format!(
        "/// Lookup map: US -> GB spelling\n\
         pub static {}_US_TO_GB: LazyLock<HashMap<String, &'static str>> = LazyLock::new(|| {{\n\
         \tlet mut map = HashMap::new();\n\
         \tfor m in {}_US_GB_MAPPINGS {{\n\
         \t\tmap.insert(m.us_word.to_lowercase(), m.gb_word);\n\
         \t}}\n\
         \tmap\n\
         }});\n\n",
        lang.to_uppercase(),
        lang.to_uppercase()
    ));

    // GB -> US lookup
    output.push_str(&format!(
        "/// Lookup map: GB -> US spelling\n\
         pub static {}_GB_TO_US: LazyLock<HashMap<String, &'static str>> = LazyLock::new(|| {{\n\
         \tlet mut map = HashMap::new();\n\
         \tfor m in {}_US_GB_MAPPINGS {{\n\
         \t\tmap.insert(m.gb_word.to_lowercase(), m.us_word);\n\
         \t}}\n\
         \tmap\n\
         }});\n\n",
        lang.to_uppercase(),
        lang.to_uppercase()
    ));

    // Generate lookup functions
    output.push_str(&format!(
        "/// Convert US spelling to GB spelling\n\
         pub fn us_to_gb(word: &str) -> Option<&'static str> {{\n\
         \t{}_US_TO_GB.get(&word.to_lowercase()).copied()\n\
         }}\n\n",
        lang.to_uppercase()
    ));

    output.push_str(&format!(
        "/// Convert GB spelling to US spelling\n\
         pub fn gb_to_us(word: &str) -> Option<&'static str> {{\n\
         \t{}_GB_TO_US.get(&word.to_lowercase()).copied()\n\
         }}\n\n",
        lang.to_uppercase()
    ));

    output.push_str(&format!(
        "/// Check if a word is a US spelling variant\n\
         pub fn is_us_spelling(word: &str) -> bool {{\n\
         \t{}_US_TO_GB.contains_key(&word.to_lowercase())\n\
         }}\n\n",
        lang.to_uppercase()
    ));

    output.push_str(&format!(
        "/// Check if a word is a GB spelling variant\n\
         pub fn is_gb_spelling(word: &str) -> bool {{\n\
         \t{}_GB_TO_US.contains_key(&word.to_lowercase())\n\
         }}\n",
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 4: Parser - confusion_sets_l2_*.txt
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_confusion_l2(path: &Path, native_lang: &str) -> Result<Vec<L2ConfusionPair>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut pairs = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Remove inline comments
        let line = if let Some(idx) = line.find('#') {
            line[..idx].trim()
        } else {
            line
        };

        // Format: word1 -> word2; factor
        // Example: abilities -> skills; 100000;
        let parts: Vec<&str> = line.split("->").collect();
        if parts.len() < 2 {
            continue;
        }

        let word1 = parts[0].trim().to_string();
        let rest = parts[1].trim();

        // Parse word2 and factor
        let parts: Vec<&str> = rest.split(';').collect();
        if parts.is_empty() {
            continue;
        }

        let word2 = parts[0].trim().to_string();
        let factor = if parts.len() > 1 {
            parts[1].trim().parse::<u64>().unwrap_or(100)
        } else {
            100
        };

        if word1.is_empty() || word2.is_empty() {
            continue;
        }

        pairs.push(L2ConfusionPair {
            word1,
            word2,
            factor,
            native_language: native_lang.to_string(),
        });
    }

    // Sort by word1
    pairs.sort_by(|a, b| a.word1.to_lowercase().cmp(&b.word1.to_lowercase()));

    Ok(pairs)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 4: Generator - L2 confusion pairs
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_confusion_l2_file(pairs: &[L2ConfusionPair], lang: &str, native_lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    let native_lang_name = match native_lang {
        "de" => "German",
        "es" => "Spanish",
        "fr" => "French",
        "nl" => "Dutch",
        _ => native_lang,
    };

    output.push_str(&format!(
        "//! Auto-generated L2 confusion pairs for {} native speakers writing {}\n\
         //! Synced: {}\n\
         //! Total pairs: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool confusion_sets_l2_{}.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! False friends and common mistakes made by {} native speakers.\n\n",
        native_lang_name,
        lang.to_uppercase(),
        timestamp,
        pairs.len(),
        native_lang,
        native_lang_name
    ));

    // Define the entry struct
    output.push_str("/// An L2 confusion pair (false friend)\n");
    output.push_str("#[derive(Debug, Clone, Copy)]\n");
    output.push_str("pub struct L2ConfusionPair {\n");
    output.push_str("    /// The word often confused (false friend in L1)\n");
    output.push_str("    pub word1: &'static str,\n");
    output.push_str("    /// The correct word to use instead\n");
    output.push_str("    pub word2: &'static str,\n");
    output.push_str("    /// Confidence factor (higher = more likely confusion)\n");
    output.push_str("    pub factor: u64,\n");
    output.push_str("}\n\n");

    output.push_str(&format!(
        "/// L2 confusion pairs for {} native speakers (sorted by word1)\n",
        native_lang_name
    ));
    output.push_str(&format!("/// Total pairs: {}\n", pairs.len()));
    output.push_str(&format!(
        "pub const {}_L2_{}_CONFUSION_PAIRS: &[L2ConfusionPair] = &[\n",
        lang.to_uppercase(),
        native_lang.to_uppercase()
    ));

    for pair in pairs {
        output.push_str(&format!(
            "    L2ConfusionPair {{ word1: \"{}\", word2: \"{}\", factor: {} }},\n",
            escape_string(&pair.word1),
            escape_string(&pair.word2),
            pair.factor
        ));
    }

    output.push_str("];\n\n");

    // Build lookup map
    output.push_str("use std::collections::HashMap;\n");
    output.push_str("use std::sync::LazyLock;\n\n");

    output.push_str(&format!(
        "/// Lookup map for L2 confusion pairs by word1\n\
         pub static {}_L2_{}_CONFUSION_LOOKUP: LazyLock<HashMap<String, &'static L2ConfusionPair>> = LazyLock::new(|| {{\n\
         \tlet mut map = HashMap::new();\n\
         \tfor pair in {}_L2_{}_CONFUSION_PAIRS {{\n\
         \t\tmap.insert(pair.word1.to_lowercase(), pair);\n\
         \t}}\n\
         \tmap\n\
         }});\n\n",
        lang.to_uppercase(),
        native_lang.to_uppercase(),
        lang.to_uppercase(),
        native_lang.to_uppercase()
    ));

    // Generate lookup function
    output.push_str(&format!(
        "/// Check if a word might be a false friend for {} native speakers\n\
         pub fn get_{}_l2_{}_confusion(word: &str) -> Option<&'static L2ConfusionPair> {{\n\
         \t{}_L2_{}_CONFUSION_LOOKUP.get(&word.to_lowercase()).copied()\n\
         }}\n",
        native_lang_name,
        lang.to_lowercase(),
        native_lang,
        lang.to_uppercase(),
        native_lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 4: Parser - added.txt (POS-tagged words)
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_pos_tagged_words(path: &Path) -> Result<Vec<PosTaggedWord>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Format: fullform\tbaseform\tpostags
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 3 {
            continue;
        }

        let word = parts[0].trim();
        let base_form = parts[1].trim();
        let pos_tag = parts[2].trim();

        if word.is_empty() || pos_tag.is_empty() {
            continue;
        }

        words.push(PosTaggedWord {
            word: word.to_string(),
            base_form: base_form.to_string(),
            pos_tag: pos_tag.to_string(),
        });
    }

    // Sort by word
    words.sort_by(|a, b| a.word.to_lowercase().cmp(&b.word.to_lowercase()));

    Ok(words)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 4: Generator - POS-tagged words (added.txt)
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_pos_tagged_words_file(words: &[PosTaggedWord], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated POS-tagged words for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total entries: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool added.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! Additional words with POS tags not in the main dictionary.\n\n",
        lang.to_uppercase(),
        timestamp,
        words.len()
    ));

    // Define the entry struct
    output.push_str("/// A POS-tagged word entry\n");
    output.push_str("#[derive(Debug, Clone, Copy)]\n");
    output.push_str("pub struct PosTaggedWord {\n");
    output.push_str("    /// The word form\n");
    output.push_str("    pub word: &'static str,\n");
    output.push_str("    /// The base/lemma form\n");
    output.push_str("    pub base_form: &'static str,\n");
    output.push_str("    /// The POS tag\n");
    output.push_str("    pub pos_tag: &'static str,\n");
    output.push_str("}\n\n");

    output.push_str(&format!(
        "/// Added POS-tagged words for {} (sorted by word)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!("/// Total entries: {}\n", words.len()));
    output.push_str(&format!(
        "pub const {}_ADDED_WORDS: &[PosTaggedWord] = &[\n",
        lang.to_uppercase()
    ));

    for word in words {
        output.push_str(&format!(
            "    PosTaggedWord {{ word: \"{}\", base_form: \"{}\", pos_tag: \"{}\" }},\n",
            escape_string(&word.word),
            escape_string(&word.base_form),
            escape_string(&word.pos_tag)
        ));
    }

    output.push_str("];\n\n");

    // Build lookup map
    output.push_str("use std::collections::HashMap;\n");
    output.push_str("use std::sync::LazyLock;\n\n");

    output.push_str(&format!(
        "/// Lookup map for added words by word form\n\
         pub static {}_ADDED_WORD_LOOKUP: LazyLock<HashMap<String, &'static PosTaggedWord>> = LazyLock::new(|| {{\n\
         \tlet mut map = HashMap::new();\n\
         \tfor entry in {}_ADDED_WORDS {{\n\
         \t\tmap.insert(entry.word.to_lowercase(), entry);\n\
         \t}}\n\
         \tmap\n\
         }});\n\n",
        lang.to_uppercase(),
        lang.to_uppercase()
    ));

    // Generate lookup function
    output.push_str(&format!(
        "/// Get POS tag for an added word (case-insensitive)\n\
         pub fn get_{}_added_word(word: &str) -> Option<&'static PosTaggedWord> {{\n\
         \t{}_ADDED_WORD_LOOKUP.get(&word.to_lowercase()).copied()\n\
         }}\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 5: Parser - antipatterns from grammar.xml
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_antipatterns(path: &Path) -> Result<Vec<Antipattern>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let mut reader = Reader::from_str(&content);
    reader.trim_text(true);

    let mut antipatterns = Vec::new();
    let mut buf = Vec::new();

    let mut current_rule_id = String::new();
    let mut in_rule = false;
    let mut in_antipattern = false;
    let mut current_antipattern_tokens: Vec<AntipatternToken> = Vec::new();
    let mut current_token: Option<AntipatternToken> = None;
    let mut in_token = false;
    let mut text_buffer = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                match name.as_str() {
                    "rule" => {
                        let id = get_attr(e, "id").unwrap_or_default();
                        if !id.is_empty() {
                            current_rule_id = id;
                            in_rule = true;
                        }
                    }
                    "antipattern" if in_rule => {
                        in_antipattern = true;
                        current_antipattern_tokens.clear();
                    }
                    "token" if in_antipattern => {
                        let mut token = AntipatternToken::default();
                        token.inflected = get_attr(e, "inflected")
                            .map(|v| v == "yes")
                            .unwrap_or(false);
                        token.negation = get_attr(e, "negate")
                            .map(|v| v == "yes")
                            .unwrap_or(false);
                        token.postag = get_attr(e, "postag");
                        token.regexp = get_attr(e, "regexp");
                        token.skip = get_attr(e, "skip")
                            .and_then(|v| v.parse().ok());
                        current_token = Some(token);
                        in_token = true;
                        text_buffer.clear();
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                match name.as_str() {
                    "rule" => {
                        in_rule = false;
                        current_rule_id.clear();
                    }
                    "antipattern" if in_antipattern => {
                        // Save the antipattern if it has valid tokens
                        if !current_antipattern_tokens.is_empty() && !current_rule_id.is_empty() {
                            // Only keep simple antipatterns (2-4 tokens, no complex features)
                            let is_simple = current_antipattern_tokens.len() >= 2
                                && current_antipattern_tokens.len() <= 4
                                && current_antipattern_tokens.iter().all(|t| {
                                    // Must have text or regexp
                                    (t.text.is_some() || t.regexp.is_some())
                                        // No skip (complex matching)
                                        && t.skip.is_none()
                                        // No postag-only tokens
                                        && !(t.text.is_none() && t.regexp.is_none() && t.postag.is_some())
                                });

                            if is_simple {
                                antipatterns.push(Antipattern {
                                    rule_id: current_rule_id.clone(),
                                    tokens: current_antipattern_tokens.clone(),
                                });
                            }
                        }
                        in_antipattern = false;
                        current_antipattern_tokens.clear();
                    }
                    "token" if in_token && in_antipattern => {
                        if let Some(mut token) = current_token.take() {
                            if !text_buffer.is_empty() {
                                token.text = Some(text_buffer.clone());
                            }
                            current_antipattern_tokens.push(token);
                        }
                        in_token = false;
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                if in_token && in_antipattern {
                    text_buffer.push_str(&text);
                }
            }
            Ok(Event::Empty(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if name == "token" && in_antipattern {
                    let mut token = AntipatternToken::default();
                    token.inflected = get_attr(e, "inflected")
                        .map(|v| v == "yes")
                        .unwrap_or(false);
                    token.negation = get_attr(e, "negate")
                        .map(|v| v == "yes")
                        .unwrap_or(false);
                    token.postag = get_attr(e, "postag");
                    token.regexp = get_attr(e, "regexp");
                    token.skip = get_attr(e, "skip")
                        .and_then(|v| v.parse().ok());
                    current_antipattern_tokens.push(token);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("Error parsing XML at position {}: {:?}", reader.buffer_position(), e);
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    // Deduplicate antipatterns (same rule_id + tokens)
    antipatterns.sort_by(|a, b| {
        a.rule_id.cmp(&b.rule_id)
            .then_with(|| {
                let a_tokens: Vec<_> = a.tokens.iter().map(|t| t.text.as_deref().unwrap_or("")).collect();
                let b_tokens: Vec<_> = b.tokens.iter().map(|t| t.text.as_deref().unwrap_or("")).collect();
                a_tokens.cmp(&b_tokens)
            })
    });

    Ok(antipatterns)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 5: Generator - antipatterns
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_antipatterns_file(antipatterns: &[Antipattern], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated antipatterns for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total antipatterns: {}\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool grammar.xml\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! Antipatterns are exceptions to grammar rules.\n\
         //! When text matches an antipattern, the rule should NOT fire.\n\n",
        lang.to_uppercase(),
        timestamp,
        antipatterns.len()
    ));

    // Only define structs for EN, other languages import from en_antipatterns
    if lang == "en" {
        // Define the token struct
        output.push_str("/// A token in an antipattern\n");
        output.push_str("#[derive(Debug, Clone)]\n");
        output.push_str("pub struct AntipatternToken {\n");
        output.push_str("    /// Literal text to match (case-insensitive)\n");
        output.push_str("    pub text: Option<&'static str>,\n");
        output.push_str("    /// Regex pattern to match\n");
        output.push_str("    pub regexp: Option<&'static str>,\n");
        output.push_str("    /// Whether to match inflected forms\n");
        output.push_str("    pub inflected: bool,\n");
        output.push_str("}\n\n");

        // Define the antipattern struct
        output.push_str("/// An antipattern (exception to a rule)\n");
        output.push_str("#[derive(Debug, Clone)]\n");
        output.push_str("pub struct Antipattern {\n");
        output.push_str("    /// The rule ID this antipattern applies to\n");
        output.push_str("    pub rule_id: &'static str,\n");
        output.push_str("    /// The token sequence that should NOT trigger the rule\n");
        output.push_str("    pub tokens: &'static [AntipatternToken],\n");
        output.push_str("}\n\n");
    } else {
        // Import types from en_antipatterns
        output.push_str("use super::en_antipatterns::{Antipattern, AntipatternToken};\n\n");
    }

    // Generate static token arrays for each antipattern
    for (idx, ap) in antipatterns.iter().enumerate() {
        output.push_str(&format!(
            "static ANTIPATTERN_{}_TOKENS: &[AntipatternToken] = &[\n",
            idx
        ));
        for token in &ap.tokens {
            let text = match &token.text {
                Some(t) => format!("Some(\"{}\")", escape_string(&t.to_lowercase())),
                None => "None".to_string(),
            };
            let regexp = match &token.regexp {
                Some(r) => format!("Some(\"{}\")", escape_string(r)),
                None => "None".to_string(),
            };
            output.push_str(&format!(
                "    AntipatternToken {{ text: {}, regexp: {}, inflected: {} }},\n",
                text, regexp, token.inflected
            ));
        }
        output.push_str("];\n");
    }
    output.push('\n');

    // Generate the main array
    output.push_str(&format!(
        "/// Antipatterns for {} (sorted by rule_id)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!("/// Total: {} antipatterns\n", antipatterns.len()));
    output.push_str(&format!(
        "pub static {}_ANTIPATTERNS: &[Antipattern] = &[\n",
        lang.to_uppercase()
    ));

    for (idx, ap) in antipatterns.iter().enumerate() {
        output.push_str(&format!(
            "    Antipattern {{ rule_id: \"{}\", tokens: ANTIPATTERN_{}_TOKENS }},\n",
            escape_string(&ap.rule_id),
            idx
        ));
    }

    output.push_str("];\n\n");

    // Build lookup map by rule_id
    output.push_str("use std::collections::HashMap;\n");
    output.push_str("use std::sync::LazyLock;\n\n");

    output.push_str(&format!(
        "/// Lookup antipatterns by rule ID\n\
         pub static {}_ANTIPATTERNS_BY_RULE: LazyLock<HashMap<&'static str, Vec<&'static Antipattern>>> = LazyLock::new(|| {{\n\
         \tlet mut map: HashMap<&'static str, Vec<&'static Antipattern>> = HashMap::new();\n\
         \tfor ap in {}_ANTIPATTERNS {{\n\
         \t\tmap.entry(ap.rule_id).or_default().push(ap);\n\
         \t}}\n\
         \tmap\n\
         }});\n\n",
        lang.to_uppercase(),
        lang.to_uppercase()
    ));

    // Generate lookup function
    output.push_str(&format!(
        "/// Get antipatterns for a rule ID\n\
         pub fn get_{}_antipatterns(rule_id: &str) -> Option<&'static Vec<&'static Antipattern>> {{\n\
         \t{}_ANTIPATTERNS_BY_RULE.get(rule_id)\n\
         }}\n",
        lang.to_lowercase(),
        lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 6: Parser - disambiguation.xml
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_disambiguation_xml(path: &Path) -> Result<Vec<DisambigRule>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let mut reader = Reader::from_str(&content);
    reader.trim_text(true);

    let mut rules = Vec::new();
    let mut buf = Vec::new();

    let mut current_rule_id = String::new();
    let mut in_rule = false;
    let mut in_pattern = false;
    let mut in_marker = false;
    let mut in_token = false;
    let mut in_disambig = false;
    let mut current_pattern: Vec<PatternToken> = Vec::new();
    let mut marker_indices: Vec<usize> = Vec::new();
    let mut current_token: Option<PatternToken> = None;
    let mut current_action: Option<DisambigAction> = None;
    let mut current_wd: Option<DisambigWd> = None;
    let mut current_postag: Option<String> = None;
    let mut text_buffer = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                match name.as_str() {
                    "rule" => {
                        let id = get_attr(e, "id").unwrap_or_default();
                        if !id.is_empty() {
                            current_rule_id = id;
                            in_rule = true;
                            current_pattern.clear();
                            marker_indices.clear();
                            current_action = None;
                            current_wd = None;
                            current_postag = None;
                        }
                    }
                    "pattern" if in_rule => {
                        in_pattern = true;
                        current_pattern.clear();
                        marker_indices.clear();
                    }
                    "marker" if in_pattern => {
                        in_marker = true;
                    }
                    "token" if in_pattern => {
                        let mut token = PatternToken::default();
                        token.inflected = get_attr(e, "inflected")
                            .map(|v| v == "yes")
                            .unwrap_or(false);
                        token.case_sensitive = get_attr(e, "case_sensitive")
                            .map(|v| v == "yes")
                            .unwrap_or(false);
                        token.negation = get_attr(e, "negate")
                            .map(|v| v == "yes")
                            .unwrap_or(false);
                        token.postag = get_attr(e, "postag");
                        token.postag_regexp = get_attr(e, "postag_regexp")
                            .map(|v| v == "yes")
                            .unwrap_or(false);
                        if get_attr(e, "regexp").map(|v| v == "yes").unwrap_or(false) {
                            // Regexp is stored in token text, marked as regexp
                            token.regexp = Some("yes".to_string());
                        }
                        current_token = Some(token);
                        in_token = true;
                        text_buffer.clear();
                    }
                    "disambig" if in_rule => {
                        in_disambig = true;
                        let action_str = get_attr(e, "action").unwrap_or_default();
                        current_action = match action_str.as_str() {
                            "replace" => Some(DisambigAction::Replace),
                            "add" => Some(DisambigAction::Add),
                            "remove" => Some(DisambigAction::Remove),
                            "ignore_spelling" => Some(DisambigAction::IgnoreSpelling),
                            "filter" => Some(DisambigAction::Filter),
                            "filterall" => Some(DisambigAction::FilterAll),
                            "unify" => Some(DisambigAction::Unify),
                            "immunize" => Some(DisambigAction::Immunize),
                            _ => None,
                        };
                        // Check for postag attribute on disambig element
                        current_postag = get_attr(e, "postag");
                    }
                    "wd" if in_disambig => {
                        current_wd = Some(DisambigWd {
                            lemma: get_attr(e, "lemma"),
                            pos: get_attr(e, "pos"),
                        });
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                match name.as_str() {
                    "rule" if in_rule => {
                        // Save the rule if we have valid data
                        if let Some(action) = current_action {
                            rules.push(DisambigRule {
                                id: current_rule_id.clone(),
                                pattern: current_pattern.clone(),
                                marker_indices: marker_indices.clone(),
                                action,
                                wd: current_wd.take(),
                                postag: current_postag.take(),
                            });
                        }
                        in_rule = false;
                        current_rule_id.clear();
                    }
                    "pattern" if in_pattern => {
                        in_pattern = false;
                    }
                    "marker" if in_marker => {
                        in_marker = false;
                    }
                    "token" if in_token => {
                        if let Some(mut token) = current_token.take() {
                            if !text_buffer.is_empty() {
                                if token.regexp.is_some() {
                                    // This is a regex pattern
                                    token.regexp = Some(text_buffer.clone());
                                    token.text = None;
                                } else {
                                    token.text = Some(text_buffer.clone());
                                }
                            }
                            // Track if this token is inside <marker>
                            if in_marker {
                                marker_indices.push(current_pattern.len());
                            }
                            current_pattern.push(token);
                        }
                        in_token = false;
                    }
                    "disambig" if in_disambig => {
                        in_disambig = false;
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                if in_token && in_pattern {
                    text_buffer.push_str(&text);
                }
            }
            Ok(Event::Empty(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match name.as_str() {
                    "token" if in_pattern => {
                        let mut token = PatternToken::default();
                        token.inflected = get_attr(e, "inflected")
                            .map(|v| v == "yes")
                            .unwrap_or(false);
                        token.case_sensitive = get_attr(e, "case_sensitive")
                            .map(|v| v == "yes")
                            .unwrap_or(false);
                        token.negation = get_attr(e, "negate")
                            .map(|v| v == "yes")
                            .unwrap_or(false);
                        token.postag = get_attr(e, "postag");
                        token.postag_regexp = get_attr(e, "postag_regexp")
                            .map(|v| v == "yes")
                            .unwrap_or(false);
                        if get_attr(e, "regexp").map(|v| v == "yes").unwrap_or(false) {
                            token.regexp = Some("yes".to_string());
                        }
                        // Track if this token is inside <marker>
                        if in_marker {
                            marker_indices.push(current_pattern.len());
                        }
                        current_pattern.push(token);
                    }
                    "disambig" if in_rule => {
                        let action_str = get_attr(e, "action").unwrap_or_default();
                        current_action = match action_str.as_str() {
                            "replace" => Some(DisambigAction::Replace),
                            "add" => Some(DisambigAction::Add),
                            "remove" => Some(DisambigAction::Remove),
                            "ignore_spelling" => Some(DisambigAction::IgnoreSpelling),
                            "filter" => Some(DisambigAction::Filter),
                            "filterall" => Some(DisambigAction::FilterAll),
                            "unify" => Some(DisambigAction::Unify),
                            "immunize" => Some(DisambigAction::Immunize),
                            _ => None,
                        };
                        current_postag = get_attr(e, "postag");
                    }
                    "wd" if in_disambig => {
                        current_wd = Some(DisambigWd {
                            lemma: get_attr(e, "lemma"),
                            pos: get_attr(e, "pos"),
                        });
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("Error parsing disambiguation.xml at position {}: {:?}", reader.buffer_position(), e);
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(rules)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 6: Extractors - disambiguation rules
// ═══════════════════════════════════════════════════════════════════════════════

/// Extract ignore_spelling patterns (words and regex)
fn extract_ignore_spelling_patterns(rules: &[DisambigRule]) -> (Vec<String>, Vec<String>) {
    let mut words = Vec::new();
    let mut regex_patterns = Vec::new();

    for rule in rules {
        if rule.action != DisambigAction::IgnoreSpelling {
            continue;
        }

        // Only extract single-token patterns or patterns with marker on single token
        let target_tokens: Vec<_> = if rule.marker_indices.is_empty() {
            // No marker - use all tokens if there's just one
            if rule.pattern.len() == 1 {
                vec![&rule.pattern[0]]
            } else {
                continue; // Multi-token without marker - skip
            }
        } else {
            // Has marker - use marked tokens
            rule.marker_indices.iter()
                .filter_map(|&i| rule.pattern.get(i))
                .collect()
        };

        for token in target_tokens {
            if let Some(ref regexp) = token.regexp {
                if regexp != "yes" && !regexp.is_empty() {
                    // This is an actual regex pattern
                    regex_patterns.push(regexp.clone());
                }
            } else if let Some(ref text) = token.text {
                if !text.is_empty() {
                    words.push(text.to_lowercase());
                }
            }
        }
    }

    // Deduplicate
    words.sort();
    words.dedup();
    regex_patterns.sort();
    regex_patterns.dedup();

    (words, regex_patterns)
}

/// Struct for single-token POS rules
#[derive(Debug, Clone)]
struct SingleTokenPosRule {
    word: Option<String>,
    regexp: Option<String>,
    lemma: String,
    pos_tag: String,
}

/// Extract single-token POS rules (replace/add actions)
fn extract_single_token_pos_rules(rules: &[DisambigRule]) -> Vec<SingleTokenPosRule> {
    let mut pos_rules = Vec::new();

    for rule in rules {
        // Only replace and add actions
        if rule.action != DisambigAction::Replace && rule.action != DisambigAction::Add {
            continue;
        }

        // Must have POS information
        let pos_tag = if let Some(ref wd) = rule.wd {
            wd.pos.clone()
        } else if let Some(ref postag) = rule.postag {
            Some(postag.clone())
        } else {
            None
        };

        let pos_tag = match pos_tag {
            Some(p) if !p.is_empty() => p,
            _ => continue,
        };

        let lemma = rule.wd.as_ref()
            .and_then(|wd| wd.lemma.clone())
            .unwrap_or_default();

        // Only single-token patterns or patterns with single marked token
        let target_token = if rule.marker_indices.len() == 1 {
            rule.pattern.get(rule.marker_indices[0])
        } else if rule.pattern.len() == 1 && rule.marker_indices.is_empty() {
            rule.pattern.get(0)
        } else {
            continue;
        };

        if let Some(token) = target_token {
            if let Some(ref regexp) = token.regexp {
                if regexp != "yes" && !regexp.is_empty() {
                    pos_rules.push(SingleTokenPosRule {
                        word: None,
                        regexp: Some(regexp.clone()),
                        lemma,
                        pos_tag,
                    });
                }
            } else if let Some(ref text) = token.text {
                if !text.is_empty() {
                    pos_rules.push(SingleTokenPosRule {
                        word: Some(text.to_lowercase()),
                        regexp: None,
                        lemma,
                        pos_tag,
                    });
                }
            }
        }
    }

    // Deduplicate by word/regexp + pos_tag
    pos_rules.sort_by(|a, b| {
        let key_a = (a.word.as_deref(), a.regexp.as_deref(), &a.pos_tag);
        let key_b = (b.word.as_deref(), b.regexp.as_deref(), &b.pos_tag);
        key_a.cmp(&key_b)
    });
    pos_rules.dedup_by(|a, b| {
        a.word == b.word && a.regexp == b.regexp && a.pos_tag == b.pos_tag
    });

    pos_rules
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 6: Generators - disambiguation files
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_disambig_skip_file(words: &[String], regex_patterns: &[String], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated disambiguation skip patterns for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total: {} words + {} regex patterns\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool disambiguation.xml (action=\"ignore_spelling\")\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! These patterns should be ignored by the spell checker.\n\n",
        lang.to_uppercase(),
        timestamp,
        words.len(),
        regex_patterns.len()
    ));

    // Generate words array
    output.push_str(&format!(
        "/// Skip words for {} spell checker (from disambiguation ignore_spelling rules)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!(
        "pub const {}_DISAMBIG_SKIP: &[&str] = &[\n",
        lang.to_uppercase()
    ));
    for word in words {
        output.push_str(&format!("    \"{}\",\n", escape_string(word)));
    }
    output.push_str("];\n\n");

    // Generate regex patterns array
    output.push_str(&format!(
        "/// Skip regex patterns for {} spell checker (from disambiguation ignore_spelling rules)\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!(
        "pub const {}_DISAMBIG_SKIP_REGEX: &[&str] = &[\n",
        lang.to_uppercase()
    ));
    for pattern in regex_patterns {
        output.push_str(&format!("    r\"{}\",\n", escape_string(pattern)));
    }
    output.push_str("];\n");

    output
}

fn generate_disambig_pos_file(rules: &[SingleTokenPosRule], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated disambiguation POS rules for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total: {} single-token rules\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool disambiguation.xml (action=\"replace\"/\"add\")\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! These rules can be used to enhance POS tagging.\n\n",
        lang.to_uppercase(),
        timestamp,
        rules.len()
    ));

    // Only define struct for EN, other languages import from en_disambig_pos
    if lang == "en" {
        output.push_str("/// A single-token POS disambiguation rule\n");
        output.push_str("#[derive(Debug, Clone)]\n");
        output.push_str("pub struct DisambigPosEntry {\n");
        output.push_str("    /// Literal word to match (case-insensitive), None if regex\n");
        output.push_str("    pub word: Option<&'static str>,\n");
        output.push_str("    /// Regex pattern to match, None if literal word\n");
        output.push_str("    pub regexp: Option<&'static str>,\n");
        output.push_str("    /// Lemma (base form) to assign\n");
        output.push_str("    pub lemma: &'static str,\n");
        output.push_str("    /// POS tag to assign\n");
        output.push_str("    pub pos_tag: &'static str,\n");
        output.push_str("}\n\n");
    } else {
        output.push_str("use super::en_disambig_pos::DisambigPosEntry;\n\n");
    }

    // Generate rules array
    output.push_str(&format!(
        "/// Single-token POS disambiguation rules for {}\n",
        lang.to_uppercase()
    ));
    output.push_str(&format!(
        "pub const {}_DISAMBIG_POS: &[DisambigPosEntry] = &[\n",
        lang.to_uppercase()
    ));

    for rule in rules {
        let word = match &rule.word {
            Some(w) => format!("Some(\"{}\")", escape_string(w)),
            None => "None".to_string(),
        };
        let regexp = match &rule.regexp {
            Some(r) => format!("Some(r\"{}\")", escape_string(r)),
            None => "None".to_string(),
        };
        output.push_str(&format!(
            "    DisambigPosEntry {{ word: {}, regexp: {}, lemma: \"{}\", pos_tag: \"{}\" }},\n",
            word, regexp, escape_string(&rule.lemma), escape_string(&rule.pos_tag)
        ));
    }

    output.push_str("];\n");

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Utilities
// ═══════════════════════════════════════════════════════════════════════════════

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 7: Generator - N-gram confusion words
// ═══════════════════════════════════════════════════════════════════════════════

fn generate_ngram_words_file(words: &[String], lang: &str) -> String {
    let mut output = String::new();
    let timestamp = chrono::Utc::now().to_rfc3339();

    output.push_str(&format!(
        "//! Auto-generated N-gram confusion words for {} from LanguageTool\n\
         //! Synced: {}\n\
         //! Total: {} unique words\n\
         //! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update\n\
         //!\n\
         //! Source: LanguageTool confusion_sets.txt + confusion_sets_extended.txt\n\
         //! License: LGPL 2.1+\n\
         //!\n\
         //! These words are used to filter N-gram data for confusion detection.\n\
         //! N-grams containing any of these words will be extracted from the\n\
         //! full N-gram corpus for efficient confusion pair checking.\n\n",
        lang.to_uppercase(),
        timestamp,
        words.len()
    ));

    // Generate words array
    output.push_str(&format!(
        "/// Words from confusion pairs that need N-gram probability data\n\
         /// These are extracted from confusion_sets.txt and confusion_sets_extended.txt\n"
    ));
    output.push_str(&format!(
        "pub const {}_NGRAM_WORDS: &[&str] = &[\n",
        lang.to_uppercase()
    ));

    for word in words {
        output.push_str(&format!("    \"{}\",\n", escape_string(word)));
    }
    output.push_str("];\n\n");

    // Generate lookup function
    output.push_str(&format!(
        "/// Check if a word needs N-gram probability data\n\
         #[inline]\n\
         pub fn is_{}_ngram_word(word: &str) -> bool {{\n\
         \x20   {}_NGRAM_WORDS.binary_search(&word).is_ok()\n\
         }}\n",
        lang, lang.to_uppercase()
    ));

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 8: N-gram extraction from Lucene indexes
// ═══════════════════════════════════════════════════════════════════════════════

fn print_help() {
    println!("sync-lt - Synchronize LanguageTool rules to grammar-rs");
    println!();
    println!("Usage:");
    println!("  cargo run --bin sync-lt                    # Clone/update LT and sync rules");
    println!("  cargo run --bin sync-lt -- --path ./lt     # Use local LanguageTool path");
    println!();
    println!("N-gram extraction:");
    println!("  cargo run --bin sync-lt -- --extract-ngrams [options]");
    println!();
    println!("Options for --extract-ngrams:");
    println!("  --ngram-path <path>   Path to extracted N-gram data (default: data/ngrams)");
    println!("  --output <path>       Output directory (default: data/ngrams)");
    println!("  --language <lang>     Language to extract: en or fr (default: en)");
    println!();
    println!("Example:");
    println!("  # 1. Download N-gram data first:");
    println!("  curl -L -o data/ngrams/ngrams-en-20150817.zip \\");
    println!("    https://languagetool.org/download/ngram-data/ngrams-en-20150817.zip");
    println!("  unzip data/ngrams/ngrams-en-20150817.zip -d data/ngrams/");
    println!();
    println!("  # 2. Extract to compact format:");
    println!("  cargo run --bin sync-lt -- --extract-ngrams --language en");
}

fn extract_ngrams(
    ngram_path: &Path,
    output_path: &Path,
    lang: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use grammar_rs::language_model::StreamingNgramBuilder;

    println!("Extracting N-grams for language: {}", lang.to_uppercase());
    println!("Input path: {}", ngram_path.display());
    println!("Output path: {}", output_path.display());

    // Create output directory
    fs::create_dir_all(output_path)?;

    // Check for SORTED TSV files (required for streaming builder)
    let sorted_1grams = ngram_path.join(format!("{}_1grams_sorted.tsv", lang));
    let sorted_2grams = ngram_path.join(format!("{}_2grams_sorted.tsv", lang));
    let sorted_3grams = ngram_path.join(format!("{}_3grams_sorted.tsv", lang));

    // Also check for unsorted TSV files
    let tsv_1grams = ngram_path.join(format!("{}_1grams.tsv", lang));
    let tsv_2grams = ngram_path.join(format!("{}_2grams.tsv", lang));
    let tsv_3grams = ngram_path.join(format!("{}_3grams.tsv", lang));

    // Debug: print paths being checked
    eprintln!("DEBUG: Checking sorted paths:");
    eprintln!("  1grams: {} (exists: {})", sorted_1grams.display(), sorted_1grams.exists());
    eprintln!("  2grams: {} (exists: {})", sorted_2grams.display(), sorted_2grams.exists());
    eprintln!("  3grams: {} (exists: {})", sorted_3grams.display(), sorted_3grams.exists());

    let has_sorted = sorted_1grams.exists() || sorted_2grams.exists() || sorted_3grams.exists();
    let has_unsorted = tsv_1grams.exists() || tsv_2grams.exists() || tsv_3grams.exists();

    eprintln!("DEBUG: has_sorted={}, has_unsorted={}", has_sorted, has_unsorted);

    if has_sorted {
        println!("Found sorted TSV files, using streaming builder...");

        let uni_path = if sorted_1grams.exists() { Some(sorted_1grams.as_path()) } else { None };
        let bi_path = if sorted_2grams.exists() { Some(sorted_2grams.as_path()) } else { None };
        let tri_path = if sorted_3grams.exists() { Some(sorted_3grams.as_path()) } else { None };

        let output_file = output_path.join(format!("{}_ngrams.bin", lang));

        let stats = StreamingNgramBuilder::build_from_sorted_tsv(
            uni_path,
            bi_path,
            tri_path,
            &output_file,
        )?;

        println!("\nN-gram extraction complete!");
        println!("{}", stats);

        return Ok(());
    }

    if has_unsorted {
        println!("Found unsorted TSV files. Please sort them first:");
        println!("  LC_ALL=C sort -t$'\\t' -k1,1 {}_1grams.tsv -o {}_1grams_sorted.tsv", lang, lang);
        println!("  LC_ALL=C sort --parallel=4 -S 4G -t$'\\t' -k1,1 {}_2grams.tsv -o {}_2grams_sorted.tsv", lang, lang);
        println!("  LC_ALL=C sort --parallel=4 -S 8G -t$'\\t' -k1,1 {}_3grams.tsv -o {}_3grams_sorted.tsv", lang, lang);
        return Err("Unsorted TSV files found. Sort them first (see above).".into());
    }

    // No TSV files - fall back to legacy Lucene extraction (limited, not recommended)
    println!("No TSV files found. For full N-gram extraction:");
    println!("  1. Download ngrams from https://languagetool.org/download/ngram-data/");
    println!("  2. Extract with Java (see scripts/ExtractNgrams.java)");
    println!("  3. Sort TSV files (see commands above)");
    println!("  4. Run this command again");

    // Legacy fallback for small extractions
    use grammar_rs::lucene::NgramIndexReader;
    use grammar_rs::language_model::CompactNgramBuilder;

    let mut builder = CompactNgramBuilder::new();
    let mut total_unigrams = 0u64;
    let mut total_bigrams = 0u64;
    let mut total_trigrams = 0u64;
    let mut total_tokens = 0u64;

    // Determine the N-gram subdirectories (try both naming conventions)
    let ngram_subdirs = match lang {
        "en" => vec!["en", "ngrams-en-20150817"],
        "fr" => vec!["fr", "ngrams-fr-20150913"],
        _ => return Err(format!("Unsupported language: {}", lang).into()),
    };

    // Find base path
    let base_path = ngram_subdirs
        .iter()
        .map(|subdir| ngram_path.join(subdir))
        .find(|p| p.exists());

    let use_tsv = false; // No TSV files available

    if use_tsv {
        println!("Found pre-extracted TSV files, using those...");

        // Process 1-grams TSV
        if tsv_1grams.exists() {
            println!("Processing 1-grams from {}...", tsv_1grams.display());
            match load_tsv_ngrams(&tsv_1grams) {
                Ok(entries) => {
                    for (ngram, count) in &entries {
                        builder.add_unigram(ngram.clone(), *count);
                        total_tokens += count;
                    }
                    total_unigrams = entries.len() as u64;
                    println!("  Loaded {} unigrams", total_unigrams);
                }
                Err(e) => println!("  Warning: Could not process 1-grams TSV: {}", e),
            }
        }

        // Process 2-grams TSV
        if tsv_2grams.exists() {
            println!("Processing 2-grams from {}...", tsv_2grams.display());
            match load_tsv_ngrams(&tsv_2grams) {
                Ok(entries) => {
                    for (ngram, count) in &entries {
                        builder.add_bigram(ngram.clone(), *count);
                    }
                    total_bigrams = entries.len() as u64;
                    println!("  Loaded {} bigrams", total_bigrams);
                }
                Err(e) => println!("  Warning: Could not process 2-grams TSV: {}", e),
            }
        }

        // Process 3-grams TSV
        if tsv_3grams.exists() {
            println!("Processing 3-grams from {}...", tsv_3grams.display());
            match load_tsv_ngrams(&tsv_3grams) {
                Ok(entries) => {
                    for (ngram, count) in &entries {
                        builder.add_trigram(ngram.clone(), *count);
                    }
                    total_trigrams = entries.len() as u64;
                    println!("  Loaded {} trigrams", total_trigrams);
                }
                Err(e) => println!("  Warning: Could not process 3-grams TSV: {}", e),
            }
        }
    } else if let Some(base_path) = base_path {
        // Fall back to Lucene index reading
        println!("No TSV files found, attempting Lucene index reading...");

        // Process 1-grams
        let unigram_path = base_path.join("1grams");
        if unigram_path.exists() {
            println!("Processing 1-grams from {}...", unigram_path.display());
            match process_ngram_directory(&unigram_path) {
                Ok(entries) => {
                    for (ngram, count) in &entries {
                        builder.add_unigram(ngram.clone(), *count);
                        total_tokens += count;
                    }
                    total_unigrams = entries.len() as u64;
                    println!("  Loaded {} unigrams", total_unigrams);
                }
                Err(e) => {
                    println!("  Warning: Could not process 1-grams: {}", e);
                }
            }
        }

        // Process 2-grams
        let bigram_path = base_path.join("2grams");
        if bigram_path.exists() {
            println!("Processing 2-grams from {}...", bigram_path.display());
            match process_ngram_directory(&bigram_path) {
                Ok(entries) => {
                    for (ngram, count) in &entries {
                        builder.add_bigram(ngram.clone(), *count);
                    }
                    total_bigrams = entries.len() as u64;
                    println!("  Loaded {} bigrams", total_bigrams);
                }
                Err(e) => {
                    println!("  Warning: Could not process 2-grams: {}", e);
                }
            }
        }

        // Process 3-grams
        let trigram_path = base_path.join("3grams");
        if trigram_path.exists() {
            println!("Processing 3-grams from {}...", trigram_path.display());
            match process_ngram_directory(&trigram_path) {
                Ok(entries) => {
                    for (ngram, count) in &entries {
                        builder.add_trigram(ngram.clone(), *count);
                    }
                    total_trigrams = entries.len() as u64;
                    println!("  Loaded {} trigrams", total_trigrams);
                }
                Err(e) => {
                    println!("  Warning: Could not process 3-grams: {}", e);
                }
            }
        }
    } else {
        return Err(format!(
            "No N-gram data found. Either:\n\
             1. Place TSV files at: {}_1grams.tsv, {}_2grams.tsv, {}_3grams.tsv\n\
             2. Or download Lucene indexes from: https://languagetool.org/download/ngram-data/ngrams-{}-*.zip\n\
             Then extract to: {}",
            ngram_path.join(lang).display(),
            ngram_path.join(lang).display(),
            ngram_path.join(lang).display(),
            lang,
            ngram_path.display()
        ).into());
    }

    builder.set_total_tokens(total_tokens);

    // Build compact file
    let output_file = output_path.join(format!("{}_ngrams.bin", lang));
    println!("\nBuilding compact N-gram file: {}", output_file.display());

    let stats = builder.build(&output_file)?;
    println!("{}", stats);

    println!("\nN-gram extraction complete!");
    println!("  Unigrams: {}", total_unigrams);
    println!("  Bigrams: {}", total_bigrams);
    println!("  Trigrams: {}", total_trigrams);
    println!("  Total tokens: {}", total_tokens);

    Ok(())
}

/// Load N-grams from a TSV file (ngram<TAB>count format)
fn load_tsv_ngrams(tsv_path: &Path) -> Result<Vec<(String, u64)>, Box<dyn std::error::Error>> {
    let file = fs::File::open(tsv_path)?;
    let reader = BufReader::with_capacity(1024 * 1024, file); // 1MB buffer

    let mut entries = Vec::new();
    let mut lines_read = 0u64;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('\t').collect();

        if parts.len() >= 2 {
            let ngram = parts[0].to_string();
            if let Ok(count) = parts[1].parse::<u64>() {
                // Skip entries with empty ngrams or very short/weird ones
                if !ngram.is_empty() && ngram.len() < 500 {
                    entries.push((ngram, count));
                }
            }
        }

        lines_read += 1;
        if lines_read % 1_000_000 == 0 {
            eprintln!("  Read {} million lines...", lines_read / 1_000_000);
        }
    }

    Ok(entries)
}

/// Process a single N-gram directory (e.g., 1grams/, 2grams/, 3grams/)
fn process_ngram_directory(dir_path: &Path) -> Result<Vec<(String, u64)>, Box<dyn std::error::Error>> {
    use grammar_rs::lucene::NgramIndexReader;

    let mut all_entries = Vec::new();

    // First, try to read the directory directly as a Lucene index
    match NgramIndexReader::open(dir_path) {
        Ok(reader) => {
            let entries = reader.entries();
            all_entries.extend(entries.iter().map(|(k, v)| (k.clone(), *v)));
            if !all_entries.is_empty() {
                return Ok(all_entries);
            }
        }
        Err(_) => {
            // Not a direct Lucene index, try subdirectories
        }
    }

    // Look for subdirectories that might contain Lucene indexes
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Try to open this subdirectory as a Lucene index
            match NgramIndexReader::open(&path) {
                Ok(reader) => {
                    let entries = reader.entries();
                    all_entries.extend(entries.iter().map(|(k, v)| (k.clone(), *v)));
                }
                Err(e) => {
                    eprintln!("Warning: Could not read {}: {}", path.display(), e);
                }
            }
        }
    }

    // If no Lucene files found, try reading from text files (alternative format)
    if all_entries.is_empty() {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "txt" || ext == "tsv") {
                let file = fs::File::open(&path)?;
                let reader = BufReader::new(file);

                for line in reader.lines() {
                    let line = line?;
                    let parts: Vec<&str> = line.split('\t').collect();
                    if parts.len() >= 2 {
                        let ngram = parts[0].to_string();
                        if let Ok(count) = parts[1].parse::<u64>() {
                            all_entries.push((ngram, count));
                        }
                    }
                }
            }
        }
    }

    Ok(all_entries)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Phase 8: Parser - complex patterns from grammar.xml
// ═══════════════════════════════════════════════════════════════════════════════

/// Extract complex rules that use regex, postag_regexp, skip, etc.
/// These rules can't be compiled to simple Rust code and need runtime interpretation.
fn extract_complex_rules(path: &Path) -> Result<Vec<ComplexRule>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let mut reader = Reader::from_str(&content);
    reader.trim_text(true);

    let mut rules = Vec::new();
    let mut buf = Vec::new();

    let mut current_rule: Option<ComplexRule> = None;
    let mut current_pattern: Vec<ComplexPatternToken> = Vec::new();
    let mut current_antipatterns: Vec<ComplexAntipattern> = Vec::new();
    let mut current_antipattern_tokens: Vec<ComplexPatternToken> = Vec::new();
    let mut in_antipattern = false;
    let mut in_message = false;
    let mut in_suggestion = false;
    let mut in_token = false;
    let mut in_example = false;
    let mut current_category = String::new();
    let mut text_buffer = String::new();
    let mut current_token: Option<ComplexPatternToken> = None;
    let mut current_example: Option<ComplexExample> = None;
    // For dynamic suggestions with <match> elements
    let mut current_suggestion_parts: Vec<SuggestionPart> = Vec::new();
    let mut suggestion_has_match = false;
    // For unification groups
    let mut in_unify = false;
    let mut current_unify_features: Vec<String> = Vec::new();
    let mut current_unify_negate = false;
    let mut current_unify_start_idx: usize = 0;
    let mut current_unification_groups: Vec<UnificationGroup> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                match name.as_str() {
                    "category" => {
                        current_category = get_attr(e, "name").unwrap_or_default();
                    }
                    "rulegroup" | "rule" => {
                        if name == "rule" || (name == "rulegroup" && current_rule.is_none()) {
                            let id = get_attr(e, "id").unwrap_or_default();
                            let rule_name = get_attr(e, "name").unwrap_or_default();
                            if !id.is_empty() {
                                current_rule = Some(ComplexRule {
                                    id,
                                    name: rule_name,
                                    category: current_category.clone(),
                                    pattern: Vec::new(),
                                    antipatterns: Vec::new(),
                                    message: String::new(),
                                    suggestions: Vec::new(),
                                    dynamic_suggestions: Vec::new(),
                                    unification_groups: Vec::new(),
                                    examples: Vec::new(),
                                });
                            }
                        }
                        current_pattern.clear();
                        current_antipatterns.clear();
                        current_unification_groups.clear();
                    }
                    "antipattern" => {
                        in_antipattern = true;
                        current_antipattern_tokens.clear();
                    }
                    "pattern" => {
                        current_pattern.clear();
                    }
                    "unify" => {
                        in_unify = true;
                        current_unify_features.clear();
                        current_unify_negate = get_attr(e, "negate").map(|v| v == "yes").unwrap_or(false);
                        // Track where the unify group starts (current pattern length)
                        current_unify_start_idx = current_pattern.len();
                    }
                    "feature" => {
                        if in_unify {
                            if let Some(feature_id) = get_attr(e, "id") {
                                current_unify_features.push(feature_id);
                            }
                        }
                    }
                    "token" => {
                        let token = ComplexPatternToken {
                            text: None,
                            regexp: get_attr(e, "regexp").map(|v| if v == "yes" { "".to_string() } else { v }),
                            postag: get_attr(e, "postag"),
                            postag_regexp: get_attr(e, "postag_regexp").map(|v| v == "yes").unwrap_or(false),
                            inflected: get_attr(e, "inflected").map(|v| v == "yes").unwrap_or(false),
                            case_sensitive: get_attr(e, "case_sensitive").map(|v| v == "yes").unwrap_or(false),
                            negation: get_attr(e, "negate").map(|v| v == "yes").unwrap_or(false),
                            min: get_attr(e, "min").and_then(|v| v.parse().ok()).unwrap_or(1),
                            max: get_attr(e, "max").and_then(|v| v.parse().ok()).unwrap_or(1),
                            skip: get_attr(e, "skip").and_then(|v| v.parse().ok()),
                        };
                        current_token = Some(token);
                        in_token = true;
                        text_buffer.clear();
                    }
                    "message" => {
                        in_message = true;
                        text_buffer.clear();
                    }
                    "suggestion" => {
                        in_suggestion = true;
                        text_buffer.clear();
                        current_suggestion_parts.clear();
                        suggestion_has_match = false;
                    }
                    "example" => {
                        let correction = get_attr(e, "correction");
                        let is_correct = get_attr(e, "type").map(|v| v != "incorrect").unwrap_or(true);
                        current_example = Some(ComplexExample {
                            text: String::new(),
                            is_correct,
                            correction,
                        });
                        in_example = true;
                        text_buffer.clear();
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                match name.as_str() {
                    "rulegroup" | "rule" => {
                        if let Some(mut rule) = current_rule.take() {
                            rule.pattern = current_pattern.clone();
                            rule.antipatterns = current_antipatterns.clone();
                            rule.unification_groups = current_unification_groups.clone();

                            // Only keep rules that have complex features
                            let is_complex = is_complex_rule(&rule);
                            if is_complex && !rule.id.is_empty() && !rule.pattern.is_empty() {
                                rules.push(rule);
                            }
                        }
                        if name == "rulegroup" {
                            current_rule = None;
                        }
                    }
                    "antipattern" => {
                        if !current_antipattern_tokens.is_empty() {
                            current_antipatterns.push(ComplexAntipattern {
                                tokens: current_antipattern_tokens.clone(),
                            });
                        }
                        in_antipattern = false;
                    }
                    "unify" => {
                        // Create unification group with all tokens since unify started
                        let token_indices: Vec<usize> = (current_unify_start_idx..current_pattern.len()).collect();
                        if !token_indices.is_empty() && !current_unify_features.is_empty() {
                            if !in_antipattern {
                                current_unification_groups.push(UnificationGroup {
                                    features: current_unify_features.clone(),
                                    token_indices,
                                    negate: current_unify_negate,
                                });
                            }
                        }
                        in_unify = false;
                    }
                    "token" => {
                        if let Some(mut token) = current_token.take() {
                            let text = text_buffer.trim().to_string();
                            if !text.is_empty() {
                                // If regexp="yes", store text in regexp field
                                if token.regexp == Some("".to_string()) {
                                    token.regexp = Some(text);
                                    token.text = None;
                                } else {
                                    token.text = Some(text);
                                }
                            }
                            if in_antipattern {
                                current_antipattern_tokens.push(token);
                            } else {
                                current_pattern.push(token);
                            }
                        }
                        in_token = false;
                    }
                    "message" => {
                        if let Some(ref mut rule) = current_rule {
                            rule.message = text_buffer.trim().to_string();
                        }
                        in_message = false;
                    }
                    "suggestion" => {
                        if let Some(ref mut rule) = current_rule {
                            let trimmed = text_buffer.trim().to_string();

                            if suggestion_has_match {
                                // Dynamic suggestion with <match> elements
                                // Add any remaining text, parsing \N references
                                if !trimmed.is_empty() {
                                    current_suggestion_parts.extend(parse_suggestion_text(&trimmed));
                                }
                                if !current_suggestion_parts.is_empty() {
                                    rule.dynamic_suggestions.push(ComplexSuggestion {
                                        parts: current_suggestion_parts.clone(),
                                    });
                                }
                            } else {
                                // Check if the text contains \N references
                                if trimmed.contains("\\") && trimmed.chars().any(|c| c.is_ascii_digit()) {
                                    // Parse as dynamic suggestion
                                    let parts = parse_suggestion_text(&trimmed);
                                    if !parts.is_empty() && parts.iter().any(|p| matches!(p, SuggestionPart::MatchRef { .. })) {
                                        rule.dynamic_suggestions.push(ComplexSuggestion { parts });
                                    } else if !trimmed.is_empty() {
                                        rule.suggestions.push(trimmed);
                                    }
                                } else if !trimmed.is_empty() {
                                    // Simple text suggestion
                                    rule.suggestions.push(trimmed);
                                }
                            }
                        }
                        in_suggestion = false;
                    }
                    "example" => {
                        if let Some(mut example) = current_example.take() {
                            // Clean up marker tags
                            example.text = text_buffer
                                .replace("<marker>", "")
                                .replace("</marker>", "")
                                .trim()
                                .to_string();
                            if let Some(ref mut rule) = current_rule {
                                if !example.text.is_empty() {
                                    rule.examples.push(example);
                                }
                            }
                        }
                        in_example = false;
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                if in_token || in_message || in_suggestion || in_example {
                    text_buffer.push_str(&text);
                }
            }
            Ok(Event::Empty(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if name == "token" {
                    let mut token = ComplexPatternToken {
                        text: None,
                        regexp: get_attr(e, "regexp").map(|v| if v == "yes" { "".to_string() } else { v }),
                        postag: get_attr(e, "postag"),
                        postag_regexp: get_attr(e, "postag_regexp").map(|v| v == "yes").unwrap_or(false),
                        inflected: get_attr(e, "inflected").map(|v| v == "yes").unwrap_or(false),
                        case_sensitive: get_attr(e, "case_sensitive").map(|v| v == "yes").unwrap_or(false),
                        negation: get_attr(e, "negate").map(|v| v == "yes").unwrap_or(false),
                        min: get_attr(e, "min").and_then(|v| v.parse().ok()).unwrap_or(1),
                        max: get_attr(e, "max").and_then(|v| v.parse().ok()).unwrap_or(1),
                        skip: get_attr(e, "skip").and_then(|v| v.parse().ok()),
                    };
                    // For empty tokens, check if there's a text in the tag
                    if token.regexp == Some("".to_string()) {
                        // No text content for empty token, just a regex flag
                        token.regexp = None;
                    }
                    if in_antipattern {
                        current_antipattern_tokens.push(token);
                    } else {
                        current_pattern.push(token);
                    }
                } else if name == "feature" && in_unify {
                    // Handle <feature id="gender"/> inside unify
                    if let Some(feature_id) = get_attr(e, "id") {
                        current_unify_features.push(feature_id);
                    }
                } else if name == "match" && in_suggestion {
                    // Handle <match no="N" .../> inside suggestions
                    suggestion_has_match = true;

                    // Save any accumulated text, parsing \N references
                    let accumulated = text_buffer.trim().to_string();
                    if !accumulated.is_empty() {
                        current_suggestion_parts.extend(parse_suggestion_text(&accumulated));
                    }
                    text_buffer.clear();

                    // Parse match attributes
                    let index = get_attr(e, "no")
                        .and_then(|v| v.parse::<usize>().ok())
                        .unwrap_or(1);
                    let regexp_match = get_attr(e, "regexp_match");
                    let regexp_replace = get_attr(e, "regexp_replace");
                    let postag = get_attr(e, "postag");
                    let postag_replace = get_attr(e, "postag_replace");
                    let case_conversion = get_attr(e, "case_conversion");

                    current_suggestion_parts.push(SuggestionPart::MatchRef {
                        index,
                        regexp_match,
                        regexp_replace,
                        postag,
                        postag_replace,
                        case_conversion,
                    });
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("Error parsing XML at position {}: {:?}", reader.buffer_position(), e);
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(rules)
}

/// Parse suggestion text and convert \N references to MatchRef parts
/// Returns a vector of SuggestionParts (Literal and MatchRef)
fn parse_suggestion_text(text: &str) -> Vec<SuggestionPart> {
    let mut parts = Vec::new();
    let mut current_literal = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            // Check if next char is a digit (token reference)
            if let Some(&next) = chars.peek() {
                if next.is_ascii_digit() {
                    // Save accumulated literal if any
                    if !current_literal.is_empty() {
                        parts.push(SuggestionPart::Literal {
                            text: current_literal.clone(),
                        });
                        current_literal.clear();
                    }

                    // Parse the number (can be multi-digit)
                    let mut num_str = String::new();
                    while let Some(&d) = chars.peek() {
                        if d.is_ascii_digit() {
                            num_str.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    if let Ok(index) = num_str.parse::<usize>() {
                        parts.push(SuggestionPart::MatchRef {
                            index,
                            regexp_match: None,
                            regexp_replace: None,
                            postag: None,
                            postag_replace: None,
                            case_conversion: None,
                        });
                    }
                } else {
                    // Not a number, keep the backslash
                    current_literal.push(c);
                }
            } else {
                // End of string, keep the backslash
                current_literal.push(c);
            }
        } else {
            current_literal.push(c);
        }
    }

    // Don't forget the last literal
    if !current_literal.is_empty() {
        parts.push(SuggestionPart::Literal {
            text: current_literal,
        });
    }

    parts
}

/// Check if a rule uses complex features that require runtime interpretation
fn is_complex_rule(rule: &ComplexRule) -> bool {
    // Check pattern tokens
    for token in &rule.pattern {
        // Has regex pattern in text
        if token.regexp.is_some() {
            return true;
        }
        // Has postag with regex
        if token.postag_regexp {
            return true;
        }
        // Has skip
        if token.skip.is_some() {
            return true;
        }
        // Has min/max different from 1
        if token.min != 1 || token.max != 1 {
            return true;
        }
    }

    // Check if we already have parsed dynamic suggestions
    if !rule.dynamic_suggestions.is_empty() {
        return true;
    }

    // Check if we have unification groups (gender/number agreement)
    if !rule.unification_groups.is_empty() {
        return true;
    }

    // Check if suggestions contain dynamic references (backslash or dollar references)
    for suggestion in &rule.suggestions {
        if suggestion.contains("\\1") || suggestion.contains("\\2") ||
           suggestion.contains("$1") || suggestion.contains("$2") ||
           suggestion.contains("<match") {
            return true;
        }
    }

    // Check if message contains dynamic references
    if rule.message.contains("\\1") || rule.message.contains("\\2") ||
       rule.message.contains("$1") || rule.message.contains("$2") ||
       rule.message.contains("<match") {
        return true;
    }

    false
}
