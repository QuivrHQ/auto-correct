//! Auto-generated POS pattern rules for EN from LanguageTool
//! Source: en/grammar.xml
//! Synced: 2026-01-23T13:17:50.206149249+00:00
//! Total rules: 94
//! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update
//!
//! These rules use POS (Part-of-Speech) tagging to match patterns.
//! They require the PosPatternChecker to be enabled.

use crate::checker::pos_pattern_checker::{PosPatternRule, PosPatternElement};

static PATTERN_0: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: None, negation: false },
    PosPatternElement { text: Some("pleaser"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBP?"), negation: false },
];

static PATTERN_1: &[PosPatternElement] = &[
    PosPatternElement { text: Some("true"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("PRP$"), negation: false },
    PosPatternElement { text: Some("words"), pos_pattern: None, negation: false },
];

static PATTERN_2: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START"), negation: false },
    PosPatternElement { text: Some("halo"), pos_pattern: None, negation: false },
];

static PATTERN_3: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("MD"), negation: false },
    PosPatternElement { text: Some("by"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("morning"), pos_pattern: Some("JJ.?|VBN"), negation: false },
];

static PATTERN_4: &[PosPatternElement] = &[
    PosPatternElement { text: Some("theses"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("NN.*"), negation: false },
];

static PATTERN_5: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("JJS"), negation: false },
    PosPatternElement { text: Some("was"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: None, negation: false },
];

