//! Auto-generated POS pattern rules for FR from LanguageTool
//! Source: fr/grammar.xml
//! Synced: 2026-01-24T12:08:49.162365+00:00
//! Total rules: 25
//! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update
//!
//! These rules use POS (Part-of-Speech) tagging to match patterns.
//! They require the PosPatternChecker to be enabled.

use crate::checker::pos_pattern_checker::{PosPatternRule, PosPatternElement};

static PATTERN_0: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("D e p"), negation: false },
    PosPatternElement { text: Some("feu"), pos_pattern: None, negation: false },
];

static PATTERN_1: &[PosPatternElement] = &[
    PosPatternElement { text: Some("la"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("gente"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("J [fe].*|V ppa [fe].*"), negation: false },
];

static PATTERN_2: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("R pers suj.*"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("V avoir.*"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("V.* inf"), negation: false },
];

static PATTERN_3: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START|C coor|M nonfin"), negation: false },
    PosPatternElement { text: Some("temps"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("qu'"), pos_pattern: None, negation: false },
];

static PATTERN_4: &[PosPatternElement] = &[
    PosPatternElement { text: Some("quelque"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("N . s"), negation: false },
];

static PATTERN_5: &[PosPatternElement] = &[
    PosPatternElement { text: Some("parton"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("A.*"), negation: false },
];

static PATTERN_6: &[PosPatternElement] = &[
    PosPatternElement { text: Some("je"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("tes"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("V.*ppa .*"), negation: false },
];

static PATTERN_7: &[PosPatternElement] = &[
    PosPatternElement { text: Some("tous"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("ce"), pos_pattern: Some("R dem e sp"), negation: false },
];

static PATTERN_8: &[PosPatternElement] = &[
    PosPatternElement { text: Some("ny"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("V.*"), negation: false },
];

static PATTERN_9: &[PosPatternElement] = &[
    PosPatternElement { text: Some("manageraccompagnerapprovisionner"), pos_pattern: Some("V.* inf"), negation: false },
    PosPatternElement { text: Some("de"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("proximité"), pos_pattern: None, negation: false },
];

static PATTERN_10: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("R pers suj.*"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("V etre.*"), negation: false },
    PosPatternElement { text: Some("fatigue"), pos_pattern: Some("V.*"), negation: false },
];

static PATTERN_11: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("D m s"), negation: false },
    PosPatternElement { text: Some("bios"), pos_pattern: None, negation: false },
];

static PATTERN_12: &[PosPatternElement] = &[
    PosPatternElement { text: Some("d'"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("entre"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("eux"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("V.*"), negation: false },
];

static PATTERN_13: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("V avoir.*"), negation: false },
    PosPatternElement { text: Some("l'"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("habitude"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("pour"), pos_pattern: None, negation: false },
];

static PATTERN_14: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START"), negation: false },
    PosPatternElement { text: Some("cela"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("est"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("D.*"), negation: false },
];

static PATTERN_15: &[PosPatternElement] = &[
    PosPatternElement { text: Some("on"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("n'"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("V.* (ind|con|sub).*"), negation: false },
    PosPatternElement { text: None, pos_pattern: None, negation: false },
];

static PATTERN_16: &[PosPatternElement] = &[
    PosPatternElement { text: Some("après"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("que"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("D.*"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("V ind (pres|futu).*"), negation: false },
];

static PATTERN_17: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("V etre.*"), negation: false },
    PosPatternElement { text: Some("légions"), pos_pattern: None, negation: false },
];

static PATTERN_18: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("V etre.*"), negation: false },
    PosPatternElement { text: Some("dans"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("le"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("trouble"), pos_pattern: None, negation: false },
];

static PATTERN_19: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START"), negation: false },
    PosPatternElement { text: Some("tout"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("d'"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("abord"), pos_pattern: None, negation: false },
];

static PATTERN_20: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START|M nonfin"), negation: false },
    PosPatternElement { text: Some("bref"), pos_pattern: None, negation: false },
];

static PATTERN_21: &[PosPatternElement] = &[
    PosPatternElement { text: Some("trop"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("J.*"), negation: false },
];

static PATTERN_22: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START|J.*|C sub|M nonfin"), negation: false },
    PosPatternElement { text: Some("d'"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("après"), pos_pattern: None, negation: false },
];

static PATTERN_23: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START"), negation: false },
    PosPatternElement { text: Some("or"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some(","), pos_pattern: None, negation: false },
];

static PATTERN_24: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("V.*"), negation: false },
    PosPatternElement { text: Some("ensemble"), pos_pattern: None, negation: false },
];

/// POS pattern rules for FR (requires POS tagging)
pub static FR_POS_PATTERN_RULES: &[PosPatternRule] = &[
    PosPatternRule {
		id: "FEUS_OU_FEUX",
		pattern: PATTERN_0,
		message: "Une erreur d'orthographe semble être détectée.",
		suggestions: &["feux"],
	},
    PosPatternRule {
		id: "GENTE_GENT",
		pattern: PATTERN_1,
		message: "gent ?",
		suggestions: &["gent"],
	},
    PosPatternRule {
		id: "R_VAVOIR_VINF",
		pattern: PATTERN_2,
		message: "Un participe passé est attendu après l'auxiliaire avoir.",
		suggestions: &[""],
	},
    PosPatternRule {
		id: "TANT_QU",
		pattern: PATTERN_3,
		message: "tant.",
		suggestions: &["tant"],
	},
    PosPatternRule {
		id: "QUELQUE_TRUC",
		pattern: PATTERN_4,
		message: "Ce groupe nominal est pluriel.",
		suggestions: &["quelques"],
	},
    PosPatternRule {
		id: "PARTON_VERBE",
		pattern: PATTERN_5,
		message: "partonsest attendu devant un adverbe.",
		suggestions: &["partons"],
	},
    PosPatternRule {
		id: "JE_TES",
		pattern: PATTERN_6,
		message: "t'ai?",
		suggestions: &["t'ai"],
	},
    PosPatternRule {
		id: "TOUT_CE",
		pattern: PATTERN_7,
		message: "Dans ce cas précis, le terme \"tout\" est invariable.",
		suggestions: &["tout ce"],
	},
    PosPatternRule {
		id: "NY",
		pattern: PATTERN_8,
		message: "Ces deux mots doivent être séparés.",
		suggestions: &["n'y"],
	},
    PosPatternRule {
		id: "GARANTIR_DE",
		pattern: PATTERN_9,
		message: "la?",
		suggestions: &["la"],
	},
    PosPatternRule {
		id: "RPERSSUJ_ETRE_V",
		pattern: PATTERN_10,
		message: "?",
		suggestions: &[""],
	},
    PosPatternRule {
		id: "BIOS",
		pattern: PATTERN_11,
		message: "L’élément informatique BIOS (Basic Input Output System) est un acronyme non lexicalisé et s’écrit tout en majuscules. Ignorez cette règle si vous vouliez parler du « bios » selon Foucault.",
		suggestions: &["BIOS"],
	},
    PosPatternRule {
		id: "DEUX_D_ENTRE_EUX",
		pattern: PATTERN_12,
		message: "Le verbe « \\5 » devrait être à la 3ᵉ personne du pluriel.",
		suggestions: &[""],
	},
    PosPatternRule {
		id: "AVOIR_L_HABITUDE_POUR",
		pattern: PATTERN_13,
		message: "Une autre préposition semble plus appropriée.",
		suggestions: &["de"],
	},
    PosPatternRule {
		id: "CELA_EST_NOM",
		pattern: PATTERN_14,
		message: "Cette structure n'est généralement pas utilisée, bien qu'elle soit grammaticalement correcte.",
		suggestions: &["c'est"],
	},
    PosPatternRule {
		id: "ON_N_A",
		pattern: PATTERN_15,
		message: "La liaison s'entend à l'oral, mais ne s'écrit pas.",
		suggestions: &["on"],
	},
    PosPatternRule {
		id: "APRES_DES_QUE",
		pattern: PATTERN_16,
		message: "Une autre préposition semble plus appropriée.",
		suggestions: &["dès que"],
	},
    PosPatternRule {
		id: "ACCORD_ETRE_LEGION",
		pattern: PATTERN_17,
		message: "Dans cette expression, le terme \"légion\" est toujours singulier après le verbe \"être\".",
		suggestions: &[""],
	},
    PosPatternRule {
		id: "ETRE_DANS_LE_TROUBLE",
		pattern: PATTERN_18,
		message: "« Être dans le trouble » peut être considéré comme un anglicisme (to be in trouble).",
		suggestions: &["avoirdes ennuis"],
	},
    PosPatternRule {
		id: "REP_TOUT_D_ABORD_TEST",
		pattern: PATTERN_19,
		message: "Dans un contexte formel, d'autres structures peuvent être utilisées pour enrichir votre style.",
		suggestions: &["en premier lieu"],
	},
    PosPatternRule {
		id: "REP_BREF",
		pattern: PATTERN_20,
		message: "Dans un contexte formel, d'autres structures peuvent être utilisées pour enrichir votre style.",
		suggestions: &["sinon"],
	},
    PosPatternRule {
		id: "REP_TROP",
		pattern: PATTERN_21,
		message: "Dans un contexte formel des synonymes peuvent enrichir votre style.",
		suggestions: &["réellement"],
	},
    PosPatternRule {
		id: "REP_D_APRES",
		pattern: PATTERN_22,
		message: "Dans un contexte formel, d'autres structures peuvent être utilisées pour enrichir votre style.",
		suggestions: &["selon"],
	},
    PosPatternRule {
		id: "REP_OR",
		pattern: PATTERN_23,
		message: "Dans un contexte formel des synonymes peuvent enrichir votre style.",
		suggestions: &["cependant"],
	},
    PosPatternRule {
		id: "REP_ENSEMBLE",
		pattern: PATTERN_24,
		message: "Dans un contexte formel, d'autres structures peuvent être utilisées pour enrichir votre style.",
		suggestions: &["conjointement"],
	},
];

/// Create a PosPatternChecker with FR rules
pub fn create_fr_pos_pattern_checker() -> crate::checker::pos_pattern_checker::PosPatternChecker {
	crate::checker::pos_pattern_checker::PosPatternChecker::with_rules(FR_POS_PATTERN_RULES)
}
