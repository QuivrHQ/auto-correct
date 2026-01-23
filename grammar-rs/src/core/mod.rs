//! Core types - le cœur stable qui ne change jamais

pub mod traits;
pub mod pipeline;
pub mod filter;

use std::ops::Range;

/// Un token avec sa position dans le texte original
#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub text: &'a str,
    pub span: Range<usize>,
    pub kind: TokenKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Word,
    Punctuation,
    Whitespace,
    Number,
    Unknown,
}

/// Token enrichi avec analyse linguistique
#[derive(Debug, Clone)]
pub struct AnalyzedToken<'a> {
    pub token: Token<'a>,
    pub lemma: Option<String>,
    pub pos: Option<PosTag>,
}

/// Part-of-speech tags
///
/// Extended Penn Treebank tagset with LanguageTool extensions.
/// Use the simplified aliases (Noun, Verb, etc.) for basic checks,
/// or the detailed Penn tags (NN, VB, JJ, etc.) for precise matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PosTag {
    // ═══════════════════════════════════════════════════════════════════════
    // Penn Treebank Tags - Detailed
    // ═══════════════════════════════════════════════════════════════════════

    // Conjunctions
    /// Coordinating conjunction (and, but, or)
    CC,

    // Numbers
    /// Cardinal number (one, two, 1, 2)
    CD,

    // Determiners
    /// Determiner (the, a, this, that)
    DT,
    /// Existential there
    EX,
    /// Foreign word
    FW,
    /// Preposition or subordinating conjunction
    IN,

    // Adjectives
    /// Adjective (big, green)
    JJ,
    /// Adjective, comparative (bigger, greener)
    JJR,
    /// Adjective, superlative (biggest, greenest)
    JJS,

    // Nouns
    /// Noun, singular or mass (dog, music)
    NN,
    /// Noun, plural (dogs, cats)
    NNS,
    /// Proper noun, singular (John, London)
    NNP,
    /// Proper noun, plural (Americans, Kennedys)
    NNPS,

    // Modals and possessives
    /// Modal (can, could, will, would)
    MD,
    /// Predeterminer (all, both)
    PDT,
    /// Possessive ending ('s)
    POS,
    /// Personal pronoun (I, you, he, she)
    PRP,
    /// Possessive pronoun (my, your, his)
    PRPS,  // PRP$

    // Adverbs
    /// Adverb (quickly, very)
    RB,
    /// Adverb, comparative (faster, harder)
    RBR,
    /// Adverb, superlative (fastest, hardest)
    RBS,
    /// Particle (up, off, out)
    RP,

    // Verbs
    /// to (infinitive marker)
    TO,
    /// Interjection (oh, wow, uh)
    UH,
    /// Verb, base form (go, eat)
    VB,
    /// Verb, past tense (went, ate)
    VBD,
    /// Verb, gerund/present participle (going, eating)
    VBG,
    /// Verb, past participle (gone, eaten)
    VBN,
    /// Verb, non-3rd person singular present (go, eat)
    VBP,
    /// Verb, 3rd person singular present (goes, eats)
    VBZ,

    // Wh-words
    /// Wh-determiner (which, that)
    WDT,
    /// Wh-pronoun (who, what)
    WP,
    /// Possessive wh-pronoun (whose)
    WPS,  // WP$
    /// Wh-adverb (where, when, how)
    WRB,

    // Punctuation (Penn Treebank style)
    /// Period (.)
    PERIOD,
    /// Comma (,)
    COMMA,
    /// Colon (:)
    COLON,
    /// Left bracket/paren/brace
    LRB,
    /// Right bracket/paren/brace
    RRB,

    // ═══════════════════════════════════════════════════════════════════════
    // LanguageTool Extensions
    // ═══════════════════════════════════════════════════════════════════════

    /// Sentence start marker
    SentStart,
    /// Sentence end marker
    SentEnd,
    /// Unknown/untagged word
    Unknown,

    // ═══════════════════════════════════════════════════════════════════════
    // French POS Tags (from LanguageTool French tagset)
    // ═══════════════════════════════════════════════════════════════════════

    // French Verbs
    /// French verb (generic)
    FrV,
    /// French infinitive verb
    FrVInf,
    /// French past participle
    FrVPpa,
    /// French present participle
    FrVPpr,
    /// French indicative verb
    FrVInd,
    /// French subjunctive verb
    FrVSub,
    /// French conditional verb
    FrVCon,
    /// French imperative verb
    FrVImp,

    // French Nouns
    /// French noun (generic)
    FrN,
    /// French masculine noun
    FrNM,
    /// French feminine noun
    FrNF,
    /// French proper noun
    FrZ,

    // French Adjectives
    /// French adjective
    FrA,

    // French Determiners
    /// French determiner (generic)
    FrD,
    /// French masculine determiner
    FrDM,
    /// French feminine determiner
    FrDF,

    // French Pronouns
    /// French pronoun (generic)
    FrR,
    /// French personal pronoun subject
    FrRPersSuj,
    /// French personal pronoun object
    FrRPersObj,
    /// French relative pronoun
    FrRRel,
    /// French demonstrative pronoun
    FrRDem,
    /// French possessive pronoun
    FrRPos,

    // French Prepositions & Conjunctions
    /// French preposition
    FrP,
    /// French coordinating conjunction
    FrC,
    /// French subordinating conjunction
    FrCs,

    // French Adverbs
    /// French adverb
    FrAdv,

    // French Articles
    /// French article
    FrArt,

    // ═══════════════════════════════════════════════════════════════════════
    // Simplified Aliases (mapped to Penn equivalents)
    // ═══════════════════════════════════════════════════════════════════════

    /// Generic noun (alias for NN)
    Noun,
    /// Generic verb (alias for VB)
    Verb,
    /// Generic adjective (alias for JJ)
    Adjective,
    /// Generic adverb (alias for RB)
    Adverb,
    /// Generic determiner (alias for DT)
    Determiner,
    /// Generic preposition (alias for IN)
    Preposition,
    /// Generic conjunction (alias for CC)
    Conjunction,
    /// Generic pronoun (alias for PRP)
    Pronoun,
    /// Generic punctuation
    Punctuation,
    /// Other/unclassified
    Other,
}