static PATTERN_6: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START"), negation: false },
    PosPatternElement { text: Some("another"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("words"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some(","), pos_pattern: None, negation: false },
];

static PATTERN_7: &[PosPatternElement] = &[
    PosPatternElement { text: Some("all"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("most"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("PDT"), negation: false },
];

static PATTERN_8: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("CD"), negation: false },
    PosPatternElement { text: Some("ore"), pos_pattern: None, negation: false },
];

static PATTERN_9: &[PosPatternElement] = &[
    PosPatternElement { text: Some("that"), pos_pattern: Some("DT"), negation: false },
    PosPatternElement { text: Some("responds"), pos_pattern: None, negation: false },
];

static PATTERN_10: &[PosPatternElement] = &[
    PosPatternElement { text: Some("whose"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("many"), pos_pattern: Some("DT"), negation: false },
];

static PATTERN_11: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START"), negation: false },
    PosPatternElement { text: Some("whose"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("IN"), negation: false },
];

static PATTERN_12: &[PosPatternElement] = &[
    PosPatternElement { text: Some("had"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("of"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBN"), negation: false },
];

static PATTERN_13: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("DT"), negation: false },
    PosPatternElement { text: Some("bail"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("out"), pos_pattern: None, negation: false },
];

static PATTERN_14: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("IN"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("PRP|NN(:UN?)?"), negation: false },
    PosPatternElement { text: Some("then"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("IN"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("PRP|NN(:UN?)?|SENT_END"), negation: false },
];

static PATTERN_15: &[PosPatternElement] = &[
    PosPatternElement { text: Some("as"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("JJ."), negation: false },
    PosPatternElement { text: Some("as"), pos_pattern: None, negation: false },
];

static PATTERN_16: &[PosPatternElement] = &[
    PosPatternElement { text: Some("you"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("'re"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("JJS"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("NN|NN:UN?"), negation: false },
];

static PATTERN_17: &[PosPatternElement] = &[
    PosPatternElement { text: Some("they"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("'re"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("JJS"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("NN|NN:UN?"), negation: false },
];

static PATTERN_18: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("WRB"), negation: false },
    PosPatternElement { text: Some("there"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBG"), negation: false },
];

static PATTERN_19: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("VBZ"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("IN|DT"), negation: false },
    PosPatternElement { text: Some("they"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("'re"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("NNP|NN|NN:UN?|NNS"), negation: false },
];

static PATTERN_20: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("IN"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBZ"), negation: false },
    PosPatternElement { text: Some("they"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("'re"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("NNP|NN|NN:UN?|NNS"), negation: false },
];

static PATTERN_21: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("DT"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBZ"), negation: false },
    PosPatternElement { text: Some("they"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("'re"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("NN[PS]?|NN:UN?"), negation: false },
];

static PATTERN_22: &[PosPatternElement] = &[
    PosPatternElement { text: Some("withe"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("DT|PRP\\$"), negation: false },
];

static PATTERN_23: &[PosPatternElement] = &[
    PosPatternElement { text: Some("too"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("CD"), negation: false },
];

static PATTERN_24: &[PosPatternElement] = &[
    PosPatternElement { text: Some("by"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("expire"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("SENT_END"), negation: false },
];

static PATTERN_25: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("IN"), negation: false },
    PosPatternElement { text: Some("other"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("date"), pos_pattern: None, negation: false },
];

static PATTERN_26: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("CD"), negation: false },
    PosPatternElement { text: Some("of"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("the"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("kind"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: None, negation: false },
];

static PATTERN_27: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("VBP|VBZ"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBG"), negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBG"), negation: false },
    PosPatternElement { text: None, pos_pattern: None, negation: false },
];

static PATTERN_28: &[PosPatternElement] = &[
    PosPatternElement { text: Some("when"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("MD"), negation: false },
    PosPatternElement { text: Some("be"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBN"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("IN"), negation: false },
];

static PATTERN_29: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("VBD"), negation: false },
    PosPatternElement { text: None, pos_pattern: None, negation: false },
    PosPatternElement { text: Some("an"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBD"), negation: false },
];

static PATTERN_30: &[PosPatternElement] = &[
    PosPatternElement { text: Some("of"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("all"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("surmise"), pos_pattern: Some("NN(:UN)?"), negation: false },
    PosPatternElement { text: None, pos_pattern: None, negation: false },
];

static PATTERN_31: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START|PCT"), negation: false },
    PosPatternElement { text: Some("some"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("time"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: None, negation: false },
];

static PATTERN_32: &[PosPatternElement] = &[
    PosPatternElement { text: Some("proofed"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VB"), negation: false },
];

static PATTERN_33: &[PosPatternElement] = &[
    PosPatternElement { text: Some("anyway"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VB"), negation: false },
];

static PATTERN_34: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START|PCT|CC"), negation: false },
    PosPatternElement { text: Some("as"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("mention"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("DT|PRP.*|NNP|EX|PCT"), negation: false },
];

static PATTERN_35: &[PosPatternElement] = &[
    PosPatternElement { text: Some("not"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("longer"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("than"), pos_pattern: Some("VB.*|JJ|IN|PCT|CC"), negation: false },
];

static PATTERN_36: &[PosPatternElement] = &[
    PosPatternElement { text: Some("please"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VB"), negation: false },
    PosPatternElement { text: Some("you"), pos_pattern: None, negation: false },
];

static PATTERN_37: &[PosPatternElement] = &[
    PosPatternElement { text: Some("need"), pos_pattern: Some("MD"), negation: false },
    PosPatternElement { text: Some("no"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBP?"), negation: false },
];

static PATTERN_38: &[PosPatternElement] = &[
    PosPatternElement { text: Some("wo"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("n't"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VB"), negation: false },
];

static PATTERN_39: &[PosPatternElement] = &[
    PosPatternElement { text: Some("life"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("long"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("JJ|NN.*"), negation: false },
];

static PATTERN_40: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START|CC|PCT"), negation: false },
    PosPatternElement { text: Some("now"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("are"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("NN.*"), negation: false },
];

static PATTERN_41: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("VBG"), negation: false },
    PosPatternElement { text: Some("be"), pos_pattern: None, negation: false },
];

static PATTERN_42: &[PosPatternElement] = &[
    PosPatternElement { text: Some("we"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("RB"), negation: false },
    PosPatternElement { text: Some("are"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VB"), negation: false },
];

static PATTERN_43: &[PosPatternElement] = &[
    PosPatternElement { text: Some("prefer"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBG"), negation: false },
];

static PATTERN_44: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("PRP"), negation: false },
    PosPatternElement { text: Some("need"), pos_pattern: Some("MD"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("CD"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("IN"), negation: false },
];

static PATTERN_45: &[PosPatternElement] = &[
    PosPatternElement { text: Some("ami"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBG"), negation: false },
];

static PATTERN_46: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START"), negation: false },
    PosPatternElement { text: Some("in"), pos_pattern: Some("IN"), negation: false },
    PosPatternElement { text: Some("addition"), pos_pattern: None, negation: false },
];

static PATTERN_47: &[PosPatternElement] = &[
    PosPatternElement { text: Some("essential"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("that"), pos_pattern: Some("DT"), negation: false },
];

static PATTERN_48: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("VBP?"), negation: false },
    PosPatternElement { text: Some("gave"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("in"), pos_pattern: None, negation: true },
];

static PATTERN_49: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("VBP?"), negation: false },
    PosPatternElement { text: Some("fond"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("of"), pos_pattern: Some("IN"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBG"), negation: false },
];

static PATTERN_50: &[PosPatternElement] = &[
    PosPatternElement { text: Some("many"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("kinds"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("of"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("NN"), negation: false },
];

static PATTERN_51: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("PRP"), negation: false },
    PosPatternElement { text: Some("will"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("like"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: None, negation: false },
];

static PATTERN_52: &[PosPatternElement] = &[
    PosPatternElement { text: Some("a"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("have"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBN"), negation: false },
];

static PATTERN_53: &[PosPatternElement] = &[
    PosPatternElement { text: Some("going"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBD"), negation: false },
];

static PATTERN_54: &[PosPatternElement] = &[
    PosPatternElement { text: Some("less"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("trigger"), pos_pattern: Some("JJ[RS]"), negation: false },
];

static PATTERN_55: &[PosPatternElement] = &[
    PosPatternElement { text: Some("need"), pos_pattern: Some("MD"), negation: false },
    PosPatternElement { text: Some("becoming"), pos_pattern: None, negation: false },
];

static PATTERN_56: &[PosPatternElement] = &[
    PosPatternElement { text: Some("must"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VB"), negation: false },
];

static PATTERN_57: &[PosPatternElement] = &[
    PosPatternElement { text: Some("eager"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VB[GZND]"), negation: false },
];

static PATTERN_58: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: None, negation: false },
    PosPatternElement { text: Some("use"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VB"), negation: false },
];

static PATTERN_59: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("PRP"), negation: false },
    PosPatternElement { text: Some("anit"), pos_pattern: None, negation: false },
];

static PATTERN_60: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("PRP|MD"), negation: false },
    PosPatternElement { text: Some("haft"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("DT|TO"), negation: false },
];

static PATTERN_61: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START"), negation: false },
    PosPatternElement { text: Some("according"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("me"), pos_pattern: None, negation: false },
];

static PATTERN_62: &[PosPatternElement] = &[
    PosPatternElement { text: Some("at"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("CD"), negation: false },
    PosPatternElement { text: Some("clock"), pos_pattern: None, negation: false },
];

static PATTERN_63: &[PosPatternElement] = &[
    PosPatternElement { text: Some("accustomed"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: Some("IN"), negation: false },
];

static PATTERN_64: &[PosPatternElement] = &[
    PosPatternElement { text: Some("according"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("to"), pos_pattern: Some("IN"), negation: false },
];

static PATTERN_65: &[PosPatternElement] = &[
    PosPatternElement { text: Some("in"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("the"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("outside"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("IN|PCT|,"), negation: false },
];

static PATTERN_66: &[PosPatternElement] = &[
    PosPatternElement { text: Some("anxious"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("of"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("PRP$"), negation: false },
];

static PATTERN_67: &[PosPatternElement] = &[
    PosPatternElement { text: Some("in"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("front"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("of"), pos_pattern: Some("IN"), negation: false },
];

static PATTERN_68: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START"), negation: false },
    PosPatternElement { text: Some("by"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("default"), pos_pattern: None, negation: false },
];

static PATTERN_69: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("MD|TO"), negation: false },
    PosPatternElement { text: Some("addend"), pos_pattern: None, negation: false },
];

static PATTERN_70: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("RB"), negation: false },
    PosPatternElement { text: Some("various"), pos_pattern: None, negation: false },
];

static PATTERN_71: &[PosPatternElement] = &[
    PosPatternElement { text: Some("for"), pos_pattern: Some("CC|SENT_START|PCT"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("PRP_S.*"), negation: false },
    PosPatternElement { text: Some("wood"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VB"), negation: false },
];

static PATTERN_72: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("MD|PRP_S.*|NNPS?|TO"), negation: false },
    PosPatternElement { text: Some("lear"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("from"), pos_pattern: None, negation: false },
];

static PATTERN_73: &[PosPatternElement] = &[
    PosPatternElement { text: Some("need"), pos_pattern: Some("MD|TO"), negation: false },
    PosPatternElement { text: Some("bee"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("JJR?"), negation: false },
];

static PATTERN_74: &[PosPatternElement] = &[
    PosPatternElement { text: Some("fir"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("PRP\\$|DT|NNP|PRP_O.*"), negation: false },
];

static PATTERN_75: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("PRP|MD|TO"), negation: false },
    PosPatternElement { text: Some("fir"), pos_pattern: None, negation: false },
];

static PATTERN_76: &[PosPatternElement] = &[
    PosPatternElement { text: Some("need"), pos_pattern: Some("MD|PRP_S[12]"), negation: false },
    PosPatternElement { text: Some("se"), pos_pattern: None, negation: false },
];

static PATTERN_77: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("MD"), negation: false },
    PosPatternElement { text: Some("he"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBN"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("CC|PCT|IN|RB"), negation: false },
];

static PATTERN_78: &[PosPatternElement] = &[
    PosPatternElement { text: Some("tor"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VB"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("IN|DT|PRP.*"), negation: false },
];

static PATTERN_79: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("VB.*"), negation: false },
    PosPatternElement { text: Some("o"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VB"), negation: false },
];

static PATTERN_80: &[PosPatternElement] = &[
    PosPatternElement { text: Some("need"), pos_pattern: Some("PRP|MD|TO"), negation: false },
    PosPatternElement { text: Some("writ"), pos_pattern: None, negation: false },
];

static PATTERN_81: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("IN|VB.*|RB|PRP|SENT_START"), negation: false },
    PosPatternElement { text: Some("what"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("it"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBG"), negation: false },
];

static PATTERN_82: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("VBG"), negation: false },
    PosPatternElement { text: Some("they"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("'re"), pos_pattern: None, negation: false },
];

static PATTERN_83: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("SENT_START"), negation: false },
    PosPatternElement { text: Some(","), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("than"), pos_pattern: None, negation: false },
];

static PATTERN_84: &[PosPatternElement] = &[
    PosPatternElement { text: Some("number"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("or"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("NNP?S"), negation: false },
    PosPatternElement { text: Some("of"), pos_pattern: None, negation: false },
];

static PATTERN_85: &[PosPatternElement] = &[
    PosPatternElement { text: Some("ally"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("NN(S|:UN)|VB[PZ]?"), negation: false },
];

static PATTERN_86: &[PosPatternElement] = &[
    PosPatternElement { text: Some("to"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("architect"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("DT|NNS?|NN:UN?"), negation: false },
];

static PATTERN_87: &[PosPatternElement] = &[
    PosPatternElement { text: Some("axed"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("PRP"), negation: false },
];

static PATTERN_88: &[PosPatternElement] = &[
    PosPatternElement { text: Some("not"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("jet"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBN"), negation: false },
];

static PATTERN_89: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("IN"), negation: false },
    PosPatternElement { text: Some("you"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("'re"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("NN.*"), negation: false },
    PosPatternElement { text: None, pos_pattern: Some("SENT_END"), negation: false },
];

static PATTERN_90: &[PosPatternElement] = &[
    PosPatternElement { text: None, pos_pattern: Some("DT|JJ.*|VB[ZD]"), negation: false },
    PosPatternElement { text: Some("whip"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("cream"), pos_pattern: None, negation: false },
];

static PATTERN_91: &[PosPatternElement] = &[
    PosPatternElement { text: Some("approximately"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("about"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("CD"), negation: false },
];

static PATTERN_92: &[PosPatternElement] = &[
    PosPatternElement { text: Some("the"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("world"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("around"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("PRP"), negation: false },
    PosPatternElement { text: None, pos_pattern: None, negation: false },
];

static PATTERN_93: &[PosPatternElement] = &[
    PosPatternElement { text: Some("try"), pos_pattern: None, negation: false },
    PosPatternElement { text: Some("and"), pos_pattern: None, negation: false },
    PosPatternElement { text: None, pos_pattern: Some("VBP?"), negation: false },
];

/// POS pattern rules for EN (requires POS tagging)
pub static EN_POS_PATTERN_RULES: &[PosPatternRule] = &[
    PosPatternRule {
		id: "PLEASER",
		pattern: PATTERN_0,
		message: "Did you mean \"please\"?",
		suggestions: &["please"],
	},
    PosPatternRule {
		id: "TRUE_TO_WORD",
		pattern: PATTERN_1,
		message: "Did you mean the idiom 'true to one's word' (=keep one's promise)?",
		suggestions: &["word"],
	},
    PosPatternRule {
		id: "HALO_HALLO",
		pattern: PATTERN_2,
		message: "Hallo, the greeting? A halo is an optical phenomenon.",
		suggestions: &["Hallo"],
	},
    PosPatternRule {
		id: "BY_PASSIVE_PARTICIPLE_BE",
		pattern: PATTERN_3,
		message: "be?",
		suggestions: &["be"],
	},
    PosPatternRule {
		id: "CONFUSION_OF_THESES_THESE",
		pattern: PATTERN_4,
		message: "these?",
		suggestions: &["these"],
	},
    PosPatternRule {
		id: "EASIEST_WAS_TO",
		pattern: PATTERN_5,
		message: "way?",
		suggestions: &["way"],
	},
    PosPatternRule {
		id: "ANOTHER_WORDS",
		pattern: PATTERN_6,
		message: "in other words?",
		suggestions: &["in other words"],
	},
    PosPatternRule {
		id: "ALL_MOST",
		pattern: PATTERN_7,
		message: "almost?",
		suggestions: &["almost"],
	},
    PosPatternRule {
		id: "ONE_ORE",
		pattern: PATTERN_8,
		message: "or?",
		suggestions: &["or"],
	},
    PosPatternRule {
		id: "DT_RESPONDS",
		pattern: PATTERN_9,
		message: "response?",
		suggestions: &["response"],
	},
    PosPatternRule {
		id: "WHOSE_DT",
		pattern: PATTERN_10,
		message: "who's?",
		suggestions: &["who's"],
	},
    PosPatternRule {
		id: "WHOS_TO",
		pattern: PATTERN_11,
		message: "Who's?",
		suggestions: &["Who's"],
	},
    PosPatternRule {
		id: "HAD_OF",
		pattern: PATTERN_12,
		message: "had.",
		suggestions: &["had"],
	},
    PosPatternRule {
		id: "DT_BAIL_OUT",
		pattern: PATTERN_13,
		message: "bailout?",
		suggestions: &["bailout"],
	},
    PosPatternRule {
		id: "IN_PRP_THEN_IN_PRP",
		pattern: PATTERN_14,
		message: "than?",
		suggestions: &["than"],
	},
    PosPatternRule {
		id: "COMPARISONS_AS_ADJECTIVE_AS",
		pattern: PATTERN_15,
		message: "?",
		suggestions: &[""],
	},
    PosPatternRule {
		id: "YOURE_JJS_NN",
		pattern: PATTERN_16,
		message: "your?",
		suggestions: &["your"],
	},
    PosPatternRule {
		id: "THEYRE_JJS_NN",
		pattern: PATTERN_17,
		message: "their?",
		suggestions: &["their"],
	},
    PosPatternRule {
		id: "WRB_THERE_THEY_RE",
		pattern: PATTERN_18,
		message: "they are?",
		suggestions: &["they are"],
	},
    PosPatternRule {
		id: "VBZ_IN_THEYRE_NN",
		pattern: PATTERN_19,
		message: "their?",
		suggestions: &["their"],
	},
    PosPatternRule {
		id: "IN_VBZ_THEYRE_NN",
		pattern: PATTERN_20,
		message: "their?",
		suggestions: &["their"],
	},
    PosPatternRule {
		id: "DT_VBZ_THEYRE_NN",
		pattern: PATTERN_21,
		message: "their?",
		suggestions: &["their"],
	},
    PosPatternRule {
		id: "WITHE_WITH",
		pattern: PATTERN_22,
		message: "?",
		suggestions: &[""],
	},
    PosPatternRule {
		id: "TOO_CARDINAL_NUMBER",
		pattern: PATTERN_23,
		message: "to?",
		suggestions: &["to"],
	},
    PosPatternRule {
		id: "BY_EXPIRE",
		pattern: PATTERN_24,
		message: "The preposition \"by\" usually has to be followed by a noun phrase or a verb ending in \"-ing\". Did you mean something else?",
		suggestions: &["expiring"],
	},
    PosPatternRule {
		id: "ANOTHER_DATE",
		pattern: PATTERN_25,
		message: "another?",
		suggestions: &["another"],
	},
    PosPatternRule {
		id: "ONE_OF_THE_KIND",
		pattern: PATTERN_26,
		message: "a\\4'?",
		suggestions: &["a"],
	},
    PosPatternRule {
		id: "BEGINNING_TO_ADDING_BROAD",
		pattern: PATTERN_27,
		message: "to?",
		suggestions: &["to"],
	},
    PosPatternRule {
		id: "CONFUSION_OF_WHEN_WHAT",
		pattern: PATTERN_28,
		message: "whatinstead?",
		suggestions: &["what"],
	},
    PosPatternRule {
		id: "PAST_AN_PAST",
		pattern: PATTERN_29,
		message: "Possible typo detected: Did you mean to write \"and\" here?",
		suggestions: &["and"],
	},
    PosPatternRule {
		id: "OF_ALL_PLURAL",
		pattern: PATTERN_30,
		message: "With the quantifier 'all', the plural form may be more appropriate here.",
		suggestions: &[""],
	},
    PosPatternRule {
		id: "SOME_TIME_SOMETIMES",
		pattern: PATTERN_31,
		message: "Instead of the noun phrase 'some time', did you mean to use the adverb?",
		suggestions: &["sometimes"],
	},
    PosPatternRule {
		id: "PROOFED_PROVED",
		pattern: PATTERN_32,
		message: "Did you mean to write 'proved'?",
		suggestions: &["proved"],
	},
    PosPatternRule {
		id: "ANY_WAY_TO_VB",
		pattern: PATTERN_33,
		message: "Instead of the adverb, did you mean to write 'any way'?",
		suggestions: &["any way"],
	},
    PosPatternRule {
		id: "AS_MENTION",
		pattern: PATTERN_34,
		message: "mentioned(past tense of \"mention\")?",
		suggestions: &["mentioned"],
	},
    PosPatternRule {
		id: "NOT_LONGER",
		pattern: PATTERN_35,
		message: "no longer?",
		suggestions: &["no longer"],
	},
    PosPatternRule {
		id: "PLEASE_TO_MEET_YOU",
		pattern: PATTERN_36,
		message: "pleased?",
		suggestions: &["pleased"],
	},
    PosPatternRule {
		id: "MD_NO_VB",
		pattern: PATTERN_37,
		message: "not.",
		suggestions: &["not"],
	},
    PosPatternRule {
		id: "WON_T_TO",
		pattern: PATTERN_38,
		message: "want?",
		suggestions: &["want"],
	},
    PosPatternRule {
		id: "LONG_COMPOUNDS",
		pattern: PATTERN_39,
		message: "lifelongis spelled as one word.",
		suggestions: &["lifelong"],
	},
    PosPatternRule {
		id: "NOW_ARE_THE_TIME",
		pattern: PATTERN_40,
		message: "is?",
		suggestions: &["is"],
	},
    PosPatternRule {
		id: "BE_IS",
		pattern: PATTERN_41,
		message: "can be?",
		suggestions: &["is"],
	},
    PosPatternRule {
		id: "WE_RB_ARE_VB",
		pattern: PATTERN_42,
		message: "?",
		suggestions: &[""],
	},
    PosPatternRule {
		id: "PREFER_TO_VBG",
		pattern: PATTERN_43,
		message: "?",
		suggestions: &[""],
	},
    PosPatternRule {
		id: "PRP_MD_CD_IN",
		pattern: PATTERN_44,
		message: "be?",
		suggestions: &["use"],
	},
    PosPatternRule {
		id: "AM_I",
		pattern: PATTERN_45,
		message: "I am?",
		suggestions: &["am I"],
	},
    PosPatternRule {
		id: "ON_ADDITION",
		pattern: PATTERN_46,
		message: "in?",
		suggestions: &["in"],
	},
    PosPatternRule {
		id: "ESSENTIAL_ESSENTIALLY",
		pattern: PATTERN_47,
		message: "essentially?",
		suggestions: &["essentially"],
	},
    PosPatternRule {
		id: "GAVE_GIVE",
		pattern: PATTERN_48,
		message: "give?",
		suggestions: &["give"],
	},
    PosPatternRule {
		id: "BE_FOND_TO",
		pattern: PATTERN_49,
		message: "of?",
		suggestions: &["of"],
	},
    PosPatternRule {
		id: "MANY_KINDS_OF",
		pattern: PATTERN_50,
		message: "?",
		suggestions: &[""],
	},
    PosPatternRule {
		id: "WILL_LIKE_TO",
		pattern: PATTERN_51,
		message: "would?",
		suggestions: &["would"],
	},
    PosPatternRule {
		id: "A_HAVE_VBN",
		pattern: PATTERN_52,
		message: "I?",
		suggestions: &["I"],
	},
    PosPatternRule {
		id: "GOING_TO_VBD",
		pattern: PATTERN_53,
		message: "The verb after 'going to' requires the base form.",
		suggestions: &[""],
	},
    PosPatternRule {
		id: "LESS_COMPARATIVE",
		pattern: PATTERN_54,
		message: "?",
		suggestions: &[""],
	},
    PosPatternRule {
		id: "WILL_BECOMING",
		pattern: PATTERN_55,
		message: "be coming?",
		suggestions: &["be coming"],
	},
    PosPatternRule {
		id: "MUST_HAVE_TO",
		pattern: PATTERN_56,
		message: "have tohere.",
		suggestions: &["must"],
	},
    PosPatternRule {
		id: "EAGER_TO",
		pattern: PATTERN_57,
		message: "With 'eager to', use the base form of the verb.",
		suggestions: &[""],
	},
    PosPatternRule {
		id: "USE_TO_VERB",
		pattern: PATTERN_58,
		message: "used to.",
		suggestions: &["used to"],
	},
    PosPatternRule {
		id: "PRP_ANIT",
		pattern: PATTERN_59,
		message: "ain't?",
		suggestions: &["ain't"],
	},
    PosPatternRule {
		id: "PRP_HAFT",
		pattern: PATTERN_60,
		message: "have?",
		suggestions: &["have"],
	},
    PosPatternRule {
		id: "ACCORDING_TO_ME",
		pattern: PATTERN_61,
		message: "I think.",
		suggestions: &["in my opinion"],
	},
    PosPatternRule {
		id: "AT_CD_CLOCK",
		pattern: PATTERN_62,
		message: "o'clock?",
		suggestions: &["o'clock"],
	},
    PosPatternRule {
		id: "ACCUSTOMED_TO",
		pattern: PATTERN_63,
		message: "accustomed to?",
		suggestions: &["accustomed to"],
	},
    PosPatternRule {
		id: "ACCORDING_TO",
		pattern: PATTERN_64,
		message: "according to? according to: (as stated/in the opinion of)",
		suggestions: &["according to"],
	},
    PosPatternRule {
		id: "IN_THE_OUTSIDE",
		pattern: PATTERN_65,
		message: "The usual preposition for \"outside\" is \"on\", not \"in\".",
		suggestions: &["on the outside"],
	},
    PosPatternRule {
		id: "ANXIOUS_OF_ABOUT",
		pattern: PATTERN_66,
		message: "anxious for?",
		suggestions: &["anxious about"],
	},
    PosPatternRule {
		id: "IN_FRONT_OF",
		pattern: PATTERN_67,
		message: "in front of?",
		suggestions: &["in front of"],
	},
    PosPatternRule {
		id: "BY_DEFAULT_COMMA",
		pattern: PATTERN_68,
		message: "By default,?",
		suggestions: &["By default,"],
	},
    PosPatternRule {
		id: "ADDEND_ATTEND",
		pattern: PATTERN_69,
		message: "attend(= to participate in an event)?",
		suggestions: &["attend"],
	},
    PosPatternRule {
		id: "VARIOUS_VARIES",
		pattern: PATTERN_70,
		message: "Did you mean to write the verb \"varies\" here?",
		suggestions: &["varies"],
	},
    PosPatternRule {
		id: "WOOD_WOULD",
		pattern: PATTERN_71,
		message: "would?",
		suggestions: &["would"],
	},
    PosPatternRule {
		id: "LEAR_LEARN",
		pattern: PATTERN_72,
		message: "learn?",
		suggestions: &["learn"],
	},
    PosPatternRule {
		id: "BEE_BE",
		pattern: PATTERN_73,
		message: "be?",
		suggestions: &["be"],
	},
    PosPatternRule {
		id: "FIR_FOR",
		pattern: PATTERN_74,
		message: "for?",
		suggestions: &["for"],
	},
    PosPatternRule {
		id: "FIR_FIT",
		pattern: PATTERN_75,
		message: "fit?",
		suggestions: &["fit"],
	},
    PosPatternRule {
		id: "SE_SEE",
		pattern: PATTERN_76,
		message: "See?",
		suggestions: &["See"],
	},
    PosPatternRule {
		id: "HE_BE",
		pattern: PATTERN_77,
		message: "be?",
		suggestions: &["be"],
	},
    PosPatternRule {
		id: "TOR_TO",
		pattern: PATTERN_78,
		message: "to?",
		suggestions: &["to"],
	},
    PosPatternRule {
		id: "O_TO",
		pattern: PATTERN_79,
		message: "to?",
		suggestions: &["to"],
	},
    PosPatternRule {
		id: "WRIT_WRITE",
		pattern: PATTERN_80,
		message: "write?",
		suggestions: &["write"],
	},
    PosPatternRule {
		id: "WHAT_IT_HAPPENING",
		pattern: PATTERN_81,
		message: "is?",
		suggestions: &["is"],
	},
    PosPatternRule {
		id: "VBG_THEYRE",
		pattern: PATTERN_82,
		message: "their?",
		suggestions: &["their"],
	},
    PosPatternRule {
		id: "COMMA_THAN",
		pattern: PATTERN_83,
		message: "then?",
		suggestions: &["then"],
	},
    PosPatternRule {
		id: "NUMBER_OF_NNS",
		pattern: PATTERN_84,
		message: "of?",
		suggestions: &["of"],
	},
    PosPatternRule {
		id: "ALLY_ALLAY",
		pattern: PATTERN_85,
		message: "allay?",
		suggestions: &["allay"],
	},
    PosPatternRule {
		id: "ARCHITECT_VERB",
		pattern: PATTERN_86,
		message: "devise?",
		suggestions: &["design"],
	},
    PosPatternRule {
		id: "AXED_ASKED",
		pattern: PATTERN_87,
		message: "asked?",
		suggestions: &["asked"],
	},
    PosPatternRule {
		id: "NOT_JET",
		pattern: PATTERN_88,
		message: "yet?",
		suggestions: &["yet"],
	},
    PosPatternRule {
		id: "IN_YOU_RE_NN",
		pattern: PATTERN_89,
		message: "your?",
		suggestions: &["your"],
	},
    PosPatternRule {
		id: "WHIP_CREAM",
		pattern: PATTERN_90,
		message: "whipped cream?",
		suggestions: &["whipped cream"],
	},
    PosPatternRule {
		id: "APPROXIMATELY_ABOUT",
		pattern: PATTERN_91,
		message: ".",
		suggestions: &[""],
	},
    PosPatternRule {
		id: "WORLD_AROUND_IT",
		pattern: PATTERN_92,
		message: "the world.",
		suggestions: &["the world"],
	},
    PosPatternRule {
		id: "TRY_AND",
		pattern: PATTERN_93,
		message: "try tois recommended for writing.",
		suggestions: &["try to"],
	},
];

/// Create a PosPatternChecker with EN rules
pub fn create_en_pos_pattern_checker() -> crate::checker::pos_pattern_checker::PosPatternChecker {
	crate::checker::pos_pattern_checker::PosPatternChecker::with_rules(EN_POS_PATTERN_RULES)
}
