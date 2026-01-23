//! Quality benchmark for error detection
//!
//! Measures detection quality (precision, recall, F1 score)
//! with annotated test cases for spell checker and grammar rules.
//!
//! Run with: cargo test quality -- --nocapture

use grammar_rs::prelude::*;
use std::sync::Once;

static WARM_UP: Once = Once::new();

fn ensure_warm() {
    WARM_UP.call_once(|| {
        grammar_rs::warm_up();
    });
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

// === Test case structures ===

#[derive(Debug)]
struct SpellTestCase {
    text: &'static str,
    expected_errors: Vec<&'static str>, // Words that should be flagged
}

impl SpellTestCase {
    fn should_detect(text: &'static str, errors: Vec<&'static str>) -> Self {
        Self {
            text,
            expected_errors: errors,
        }
    }

    fn should_be_clean(text: &'static str) -> Self {
        Self {
            text,
            expected_errors: vec![],
        }
    }
}

#[derive(Debug)]
struct RuleTestCase {
    text: &'static str,
    expected_rule: Option<&'static str>, // Rule ID that should match
}

impl RuleTestCase {
    fn should_detect(text: &'static str, rule_id: &'static str) -> Self {
        Self {
            text,
            expected_rule: Some(rule_id),
        }
    }

    fn should_be_clean(text: &'static str) -> Self {
        Self {
            text,
            expected_rule: None,
        }
    }
}

// === Category coverage tracking ===

#[derive(Debug)]
struct CategoryCoverage {
    #[allow(dead_code)] // Useful for programmatic access/serialization
    id: &'static str,
    name: &'static str,
    detected: usize,
    total: usize,
    implemented: bool,
}

impl CategoryCoverage {
    fn new(id: &'static str, name: &'static str) -> Self {
        Self {
            id,
            name,
            detected: 0,
            total: 0,
            implemented: true,
        }
    }

    #[allow(dead_code)]
    fn not_implemented(id: &'static str, name: &'static str) -> Self {
        Self {
            id,
            name,
            detected: 0,
            total: 0,
            implemented: false,
        }
    }

    fn percentage(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.detected as f64 / self.total as f64) * 100.0
        }
    }

    fn display(&self) -> String {
        if !self.implemented {
            format!("[ ] {:<30} - Non implémenté", self.name)
        } else {
            let check = if self.detected == self.total { "✓" } else { "~" };
            format!(
                "[{}] {:<30} - {}/{} détectées ({:.0}%)",
                check,
                self.name,
                self.detected,
                self.total,
                self.percentage()
            )
        }
    }
}

// === Quality metrics ===

#[derive(Debug, Default)]
struct QualityMetrics {
    true_positives: usize,
    false_positives: usize,
    false_negatives: usize,
    true_negatives: usize,
}

impl QualityMetrics {
    fn precision(&self) -> f64 {
        let total = self.true_positives + self.false_positives;
        if total == 0 {
            1.0
        } else {
            self.true_positives as f64 / total as f64
        }
    }

    fn recall(&self) -> f64 {
        let total = self.true_positives + self.false_negatives;
        if total == 0 {
            1.0
        } else {
            self.true_positives as f64 / total as f64
        }
    }

    fn f1(&self) -> f64 {
        let p = self.precision();
        let r = self.recall();
        if p + r == 0.0 {
            0.0
        } else {
            2.0 * (p * r) / (p + r)
        }
    }

    fn merge(&mut self, other: &QualityMetrics) {
        self.true_positives += other.true_positives;
        self.false_positives += other.false_positives;
        self.false_negatives += other.false_negatives;
        self.true_negatives += other.true_negatives;
    }
}

// === Spell checker quality test ===

fn run_spell_quality_tests(cases: &[SpellTestCase]) -> QualityMetrics {
    let dictionary = [
        // Common English words
        "hello", "world", "the", "quick", "brown", "fox", "jumps", "over",
        "lazy", "dog", "receive", "mail", "this", "is", "a", "test",
        "good", "morning", "how", "are", "you", "today", "beautiful",
        "weather", "outside", "cat", "sat", "on", "mat",
    ];

    let pipeline = Pipeline::new(SimpleTokenizer::new(), PassthroughAnalyzer::new())
        .with_checker(SpellChecker::new().with_words(dictionary));

    let mut metrics = QualityMetrics::default();

    for case in cases {
        let result = pipeline.check_text(case.text);
        let detected_words: Vec<&str> = result
            .matches
            .iter()
            .filter(|m| m.rule_id == "SPELL")
            .map(|m| &case.text[m.span.clone()])
            .collect();

        for expected in &case.expected_errors {
            if detected_words.iter().any(|w| w == expected) {
                metrics.true_positives += 1;
            } else {
                metrics.false_negatives += 1;
                eprintln!(
                    "  FN: '{}' not detected in \"{}\"",
                    expected, case.text
                );
            }
        }

        for detected in &detected_words {
            if !case.expected_errors.contains(detected) {
                metrics.false_positives += 1;
                eprintln!(
                    "  FP: '{}' incorrectly flagged in \"{}\"",
                    detected, case.text
                );
            }
        }

        if case.expected_errors.is_empty() && detected_words.is_empty() {
            metrics.true_negatives += 1;
        }
    }

    metrics
}

// === Rule checker quality test ===

fn run_rule_quality_tests(cases: &[RuleTestCase], checker: RuleChecker) -> QualityMetrics {
    let pipeline = Pipeline::new(SimpleTokenizer::new(), PassthroughAnalyzer::new())
        .with_checker(checker);

    let mut metrics = QualityMetrics::default();

    for case in cases {
        let result = pipeline.check_text(case.text);
        let detected_rules: Vec<&str> = result
            .matches
            .iter()
            .map(|m| m.rule_id.as_str())
            .collect();

        match case.expected_rule {
            Some(expected) => {
                if detected_rules.contains(&expected) {
                    metrics.true_positives += 1;
                } else {
                    metrics.false_negatives += 1;
                    eprintln!(
                        "  FN: '{}' not detected in \"{}\" (got: {:?})",
                        expected, case.text, detected_rules
                    );
                }
            }
            None => {
                if detected_rules.is_empty() {
                    metrics.true_negatives += 1;
                } else {
                    metrics.false_positives += 1;
                    eprintln!(
                        "  FP: unexpected rules {:?} in \"{}\"",
                        detected_rules, case.text
                    );
                }
            }
        }
    }

    metrics
}

// === Main quality test ===