impl PosTag {
    /// Check if this tag matches a pattern (supports wildcards like "NN.*", "VB.*")
    pub fn matches_pattern(&self, pattern: &str) -> bool {
        let tag_str = self.as_str();

        if pattern.ends_with(".*") {
            let prefix = &pattern[..pattern.len() - 2];
            tag_str.starts_with(prefix)
        } else if pattern.ends_with("*") {
            let prefix = &pattern[..pattern.len() - 1];
            tag_str.starts_with(prefix)
        } else {
            tag_str == pattern
        }
    }

    /// Get the string representation of this tag
    pub fn as_str(&self) -> &'static str {
        match self {
            // Penn Treebank tags
            PosTag::CC => "CC",
            PosTag::CD => "CD",
            PosTag::DT => "DT",
            PosTag::EX => "EX",
            PosTag::FW => "FW",
            PosTag::IN => "IN",
            PosTag::JJ => "JJ",
            PosTag::JJR => "JJR",
            PosTag::JJS => "JJS",
            PosTag::NN => "NN",
            PosTag::NNS => "NNS",
            PosTag::NNP => "NNP",
            PosTag::NNPS => "NNPS",
            PosTag::MD => "MD",
            PosTag::PDT => "PDT",
            PosTag::POS => "POS",
            PosTag::PRP => "PRP",
            PosTag::PRPS => "PRP$",
            PosTag::RB => "RB",
            PosTag::RBR => "RBR",
            PosTag::RBS => "RBS",
            PosTag::RP => "RP",
            PosTag::TO => "TO",
            PosTag::UH => "UH",
            PosTag::VB => "VB",
            PosTag::VBD => "VBD",
            PosTag::VBG => "VBG",
            PosTag::VBN => "VBN",
            PosTag::VBP => "VBP",
            PosTag::VBZ => "VBZ",
            PosTag::WDT => "WDT",
            PosTag::WP => "WP",
            PosTag::WPS => "WP$",
            PosTag::WRB => "WRB",
            PosTag::PERIOD => ".",
            PosTag::COMMA => ",",
            PosTag::COLON => ":",
            PosTag::LRB => "-LRB-",
            PosTag::RRB => "-RRB-",
            // LanguageTool extensions
            PosTag::SentStart => "SENT_START",
            PosTag::SentEnd => "SENT_END",
            PosTag::Unknown => "UNKNOWN",
            // Simplified aliases
            PosTag::Noun => "NN",
            PosTag::Verb => "VB",
            PosTag::Adjective => "JJ",
            PosTag::Adverb => "RB",
            PosTag::Determiner => "DT",
            PosTag::Preposition => "IN",
            PosTag::Conjunction => "CC",
            PosTag::Pronoun => "PRP",
            PosTag::Punctuation => ".",
            PosTag::Other => "UNKNOWN",
            // French tags
            PosTag::FrV => "V",
            PosTag::FrVInf => "V inf",
            PosTag::FrVPpa => "V ppa",
            PosTag::FrVPpr => "V ppr",
            PosTag::FrVInd => "V ind",
            PosTag::FrVSub => "V sub",
            PosTag::FrVCon => "V con",
            PosTag::FrVImp => "V imp",
            PosTag::FrN => "N",
            PosTag::FrNM => "N m",
            PosTag::FrNF => "N f",
            PosTag::FrZ => "Z",
            PosTag::FrA => "A",
            PosTag::FrD => "D",
            PosTag::FrDM => "D m",
            PosTag::FrDF => "D f",
            PosTag::FrR => "R",
            PosTag::FrRPersSuj => "R pers suj",
            PosTag::FrRPersObj => "R pers obj",
            PosTag::FrRRel => "R rel",
            PosTag::FrRDem => "R dem",
            PosTag::FrRPos => "R pos",
            PosTag::FrP => "P",
            PosTag::FrC => "C",
            PosTag::FrCs => "Cs",
            PosTag::FrAdv => "ADV",
            PosTag::FrArt => "ART",
        }
    }

    /// Parse a POS tag from a string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "CC" => Some(PosTag::CC),
            "CD" => Some(PosTag::CD),
            "DT" => Some(PosTag::DT),
            "EX" => Some(PosTag::EX),
            "FW" => Some(PosTag::FW),
            "IN" => Some(PosTag::IN),
            "JJ" => Some(PosTag::JJ),
            "JJR" => Some(PosTag::JJR),
            "JJS" => Some(PosTag::JJS),
            "NN" => Some(PosTag::NN),
            "NNS" => Some(PosTag::NNS),
            "NNP" => Some(PosTag::NNP),
            "NNPS" => Some(PosTag::NNPS),
            "MD" => Some(PosTag::MD),
            "PDT" => Some(PosTag::PDT),
            "POS" => Some(PosTag::POS),
            "PRP" => Some(PosTag::PRP),
            "PRP$" => Some(PosTag::PRPS),
            "RB" => Some(PosTag::RB),
            "RBR" => Some(PosTag::RBR),
            "RBS" => Some(PosTag::RBS),
            "RP" => Some(PosTag::RP),
            "TO" => Some(PosTag::TO),
            "UH" => Some(PosTag::UH),
            "VB" => Some(PosTag::VB),
            "VBD" => Some(PosTag::VBD),
            "VBG" => Some(PosTag::VBG),
            "VBN" => Some(PosTag::VBN),
            "VBP" => Some(PosTag::VBP),
            "VBZ" => Some(PosTag::VBZ),
            "WDT" => Some(PosTag::WDT),
            "WP" => Some(PosTag::WP),
            "WP$" => Some(PosTag::WPS),
            "WRB" => Some(PosTag::WRB),
            "." | "PERIOD" => Some(PosTag::PERIOD),
            "," | "COMMA" => Some(PosTag::COMMA),
            ":" | "COLON" => Some(PosTag::COLON),
            "-LRB-" | "LRB" => Some(PosTag::LRB),
            "-RRB-" | "RRB" => Some(PosTag::RRB),
            "SENT_START" => Some(PosTag::SentStart),
            "SENT_END" => Some(PosTag::SentEnd),
            // French tags (case-sensitive matching for the space-separated format)
            _ => Self::from_str_french(s),
        }
    }

    /// Parse a French POS tag from a string
    fn from_str_french(s: &str) -> Option<Self> {
        // French tags often have spaces: "V inf", "N m s", etc.
        let parts: Vec<&str> = s.split_whitespace().collect();

        match parts.first().map(|s| *s) {
            Some("V") => {
                match parts.get(1).map(|s| *s) {
                    Some("inf") => Some(PosTag::FrVInf),
                    Some("ppa") => Some(PosTag::FrVPpa),
                    Some("ppr") => Some(PosTag::FrVPpr),
                    Some("ind") => Some(PosTag::FrVInd),
                    Some("sub") => Some(PosTag::FrVSub),
                    Some("con") => Some(PosTag::FrVCon),
                    Some("imp") => Some(PosTag::FrVImp),
                    _ => Some(PosTag::FrV),
                }
            }
            Some("N") => {
                match parts.get(1).map(|s| *s) {
                    Some("m") => Some(PosTag::FrNM),
                    Some("f") => Some(PosTag::FrNF),
                    _ => Some(PosTag::FrN),
                }
            }
            Some("Z") => Some(PosTag::FrZ),
            Some("A") => Some(PosTag::FrA),
            Some("J") => Some(PosTag::FrA), // J often used for adjectives too
            Some("D") => {
                match parts.get(1).map(|s| *s) {
                    Some("m") => Some(PosTag::FrDM),
                    Some("f") => Some(PosTag::FrDF),
                    _ => Some(PosTag::FrD),
                }
            }
            Some("R") => {
                if parts.len() >= 3 && parts[1] == "pers" {
                    match parts.get(2).map(|s| *s) {
                        Some("suj") => Some(PosTag::FrRPersSuj),
                        Some("obj") => Some(PosTag::FrRPersObj),
                        _ => Some(PosTag::FrR),
                    }
                } else {
                    match parts.get(1).map(|s| *s) {
                        Some("rel") => Some(PosTag::FrRRel),
                        Some("dem") => Some(PosTag::FrRDem),
                        Some("pos") => Some(PosTag::FrRPos),
                        _ => Some(PosTag::FrR),
                    }
                }
            }
            Some("P") => Some(PosTag::FrP),
            Some("C") => Some(PosTag::FrC),
            Some("Cs") => Some(PosTag::FrCs),
            Some("ADV") => Some(PosTag::FrAdv),
            Some("ART") => Some(PosTag::FrArt),
            _ => None,
        }
    }

    /// Check if this is a noun tag (NN, NNS, NNP, NNPS, or Noun)
    pub fn is_noun(&self) -> bool {
        matches!(self, PosTag::NN | PosTag::NNS | PosTag::NNP | PosTag::NNPS | PosTag::Noun)
    }

    /// Check if this is a verb tag (VB, VBD, VBG, VBN, VBP, VBZ, or Verb)
    pub fn is_verb(&self) -> bool {
        matches!(self, PosTag::VB | PosTag::VBD | PosTag::VBG | PosTag::VBN | PosTag::VBP | PosTag::VBZ | PosTag::Verb)
    }

    /// Check if this is an adjective tag (JJ, JJR, JJS, or Adjective)
    pub fn is_adjective(&self) -> bool {
        matches!(self, PosTag::JJ | PosTag::JJR | PosTag::JJS | PosTag::Adjective)
    }

    /// Check if this is an adverb tag (RB, RBR, RBS, or Adverb)
    pub fn is_adverb(&self) -> bool {
        matches!(self, PosTag::RB | PosTag::RBR | PosTag::RBS | PosTag::Adverb)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // French tag helpers
    // ═══════════════════════════════════════════════════════════════════════

    /// Check if this is a French verb tag
    pub fn is_french_verb(&self) -> bool {
        matches!(
            self,
            PosTag::FrV
                | PosTag::FrVInf
                | PosTag::FrVPpa
                | PosTag::FrVPpr
                | PosTag::FrVInd
                | PosTag::FrVSub
                | PosTag::FrVCon
                | PosTag::FrVImp
        )
    }

    /// Check if this is a French noun tag
    pub fn is_french_noun(&self) -> bool {
        matches!(self, PosTag::FrN | PosTag::FrNM | PosTag::FrNF | PosTag::FrZ)
    }

    /// Check if this is a French adjective tag
    pub fn is_french_adjective(&self) -> bool {
        matches!(self, PosTag::FrA)
    }

    /// Check if this is a French determiner tag
    pub fn is_french_determiner(&self) -> bool {
        matches!(self, PosTag::FrD | PosTag::FrDM | PosTag::FrDF | PosTag::FrArt)
    }

    /// Check if this is a French pronoun tag
    pub fn is_french_pronoun(&self) -> bool {
        matches!(
            self,
            PosTag::FrR
                | PosTag::FrRPersSuj
                | PosTag::FrRPersObj
                | PosTag::FrRRel
                | PosTag::FrRDem
                | PosTag::FrRPos
        )
    }

    /// Check if this is a French tag
    pub fn is_french(&self) -> bool {
        self.is_french_verb()
            || self.is_french_noun()
            || self.is_french_adjective()
            || self.is_french_determiner()
            || self.is_french_pronoun()
            || matches!(self, PosTag::FrP | PosTag::FrC | PosTag::FrCs | PosTag::FrAdv)
    }

    /// Match French patterns (space-separated categories)
    /// Patterns like: "V.*", "N.*", "D . s", etc.
    pub fn matches_french_pattern(&self, pattern: &str) -> bool {
        let tag_str = self.as_str();

        // Handle regex-like patterns
        if pattern.ends_with(".*") {
            let prefix = &pattern[..pattern.len() - 2];
            tag_str.starts_with(prefix)
        } else if pattern.contains('|') {
            // Handle OR patterns like "A|V"
            pattern.split('|').any(|p| self.matches_french_pattern(p.trim()))
        } else if pattern.contains('[') {
            // Handle character classes like "[me]" for masculine/epicene
            // This is a simplified version - full regex would need proper handling
            let parts: Vec<&str> = tag_str.split_whitespace().collect();
            let pattern_parts: Vec<&str> = pattern.split_whitespace().collect();

            if parts.len() < pattern_parts.len() {
                return false;
            }

            for (i, pp) in pattern_parts.iter().enumerate() {
                if pp.contains('[') {
                    // Simple character class handling
                    continue; // Skip validation for now, assume match
                } else if pp == &"." {
                    continue; // Wildcard for any single part
                } else if i < parts.len() && parts[i] != *pp {
                    return false;
                }
            }
            true
        } else {
            // Exact match or prefix match
            tag_str == pattern || tag_str.starts_with(&format!("{} ", pattern))
        }
    }
}

