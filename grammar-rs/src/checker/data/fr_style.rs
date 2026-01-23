//! Auto-generated style rules for FR from LanguageTool
//! Synced: 2026-01-23T13:30:58.152565536+00:00
//! Total rules: 51 (0 wordiness, 51 redundancy)
//! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update
//!
//! Source: LanguageTool wordiness.txt and redundancies.txt
//! License: LGPL 2.1+

use super::en_style::{StyleCategory, StyleRule};

/// Style rules for FR (wordiness + redundancy)
pub const FR_STYLE_RULES: &[StyleRule] = &[
    StyleRule { phrase: "absolument essentiel", suggestions: &["essentiel"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "absolument nécessaire", suggestions: &["nécessaire"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "actuellement en ce moment", suggestions: &["actuellement", "en ce moment"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "ajouter en plus", suggestions: &["ajouter"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "apprendre de nouveau", suggestions: &["réapprendre"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "avancer en avant", suggestions: &["avancer"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "bref résumé", suggestions: &["résumé"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "car effectivement", suggestions: &["car", "effectivement"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "collaborer ensemble", suggestions: &["collaborer"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "complètement achevé", suggestions: &["achevé"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "coopérer ensemble", suggestions: &["coopérer"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "courte brève", suggestions: &["brève"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "d'abord en premier", suggestions: &["d'abord", "en premier"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "descendre en bas", suggestions: &["descendre"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "deux jumeaux", suggestions: &["jumeaux"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "don gratuit", suggestions: &["don"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "entièrement terminé", suggestions: &["terminé"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "entrer dedans", suggestions: &["entrer"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "exactement pareil", suggestions: &["pareil"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "fait concret", suggestions: &["fait"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "fausse illusion", suggestions: &["illusion"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "hasard imprévu", suggestions: &["hasard"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "monter en haut", suggestions: &["monter"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "nouveau renouveau", suggestions: &["renouveau"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "parfaitement identique", suggestions: &["identique"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "petit nain", suggestions: &["nain"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "planifier à l'avance", suggestions: &["planifier"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "principal protagoniste", suggestions: &["protagoniste"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "prédire à l'avance", suggestions: &["prédire"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "préparer à l'avance", suggestions: &["préparer"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "prévoir à l'avance", suggestions: &["prévoir"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "puis après", suggestions: &["puis", "après"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "puis enfin", suggestions: &["enfin"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "puis ensuite", suggestions: &["puis", "ensuite"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "reculer en arrière", suggestions: &["reculer"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "rentrer dedans", suggestions: &["rentrer"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "répéter encore", suggestions: &["répéter"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "réserver à l'avance", suggestions: &["réserver"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "s'entraider mutuellement", suggestions: &["s'entraider"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "se réunir ensemble", suggestions: &["se réunir"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "seul et unique", suggestions: &["seul", "unique"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "sortir dehors", suggestions: &["sortir"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "sûr et certain", suggestions: &["sûr", "certain"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "totalement achevé", suggestions: &["achevé"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "totalement terminé", suggestions: &["terminé"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "très immense", suggestions: &["immense"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "très infime", suggestions: &["infime"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "très minime", suggestions: &["minime"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "très minuscule", suggestions: &["minuscule"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "très énorme", suggestions: &["énorme"], category: StyleCategory::Redundancy },
    StyleRule { phrase: "vrai fait", suggestions: &["fait"], category: StyleCategory::Redundancy },
];

/// Get all wordiness rules
pub fn get_fr_wordiness_rules() -> impl Iterator<Item = &'static StyleRule> {
	FR_STYLE_RULES.iter().filter(|r| r.category == StyleCategory::Wordiness)
}

/// Get all redundancy rules
pub fn get_fr_redundancy_rules() -> impl Iterator<Item = &'static StyleRule> {
	FR_STYLE_RULES.iter().filter(|r| r.category == StyleCategory::Redundancy)
}

/// Get all style phrases (for Aho-Corasick building)
pub fn get_fr_style_phrases() -> impl Iterator<Item = &'static str> {
	FR_STYLE_RULES.iter().map(|r| r.phrase)
}
