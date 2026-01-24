//! Auto-generated disambiguation POS rules for EN from LanguageTool
//! Synced: 2026-01-24T12:08:47.729480+00:00
//! Total: 24 single-token rules
//! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update
//!
//! Source: LanguageTool disambiguation.xml (action="replace"/"add")
//! License: LGPL 2.1+
//!
//! These rules can be used to enhance POS tagging.

/// A single-token POS disambiguation rule
#[derive(Debug, Clone)]
pub struct DisambigPosEntry {
    /// Literal word to match (case-insensitive), None if regex
    pub word: Option<&'static str>,
    /// Regex pattern to match, None if literal word
    pub regexp: Option<&'static str>,
    /// Lemma (base form) to assign
    pub lemma: &'static str,
    /// POS tag to assign
    pub pos_tag: &'static str,
}

/// Single-token POS disambiguation rules for EN
pub const EN_DISAMBIG_POS: &[DisambigPosEntry] = &[
    DisambigPosEntry { word: None, regexp: Some(r"([a-z]+-)?first|([a-z]+-)?second|([a-z]+-)?third|([a-z]+-)?fourth|([a-z]+-)?fifth|([a-z]+-)?sixth|([a-z]+-)?seventh|([a-z]+-)?eighth|([a-z]+-)?ninth|tenth|eleventh|twelfth|thirteenth|fourteenth|fifteenth|sixteenth|seventeenth|eighteenth|nineteenth|twentieth|thirtieth|f(if|or)tieth|sixtieth|seventieth|eightieth|ninetieth|hundredth|thousandth|millionth|billionth|trillionth"), lemma: "", pos_tag: "ORD" },
    DisambigPosEntry { word: None, regexp: Some(r"(second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth)-(largest|most|biggest|worst|best|lowest|highest|latest|longest|wealthiest|busiest|tallest|smallest|weakest|strongest|oldest|newest|greatest|poorest|deadliest|richest|least)"), lemma: "", pos_tag: "JJS" },
    DisambigPosEntry { word: None, regexp: Some(r"[Hh]ave|HAVE"), lemma: "have", pos_tag: "VB" },
    DisambigPosEntry { word: None, regexp: Some(r"[\\.,;:…!\\?]"), lemma: "", pos_tag: "PCT" },
    DisambigPosEntry { word: None, regexp: Some(r"\\d+"), lemma: "", pos_tag: "NNP" },
    DisambigPosEntry { word: None, regexp: Some(r"cc|rsvp|pm|dm"), lemma: "cc", pos_tag: "VBN" },
    DisambigPosEntry { word: None, regexp: Some(r"cc|rsvp|pm|dm|dj"), lemma: "cc", pos_tag: "VBD" },
    DisambigPosEntry { word: Some("'s"), regexp: None, lemma: "have", pos_tag: "VBZ" },
    DisambigPosEntry { word: Some("'ve"), regexp: None, lemma: "have", pos_tag: "VBP" },
    DisambigPosEntry { word: Some(","), regexp: None, lemma: "", pos_tag: "," },
    DisambigPosEntry { word: Some("about"), regexp: None, lemma: "", pos_tag: "NN" },
    DisambigPosEntry { word: Some("avant-garde"), regexp: None, lemma: "", pos_tag: "JJ" },
    DisambigPosEntry { word: Some("avant-garde"), regexp: None, lemma: "", pos_tag: "NN:U" },
    DisambigPosEntry { word: Some("belles-lettres"), regexp: None, lemma: "", pos_tag: "NNS" },
    DisambigPosEntry { word: Some("billet-doux"), regexp: None, lemma: "", pos_tag: "NN" },
    DisambigPosEntry { word: Some("billets-doux"), regexp: None, lemma: "", pos_tag: "NNS" },
    DisambigPosEntry { word: Some("blessed"), regexp: None, lemma: "", pos_tag: "NNS" },
    DisambigPosEntry { word: Some("catch"), regexp: None, lemma: "", pos_tag: "NN:UN" },
    DisambigPosEntry { word: Some("mos"), regexp: None, lemma: "", pos_tag: "NNP" },
    DisambigPosEntry { word: Some("other"), regexp: None, lemma: "", pos_tag: "RB" },
    DisambigPosEntry { word: Some("sports"), regexp: None, lemma: "sports", pos_tag: "JJ" },
    DisambigPosEntry { word: Some("super"), regexp: None, lemma: "", pos_tag: "NNP" },
    DisambigPosEntry { word: Some("taxes"), regexp: None, lemma: "tax", pos_tag: "NNS" },
    DisambigPosEntry { word: Some("times"), regexp: None, lemma: "", pos_tag: "NNP" },
];