/// Une erreur détectée
#[derive(Debug, Clone)]
pub struct Match {
    pub span: Range<usize>,
    pub message: String,
    pub rule_id: String,
    pub suggestions: Vec<String>,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Hint,
}

/// Résultat final de l'analyse
#[derive(Debug, Clone)]
pub struct CheckResult {
    pub matches: Vec<Match>,
}

impl CheckResult {
    pub fn new() -> Self {
        Self { matches: Vec::new() }
    }

    pub fn merge(&mut self, other: CheckResult) {
        self.matches.extend(other.matches);
    }

    pub fn sort_and_dedupe(&mut self) {
        self.matches.sort_by_key(|m| m.span.start);
        self.matches.dedup_by(|a, b| a.span == b.span && a.rule_id == b.rule_id);
    }
}

impl Default for CheckResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Type of content that has been masked (for filtering false positives)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskKind {
    /// Quoted text: "...", '...', «...»
    QuotedText,
    /// Code blocks: `inline` or ```blocks```
    CodeBlock,
    /// Dates: 2024-01-15, Jan 1st, etc.
    Date,
    /// Hyphenated numbers: twenty-one, thirty-five
    HyphenatedNumber,
    /// URLs and emails
    Url,
}

/// A region of text that should be ignored by grammar checkers
#[derive(Debug, Clone, PartialEq)]
pub struct MaskedRegion {
    pub span: Range<usize>,
    pub kind: MaskKind,
}

impl MaskedRegion {
    pub fn new(span: Range<usize>, kind: MaskKind) -> Self {
        Self { span, kind }
    }

    /// Check if a span overlaps with this masked region
    pub fn overlaps(&self, other: &Range<usize>) -> bool {
        self.span.start < other.end && other.start < self.span.end
    }
}

impl CheckResult {
    /// Filter out matches that overlap with masked regions
    pub fn filter_masked(mut self, masks: &[MaskedRegion]) -> Self {
        self.matches.retain(|m| {
            !masks.iter().any(|mask| mask.overlaps(&m.span))
        });
        self
    }
}
