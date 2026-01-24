//! Auto-generated disambiguation POS rules for FR from LanguageTool
//! Synced: 2026-01-24T12:08:49.459176+00:00
//! Total: 28 single-token rules
//! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update
//!
//! Source: LanguageTool disambiguation.xml (action="replace"/"add")
//! License: LGPL 2.1+
//!
//! These rules can be used to enhance POS tagging.

use super::en_disambig_pos::DisambigPosEntry;

/// Single-token POS disambiguation rules for FR
pub const FR_DISAMBIG_POS: &[DisambigPosEntry] = &[
    DisambigPosEntry { word: None, regexp: Some(r"(?-i)Black"), lemma: "", pos_tag: "Z e sp" },
    DisambigPosEntry { word: None, regexp: Some(r"(?-i)Central"), lemma: "", pos_tag: "Z e sp" },
    DisambigPosEntry { word: None, regexp: Some(r"(?-i)US"), lemma: "", pos_tag: "Z e sp" },
    DisambigPosEntry { word: None, regexp: Some(r"(?-i)VIE"), lemma: "", pos_tag: "Z m sp" },
    DisambigPosEntry { word: None, regexp: Some(r"(?-i)[A-Z].*"), lemma: "", pos_tag: "Z f sp" },
    DisambigPosEntry { word: None, regexp: Some(r"(?-i)[A-Z].*"), lemma: "", pos_tag: "Z m sp" },
    DisambigPosEntry { word: None, regexp: Some(r"(https?|ftp)://.*|www.*"), lemma: "IS_URL", pos_tag: "IS_URL" },
    DisambigPosEntry { word: None, regexp: Some(r"EC|CE|AEC"), lemma: "", pos_tag: "A" },
    DisambigPosEntry { word: None, regexp: Some(r"envie|avantage"), lemma: "", pos_tag: "N.*" },
    DisambigPosEntry { word: None, regexp: Some(r"projets?|qualité"), lemma: "", pos_tag: "J e sp" },
    DisambigPosEntry { word: None, regexp: Some(r"radio|radar|météo"), lemma: "", pos_tag: "J e sp" },
    DisambigPosEntry { word: Some("203"), regexp: None, lemma: "", pos_tag: "D f s" },
    DisambigPosEntry { word: Some("business"), regexp: None, lemma: "", pos_tag: "N f s" },
    DisambigPosEntry { word: Some("cervical"), regexp: None, lemma: "", pos_tag: "J e sp" },
    DisambigPosEntry { word: Some("data"), regexp: None, lemma: "", pos_tag: "N f s" },
    DisambigPosEntry { word: Some("data"), regexp: None, lemma: "", pos_tag: "N m s" },
    DisambigPosEntry { word: Some("envie"), regexp: None, lemma: "", pos_tag: "N f s" },
    DisambigPosEntry { word: Some("frites"), regexp: None, lemma: "", pos_tag: "J e sp" },
    DisambigPosEntry { word: Some("google"), regexp: None, lemma: "", pos_tag: "Z e sp" },
    DisambigPosEntry { word: Some("grand"), regexp: None, lemma: "", pos_tag: "J e s" },
    DisambigPosEntry { word: Some("mans"), regexp: None, lemma: "", pos_tag: "Z e sp" },
    DisambigPosEntry { word: Some("putain"), regexp: None, lemma: "", pos_tag: "I" },
    DisambigPosEntry { word: Some("quelques"), regexp: None, lemma: "", pos_tag: "J e p" },
    DisambigPosEntry { word: Some("soul"), regexp: None, lemma: "", pos_tag: "N f s" },
    DisambigPosEntry { word: Some("suitefaire"), regexp: None, lemma: "", pos_tag: "P" },
    DisambigPosEntry { word: Some("surprise"), regexp: None, lemma: "", pos_tag: "J e sp" },
    DisambigPosEntry { word: Some("tissu"), regexp: None, lemma: "", pos_tag: "J e sp" },
    DisambigPosEntry { word: Some("type"), regexp: None, lemma: "", pos_tag: "J e sp" },
];