#[test]
fn test_quality_metrics() {
    ensure_warm();

    println!("\n");
    println!("╔════════════════════════════════════════════╗");
    println!("║       QUALITY BENCHMARK RESULTS            ║");
    println!("║       grammar-rs v{}                     ║", VERSION);
    println!("╚════════════════════════════════════════════╝\n");

    let mut total_metrics = QualityMetrics::default();
    let mut categories: Vec<CategoryCoverage> = Vec::new();

    // --- Spell Checker Tests ---
    let mut spell_cat = CategoryCoverage::new("SPELL", "Orthographe (SPELL)");
    println!("--- Spell Checker ---");
    let spell_cases = vec![
        // True positives: should detect
        SpellTestCase::should_detect("helo world", vec!["helo"]),
        SpellTestCase::should_detect("teh quick fox", vec!["teh"]),
        SpellTestCase::should_detect("recieve the mail", vec!["recieve"]),
        SpellTestCase::should_detect("beautful weather", vec!["beautful"]),
        SpellTestCase::should_detect("godo morning", vec!["godo"]),
        // True negatives: should NOT detect
        SpellTestCase::should_be_clean("hello world"),
        SpellTestCase::should_be_clean("the quick brown fox"),
        SpellTestCase::should_be_clean("good morning"),
        SpellTestCase::should_be_clean("beautiful weather outside"),
        SpellTestCase::should_be_clean("the cat sat on the mat"),
    ];

    let spell_metrics = run_spell_quality_tests(&spell_cases);
    spell_cat.detected = spell_metrics.true_positives;
    spell_cat.total = spell_metrics.true_positives + spell_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        spell_metrics.true_positives,
        spell_metrics.false_positives,
        spell_metrics.false_negatives,
        spell_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", spell_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", spell_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", spell_metrics.f1() * 100.0);
    total_metrics.merge(&spell_metrics);
    categories.push(spell_cat);

    // --- Double Space Rule ---
    let mut ds_cat = CategoryCoverage::new("DOUBLE_SPACE", "Double espace (DOUBLE_SPACE)");
    println!("\n--- Double Space Rule ---");
    let double_space_cases = vec![
        RuleTestCase::should_detect("Hello  world", "DOUBLE_SPACE"),
        RuleTestCase::should_detect("Foo  bar  baz", "DOUBLE_SPACE"),
        RuleTestCase::should_detect("Test   space", "DOUBLE_SPACE"),
        RuleTestCase::should_be_clean("Hello world"),
        RuleTestCase::should_be_clean("Normal spacing here"),
    ];

    let ds_checker = RuleChecker::new().with_rule(grammar_rs::checker::DoubleSpaceRule);
    let ds_metrics = run_rule_quality_tests(&double_space_cases, ds_checker);
    ds_cat.detected = ds_metrics.true_positives;
    ds_cat.total = ds_metrics.true_positives + ds_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        ds_metrics.true_positives,
        ds_metrics.false_positives,
        ds_metrics.false_negatives,
        ds_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", ds_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", ds_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", ds_metrics.f1() * 100.0);
    total_metrics.merge(&ds_metrics);
    categories.push(ds_cat);

    // --- Repeated Word Rule ---
    let mut rw_cat = CategoryCoverage::new("REPEATED_WORD", "Mot répété (REPEATED_WORD)");
    println!("\n--- Repeated Word Rule ---");
    let repeated_cases = vec![
        RuleTestCase::should_detect("The the cat", "REPEATED_WORD"),
        RuleTestCase::should_detect("I saw saw it", "REPEATED_WORD"),
        RuleTestCase::should_detect("Go go now", "REPEATED_WORD"),
        RuleTestCase::should_be_clean("The cat sat"),
        RuleTestCase::should_be_clean("I saw it yesterday"),
    ];

    let rw_checker = RuleChecker::new().with_rule(grammar_rs::checker::RepeatedWordRule);
    let rw_metrics = run_rule_quality_tests(&repeated_cases, rw_checker);
    rw_cat.detected = rw_metrics.true_positives;
    rw_cat.total = rw_metrics.true_positives + rw_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        rw_metrics.true_positives,
        rw_metrics.false_positives,
        rw_metrics.false_negatives,
        rw_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", rw_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", rw_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", rw_metrics.f1() * 100.0);
    total_metrics.merge(&rw_metrics);
    categories.push(rw_cat);

    // --- A/An Rule ---
    let mut aan_cat = CategoryCoverage::new("EN_A_AN", "A/An (EN_A_AN)");
    println!("\n--- A/An Rule ---");
    let aan_cases = vec![
        RuleTestCase::should_detect("I want a apple", "EN_A_AN"),
        RuleTestCase::should_detect("Give me a orange", "EN_A_AN"),
        RuleTestCase::should_detect("I saw an cat", "EN_A_AN"),
        RuleTestCase::should_detect("There is an dog", "EN_A_AN"),
        RuleTestCase::should_be_clean("I want an apple"),
        RuleTestCase::should_be_clean("I saw a cat"),
        RuleTestCase::should_be_clean("The dog is big"),
    ];

    let aan_checker = RuleChecker::new().with_rule(grammar_rs::checker::AAnRule);
    let aan_metrics = run_rule_quality_tests(&aan_cases, aan_checker);
    aan_cat.detected = aan_metrics.true_positives;
    aan_cat.total = aan_metrics.true_positives + aan_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        aan_metrics.true_positives,
        aan_metrics.false_positives,
        aan_metrics.false_negatives,
        aan_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", aan_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", aan_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", aan_metrics.f1() * 100.0);
    total_metrics.merge(&aan_metrics);
    categories.push(aan_cat);

    // --- French Punctuation Rule ---
    let mut fr_cat = CategoryCoverage::new("FR_PUNCT_SPACE", "Ponctuation FR (FR_PUNCT_SPACE)");
    println!("\n--- French Punctuation Rule ---");
    let fr_punct_cases = vec![
        RuleTestCase::should_detect("Comment?", "FR_PUNCT_SPACE"),
        RuleTestCase::should_detect("Bonjour!", "FR_PUNCT_SPACE"),
        RuleTestCase::should_detect("Oui:", "FR_PUNCT_SPACE"),
        RuleTestCase::should_detect("Vraiment;", "FR_PUNCT_SPACE"),
        RuleTestCase::should_be_clean("Comment ?"),
        RuleTestCase::should_be_clean("Bonjour !"),
        RuleTestCase::should_be_clean("Oui :"),
    ];

    let fr_checker = RuleChecker::new().with_rule(grammar_rs::checker::FrenchPunctuationRule);
    let fr_metrics = run_rule_quality_tests(&fr_punct_cases, fr_checker);
    fr_cat.detected = fr_metrics.true_positives;
    fr_cat.total = fr_metrics.true_positives + fr_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        fr_metrics.true_positives,
        fr_metrics.false_positives,
        fr_metrics.false_negatives,
        fr_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", fr_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", fr_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", fr_metrics.f1() * 100.0);
    total_metrics.merge(&fr_metrics);
    categories.push(fr_cat);

    // --- Uppercase Sentence Start Rule ---
    let mut uc_cat = CategoryCoverage::new("UPPERCASE_SENTENCE_START", "Majuscule début phrase");
    println!("\n--- Uppercase Sentence Start Rule ---");
    let uc_cases = vec![
        RuleTestCase::should_detect("hello world", "UPPERCASE_SENTENCE_START"),
        RuleTestCase::should_detect("the cat sat", "UPPERCASE_SENTENCE_START"),
        RuleTestCase::should_detect("Hello. the dog", "UPPERCASE_SENTENCE_START"),
        RuleTestCase::should_be_clean("Hello world"),
        RuleTestCase::should_be_clean("The cat sat"),
        RuleTestCase::should_be_clean("Hello. The dog"),
    ];

    let uc_checker = RuleChecker::new().with_rule(grammar_rs::checker::UppercaseSentenceStartRule);
    let uc_metrics = run_rule_quality_tests(&uc_cases, uc_checker);
    uc_cat.detected = uc_metrics.true_positives;
    uc_cat.total = uc_metrics.true_positives + uc_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        uc_metrics.true_positives,
        uc_metrics.false_positives,
        uc_metrics.false_negatives,
        uc_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", uc_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", uc_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", uc_metrics.f1() * 100.0);
    total_metrics.merge(&uc_metrics);
    categories.push(uc_cat);

    // --- Repeated Punctuation Rule ---
    let mut rp_cat = CategoryCoverage::new("REPEATED_PUNCTUATION", "Ponctuation répétée");
    println!("\n--- Repeated Punctuation Rule ---");
    let rp_cases = vec![
        RuleTestCase::should_detect("Hello!!", "REPEATED_PUNCTUATION"),
        RuleTestCase::should_detect("What??", "REPEATED_PUNCTUATION"),
        RuleTestCase::should_detect("Stop,, now", "REPEATED_PUNCTUATION"),
        RuleTestCase::should_be_clean("Hello!"),
        RuleTestCase::should_be_clean("What?"),
        RuleTestCase::should_be_clean("Hello..."), // ellipsis allowed
    ];

    let rp_checker = RuleChecker::new().with_rule(grammar_rs::checker::RepeatedPunctuationRule);
    let rp_metrics = run_rule_quality_tests(&rp_cases, rp_checker);
    rp_cat.detected = rp_metrics.true_positives;
    rp_cat.total = rp_metrics.true_positives + rp_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        rp_metrics.true_positives,
        rp_metrics.false_positives,
        rp_metrics.false_negatives,
        rp_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", rp_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", rp_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", rp_metrics.f1() * 100.0);
    total_metrics.merge(&rp_metrics);
    categories.push(rp_cat);

    // --- Missing Space After Punct Rule ---
    let mut ms_cat = CategoryCoverage::new("MISSING_SPACE_AFTER_PUNCT", "Espace après ponctuation");
    println!("\n--- Missing Space After Punct Rule ---");
    let ms_cases = vec![
        RuleTestCase::should_detect("Hello.World", "MISSING_SPACE_AFTER_PUNCT"),
        RuleTestCase::should_detect("Hi,there", "MISSING_SPACE_AFTER_PUNCT"),
        RuleTestCase::should_detect("What?Yes", "MISSING_SPACE_AFTER_PUNCT"),
        RuleTestCase::should_be_clean("Hello. World"),
        RuleTestCase::should_be_clean("The value is 1.5"),
        RuleTestCase::should_be_clean("Visit example.com"),
    ];

    let ms_checker = RuleChecker::new().with_rule(grammar_rs::checker::MissingSpaceAfterPunctRule);
    let ms_metrics = run_rule_quality_tests(&ms_cases, ms_checker);
    ms_cat.detected = ms_metrics.true_positives;
    ms_cat.total = ms_metrics.true_positives + ms_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        ms_metrics.true_positives,
        ms_metrics.false_positives,
        ms_metrics.false_negatives,
        ms_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", ms_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", ms_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", ms_metrics.f1() * 100.0);
    total_metrics.merge(&ms_metrics);
    categories.push(ms_cat);

    // --- Subject-Verb Agreement Rule ---
    let mut sv_cat = CategoryCoverage::new("SUBJECT_VERB_AGREEMENT", "Accord sujet-verbe (EN)");
    println!("\n--- Subject-Verb Agreement Rule ---");
    let sv_cases = vec![
        RuleTestCase::should_detect("He go to school", "SUBJECT_VERB_AGREEMENT"),
        RuleTestCase::should_detect("She have a car", "SUBJECT_VERB_AGREEMENT"),
        RuleTestCase::should_detect("It make sense", "SUBJECT_VERB_AGREEMENT"),
        RuleTestCase::should_detect("They goes home", "SUBJECT_VERB_AGREEMENT"),
        RuleTestCase::should_be_clean("He goes to school"),
        RuleTestCase::should_be_clean("She has a car"),
        RuleTestCase::should_be_clean("They go home"),
    ];

    let sv_checker = RuleChecker::new().with_rule(grammar_rs::checker::SubjectVerbAgreementRule);
    let sv_metrics = run_rule_quality_tests(&sv_cases, sv_checker);
    sv_cat.detected = sv_metrics.true_positives;
    sv_cat.total = sv_metrics.true_positives + sv_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        sv_metrics.true_positives,
        sv_metrics.false_positives,
        sv_metrics.false_negatives,
        sv_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", sv_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", sv_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", sv_metrics.f1() * 100.0);
    total_metrics.merge(&sv_metrics);
    categories.push(sv_cat);

    // --- Its/It's Rule ---
    let mut its_cat = CategoryCoverage::new("EN_ITS_ITS", "Its/It's confusion (EN)");
    println!("\n--- Its/It's Rule ---");
    let its_cases = vec![
        RuleTestCase::should_detect("Its great to see you", "EN_ITS_ITS"),
        RuleTestCase::should_detect("Its a good day", "EN_ITS_ITS"),
        RuleTestCase::should_detect("Its not working", "EN_ITS_ITS"),
        RuleTestCase::should_be_clean("Its color is blue"),
        RuleTestCase::should_be_clean("Its size is perfect"),
        RuleTestCase::should_be_clean("Its own way"),
    ];

    let its_checker = RuleChecker::new().with_rule(grammar_rs::checker::ItsItsRule);
    let its_metrics = run_rule_quality_tests(&its_cases, its_checker);
    its_cat.detected = its_metrics.true_positives;
    its_cat.total = its_metrics.true_positives + its_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        its_metrics.true_positives,
        its_metrics.false_positives,
        its_metrics.false_negatives,
        its_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", its_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", its_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", its_metrics.f1() * 100.0);
    total_metrics.merge(&its_metrics);
    categories.push(its_cat);

    // --- Your/You're Rule ---
    let mut your_cat = CategoryCoverage::new("EN_YOUR_YOURE", "Your/You're confusion (EN)");
    println!("\n--- Your/You're Rule ---");
    let your_cases = vec![
        RuleTestCase::should_detect("Your welcome here", "EN_YOUR_YOURE"),
        RuleTestCase::should_detect("Your going to love it", "EN_YOUR_YOURE"),
        RuleTestCase::should_detect("Your right about that", "EN_YOUR_YOURE"),
        RuleTestCase::should_be_clean("Your car is nice"),
        RuleTestCase::should_be_clean("Your friend called"),
        RuleTestCase::should_be_clean("Your name is great"),
    ];

    let your_checker = RuleChecker::new().with_rule(grammar_rs::checker::YourYoureRule);
    let your_metrics = run_rule_quality_tests(&your_cases, your_checker);
    your_cat.detected = your_metrics.true_positives;
    your_cat.total = your_metrics.true_positives + your_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        your_metrics.true_positives,
        your_metrics.false_positives,
        your_metrics.false_negatives,
        your_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", your_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", your_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", your_metrics.f1() * 100.0);
    total_metrics.merge(&your_metrics);
    categories.push(your_cat);

    // --- Improved A/An Rule ---
    let mut aan_imp_cat = CategoryCoverage::new("EN_A_AN_IMPROVED", "A/An amélioré (silent h)");
    println!("\n--- Improved A/An Rule ---");
    let aan_imp_cases = vec![
        RuleTestCase::should_detect("A hour ago", "EN_A_AN_IMPROVED"),
        RuleTestCase::should_detect("A honest person", "EN_A_AN_IMPROVED"),
        RuleTestCase::should_detect("An university", "EN_A_AN_IMPROVED"),
        RuleTestCase::should_be_clean("An hour ago"),
        RuleTestCase::should_be_clean("An honest person"),
        RuleTestCase::should_be_clean("A university"),
    ];

    let aan_imp_checker = RuleChecker::new().with_rule(grammar_rs::checker::ImprovedAAnRule);
    let aan_imp_metrics = run_rule_quality_tests(&aan_imp_cases, aan_imp_checker);
    aan_imp_cat.detected = aan_imp_metrics.true_positives;
    aan_imp_cat.total = aan_imp_metrics.true_positives + aan_imp_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        aan_imp_metrics.true_positives,
        aan_imp_metrics.false_positives,
        aan_imp_metrics.false_negatives,
        aan_imp_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", aan_imp_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", aan_imp_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", aan_imp_metrics.f1() * 100.0);
    total_metrics.merge(&aan_imp_metrics);
    categories.push(aan_imp_cat);

    // --- French ce/se Rule ---
    let mut cese_cat = CategoryCoverage::new("FR_CE_SE", "Ce/Se confusion (FR)");
    println!("\n--- French Ce/Se Rule ---");
    let cese_cases = vec![
        RuleTestCase::should_detect("Il ce lève tôt", "FR_CE_SE"),
        RuleTestCase::should_detect("Se livre est beau", "FR_CE_SE"),
        RuleTestCase::should_be_clean("Ce livre est beau"),
        RuleTestCase::should_be_clean("Il se lève tôt"),
    ];

    let cese_checker = RuleChecker::new().with_rule(grammar_rs::checker::FrenchCeSeRule);
    let cese_metrics = run_rule_quality_tests(&cese_cases, cese_checker);
    cese_cat.detected = cese_metrics.true_positives;
    cese_cat.total = cese_metrics.true_positives + cese_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        cese_metrics.true_positives,
        cese_metrics.false_positives,
        cese_metrics.false_negatives,
        cese_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", cese_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", cese_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", cese_metrics.f1() * 100.0);
    total_metrics.merge(&cese_metrics);
    categories.push(cese_cat);

    // --- Their/They're/There Rule ---
    let mut ttt_cat = CategoryCoverage::new("EN_THEIR_THEYRE_THERE", "Their/They're/There (EN)");
    println!("\n--- Their/They're/There Rule ---");
    let ttt_cases = vec![
        RuleTestCase::should_detect("There going to the park", "EN_THEIR_THEYRE_THERE"),
        RuleTestCase::should_detect("Their coming soon", "EN_THEIR_THEYRE_THERE"),
        RuleTestCase::should_detect("They took there car", "EN_THEIR_THEYRE_THERE"),
        RuleTestCase::should_be_clean("They're going to the park"),
        RuleTestCase::should_be_clean("Their car is nice"),
        RuleTestCase::should_be_clean("There is a cat"),
    ];

    let ttt_checker = RuleChecker::new().with_rule(grammar_rs::checker::TheirTheyreThereRule);
    let ttt_metrics = run_rule_quality_tests(&ttt_cases, ttt_checker);
    ttt_cat.detected = ttt_metrics.true_positives;
    ttt_cat.total = ttt_metrics.true_positives + ttt_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        ttt_metrics.true_positives,
        ttt_metrics.false_positives,
        ttt_metrics.false_negatives,
        ttt_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", ttt_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", ttt_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", ttt_metrics.f1() * 100.0);
    total_metrics.merge(&ttt_metrics);
    categories.push(ttt_cat);

    // --- Comma Splice Rule ---
    let mut cs_cat = CategoryCoverage::new("COMMA_SPLICE", "Comma splice (EN)");
    println!("\n--- Comma Splice Rule ---");
    let cs_cases = vec![
        RuleTestCase::should_detect("I went home, I was tired", "COMMA_SPLICE"),
        RuleTestCase::should_detect("She is smart, she works hard", "COMMA_SPLICE"),
        RuleTestCase::should_detect("The sun was hot, we went inside", "COMMA_SPLICE"),
        RuleTestCase::should_be_clean("I went home, and I was tired"),
        RuleTestCase::should_be_clean("She is smart and works hard"),
        RuleTestCase::should_be_clean("Although tired, I kept working"),
    ];

    let cs_checker = RuleChecker::new().with_rule(grammar_rs::checker::CommaSpliceRule);
    let cs_metrics = run_rule_quality_tests(&cs_cases, cs_checker);
    cs_cat.detected = cs_metrics.true_positives;
    cs_cat.total = cs_metrics.true_positives + cs_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        cs_metrics.true_positives,
        cs_metrics.false_positives,
        cs_metrics.false_negatives,
        cs_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", cs_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", cs_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", cs_metrics.f1() * 100.0);
    total_metrics.merge(&cs_metrics);
    categories.push(cs_cat);

    // --- Passive Voice Rule ---
    let mut pv_cat = CategoryCoverage::new("PASSIVE_VOICE", "Passive voice detection");
    println!("\n--- Passive Voice Rule ---");
    let pv_cases = vec![
        RuleTestCase::should_detect("The cake was eaten", "PASSIVE_VOICE"),
        RuleTestCase::should_detect("The report is being written", "PASSIVE_VOICE"),
        RuleTestCase::should_detect("The ball was thrown", "PASSIVE_VOICE"),
        RuleTestCase::should_be_clean("John ate the cake"),
        RuleTestCase::should_be_clean("She is running fast"),
        RuleTestCase::should_be_clean("They will finish soon"),
    ];

    let pv_checker = RuleChecker::new().with_rule(grammar_rs::checker::PassiveVoiceRule);
    let pv_metrics = run_rule_quality_tests(&pv_cases, pv_checker);
    pv_cat.detected = pv_metrics.true_positives;
    pv_cat.total = pv_metrics.true_positives + pv_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        pv_metrics.true_positives,
        pv_metrics.false_positives,
        pv_metrics.false_negatives,
        pv_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", pv_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", pv_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", pv_metrics.f1() * 100.0);
    total_metrics.merge(&pv_metrics);
    categories.push(pv_cat);

    // --- Wordiness Rule ---
    let mut wd_cat = CategoryCoverage::new("WORDINESS", "Wordiness detection");
    println!("\n--- Wordiness Rule ---");
    let wd_cases = vec![
        RuleTestCase::should_detect("In order to succeed you must work", "WORDINESS"),
        RuleTestCase::should_detect("Due to the fact that it rained", "WORDINESS"),
        RuleTestCase::should_detect("At this point in time we need", "WORDINESS"),
        RuleTestCase::should_be_clean("To succeed you must work"),
        RuleTestCase::should_be_clean("Because it rained"),
        RuleTestCase::should_be_clean("Now we need"),
    ];

    let wd_checker = RuleChecker::new().with_rule(grammar_rs::checker::WordinessRule);
    let wd_metrics = run_rule_quality_tests(&wd_cases, wd_checker);
    wd_cat.detected = wd_metrics.true_positives;
    wd_cat.total = wd_metrics.true_positives + wd_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        wd_metrics.true_positives,
        wd_metrics.false_positives,
        wd_metrics.false_negatives,
        wd_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", wd_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", wd_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", wd_metrics.f1() * 100.0);
    total_metrics.merge(&wd_metrics);
    categories.push(wd_cat);

    // --- Sentence Fragment Rule ---
    let mut sf_cat = CategoryCoverage::new("SENTENCE_FRAGMENT", "Sentence fragments");
    println!("\n--- Sentence Fragment Rule ---");
    let sf_cases = vec![
        RuleTestCase::should_detect("Because it rained.", "SENTENCE_FRAGMENT"),
        RuleTestCase::should_detect("Although very tired.", "SENTENCE_FRAGMENT"),
        RuleTestCase::should_detect("When the sun sets.", "SENTENCE_FRAGMENT"),
        RuleTestCase::should_be_clean("Because it rained, we stayed home"),
        RuleTestCase::should_be_clean("The sun sets in the west"),
        RuleTestCase::should_be_clean("I am tired"),
    ];

    let sf_checker = RuleChecker::new().with_rule(grammar_rs::checker::SentenceFragmentRule);
    let sf_metrics = run_rule_quality_tests(&sf_cases, sf_checker);
    sf_cat.detected = sf_metrics.true_positives;
    sf_cat.total = sf_metrics.true_positives + sf_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        sf_metrics.true_positives,
        sf_metrics.false_positives,
        sf_metrics.false_negatives,
        sf_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", sf_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", sf_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", sf_metrics.f1() * 100.0);
    total_metrics.merge(&sf_metrics);
    categories.push(sf_cat);

    // --- French Subject-Verb Rule ---
    let mut frsv_cat = CategoryCoverage::new("FR_SUBJECT_VERB", "Accord sujet-verbe (FR)");
    println!("\n--- French Subject-Verb Rule ---");
    let frsv_cases = vec![
        RuleTestCase::should_detect("Il mangent bien", "FR_SUBJECT_VERB"),
        RuleTestCase::should_detect("Ils mange bien", "FR_SUBJECT_VERB"),
        RuleTestCase::should_detect("Elle sont partie", "FR_SUBJECT_VERB"),
        RuleTestCase::should_be_clean("Il mange bien"),
        RuleTestCase::should_be_clean("Ils mangent bien"),
        RuleTestCase::should_be_clean("Elle est partie"),
    ];

    let frsv_checker = RuleChecker::new().with_rule(grammar_rs::checker::FrenchSubjectVerbRule);
    let frsv_metrics = run_rule_quality_tests(&frsv_cases, frsv_checker);
    frsv_cat.detected = frsv_metrics.true_positives;
    frsv_cat.total = frsv_metrics.true_positives + frsv_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        frsv_metrics.true_positives,
        frsv_metrics.false_positives,
        frsv_metrics.false_negatives,
        frsv_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", frsv_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", frsv_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", frsv_metrics.f1() * 100.0);
    total_metrics.merge(&frsv_metrics);
    categories.push(frsv_cat);

    // --- French Adjective-Noun Rule ---
    let mut fran_cat = CategoryCoverage::new("FR_ADJ_NOUN", "Accord adjectif-nom (FR)");
    println!("\n--- French Adjective-Noun Rule ---");
    let fran_cases = vec![
        RuleTestCase::should_detect("Une petit maison", "FR_ADJ_NOUN"),
        RuleTestCase::should_detect("Un grande jardin", "FR_ADJ_NOUN"),
        RuleTestCase::should_detect("Le chat noire", "FR_ADJ_NOUN"),
        RuleTestCase::should_be_clean("Une petite maison"),
        RuleTestCase::should_be_clean("Un grand jardin"),
        RuleTestCase::should_be_clean("Le chat noir"),
    ];

    let fran_checker = RuleChecker::new().with_rule(grammar_rs::checker::FrenchAdjectiveNounRule);
    let fran_metrics = run_rule_quality_tests(&fran_cases, fran_checker);
    fran_cat.detected = fran_metrics.true_positives;
    fran_cat.total = fran_metrics.true_positives + fran_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        fran_metrics.true_positives,
        fran_metrics.false_positives,
        fran_metrics.false_negatives,
        fran_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", fran_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", fran_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", fran_metrics.f1() * 100.0);
    total_metrics.merge(&fran_metrics);
    categories.push(fran_cat);

    // --- Typographic Quotes Rule ---
    let mut tq_cat = CategoryCoverage::new("TYPOGRAPHIC_QUOTES", "Guillemets typographiques");
    println!("\n--- Typographic Quotes Rule ---");
    let tq_cases = vec![
        RuleTestCase::should_detect("He said \"hello\"", "TYPOGRAPHIC_QUOTES"),
        RuleTestCase::should_detect("The \"test\" worked", "TYPOGRAPHIC_QUOTES"),
        RuleTestCase::should_detect("Called \"foo\"", "TYPOGRAPHIC_QUOTES"),
        RuleTestCase::should_be_clean("He said hello"),
        RuleTestCase::should_be_clean("The test worked"),
        RuleTestCase::should_be_clean("Called foo"),
    ];

    let tq_checker = RuleChecker::new().with_rule(grammar_rs::checker::TypographicQuotesRule);
    let tq_metrics = run_rule_quality_tests(&tq_cases, tq_checker);
    tq_cat.detected = tq_metrics.true_positives;
    tq_cat.total = tq_metrics.true_positives + tq_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        tq_metrics.true_positives,
        tq_metrics.false_positives,
        tq_metrics.false_negatives,
        tq_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", tq_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", tq_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", tq_metrics.f1() * 100.0);
    total_metrics.merge(&tq_metrics);
    categories.push(tq_cat);

    // --- English Confusion Rule ---
    let mut en_conf_cat = CategoryCoverage::new("EN_CONFUSION", "English confusion pairs");
    println!("\n--- English Confusion Rule ---");
    let en_conf_cases = vec![
        RuleTestCase::should_detect("the affect of the medicine", "EN_CONFUSION"),
        RuleTestCase::should_detect("I want to loose weight", "EN_CONFUSION"),
        RuleTestCase::should_detect("the team was lead by him", "EN_CONFUSION"),
        RuleTestCase::should_be_clean("to affect the outcome"),
        RuleTestCase::should_be_clean("the loose screw"),
        RuleTestCase::should_be_clean("lead the way"),
    ];

    let en_conf_checker = RuleChecker::new().with_rule(grammar_rs::checker::EnglishConfusionRule);
    let en_conf_metrics = run_rule_quality_tests(&en_conf_cases, en_conf_checker);
    en_conf_cat.detected = en_conf_metrics.true_positives;
    en_conf_cat.total = en_conf_metrics.true_positives + en_conf_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        en_conf_metrics.true_positives,
        en_conf_metrics.false_positives,
        en_conf_metrics.false_negatives,
        en_conf_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", en_conf_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", en_conf_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", en_conf_metrics.f1() * 100.0);
    total_metrics.merge(&en_conf_metrics);
    categories.push(en_conf_cat);

    // --- French Confusion Rule ---
    let mut fr_conf_cat = CategoryCoverage::new("FR_CONFUSION", "French confusion pairs");
    println!("\n--- French Confusion Rule ---");
    let fr_conf_cases = vec![
        RuleTestCase::should_detect("c'est le notre", "FR_CONFUSION"),
        RuleTestCase::should_be_clean("notre maison est belle"),
        RuleTestCase::should_be_clean("le vin rouge"),
    ];

    let fr_conf_checker = RuleChecker::new().with_rule(grammar_rs::checker::FrenchConfusionRule);
    let fr_conf_metrics = run_rule_quality_tests(&fr_conf_cases, fr_conf_checker);
    fr_conf_cat.detected = fr_conf_metrics.true_positives;
    fr_conf_cat.total = fr_conf_metrics.true_positives + fr_conf_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        fr_conf_metrics.true_positives,
        fr_conf_metrics.false_positives,
        fr_conf_metrics.false_negatives,
        fr_conf_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", fr_conf_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", fr_conf_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", fr_conf_metrics.f1() * 100.0);
    total_metrics.merge(&fr_conf_metrics);
    categories.push(fr_conf_cat);

    // --- Less/Fewer Rule ---
    let mut lf_cat = CategoryCoverage::new("LESS_FEWER", "Less/Fewer (EN)");
    println!("\n--- Less/Fewer Rule ---");
    let lf_cases = vec![
        RuleTestCase::should_detect("I have less items than you", "LESS_FEWER"),
        RuleTestCase::should_detect("There are less people here", "LESS_FEWER"),
        RuleTestCase::should_detect("We need less cars on the road", "LESS_FEWER"),
        RuleTestCase::should_be_clean("I have fewer items than you"),
        RuleTestCase::should_be_clean("There is less water"),
        RuleTestCase::should_be_clean("We need less time"),
    ];

    let lf_checker = RuleChecker::new().with_rule(grammar_rs::checker::LessFewerRule);
    let lf_metrics = run_rule_quality_tests(&lf_cases, lf_checker);
    lf_cat.detected = lf_metrics.true_positives;
    lf_cat.total = lf_metrics.true_positives + lf_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        lf_metrics.true_positives,
        lf_metrics.false_positives,
        lf_metrics.false_negatives,
        lf_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", lf_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", lf_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", lf_metrics.f1() * 100.0);
    total_metrics.merge(&lf_metrics);
    categories.push(lf_cat);

    // --- Who/Whom Rule ---
    let mut ww_cat = CategoryCoverage::new("WHO_WHOM", "Who/Whom (EN)");
    println!("\n--- Who/Whom Rule ---");
    let ww_cases = vec![
        RuleTestCase::should_detect("To who did you give it", "WHO_WHOM"),
        RuleTestCase::should_detect("For who is this gift", "WHO_WHOM"),
        RuleTestCase::should_detect("With who are you going", "WHO_WHOM"),
        RuleTestCase::should_be_clean("To whom did you give it"),
        RuleTestCase::should_be_clean("Who is coming to the party"),
        RuleTestCase::should_be_clean("Who wants ice cream"),
    ];

    let ww_checker = RuleChecker::new().with_rule(grammar_rs::checker::WhoWhomRule);
    let ww_metrics = run_rule_quality_tests(&ww_cases, ww_checker);
    ww_cat.detected = ww_metrics.true_positives;
    ww_cat.total = ww_metrics.true_positives + ww_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        ww_metrics.true_positives,
        ww_metrics.false_positives,
        ww_metrics.false_negatives,
        ww_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", ww_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", ww_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", ww_metrics.f1() * 100.0);
    total_metrics.merge(&ww_metrics);
    categories.push(ww_cat);

    // --- Good/Well Rule ---
    let mut gw_cat = CategoryCoverage::new("GOOD_WELL", "Good/Well (EN)");
    println!("\n--- Good/Well Rule ---");
    let gw_cases = vec![
        RuleTestCase::should_detect("You did good on the test", "GOOD_WELL"),
        RuleTestCase::should_detect("She plays good", "GOOD_WELL"),
        RuleTestCase::should_detect("He runs good", "GOOD_WELL"),
        RuleTestCase::should_be_clean("You did well on the test"),
        RuleTestCase::should_be_clean("That is a good idea"),
        RuleTestCase::should_be_clean("Good morning everyone"),
    ];

    let gw_checker = RuleChecker::new().with_rule(grammar_rs::checker::GoodWellRule);
    let gw_metrics = run_rule_quality_tests(&gw_cases, gw_checker);
    gw_cat.detected = gw_metrics.true_positives;
    gw_cat.total = gw_metrics.true_positives + gw_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        gw_metrics.true_positives,
        gw_metrics.false_positives,
        gw_metrics.false_negatives,
        gw_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", gw_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", gw_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", gw_metrics.f1() * 100.0);
    total_metrics.merge(&gw_metrics);
    categories.push(gw_cat);

    // --- Double Negative Rule ---
    let mut dn_cat = CategoryCoverage::new("DOUBLE_NEGATIVE", "Double negative (EN)");
    println!("\n--- Double Negative Rule ---");
    let dn_cases = vec![
        RuleTestCase::should_detect("I do not have no money", "DOUBLE_NEGATIVE"),
        RuleTestCase::should_detect("She cannot do nothing about it", "DOUBLE_NEGATIVE"),
        RuleTestCase::should_detect("They never go nowhere", "DOUBLE_NEGATIVE"),
        RuleTestCase::should_be_clean("I do not have any money"),
        RuleTestCase::should_be_clean("She cannot do anything"),
        RuleTestCase::should_be_clean("They never go anywhere"),
    ];

    let dn_checker = RuleChecker::new().with_rule(grammar_rs::checker::DoubleNegativeRule);
    let dn_metrics = run_rule_quality_tests(&dn_cases, dn_checker);
    dn_cat.detected = dn_metrics.true_positives;
    dn_cat.total = dn_metrics.true_positives + dn_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        dn_metrics.true_positives,
        dn_metrics.false_positives,
        dn_metrics.false_negatives,
        dn_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", dn_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", dn_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", dn_metrics.f1() * 100.0);
    total_metrics.merge(&dn_metrics);
    categories.push(dn_cat);

    // --- French Conditionnel Si Rule ---
    let mut frcs_cat = CategoryCoverage::new("FR_CONDITIONNEL_SI", "Conditionnel après si (FR)");
    println!("\n--- French Conditionnel Si Rule ---");
    let frcs_cases = vec![
        RuleTestCase::should_detect("Si j'aurais su", "FR_CONDITIONNEL_SI"),
        RuleTestCase::should_detect("Si tu aurais le temps", "FR_CONDITIONNEL_SI"),
        RuleTestCase::should_detect("Si on aurait de l'argent", "FR_CONDITIONNEL_SI"),
        RuleTestCase::should_be_clean("Si j'avais su"),
        RuleTestCase::should_be_clean("Si tu avais le temps"),
        RuleTestCase::should_be_clean("J'aurais aimé venir"),
    ];

    let frcs_checker = RuleChecker::new().with_rule(grammar_rs::checker::FrenchConditionnelSiRule);
    let frcs_metrics = run_rule_quality_tests(&frcs_cases, frcs_checker);
    frcs_cat.detected = frcs_metrics.true_positives;
    frcs_cat.total = frcs_metrics.true_positives + frcs_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        frcs_metrics.true_positives,
        frcs_metrics.false_positives,
        frcs_metrics.false_negatives,
        frcs_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", frcs_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", frcs_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", frcs_metrics.f1() * 100.0);
    total_metrics.merge(&frcs_metrics);
    categories.push(frcs_cat);

    // --- French Tout Accord Rule ---
    let mut frta_cat = CategoryCoverage::new("FR_TOUT_ACCORD", "Accord de tout (FR)");
    println!("\n--- French Tout Accord Rule ---");
    let frta_cases = vec![
        RuleTestCase::should_detect("Tout les jours", "FR_TOUT_ACCORD"),
        RuleTestCase::should_detect("Tout les enfants jouent", "FR_TOUT_ACCORD"),
        RuleTestCase::should_detect("Tout les matins", "FR_TOUT_ACCORD"),
        RuleTestCase::should_be_clean("Tous les jours"),
        RuleTestCase::should_be_clean("Tout le monde"),
        RuleTestCase::should_be_clean("Toutes les femmes"),
    ];

    let frta_checker = RuleChecker::new().with_rule(grammar_rs::checker::FrenchToutAccordRule);
    let frta_metrics = run_rule_quality_tests(&frta_cases, frta_checker);
    frta_cat.detected = frta_metrics.true_positives;
    frta_cat.total = frta_metrics.true_positives + frta_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        frta_metrics.true_positives,
        frta_metrics.false_positives,
        frta_metrics.false_negatives,
        frta_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", frta_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", frta_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", frta_metrics.f1() * 100.0);
    total_metrics.merge(&frta_metrics);
    categories.push(frta_cat);

    // --- Sentence Length Rule ---
    let mut sl_cat = CategoryCoverage::new("SENTENCE_LENGTH", "Longueur de phrase");
    println!("\n--- Sentence Length Rule ---");
    let sl_cases = vec![
        RuleTestCase::should_detect("The quick brown fox jumps over the lazy dog and runs through the forest while the birds sing and the sun shines brightly in the clear blue sky above the mountains where the snow never melts and the wind blows cold.", "SENTENCE_LENGTH"),
        RuleTestCase::should_detect("When I was young I used to play in the garden with my friends and we would run around all day long until our parents called us in for dinner and then we would eat and go to bed tired but happy.", "SENTENCE_LENGTH"),
        RuleTestCase::should_be_clean("The quick brown fox jumps over the lazy dog."),
        RuleTestCase::should_be_clean("This is a normal sentence."),
        RuleTestCase::should_be_clean("Short and sweet."),
    ];

    let sl_checker = RuleChecker::new().with_rule(grammar_rs::checker::SentenceLengthRule::new());
    let sl_metrics = run_rule_quality_tests(&sl_cases, sl_checker);
    sl_cat.detected = sl_metrics.true_positives;
    sl_cat.total = sl_metrics.true_positives + sl_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        sl_metrics.true_positives,
        sl_metrics.false_positives,
        sl_metrics.false_negatives,
        sl_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", sl_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", sl_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", sl_metrics.f1() * 100.0);
    total_metrics.merge(&sl_metrics);
    categories.push(sl_cat);

    // --- Cliche Rule ---
    let mut cl_cat = CategoryCoverage::new("CLICHE", "Clichés détectés");
    println!("\n--- Cliche Rule ---");
    let cl_cases = vec![
        RuleTestCase::should_detect("We need to think outside the box", "CLICHE"),
        RuleTestCase::should_detect("Let's touch base tomorrow", "CLICHE"),
        RuleTestCase::should_detect("We should leverage this opportunity", "CLICHE"),
        RuleTestCase::should_be_clean("We need creative solutions"),
        RuleTestCase::should_be_clean("Let's discuss this later"),
        RuleTestCase::should_be_clean("We should combine our strengths"),
    ];

    let cl_checker = RuleChecker::new().with_rule(grammar_rs::checker::ClicheRule);
    let cl_metrics = run_rule_quality_tests(&cl_cases, cl_checker);
    cl_cat.detected = cl_metrics.true_positives;
    cl_cat.total = cl_metrics.true_positives + cl_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        cl_metrics.true_positives,
        cl_metrics.false_positives,
        cl_metrics.false_negatives,
        cl_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", cl_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", cl_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", cl_metrics.f1() * 100.0);
    total_metrics.merge(&cl_metrics);
    categories.push(cl_cat);

    // --- Redundancy Rule ---
    let mut rd_cat = CategoryCoverage::new("REDUNDANCY", "Pléonasmes détectés");
    println!("\n--- Redundancy Rule ---");
    let rd_cases = vec![
        RuleTestCase::should_detect("Let me tell you about the past history", "REDUNDANCY"),
        RuleTestCase::should_detect("Please return back to sender", "REDUNDANCY"),
        RuleTestCase::should_detect("We made a new innovation", "REDUNDANCY"),
        RuleTestCase::should_be_clean("Let me tell you about the history"),
        RuleTestCase::should_be_clean("Please return to sender"),
        RuleTestCase::should_be_clean("We made an innovation"),
    ];

    let rd_checker = RuleChecker::new().with_rule(grammar_rs::checker::RedundancyRule);
    let rd_metrics = run_rule_quality_tests(&rd_cases, rd_checker);
    rd_cat.detected = rd_metrics.true_positives;
    rd_cat.total = rd_metrics.true_positives + rd_metrics.false_negatives;
    println!("  TP: {}, FP: {}, FN: {}, TN: {}",
        rd_metrics.true_positives,
        rd_metrics.false_positives,
        rd_metrics.false_negatives,
        rd_metrics.true_negatives
    );
    println!("  Precision: {:.1}%", rd_metrics.precision() * 100.0);
    println!("  Recall:    {:.1}%", rd_metrics.recall() * 100.0);
    println!("  F1 Score:  {:.1}%", rd_metrics.f1() * 100.0);
    total_metrics.merge(&rd_metrics);
    categories.push(rd_cat);

    // --- StyleChecker EN (Wordiness/Redundancy) ---
    let mut style_en_cat = CategoryCoverage::new("STYLE_EN", "StyleChecker EN (wordiness)");
    println!("\n--- StyleChecker EN ---");
    {
        use grammar_rs::checker::StyleChecker;
        use grammar_rs::core::traits::Checker;

        let checker = StyleChecker::new();
        let tokenizer = SimpleTokenizer::new();
        let analyzer = PassthroughAnalyzer::new();

        let style_cases = vec![
            ("In order to succeed you must work hard.", true, "WORDINESS"),
            ("Due to the fact that it rained we stayed.", true, "WORDINESS"),
            ("Meet me at 12 noon for lunch.", true, "REDUNDANCY"),
            ("The end result was positive.", true, "REDUNDANCY"),
            ("To succeed you must work.", false, ""),
            ("Because it rained we stayed.", false, ""),
        ];

        let mut tp = 0;
        let mut fp = 0;
        let mut fn_count = 0;
        let mut tn = 0;

        for (text, should_detect, expected_type) in &style_cases {
            let tokens = tokenizer.tokenize(text);
            let analyzed = analyzer.analyze(tokens);
            let result = checker.check(text, &analyzed);

            let has_match = result.matches.iter().any(|m|
                m.rule_id.contains("WORDINESS") || m.rule_id.contains("REDUNDANCY")
            );

            if *should_detect {
                if has_match {
                    tp += 1;
                } else {
                    fn_count += 1;
                    eprintln!("  FN: {} not detected in \"{}\"", expected_type, text);
                }
            } else {
                if has_match {
                    fp += 1;
                    eprintln!("  FP: unexpected match in \"{}\"", text);
                } else {
                    tn += 1;
                }
            }
        }

        style_en_cat.detected = tp;
        style_en_cat.total = tp + fn_count;
        println!("  TP: {}, FP: {}, FN: {}, TN: {}", tp, fp, fn_count, tn);
        let style_metrics = QualityMetrics {
            true_positives: tp,
            false_positives: fp,
            false_negatives: fn_count,
            true_negatives: tn,
        };
        println!("  Precision: {:.1}%", style_metrics.precision() * 100.0);
        println!("  Recall:    {:.1}%", style_metrics.recall() * 100.0);
        println!("  F1 Score:  {:.1}%", style_metrics.f1() * 100.0);
        total_metrics.merge(&style_metrics);
    }
    categories.push(style_en_cat);

    // --- StyleChecker FR ---
    let mut style_fr_cat = CategoryCoverage::new("STYLE_FR", "StyleChecker FR");
    println!("\n--- StyleChecker FR ---");
    {
        use grammar_rs::checker::StyleChecker;
        use grammar_rs::core::traits::Checker;

        let checker = StyleChecker::french();
        let tokenizer = SimpleTokenizer::new();
        let analyzer = PassthroughAnalyzer::new();

        let style_cases = vec![
            ("Afin de réussir il faut travailler.", true, "WORDINESS"),
            ("Il est important de noter que cela marche.", true, "WORDINESS"),
            ("Le chat dort sur le tapis.", false, ""),
        ];

        let mut tp = 0;
        let mut fp = 0;
        let mut fn_count = 0;
        let mut tn = 0;

        for (text, should_detect, expected_type) in &style_cases {
            let tokens = tokenizer.tokenize(text);
            let analyzed = analyzer.analyze(tokens);
            let result = checker.check(text, &analyzed);

            let has_match = result.matches.iter().any(|m|
                m.rule_id.contains("WORDINESS") || m.rule_id.contains("REDUNDANCY")
            );

            if *should_detect {
                if has_match {
                    tp += 1;
                } else {
                    fn_count += 1;
                    eprintln!("  FN: {} not detected in \"{}\"", expected_type, text);
                }
            } else {
                if has_match {
                    fp += 1;
                    eprintln!("  FP: unexpected match in \"{}\"", text);
                } else {
                    tn += 1;
                }
            }
        }

        style_fr_cat.detected = tp;
        style_fr_cat.total = tp + fn_count;
        println!("  TP: {}, FP: {}, FN: {}, TN: {}", tp, fp, fn_count, tn);
        let style_metrics = QualityMetrics {
            true_positives: tp,
            false_positives: fp,
            false_negatives: fn_count,
            true_negatives: tn,
        };
        println!("  Precision: {:.1}%", style_metrics.precision() * 100.0);
        println!("  Recall:    {:.1}%", style_metrics.recall() * 100.0);
        println!("  F1 Score:  {:.1}%", style_metrics.f1() * 100.0);
        total_metrics.merge(&style_metrics);
    }
    categories.push(style_fr_cat);

    // --- CompoundWordChecker EN ---
    let mut compound_en_cat = CategoryCoverage::new("COMPOUND_EN", "CompoundWordChecker EN");
    println!("\n--- CompoundWordChecker EN ---");
    {
        use grammar_rs::checker::CompoundWordChecker;
        use grammar_rs::core::traits::Checker;

        let checker = CompoundWordChecker::new();
        let tokenizer = SimpleTokenizer::new();
        let analyzer = PassthroughAnalyzer::new();

        let compound_cases = vec![
            ("The air plane landed safely.", true, "airplane"),
            ("Your well being matters.", true, "well-being"),
            ("The airplane is ready.", false, ""),
            ("The cat sat on the mat.", false, ""),
        ];

        let mut tp = 0;
        let mut fp = 0;
        let mut fn_count = 0;
        let mut tn = 0;

        for (text, should_detect, expected_suggestion) in &compound_cases {
            let tokens = tokenizer.tokenize(text);
            let analyzed = analyzer.analyze(tokens);
            let result = checker.check(text, &analyzed);

            let has_match = result.matches.iter().any(|m| m.rule_id.contains("COMPOUND"));

            if *should_detect {
                if has_match {
                    tp += 1;
                    // Verify suggestion
                    let has_correct_suggestion = result.matches.iter()
                        .any(|m| m.suggestions.contains(&expected_suggestion.to_string()));
                    if !has_correct_suggestion && !expected_suggestion.is_empty() {
                        eprintln!("  Warning: expected suggestion '{}' not found", expected_suggestion);
                    }
                } else {
                    fn_count += 1;
                    eprintln!("  FN: compound not detected in \"{}\"", text);
                }
            } else {
                if has_match {
                    fp += 1;
                    eprintln!("  FP: unexpected compound match in \"{}\"", text);
                } else {
                    tn += 1;
                }
            }
        }

        compound_en_cat.detected = tp;
        compound_en_cat.total = tp + fn_count;
        println!("  TP: {}, FP: {}, FN: {}, TN: {}", tp, fp, fn_count, tn);
        let compound_metrics = QualityMetrics {
            true_positives: tp,
            false_positives: fp,
            false_negatives: fn_count,
            true_negatives: tn,
        };
        println!("  Precision: {:.1}%", compound_metrics.precision() * 100.0);
        println!("  Recall:    {:.1}%", compound_metrics.recall() * 100.0);
        println!("  F1 Score:  {:.1}%", compound_metrics.f1() * 100.0);
        total_metrics.merge(&compound_metrics);
    }
    categories.push(compound_en_cat);

    // --- CompoundWordChecker FR ---
    let mut compound_fr_cat = CategoryCoverage::new("COMPOUND_FR", "CompoundWordChecker FR");
    println!("\n--- CompoundWordChecker FR ---");
    {
        use grammar_rs::checker::CompoundWordChecker;
        use grammar_rs::core::traits::Checker;

        let checker = CompoundWordChecker::french();
        let tokenizer = SimpleTokenizer::new();
        let analyzer = PassthroughAnalyzer::new();

        let compound_cases = vec![
            ("J'ai pris un aller retour.", true, "aller-retour"),
            ("C'est un chef d'oeuvre.", true, "chef-d'oeuvre"),
            ("Le chat dort sur le tapis.", false, ""),
        ];

        let mut tp = 0;
        let mut fp = 0;
        let mut fn_count = 0;
        let mut tn = 0;

        for (text, should_detect, expected_suggestion) in &compound_cases {
            let tokens = tokenizer.tokenize(text);
            let analyzed = analyzer.analyze(tokens);
            let result = checker.check(text, &analyzed);

            let has_match = result.matches.iter().any(|m| m.rule_id.contains("COMPOUND"));

            if *should_detect {
                if has_match {
                    tp += 1;
                } else {
                    fn_count += 1;
                    eprintln!("  FN: compound '{}' not detected in \"{}\"", expected_suggestion, text);
                }
            } else {
                if has_match {
                    fp += 1;
                    eprintln!("  FP: unexpected compound match in \"{}\"", text);
                } else {
                    tn += 1;
                }
            }
        }

        compound_fr_cat.detected = tp;
        compound_fr_cat.total = tp + fn_count;
        println!("  TP: {}, FP: {}, FN: {}, TN: {}", tp, fp, fn_count, tn);
        let compound_metrics = QualityMetrics {
            true_positives: tp,
            false_positives: fp,
            false_negatives: fn_count,
            true_negatives: tn,
        };
        println!("  Precision: {:.1}%", compound_metrics.precision() * 100.0);
        println!("  Recall:    {:.1}%", compound_metrics.recall() * 100.0);
        println!("  F1 Score:  {:.1}%", compound_metrics.f1() * 100.0);
        total_metrics.merge(&compound_metrics);
    }
    categories.push(compound_fr_cat);

    // --- ProhibitChecker EN ---
    let mut prohibit_cat = CategoryCoverage::new("PROHIBIT", "ProhibitChecker EN");
    println!("\n--- ProhibitChecker EN ---");
    {
        use grammar_rs::checker::ProhibitChecker;
        use grammar_rs::core::traits::Checker;

        let checker = ProhibitChecker::new();
        let tokenizer = SimpleTokenizer::new();
        let analyzer = PassthroughAnalyzer::new();

        let prohibit_cases = vec![
            ("Christoper went to the store.", true, "Christopher"),
            ("The GDPR-complaint system works.", true, "GDPR-compliant"),
            ("Christopher is here.", false, ""),
            ("The system is GDPR-compliant.", false, ""),
        ];

        let mut tp = 0;
        let mut fp = 0;
        let mut fn_count = 0;
        let mut tn = 0;

        for (text, should_detect, expected_suggestion) in &prohibit_cases {
            let tokens = tokenizer.tokenize(text);
            let analyzed = analyzer.analyze(tokens);
            let result = checker.check(text, &analyzed);

            let has_match = result.matches.iter().any(|m| m.rule_id == "PROHIBIT");

            if *should_detect {
                if has_match {
                    tp += 1;
                    // Verify suggestion
                    let has_correct_suggestion = result.matches.iter()
                        .any(|m| m.suggestions.contains(&expected_suggestion.to_string()));
                    if !has_correct_suggestion && !expected_suggestion.is_empty() {
                        eprintln!("  Warning: expected suggestion '{}' not found", expected_suggestion);
                    }
                } else {
                    fn_count += 1;
                    eprintln!("  FN: prohibit word not detected in \"{}\"", text);
                }
            } else {
                if has_match {
                    fp += 1;
                    eprintln!("  FP: unexpected prohibit match in \"{}\"", text);
                } else {
                    tn += 1;
                }
            }
        }

        prohibit_cat.detected = tp;
        prohibit_cat.total = tp + fn_count;
        println!("  TP: {}, FP: {}, FN: {}, TN: {}", tp, fp, fn_count, tn);
        let prohibit_metrics = QualityMetrics {
            true_positives: tp,
            false_positives: fp,
            false_negatives: fn_count,
            true_negatives: tn,
        };
        println!("  Precision: {:.1}%", prohibit_metrics.precision() * 100.0);
        println!("  Recall:    {:.1}%", prohibit_metrics.recall() * 100.0);
        println!("  F1 Score:  {:.1}%", prohibit_metrics.f1() * 100.0);
        total_metrics.merge(&prohibit_metrics);
    }
    categories.push(prohibit_cat);

    // --- Non-implemented categories (roadmap) ---
    let not_implemented: Vec<CategoryCoverage> = vec![
        // All rules from roadmap now implemented!
    ];

    // --- Category Coverage Summary ---
    println!("\n╔════════════════════════════════════════════╗");
    println!("║         COUVERTURE PAR CATÉGORIE           ║");
    println!("╚════════════════════════════════════════════╝");

    let implemented_count = categories.len();
    let total_categories = implemented_count + not_implemented.len();

    for cat in &categories {
        println!("{}", cat.display());
    }

    println!("────────────────────────────────────────────");
    for cat in &not_implemented {
        println!("{}", cat.display());
    }

    println!("\nCatégories implémentées: {}/{} ({:.0}%)",
        implemented_count,
        total_categories,
        (implemented_count as f64 / total_categories as f64) * 100.0
    );

    // --- Overall Results ---
    println!("\n╔════════════════════════════════════════════╗");
    println!("║            RÉSULTATS GLOBAUX               ║");
    println!("╚════════════════════════════════════════════╝");
    println!("Total TP: {}, FP: {}, FN: {}, TN: {}",
        total_metrics.true_positives,
        total_metrics.false_positives,
        total_metrics.false_negatives,
        total_metrics.true_negatives
    );
    println!("Overall Precision: {:.1}%", total_metrics.precision() * 100.0);
    println!("Overall Recall:    {:.1}%", total_metrics.recall() * 100.0);
    println!("Overall F1 Score:  {:.1}%", total_metrics.f1() * 100.0);

    // --- Compact Summary ---
    let total_test_cases: usize = categories.iter().map(|c| c.total).sum();
    println!("\n╔════════════════════════════════════════════╗");
    println!("║     RÉSUMÉ (grammar-rs v{})             ║", VERSION);
    println!("╚════════════════════════════════════════════╝");
    println!("Règles implémentées: {}", implemented_count);
    println!("Cas de test: {}", total_test_cases);
    println!("Precision: {:.1}% | Recall: {:.1}% | F1: {:.1}%",
        total_metrics.precision() * 100.0,
        total_metrics.recall() * 100.0,
        total_metrics.f1() * 100.0
    );
    println!("════════════════════════════════════════════\n");

    // Quality thresholds
    assert!(
        total_metrics.precision() >= 0.8,
        "Precision too low: {:.1}% (expected >= 80%)",
        total_metrics.precision() * 100.0
    );
    assert!(
        total_metrics.recall() >= 0.7,
        "Recall too low: {:.1}% (expected >= 70%)",
        total_metrics.recall() * 100.0
    );
    assert!(
        total_metrics.f1() >= 0.75,
        "F1 Score too low: {:.1}% (expected >= 75%)",
        total_metrics.f1() * 100.0
    );
}
