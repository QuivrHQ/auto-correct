//! Auto-generated antipatterns for EN from LanguageTool
//! Synced: 2026-01-23T14:23:45.047318463+00:00
//! Total antipatterns: 1053
//! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update
//!
//! Source: LanguageTool grammar.xml
//! License: LGPL 2.1+
//!
//! Antipatterns are exceptions to grammar rules.
//! When text matches an antipattern, the rule should NOT fire.

/// A token in an antipattern
#[derive(Debug, Clone)]
pub struct AntipatternToken {
    /// Literal text to match (case-insensitive)
    pub text: Option<&'static str>,
    /// Regex pattern to match
    pub regexp: Option<&'static str>,
    /// Whether to match inflected forms
    pub inflected: bool,
}

/// An antipattern (exception to a rule)
#[derive(Debug, Clone)]
pub struct Antipattern {
    /// The rule ID this antipattern applies to
    pub rule_id: &'static str,
    /// The token sequence that should NOT trigger the rule
    pub tokens: &'static [AntipatternToken],
}

static ANTIPATTERN_0_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_1_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("at"), regexp: None, inflected: false },
    AntipatternToken { text: Some("all|once|first|le?ast"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_2_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("according"), regexp: None, inflected: false },
    AntipatternToken { text: Some("as"), regexp: None, inflected: false },
];
static ANTIPATTERN_3_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("make"), regexp: None, inflected: true },
    AntipatternToken { text: Some("my|your|her|his"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("heart|head"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_4_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("enough"), regexp: None, inflected: false },
    AntipatternToken { text: Some("be"), regexp: None, inflected: true },
    AntipatternToken { text: Some("enough"), regexp: None, inflected: false },
];
static ANTIPATTERN_5_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("first"), regexp: None, inflected: false },
    AntipatternToken { text: Some("come"), regexp: None, inflected: false },
    AntipatternToken { text: Some("first"), regexp: None, inflected: false },
];
static ANTIPATTERN_6_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("interested"), regexp: None, inflected: false },
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
];
static ANTIPATTERN_7_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("never"), regexp: None, inflected: false },
    AntipatternToken { text: Some("say"), regexp: None, inflected: false },
    AntipatternToken { text: Some("never"), regexp: None, inflected: false },
];
static ANTIPATTERN_8_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("advertisement"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the|all|this"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("year|month|week|day|time"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_9_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("ai"), regexp: None, inflected: false },
];
static ANTIPATTERN_10_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[#=]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("ai"), regexp: None, inflected: false },
];
static ANTIPATTERN_11_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\p{ll}\\p{ll}+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("ai"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\p{ll}\\p{ll}+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_12_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("t"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("ai"), regexp: None, inflected: false },
    AntipatternToken { text: Some("chi"), regexp: None, inflected: false },
];
static ANTIPATTERN_13_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("allow"), regexp: None, inflected: true },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("please"), regexp: None, inflected: false },
];
static ANTIPATTERN_14_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("humanity"), regexp: None, inflected: false },
];
static ANTIPATTERN_15_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("all|most|some"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_16_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("all|most|some"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z]{2,5}"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_17_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("all|most|some"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("last|next"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("weeks|months|quarters|seasons|years"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_18_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\p{lu}\\p{l}+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("\\p{lu}\\p{l}+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_19_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ways"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("course"), regexp: None, inflected: false },
];
static ANTIPATTERN_20_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("in|by|of"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ways"), regexp: None, inflected: false },
];
static ANTIPATTERN_21_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ways"), regexp: None, inflected: false },
    AntipatternToken { text: Some("lead"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_22_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("americano"), regexp: None, inflected: false },
    AntipatternToken { text: Some("cocktails?|coffees?|flavou?r|recipes?|calories"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_23_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("anymore"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_24_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("anymore"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\(|to"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_25_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("anymore"), regexp: None, inflected: false },
    AntipatternToken { text: Some("be"), regexp: None, inflected: true },
];
static ANTIPATTERN_26_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("anymore"), regexp: None, inflected: false },
    AntipatternToken { text: Some("due"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_27_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("'(s|d|ve|ve|ve|re|re|re|t|ll|ll|ll)"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_28_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("if"), regexp: None, inflected: false },
    AntipatternToken { text: Some("so|yes|not?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_29_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("amazon"), regexp: None, inflected: false },
    AntipatternToken { text: Some("appstore"), regexp: None, inflected: false },
];
static ANTIPATTERN_30_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("on"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("market|scene"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_31_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("on"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("morning|afternoon|evening|dawn|first"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_32_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("little"), regexp: None, inflected: false },
];
static ANTIPATTERN_33_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ask"), regexp: None, inflected: true },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("party|prom|meeting"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_34_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("was|were|are|is|been|be"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("asked"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_35_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("assassin|assassin"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("s|s"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("creed|creed"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_36_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("as"), regexp: None, inflected: false },
    AntipatternToken { text: Some("well|intended|expected"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_37_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("back"), regexp: None, inflected: false },
    AntipatternToken { text: Some("then"), regexp: None, inflected: false },
];
static ANTIPATTERN_38_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sad"), regexp: None, inflected: false },
    AntipatternToken { text: Some("as"), regexp: None, inflected: false },
];
static ANTIPATTERN_39_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("look"), regexp: None, inflected: true },
    AntipatternToken { text: Some("at"), regexp: None, inflected: false },
];
static ANTIPATTERN_40_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("kitchen"), regexp: None, inflected: false },
    AntipatternToken { text: Some("counters?|sinks?|tables?|island|door"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_41_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("kitchen"), regexp: None, inflected: false },
    AntipatternToken { text: Some("counters?|sinks?|tables?|island|door"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_42_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("room"), regexp: None, inflected: false },
    AntipatternToken { text: Some("tables?|doors?|windows?|pressure"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_43_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("toilet"), regexp: None, inflected: false },
    AntipatternToken { text: Some("seats?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_44_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("remain|stay"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("at"), regexp: None, inflected: false },
];
static ANTIPATTERN_45_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("shot"), regexp: None, inflected: false },
    AntipatternToken { text: Some("at"), regexp: None, inflected: false },
];
static ANTIPATTERN_46_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("au"), regexp: None, inflected: false },
];
static ANTIPATTERN_47_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(".*[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("au"), regexp: None, inflected: false },
];
static ANTIPATTERN_48_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("/"), regexp: None, inflected: false },
    AntipatternToken { text: Some("au"), regexp: None, inflected: false },
    AntipatternToken { text: Some("/"), regexp: None, inflected: false },
];
static ANTIPATTERN_49_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("voyage"), regexp: None, inflected: false },
    AntipatternToken { text: Some("au"), regexp: None, inflected: false },
    AntipatternToken { text: Some("congo"), regexp: None, inflected: false },
];
static ANTIPATTERN_50_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("au"), regexp: None, inflected: false },
];
static ANTIPATTERN_51_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("au"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("com"), regexp: None, inflected: false },
];
static ANTIPATTERN_52_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("au"), regexp: None, inflected: false },
    AntipatternToken { text: Some(".*[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_53_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("au"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pairs?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_54_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("café|cafe|sauce|invitation|port|suite|phrase|appliqués?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("au"), regexp: None, inflected: false },
];
static ANTIPATTERN_55_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("l"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("artiste"), regexp: None, inflected: false },
    AntipatternToken { text: Some("au"), regexp: None, inflected: false },
];
static ANTIPATTERN_56_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("lettre"), regexp: None, inflected: false },
    AntipatternToken { text: Some("au"), regexp: None, inflected: false },
];
static ANTIPATTERN_57_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bit"), regexp: None, inflected: false },
    AntipatternToken { text: Some("old"), regexp: None, inflected: false },
    AntipatternToken { text: Some("school"), regexp: None, inflected: false },
];
static ANTIPATTERN_58_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bunch|lot|pair|majority|bit"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
    AntipatternToken { text: Some("consuming"), regexp: None, inflected: false },
];
static ANTIPATTERN_59_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("next"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_60_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("pair"), regexp: None, inflected: false },
    AntipatternToken { text: Some("example"), regexp: None, inflected: false },
];
static ANTIPATTERN_61_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("thanks|thx"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_62_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_63_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("worth"), regexp: None, inflected: false },
    AntipatternToken { text: Some("an?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_64_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("worth"), regexp: None, inflected: false },
    AntipatternToken { text: Some("more"), regexp: None, inflected: false },
    AntipatternToken { text: Some("th[ea]n"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("an?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_65_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_66_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_67_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("l[aa]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_68_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
    AntipatternToken { text: Some("zing"), regexp: None, inflected: false },
];
static ANTIPATTERN_69_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z]+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_70_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
];
static ANTIPATTERN_71_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("and|but"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("no"), regexp: None, inflected: false },
    AntipatternToken { text: Some("your|his|her|my"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_72_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("be|do|have"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("not"), regexp: None, inflected: true },
    AntipatternToken { text: Some("this|that|some|each"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_73_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("my"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hero"), regexp: None, inflected: false },
    AntipatternToken { text: Some("project"), regexp: None, inflected: false },
];
static ANTIPATTERN_74_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("oo?hh?|aa?hh?|hell|fuck"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("no"), regexp: None, inflected: false },
];
static ANTIPATTERN_75_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("think|hope"), regexp: None, inflected: false },
    AntipatternToken { text: Some("this|that|some|each"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_76_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_77_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_78_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("an"), regexp: None, inflected: false },
];
static ANTIPATTERN_79_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("always"), regexp: None, inflected: false },
    AntipatternToken { text: Some("product|company|corporation|corp|brand"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_80_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("twice"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bands?|albums?|songs?|concerts?|tour|members?|tracks?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_81_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("wisely"), regexp: None, inflected: false },
    AntipatternToken { text: Some("cards?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_82_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("an"), regexp: None, inflected: false },
    AntipatternToken { text: Some("also"), regexp: None, inflected: false },
];
static ANTIPATTERN_83_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("an?|the"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z].+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_84_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("an?|the"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("about"), regexp: None, inflected: false },
    AntipatternToken { text: Some("face|section|(sub-?)?page|website|tab|text|you|turns?|menu|buttons?|links?|screen"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_85_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bit"), regexp: None, inflected: false },
    AntipatternToken { text: Some("part"), regexp: None, inflected: false },
];
static ANTIPATTERN_86_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("between"), regexp: None, inflected: false },
];
static ANTIPATTERN_87_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("indeed"), regexp: None, inflected: false },
    AntipatternToken { text: Some("profiles?|accounts?|users?|jobs?|logins?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_88_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("kind|sort"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_89_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("life"), regexp: None, inflected: false },
    AntipatternToken { text: Some("changing"), regexp: None, inflected: false },
];
static ANTIPATTERN_90_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("nearly"), regexp: None, inflected: false },
    AntipatternToken { text: Some("man"), regexp: None, inflected: false },
];
static ANTIPATTERN_91_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("never"), regexp: None, inflected: false },
    AntipatternToken { text: Some("say"), regexp: None, inflected: false },
    AntipatternToken { text: Some("never"), regexp: None, inflected: false },
];
static ANTIPATTERN_92_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("never"), regexp: None, inflected: false },
    AntipatternToken { text: Some("smoker|event|trumper"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_93_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("notwithstanding"), regexp: None, inflected: false },
    AntipatternToken { text: Some("clause"), regexp: None, inflected: false },
];
static ANTIPATTERN_94_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("sudden"), regexp: None, inflected: false },
];
static ANTIPATTERN_95_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("once"), regexp: None, inflected: false },
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("while"), regexp: None, inflected: false },
];
static ANTIPATTERN_96_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("rather"), regexp: None, inflected: false },
    AntipatternToken { text: Some("out"), regexp: None, inflected: false },
];
static ANTIPATTERN_97_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("really"), regexp: None, inflected: false },
    AntipatternToken { text: Some("long|short"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_98_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("please|please|about|about"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_99_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("tragically"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hip"), regexp: None, inflected: false },
];
static ANTIPATTERN_100_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("therefore|thus|about"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("statements?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_101_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the|an?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_102_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("scissor"), regexp: None, inflected: false },
    AntipatternToken { text: Some("sisters"), regexp: None, inflected: false },
];
static ANTIPATTERN_103_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("scissors"), regexp: None, inflected: false },
];
static ANTIPATTERN_104_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("let"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_105_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_106_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_107_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("press|click|tap"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("on"), regexp: None, inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_108_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("an?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_109_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("back"), regexp: None, inflected: false },
    AntipatternToken { text: Some("stage"), regexp: None, inflected: false },
    AntipatternToken { text: Some("magazine"), regexp: None, inflected: false },
];
static ANTIPATTERN_110_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("back"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("back"), regexp: None, inflected: false },
];
static ANTIPATTERN_111_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bring"), regexp: None, inflected: true },
    AntipatternToken { text: Some("back"), regexp: None, inflected: false },
    AntipatternToken { text: Some("stories"), regexp: None, inflected: false },
];
static ANTIPATTERN_112_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("hand"), regexp: None, inflected: false },
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hand|glove"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_113_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("take|come"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("back"), regexp: None, inflected: false },
];
static ANTIPATTERN_114_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("top|right|bottom|left|side|north|south|east|west"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("and|&"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("back"), regexp: None, inflected: false },
];
static ANTIPATTERN_115_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("#"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bayern"), regexp: None, inflected: false },
];
static ANTIPATTERN_116_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bayern"), regexp: None, inflected: false },
    AntipatternToken { text: Some("munich|munchen|münchen"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_117_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bayern"), regexp: None, inflected: false },
    AntipatternToken { text: Some("vs|versus|against"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_118_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("fc|aok"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("bayern"), regexp: None, inflected: false },
];
static ANTIPATTERN_119_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("vs|versus|against"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("bayern"), regexp: None, inflected: false },
];
static ANTIPATTERN_120_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bay|bay"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("area|area"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_121_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tampa|winyah|chesapeake|turtle|monterey"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("bay"), regexp: None, inflected: false },
    AntipatternToken { text: Some("area"), regexp: None, inflected: false },
];
static ANTIPATTERN_122_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bean"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bags?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_123_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("will|'ll"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("have"), regexp: None, inflected: false },
];
static ANTIPATTERN_124_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("looking"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("working"), regexp: None, inflected: false },
];
static ANTIPATTERN_125_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("baha"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("i"), regexp: None, inflected: false },
    AntipatternToken { text: Some("beliefs"), regexp: None, inflected: false },
];
static ANTIPATTERN_126_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("may"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[rb]elief"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_127_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("each"), regexp: None, inflected: false },
    AntipatternToken { text: Some("believes"), regexp: None, inflected: false },
    AntipatternToken { text: Some("that|it(self)?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_128_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("this"), regexp: None, inflected: false },
    AntipatternToken { text: Some("day"), regexp: None, inflected: false },
];
static ANTIPATTERN_129_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bended"), regexp: None, inflected: false },
    AntipatternToken { text: Some("knees?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_130_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("point"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_131_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("beware"), regexp: None, inflected: true },
    AntipatternToken { text: Some("as"), regexp: None, inflected: false },
    AntipatternToken { text: Some("well(-.+)?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_132_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("its"), regexp: None, inflected: false },
    AntipatternToken { text: Some("own"), regexp: None, inflected: false },
    AntipatternToken { text: Some("way"), regexp: None, inflected: false },
];
static ANTIPATTERN_133_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("very"), regexp: None, inflected: false },
    AntipatternToken { text: Some("interest"), regexp: None, inflected: false },
];
static ANTIPATTERN_134_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("answers?|choices?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("be"), regexp: None, inflected: true },
    AntipatternToken { text: Some("no"), regexp: None, inflected: false },
];
static ANTIPATTERN_135_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("due"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_136_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("no"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bid"), regexp: None, inflected: false },
    AntipatternToken { text: Some("on"), regexp: None, inflected: false },
];
static ANTIPATTERN_137_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("no"), regexp: None, inflected: false },
    AntipatternToken { text: Some("different|less|more|secret|proof|emergency|shock|prize|chance|cheating|guessing"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_138_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("no"), regexp: None, inflected: false },
    AntipatternToken { text: Some("longer"), regexp: None, inflected: false },
];
static ANTIPATTERN_139_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("no"), regexp: None, inflected: false },
    AntipatternToken { text: Some("on"), regexp: None, inflected: false },
];
static ANTIPATTERN_140_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("be"), regexp: None, inflected: false },
    AntipatternToken { text: Some("no"), regexp: None, inflected: false },
];
static ANTIPATTERN_141_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("may"), regexp: None, inflected: false },
    AntipatternToken { text: Some("be"), regexp: None, inflected: false },
];
static ANTIPATTERN_142_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("here"), regexp: None, inflected: false },
    AntipatternToken { text: Some("is|'s"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_143_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("heres"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_144_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("those"), regexp: None, inflected: false },
    AntipatternToken { text: Some("that"), regexp: None, inflected: false },
    AntipatternToken { text: Some("are|were"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_145_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bis"), regexp: None, inflected: false },
    AntipatternToken { text: Some("sextum"), regexp: None, inflected: false },
];
static ANTIPATTERN_146_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("black|dead|black|dead"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("sea|sea"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_147_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sea"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bass(es)?|turtles?|bags?|gulls?|grass(es)?|birds?|anchors?|biscuits?|angels?|boats?|beds?|captains?|channels?|coals?|dogs?|eagles?|farers?|fans?|foods?|horses?|m[ea]n|otters?|oats|potato(es)?|shells?|wasps?|creatures?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_148_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("max"), regexp: None, inflected: false },
    AntipatternToken { text: Some("capacity"), regexp: None, inflected: false },
];
static ANTIPATTERN_149_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bon"), regexp: None, inflected: false },
    AntipatternToken { text: Some("appétit"), regexp: None, inflected: false },
];
static ANTIPATTERN_150_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ice"), regexp: None, inflected: false },
    AntipatternToken { text: Some("breaker"), regexp: None, inflected: false },
];
static ANTIPATTERN_151_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("loop"), regexp: None, inflected: false },
];
static ANTIPATTERN_152_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bu"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hasa"), regexp: None, inflected: false },
];
static ANTIPATTERN_153_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("korle"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bu"), regexp: None, inflected: false },
];
static ANTIPATTERN_154_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("buy"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ratings?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_155_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to|please"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("buy"), regexp: None, inflected: false },
];
static ANTIPATTERN_156_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("call|call"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("of|of|of"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("duty|duty"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_157_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("subject|response|addition|attention"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("backup"), regexp: None, inflected: false },
];
static ANTIPATTERN_158_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[^c]heckout"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_159_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("cam"), regexp: None, inflected: false },
    AntipatternToken { text: Some("down|off|out|up"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_160_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("your|their|his|her|my"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("own"), regexp: None, inflected: false },
    AntipatternToken { text: Some("good"), regexp: None, inflected: false },
];
static ANTIPATTERN_161_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("carnegie|carnegie"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("mellon|mellon"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_162_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("think|hope|believe"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_163_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("let"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_164_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("coast"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("coast"), regexp: None, inflected: false },
    AntipatternToken { text: Some("am"), regexp: None, inflected: false },
];
static ANTIPATTERN_165_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("as"), regexp: None, inflected: false },
    AntipatternToken { text: Some("if"), regexp: None, inflected: false },
];
static ANTIPATTERN_166_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_167_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_168_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("\\d{2}"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("campaign"), regexp: None, inflected: false },
];
static ANTIPATTERN_169_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("#"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_170_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d{2}"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("gun|revolver|pistol"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_171_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(".*[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_172_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("1000|thousand"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("island"), regexp: None, inflected: false },
];
static ANTIPATTERN_173_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("360"), regexp: None, inflected: false },
    AntipatternToken { text: Some("mode|camera"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_174_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("360|180"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("turn|flip"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_175_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("44"), regexp: None, inflected: false },
    AntipatternToken { text: Some("magnum"), regexp: None, inflected: false },
    AntipatternToken { text: Some("gun|revolver|pistol"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_176_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("50"), regexp: None, inflected: false },
    AntipatternToken { text: Some("cent"), regexp: None, inflected: false },
];
static ANTIPATTERN_177_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("502"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bad"), regexp: None, inflected: false },
    AntipatternToken { text: Some("gateway"), regexp: None, inflected: false },
];
static ANTIPATTERN_178_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("504"), regexp: None, inflected: false },
    AntipatternToken { text: Some("gateway"), regexp: None, inflected: false },
    AntipatternToken { text: Some("time-?out"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_179_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("504"), regexp: None, inflected: false },
    AntipatternToken { text: Some("education"), regexp: None, inflected: false },
    AntipatternToken { text: Some("plan|classroom"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_180_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("8"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ball"), regexp: None, inflected: false },
];
static ANTIPATTERN_181_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("8|16|32|64|128|256|512|1024|2048|4096"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("bit"), regexp: None, inflected: false },
];
static ANTIPATTERN_182_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("9|nine"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("5|five"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_183_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("billboard"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hot"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[12]00"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_184_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("six"), regexp: None, inflected: false },
    AntipatternToken { text: Some("flags"), regexp: None, inflected: false },
];
static ANTIPATTERN_185_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("two"), regexp: None, inflected: false },
    AntipatternToken { text: Some("city"), regexp: None, inflected: false },
    AntipatternToken { text: Some("center"), regexp: None, inflected: false },
];
static ANTIPATTERN_186_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[0-9]+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("\\p{lu}\\p{l}+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_187_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[0-9]+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("grad"), regexp: None, inflected: false },
    AntipatternToken { text: Some("="), regexp: None, inflected: false },
];
static ANTIPATTERN_188_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("vac"), regexp: None, inflected: false },
    AntipatternToken { text: Some("circuit"), regexp: None, inflected: false },
];
static ANTIPATTERN_189_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("pro"), regexp: None, inflected: false },
    AntipatternToken { text: Some("max"), regexp: None, inflected: false },
];
static ANTIPATTERN_190_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("sept|version"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_191_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("vote"), regexp: None, inflected: false },
];
static ANTIPATTERN_192_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("urinalysis|tuberculosis|nausea|cardiosclerosis|hypertension|kyphosis"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_193_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("volt"), regexp: None, inflected: false },
    AntipatternToken { text: Some("starter"), regexp: None, inflected: false },
];
static ANTIPATTERN_194_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("word|sentence|paragraph"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("essay"), regexp: None, inflected: false },
];
static ANTIPATTERN_195_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+0"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("decade"), regexp: None, inflected: false },
];
static ANTIPATTERN_196_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d{3}"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("error"), regexp: None, inflected: false },
];
static ANTIPATTERN_197_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("apple"), regexp: None, inflected: false },
    AntipatternToken { text: Some("watch"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_198_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("base"), regexp: None, inflected: false },
    AntipatternToken { text: Some("10|ten"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("model"), regexp: None, inflected: false },
];
static ANTIPATTERN_199_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("cat(egory)?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[1-8]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("cable|product|ethernet|lan|network|socket|connection|hurricane"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_200_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("covid"), regexp: None, inflected: false },
    AntipatternToken { text: Some("-"), regexp: None, inflected: false },
    AntipatternToken { text: Some("19"), regexp: None, inflected: false },
];
static ANTIPATTERN_201_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("day"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("session"), regexp: None, inflected: false },
];
static ANTIPATTERN_202_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("door"), regexp: None, inflected: false },
    AntipatternToken { text: Some("2"), regexp: None, inflected: false },
    AntipatternToken { text: Some("door"), regexp: None, inflected: false },
];
static ANTIPATTERN_203_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("face"), regexp: None, inflected: false },
    AntipatternToken { text: Some("2"), regexp: None, inflected: false },
    AntipatternToken { text: Some("face"), regexp: None, inflected: false },
];
static ANTIPATTERN_204_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("four|4|two|2"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("wheel"), regexp: None, inflected: false },
    AntipatternToken { text: Some("driv.+|cars?|suvs?|trucks?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_205_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("gta"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_206_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("half"), regexp: None, inflected: false },
    AntipatternToken { text: Some("an"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hour"), regexp: None, inflected: false },
];
static ANTIPATTERN_207_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d\\d\\d\\d"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_208_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("interstate|route"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_209_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("magnitude"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("earthquake"), regexp: None, inflected: false },
];
static ANTIPATTERN_210_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("need"), regexp: None, inflected: false },
    AntipatternToken { text: Some("4"), regexp: None, inflected: false },
    AntipatternToken { text: Some("speed"), regexp: None, inflected: false },
];
static ANTIPATTERN_211_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("no"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_212_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("page"), regexp: None, inflected: false },
    AntipatternToken { text: Some("three"), regexp: None, inflected: false },
    AntipatternToken { text: Some("girl"), regexp: None, inflected: false },
];
static ANTIPATTERN_213_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("page?|pg|clause|stage|phase|paper|sect?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_214_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("pg|ex|ch"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_215_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("room"), regexp: None, inflected: false },
    AntipatternToken { text: Some("51"), regexp: None, inflected: false },
];
static ANTIPATTERN_216_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("six|6"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("sigma"), regexp: None, inflected: false },
];
static ANTIPATTERN_217_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("suite"), regexp: None, inflected: false },
    AntipatternToken { text: Some("360"), regexp: None, inflected: false },
];
static ANTIPATTERN_218_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("top"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ten|10|five|5|three|3"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_219_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("web|industry"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("\\d"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("0"), regexp: None, inflected: false },
];
static ANTIPATTERN_220_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("year"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_221_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_222_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(".*[023456789]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("st"), regexp: None, inflected: false },
];
static ANTIPATTERN_223_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(".*[124567890]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("rd"), regexp: None, inflected: false },
];
static ANTIPATTERN_224_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(".*[134567890]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("nd"), regexp: None, inflected: false },
];
static ANTIPATTERN_225_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("need"), regexp: None, inflected: false },
    AntipatternToken { text: Some("click|view"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_226_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("pikes"), regexp: None, inflected: false },
    AntipatternToken { text: Some("peak"), regexp: None, inflected: false },
];
static ANTIPATTERN_227_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("roses"), regexp: None, inflected: false },
    AntipatternToken { text: Some("parade"), regexp: None, inflected: false },
];
static ANTIPATTERN_228_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("my"), regexp: None, inflected: false },
    AntipatternToken { text: Some("dears?|pals?|friends?|darlings?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_229_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("when"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
    AntipatternToken { text: Some("comes"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_230_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("("), regexp: None, inflected: false },
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some(")"), regexp: None, inflected: false },
];
static ANTIPATTERN_231_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
];
static ANTIPATTERN_232_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
];
static ANTIPATTERN_233_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some(".+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_234_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("net"), regexp: None, inflected: false },
];
static ANTIPATTERN_235_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[!?]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[\\.,]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[!?]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_236_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("['‘\"“]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("which"), regexp: None, inflected: false },
    AntipatternToken { text: Some("['’\"”]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_237_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("what"), regexp: None, inflected: false },
    AntipatternToken { text: Some("and|&|or"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("when"), regexp: None, inflected: false },
];
static ANTIPATTERN_238_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("a|some|any|the|lesser"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("degree|extent"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_239_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("point"), regexp: None, inflected: false },
    AntipatternToken { text: Some("counter"), regexp: None, inflected: false },
    AntipatternToken { text: Some("point"), regexp: None, inflected: false },
];
static ANTIPATTERN_240_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("couple"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("times|things"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_241_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("so"), regexp: None, inflected: false },
    AntipatternToken { text: Some("long"), regexp: None, inflected: false },
];
static ANTIPATTERN_242_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("sure|free|now|re[ae]l[sz]?|reasons"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_243_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[$€£¥฿\\u8371]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_244_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("college"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("cyber"), regexp: None, inflected: false },
    AntipatternToken { text: Some("security"), regexp: None, inflected: false },
];
static ANTIPATTERN_245_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("national"), regexp: None, inflected: false },
    AntipatternToken { text: Some("cyber"), regexp: None, inflected: false },
    AntipatternToken { text: Some("security"), regexp: None, inflected: false },
];
static ANTIPATTERN_246_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("any"), regexp: None, inflected: false },
    AntipatternToken { text: Some("sort|kind"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_247_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("deus"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ex"), regexp: None, inflected: false },
    AntipatternToken { text: Some("machina"), regexp: None, inflected: false },
];
static ANTIPATTERN_248_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("be|get"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("rid"), regexp: None, inflected: false },
];
static ANTIPATTERN_249_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("pit"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bull"), regexp: None, inflected: false },
];
static ANTIPATTERN_250_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("dog"), regexp: None, inflected: false },
    AntipatternToken { text: Some("food"), regexp: None, inflected: false },
];
static ANTIPATTERN_251_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("do"), regexp: None, inflected: true },
    AntipatternToken { text: Some("this|that|th[eo]se"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("mean"), regexp: None, inflected: false },
];
static ANTIPATTERN_252_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("cause"), regexp: None, inflected: true },
    AntipatternToken { text: Some("doubt"), regexp: None, inflected: false },
];
static ANTIPATTERN_253_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("a|the"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("moment|day|second|while"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_254_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("more"), regexp: None, inflected: false },
    AntipatternToken { text: Some("than"), regexp: None, inflected: false },
];
static ANTIPATTERN_255_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("now"), regexp: None, inflected: false },
];
static ANTIPATTERN_256_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("down"), regexp: None, inflected: false },
    AntipatternToken { text: Some("beat"), regexp: None, inflected: false },
];
static ANTIPATTERN_257_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("hold|lay|shut|cut|bring|come|close|drill|run|jump|fall|hike|take|slow|go|write|shrink"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("it(self)?|that|this|th[oe]se|them|him|her|us"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("down"), regexp: None, inflected: false },
];
static ANTIPATTERN_258_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("side"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("side"), regexp: None, inflected: false },
];
static ANTIPATTERN_259_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
    AntipatternToken { text: Some("after|to|and|&"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
];
static ANTIPATTERN_260_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
    AntipatternToken { text: Some("frames?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_261_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("side"), regexp: None, inflected: false },
    AntipatternToken { text: Some("by"), regexp: None, inflected: false },
    AntipatternToken { text: Some("side"), regexp: None, inflected: false },
];
static ANTIPATTERN_262_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("april"), regexp: None, inflected: false },
    AntipatternToken { text: Some("fools"), regexp: None, inflected: false },
];
static ANTIPATTERN_263_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("cell"), regexp: None, inflected: false },
    AntipatternToken { text: Some("phones"), regexp: None, inflected: false },
];
static ANTIPATTERN_264_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("energy"), regexp: None, inflected: false },
    AntipatternToken { text: Some("drinks"), regexp: None, inflected: false },
];
static ANTIPATTERN_265_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("them|it"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("when(ever)?|however"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_266_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("black|dress|red"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("dress"), regexp: None, inflected: false },
];
static ANTIPATTERN_267_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("with"), regexp: None, inflected: false },
    AntipatternToken { text: Some("style"), regexp: None, inflected: false },
];
static ANTIPATTERN_268_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("drop"), regexp: None, inflected: false },
    AntipatternToken { text: Some("dead"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z].+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_269_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("drop"), regexp: None, inflected: false },
    AntipatternToken { text: Some("dead"), regexp: None, inflected: false },
    AntipatternToken { text: Some("!"), regexp: None, inflected: false },
];
static ANTIPATTERN_270_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_271_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("i"), regexp: None, inflected: false },
    AntipatternToken { text: Some("/"), regexp: None, inflected: false },
    AntipatternToken { text: Some("os?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_272_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("i"), regexp: None, inflected: false },
    AntipatternToken { text: Some("olympiad"), regexp: None, inflected: false },
];
static ANTIPATTERN_273_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_274_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("i"), regexp: None, inflected: false },
];
static ANTIPATTERN_275_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z].+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z].+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z].+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_276_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[aa]n?|[tt]he"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("my"), regexp: None, inflected: false },
    AntipatternToken { text: Some("choice|chevrolet|chevy|gm"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_277_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z]+i"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_278_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("an?|the"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("'.+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_279_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("an?|the"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[0-9]+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_280_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("an|the"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("(?-i)it"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("girl"), regexp: None, inflected: true },
];
static ANTIPATTERN_281_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("be"), regexp: None, inflected: true },
    AntipatternToken { text: Some("an?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("it|s?he"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_282_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("dot"), regexp: None, inflected: true },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("i"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_283_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("q"), regexp: None, inflected: false },
    AntipatternToken { text: Some("and"), regexp: None, inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_284_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z]+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_285_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("my"), regexp: None, inflected: false },
    AntipatternToken { text: Some("account|profile"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_286_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("my"), regexp: None, inflected: false },
    AntipatternToken { text: Some("lai"), regexp: None, inflected: false },
];
static ANTIPATTERN_287_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("me"), regexp: None, inflected: false },
    AntipatternToken { text: Some("film"), regexp: None, inflected: false },
    AntipatternToken { text: Some("festival"), regexp: None, inflected: false },
];
static ANTIPATTERN_288_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the|an?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("me"), regexp: None, inflected: false },
    AntipatternToken { text: Some("too"), regexp: None, inflected: false },
];
static ANTIPATTERN_289_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the|an?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hands|saints|staff"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_290_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the|an?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
    AntipatternToken { text: Some("inclusive|day|night|time|year|week|month|natural"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_291_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_292_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_293_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("earl"), regexp: None, inflected: false },
    AntipatternToken { text: Some("grey"), regexp: None, inflected: false },
];
static ANTIPATTERN_294_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("allan|allan"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("poe|poe"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_295_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("emmanuel|emmanuel"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("macron|macron"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_296_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("enter"), regexp: None, inflected: true },
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("books"), regexp: None, inflected: false },
];
static ANTIPATTERN_297_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("have"), regexp: None, inflected: true },
    AntipatternToken { text: Some("been"), regexp: None, inflected: false },
    AntipatternToken { text: Some("entered"), regexp: None, inflected: false },
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
];
static ANTIPATTERN_298_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("-|–"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_299_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("er"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
];
static ANTIPATTERN_300_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("er"), regexp: None, inflected: false },
    AntipatternToken { text: Some("/"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
];
static ANTIPATTERN_301_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("throw"), regexp: None, inflected: false },
    AntipatternToken { text: Some("er"), regexp: None, inflected: false },
    AntipatternToken { text: Some(";"), regexp: None, inflected: false },
    AntipatternToken { text: Some("/"), regexp: None, inflected: false },
];
static ANTIPATTERN_302_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("handed"), regexp: None, inflected: false },
    AntipatternToken { text: Some("out|down"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_303_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("have"), regexp: None, inflected: true },
    AntipatternToken { text: Some("not"), regexp: None, inflected: true },
    AntipatternToken { text: Some("even"), regexp: None, inflected: false },
    AntipatternToken { text: Some("handed"), regexp: None, inflected: false },
];
static ANTIPATTERN_304_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("body"), regexp: None, inflected: false },
    AntipatternToken { text: Some("is"), regexp: None, inflected: false },
    AntipatternToken { text: Some("beautiful|wonderful"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_305_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("body"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("water"), regexp: None, inflected: false },
];
static ANTIPATTERN_306_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("body"), regexp: None, inflected: false },
    AntipatternToken { text: Some("tag|element|statement"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_307_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("every"), regexp: None, inflected: false },
    AntipatternToken { text: Some("body|body"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_308_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("each"), regexp: None, inflected: false },
    AntipatternToken { text: Some("others"), regexp: None, inflected: false },
];
static ANTIPATTERN_309_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("at"), regexp: None, inflected: false },
    AntipatternToken { text: Some("all|first|last"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_310_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("at"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("end|best|earliest|drop|bottom|beginning|mid-.*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_311_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("at"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("same"), regexp: None, inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
];
static ANTIPATTERN_312_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
    AntipatternToken { text: Some("its?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_313_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("several|some|many"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("minutes|hours|days|weeks|months|years"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_314_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("first|last"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
];
static ANTIPATTERN_315_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("course"), regexp: None, inflected: false },
];
static ANTIPATTERN_316_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sure"), regexp: None, inflected: false },
    AntipatternToken { text: Some("as"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hell"), regexp: None, inflected: false },
];
static ANTIPATTERN_317_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("course"), regexp: None, inflected: false },
];
static ANTIPATTERN_318_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("so"), regexp: None, inflected: false },
    AntipatternToken { text: Some("far"), regexp: None, inflected: false },
];
static ANTIPATTERN_319_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("fee"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_320_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("fee"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_321_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("prize"), regexp: None, inflected: false },
    AntipatternToken { text: Some("fighter"), regexp: None, inflected: false },
];
static ANTIPATTERN_322_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("more"), regexp: None, inflected: false },
    AntipatternToken { text: Some("th[ea]n"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("one"), regexp: None, inflected: false },
];
static ANTIPATTERN_323_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(".*[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[_.\\[\\(\\/:]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some(".*[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_324_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("?"), regexp: None, inflected: false },
    AntipatternToken { text: Some("php"), regexp: None, inflected: false },
];
static ANTIPATTERN_325_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("doctype"), regexp: None, inflected: false },
    AntipatternToken { text: Some("html"), regexp: None, inflected: false },
];
static ANTIPATTERN_326_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[._\\/<#]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z]+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_327_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z\\/].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[_>]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_328_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("php"), regexp: None, inflected: false },
    AntipatternToken { text: Some("-"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z0-9].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_329_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("couple|number|lot|ton|bunch|shortage|majority|absence|whole|throne|role|offices?|ranks?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_330_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("course"), regexp: None, inflected: false },
];
static ANTIPATTERN_331_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("my|their|her|his|y?our"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("day|week(end)?|month|year"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_332_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
];
static ANTIPATTERN_333_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("love"), regexp: None, inflected: false },
    AntipatternToken { text: Some("actually"), regexp: None, inflected: false },
];
static ANTIPATTERN_334_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("almost"), regexp: None, inflected: false },
    AntipatternToken { text: Some("certainly"), regexp: None, inflected: false },
];
static ANTIPATTERN_335_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("always"), regexp: None, inflected: false },
    AntipatternToken { text: Some("be"), regexp: None, inflected: false },
    AntipatternToken { text: Some("my"), regexp: None, inflected: false },
    AntipatternToken { text: Some("maybe"), regexp: None, inflected: false },
];
static ANTIPATTERN_336_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("be"), regexp: None, inflected: true },
    AntipatternToken { text: Some("actually"), regexp: None, inflected: false },
];
static ANTIPATTERN_337_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("no|on"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("complete|one|purpose"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("certainly|apparently|obviously"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_338_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("this|an?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("course"), regexp: None, inflected: false },
];
static ANTIPATTERN_339_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("and"), regexp: None, inflected: false },
    AntipatternToken { text: Some("fro"), regexp: None, inflected: false },
];
static ANTIPATTERN_340_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("frequently"), regexp: None, inflected: false },
    AntipatternToken { text: Some("asked"), regexp: None, inflected: false },
    AntipatternToken { text: Some("questions?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_341_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("bullets?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_342_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[′'’]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[″\"”]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_343_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("a|the"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("full"), regexp: None, inflected: false },
    AntipatternToken { text: Some("fill"), regexp: None, inflected: false },
];
static ANTIPATTERN_344_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("full|part"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("time|time"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_345_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("full|part"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
    AntipatternToken { text: Some("for|[io]n"), regexp: None, inflected: false },
];
static ANTIPATTERN_346_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("full|part"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
];
static ANTIPATTERN_347_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("with"), regexp: None, inflected: false },
    AntipatternToken { text: Some("respect"), regexp: None, inflected: false },
];
static ANTIPATTERN_348_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("god|allah"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("himself"), regexp: None, inflected: false },
];
static ANTIPATTERN_349_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("my|oh"), regexp: None, inflected: false },
    AntipatternToken { text: Some("days|stars|gods"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_350_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("holiday"), regexp: None, inflected: false },
    AntipatternToken { text: Some("inn"), regexp: None, inflected: false },
];
static ANTIPATTERN_351_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of|no|some|the"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("good"), regexp: None, inflected: false },
];
static ANTIPATTERN_352_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("did"), regexp: None, inflected: false },
    AntipatternToken { text: Some("n't"), regexp: None, inflected: false },
];
static ANTIPATTERN_353_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_354_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("google"), regexp: None, inflected: false },
    AntipatternToken { text: Some("play"), regexp: None, inflected: false },
];
static ANTIPATTERN_355_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("home"), regexp: None, inflected: false },
    AntipatternToken { text: Some("base|page|depot|decor"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_356_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("away"), regexp: None, inflected: false },
    AntipatternToken { text: Some("game.*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_357_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("gr[ae]y|gr[ae]y"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("s|s"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("anatomy|anatomy"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_358_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("so"), regexp: None, inflected: false },
    AntipatternToken { text: Some("long"), regexp: None, inflected: false },
];
static ANTIPATTERN_359_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("sure|free|now|re[ae]l[sz]?|reasons"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_360_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[_.]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("g"), regexp: None, inflected: false },
];
static ANTIPATTERN_361_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("had"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ripple"), regexp: None, inflected: false },
    AntipatternToken { text: Some("effects"), regexp: None, inflected: false },
];
static ANTIPATTERN_362_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("have"), regexp: None, inflected: true },
    AntipatternToken { text: Some("had"), regexp: None, inflected: false },
];
static ANTIPATTERN_363_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("harper|harper"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("'s|'s"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("bazaar|bazaar|magazine|magazine"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_364_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("heat"), regexp: None, inflected: false },
    AntipatternToken { text: Some("problems?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_365_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("t?here"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[ii]t"), regexp: None, inflected: false },
    AntipatternToken { text: Some("have"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_366_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("head"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hunters"), regexp: None, inflected: false },
];
static ANTIPATTERN_367_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("head"), regexp: None, inflected: false },
    AntipatternToken { text: Some("lands"), regexp: None, inflected: false },
    AntipatternToken { text: Some("on|in|at"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_368_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("shot"), regexp: None, inflected: false },
    AntipatternToken { text: Some("off"), regexp: None, inflected: false },
];
static ANTIPATTERN_369_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("shots?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
];
static ANTIPATTERN_370_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("pl[sz]|please|kindly"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("help"), regexp: None, inflected: false },
];
static ANTIPATTERN_371_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("hep"), regexp: None, inflected: false },
    AntipatternToken { text: Some("house|stars|a|b|c|d|e|f|g2?|cahill|cats?|jazz|locks?|neutrinos?|five|navio|group|riots|t"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_372_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("garbage|trash|rubbish|the|a"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("can"), regexp: None, inflected: false },
];
static ANTIPATTERN_373_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[\\[\\(]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("t"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[\\]\\)]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("he"), regexp: None, inflected: false },
];
static ANTIPATTERN_374_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("do|be"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("n't"), regexp: None, inflected: false },
    AntipatternToken { text: Some("s?he|it"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_375_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("do|be"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("s?he|it"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_376_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("huang|liu|zheng"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("he"), regexp: None, inflected: false },
];
static ANTIPATTERN_377_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("("), regexp: None, inflected: false },
    AntipatternToken { text: Some("t"), regexp: None, inflected: false },
    AntipatternToken { text: Some(")"), regexp: None, inflected: false },
    AntipatternToken { text: Some("he"), regexp: None, inflected: false },
];
static ANTIPATTERN_378_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("["), regexp: None, inflected: false },
    AntipatternToken { text: Some("t"), regexp: None, inflected: false },
    AntipatternToken { text: Some("]"), regexp: None, inflected: false },
    AntipatternToken { text: Some("he"), regexp: None, inflected: false },
];
static ANTIPATTERN_379_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("he"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hand"), regexp: None, inflected: false },
    AntipatternToken { text: Some("writewritten"), regexp: None, inflected: true },
];
static ANTIPATTERN_380_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("is|was|does|did|has|had"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("he"), regexp: None, inflected: false },
];
static ANTIPATTERN_381_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("hing"), regexp: None, inflected: false },
    AntipatternToken { text: Some("tong"), regexp: None, inflected: false },
];
static ANTIPATTERN_382_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("shun|lew|donald|john"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("hings?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_383_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z][a-z].+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z][a-z].+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_384_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z]+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[hh]olders?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_385_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("credit|green|blue|ghana|business"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("card"), regexp: None, inflected: false },
];
static ANTIPATTERN_386_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("back"), regexp: None, inflected: false },
    AntipatternToken { text: Some("home"), regexp: None, inflected: false },
    AntipatternToken { text: Some("made"), regexp: None, inflected: false },
];
static ANTIPATTERN_387_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("google|connect|smart"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("home"), regexp: None, inflected: false },
];
static ANTIPATTERN_388_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("home"), regexp: None, inflected: false },
    AntipatternToken { text: Some("made"), regexp: None, inflected: false },
    AntipatternToken { text: Some("me|my|him|his|her|it|them|their|us|our|your?|the|an?|many|no"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_389_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("home"), regexp: None, inflected: false },
    AntipatternToken { text: Some("work"), regexp: None, inflected: false },
    AntipatternToken { text: Some("stations?|spaces?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_390_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("stay"), regexp: None, inflected: true },
    AntipatternToken { text: Some("at"), regexp: None, inflected: false },
    AntipatternToken { text: Some("home"), regexp: None, inflected: false },
];
static ANTIPATTERN_391_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("work"), regexp: None, inflected: true },
    AntipatternToken { text: Some("from|at"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("home"), regexp: None, inflected: false },
];
static ANTIPATTERN_392_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("father|baby"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("hood"), regexp: None, inflected: false },
];
static ANTIPATTERN_393_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(":"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_394_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("response"), regexp: None, inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
];
static ANTIPATTERN_395_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("dr"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("house"), regexp: None, inflected: false },
];
static ANTIPATTERN_396_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("house"), regexp: None, inflected: false },
    AntipatternToken { text: Some("work"), regexp: None, inflected: false },
];
static ANTIPATTERN_397_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("earth"), regexp: None, inflected: false },
    AntipatternToken { text: Some("house"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hold"), regexp: None, inflected: false },
];
static ANTIPATTERN_398_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("guest"), regexp: None, inflected: false },
    AntipatternToken { text: Some("house"), regexp: None, inflected: false },
    AntipatternToken { text: Some("rooms?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_399_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z][a-z].*reason|ask|question|wonder|know"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("hwy"), regexp: None, inflected: false },
];
static ANTIPATTERN_400_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("hwy"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+.*|[b-h]|[j-z]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_401_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("hwy"), regexp: None, inflected: false },
    AntipatternToken { text: Some("n[ro]|#"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+.*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_402_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("reason|ask|question|wonder|know"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hwy"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
];
static ANTIPATTERN_403_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the|our|their|another|a|follow|drive|on"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("hwy"), regexp: None, inflected: false },
];
static ANTIPATTERN_404_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some(".+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_405_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("imitation"), regexp: None, inflected: true },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_406_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("msn"), regexp: None, inflected: false },
    AntipatternToken { text: Some("im"), regexp: None, inflected: false },
];
static ANTIPATTERN_407_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("im"), regexp: None, inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
];
static ANTIPATTERN_408_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("word"), regexp: None, inflected: false },
    AntipatternToken { text: Some("im"), regexp: None, inflected: false },
];
static ANTIPATTERN_409_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("capital"), regexp: None, inflected: false },
    AntipatternToken { text: Some("and"), regexp: None, inflected: false },
    AntipatternToken { text: Some("interest"), regexp: None, inflected: false },
];
static ANTIPATTERN_410_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("by"), regexp: None, inflected: false },
    AntipatternToken { text: Some("next|last"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_411_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("by"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("way"), regexp: None, inflected: false },
];
static ANTIPATTERN_412_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("so"), regexp: None, inflected: false },
    AntipatternToken { text: Some("long"), regexp: None, inflected: false },
];
static ANTIPATTERN_413_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("sure|free|now|re[ae]l[sz]?|reasons"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_414_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("interest"), regexp: None, inflected: false },
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
];
static ANTIPATTERN_415_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("course|usd|\\$|euro?|€|more|less"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_416_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_417_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("trouble"), regexp: None, inflected: false },
    AntipatternToken { text: Some("shoot(ing|s)?|making"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_418_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("course"), regexp: None, inflected: false },
];
static ANTIPATTERN_419_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("facebook|twitter|pinterest|wikipedia|amazon|linkedin|yahoo|blogger|youtube|instagram|wordpress|reddit|tiktok"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_420_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("twitter"), regexp: None, inflected: false },
    AntipatternToken { text: Some("mentions|replies|tweets"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_421_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("you|them|him|her"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_422_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("1|2"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("[0-9]{3}"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_423_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("participation"), regexp: None, inflected: false },
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
];
static ANTIPATTERN_424_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("terms"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_425_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("internet"), regexp: None, inflected: false },
    AntipatternToken { text: Some("age"), regexp: None, inflected: false },
];
static ANTIPATTERN_426_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("internet|website|tv|television|web"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_427_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("on"), regexp: None, inflected: false },
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
];
static ANTIPATTERN_428_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("long|short"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("term|run"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_429_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("an?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("on|in"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("website"), regexp: None, inflected: false },
];
static ANTIPATTERN_430_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("kind|sort|type|style|category"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_431_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("windows|macos|linux|ubuntu"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_432_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("windows|macos|linux|ubuntu"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some(".*[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_433_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("there"), regexp: None, inflected: false },
    AntipatternToken { text: Some("are"), regexp: None, inflected: false },
];
static ANTIPATTERN_434_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("challenge|question|concern"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("is"), regexp: None, inflected: false },
];
static ANTIPATTERN_435_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bad"), regexp: None, inflected: false },
    AntipatternToken { text: Some("manners"), regexp: None, inflected: false },
];
static ANTIPATTERN_436_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("good|bad|excellent|huge|significant"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("news"), regexp: None, inflected: false },
];
static ANTIPATTERN_437_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("be"), regexp: None, inflected: true },
    AntipatternToken { text: Some("on"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_438_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("be"), regexp: None, inflected: true },
    AntipatternToken { text: Some("not"), regexp: None, inflected: true },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_439_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_440_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("think|hope|doubt|assume"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_441_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("worth"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_442_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[ds]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_443_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("'[ds]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_444_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("use"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
    AntipatternToken { text: Some("act"), regexp: None, inflected: false },
];
static ANTIPATTERN_445_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("and|or|but"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("doubt|think|hope|believe|assume|say"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_446_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("buy"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
    AntipatternToken { text: Some("now"), regexp: None, inflected: false },
    AntipatternToken { text: Some("buttons?|links?|teasers?|ctas?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_447_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("dam[nm]?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_448_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("general"), regexp: None, inflected: false },
];
static ANTIPATTERN_449_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("sync"), regexp: None, inflected: false },
];
static ANTIPATTERN_450_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
    AntipatternToken { text: Some("close"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_451_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
    AntipatternToken { text: Some("need"), regexp: None, inflected: false },
    AntipatternToken { text: Some("not"), regexp: None, inflected: false },
];
static ANTIPATTERN_452_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
    AntipatternToken { text: Some("over"), regexp: None, inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
];
static ANTIPATTERN_453_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
    AntipatternToken { text: Some("perfect"), regexp: None, inflected: false },
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
];
static ANTIPATTERN_454_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pretty"), regexp: None, inflected: false },
    AntipatternToken { text: Some("much"), regexp: None, inflected: false },
];
static ANTIPATTERN_455_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
    AntipatternToken { text: Some("for|to"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_456_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("jump"), regexp: None, inflected: false },
    AntipatternToken { text: Some("start(s|ed)"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_457_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("let"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_458_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("support"), regexp: None, inflected: false },
    AntipatternToken { text: Some("for|from"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_459_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("then"), regexp: None, inflected: false },
    AntipatternToken { text: Some("so"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
    AntipatternToken { text: Some("be"), regexp: None, inflected: false },
];
static ANTIPATTERN_460_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("think|hope"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
    AntipatternToken { text: Some("please"), regexp: None, inflected: false },
];
static ANTIPATTERN_461_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("do"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_462_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("worth"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_463_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\p{lu}.*|bywhat"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("ive"), regexp: None, inflected: false },
];
static ANTIPATTERN_464_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("worry"), regexp: None, inflected: false },
    AntipatternToken { text: Some("free"), regexp: None, inflected: false },
];
static ANTIPATTERN_465_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("i"), regexp: None, inflected: false },
];
static ANTIPATTERN_466_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(".*[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_467_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("personal"), regexp: None, inflected: false },
    AntipatternToken { text: Some("computer"), regexp: None, inflected: false },
];
static ANTIPATTERN_468_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("#"), regexp: None, inflected: false },
    AntipatternToken { text: Some("japan"), regexp: None, inflected: false },
];
static ANTIPATTERN_469_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("does|did|do"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("n't"), regexp: None, inflected: false },
    AntipatternToken { text: Some("japan"), regexp: None, inflected: false },
];
static ANTIPATTERN_470_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("japan"), regexp: None, inflected: false },
    AntipatternToken { text: Some("black"), regexp: None, inflected: false },
];
static ANTIPATTERN_471_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("jennifer|jennifer"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("aniston|aniston"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_472_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bar"), regexp: None, inflected: false },
    AntipatternToken { text: Some("keepers"), regexp: None, inflected: false },
];
static ANTIPATTERN_473_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("zoo"), regexp: None, inflected: false },
    AntipatternToken { text: Some("keeper"), regexp: None, inflected: false },
];
static ANTIPATTERN_474_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bar"), regexp: None, inflected: false },
    AntipatternToken { text: Some("keepers"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("friend"), regexp: None, inflected: false },
];
static ANTIPATTERN_475_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("by"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
];
static ANTIPATTERN_476_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
    AntipatternToken { text: Some("after"), regexp: None, inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
];
static ANTIPATTERN_477_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("lane"), regexp: None, inflected: false },
    AntipatternToken { text: Some("keep"), regexp: None, inflected: false },
    AntipatternToken { text: Some("assist"), regexp: None, inflected: false },
];
static ANTIPATTERN_478_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("stokes"), regexp: None, inflected: false },
    AntipatternToken { text: Some("drag"), regexp: None, inflected: false },
];
static ANTIPATTERN_479_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hurry"), regexp: None, inflected: false },
];
static ANTIPATTERN_480_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("device"), regexp: None, inflected: false },
    AntipatternToken { text: Some("type"), regexp: None, inflected: false },
];
static ANTIPATTERN_481_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the|some|own"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("kind"), regexp: None, inflected: false },
];
static ANTIPATTERN_482_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("law"), regexp: None, inflected: false },
    AntipatternToken { text: Some("suits?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("the|an?|his|her|their|my|y?our"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_483_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("la|la"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("paz|paz"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_484_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("del?|que"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
];
static ANTIPATTERN_485_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-zöüäßa-zöäü].+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("("), regexp: None, inflected: false },
];
static ANTIPATTERN_486_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-zöüäßa-zöäü].+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[\\.:_#;]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_487_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("env"), regexp: None, inflected: false },
];
static ANTIPATTERN_488_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("length"), regexp: None, inflected: false },
];
static ANTIPATTERN_489_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(".*[a-z]$"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_490_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("@.+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some(".+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_491_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("seven"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("one"), regexp: None, inflected: false },
];
static ANTIPATTERN_492_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("cargo"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("one"), regexp: None, inflected: false },
];
static ANTIPATTERN_493_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("covet"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("pics"), regexp: None, inflected: false },
];
static ANTIPATTERN_494_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("e"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("commerce|mails?|mobility|bikes?|go"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_495_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("fritz"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("box"), regexp: None, inflected: false },
];
static ANTIPATTERN_496_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("java"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("util"), regexp: None, inflected: false },
];
static ANTIPATTERN_497_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("math"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("rand(om)?|sin|radians?|pow|floor|round|ceil|pi|abs|log|sign"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_498_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("notion"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("so"), regexp: None, inflected: false },
];
static ANTIPATTERN_499_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("s"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("oliver"), regexp: None, inflected: false },
];
static ANTIPATTERN_500_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("str"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
];
static ANTIPATTERN_501_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("stud"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("ip"), regexp: None, inflected: false },
];
static ANTIPATTERN_502_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("window|global|this|urllib"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
];
static ANTIPATTERN_503_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("free|school"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_504_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("lehman"), regexp: None, inflected: false },
    AntipatternToken { text: Some("brothers|college"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_505_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("leroy|leroy"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("sané|sané"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_506_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("or"), regexp: None, inflected: false },
    AntipatternToken { text: Some("less"), regexp: None, inflected: false },
    AntipatternToken { text: Some("number"), regexp: None, inflected: false },
];
static ANTIPATTERN_507_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("by|back"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("then"), regexp: None, inflected: false },
];
static ANTIPATTERN_508_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("one"), regexp: None, inflected: false },
    AntipatternToken { text: Some("more"), regexp: None, inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
    AntipatternToken { text: Some("then"), regexp: None, inflected: false },
];
static ANTIPATTERN_509_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("then"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\p{p}"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_510_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("then"), regexp: None, inflected: false },
    AntipatternToken { text: Some("because|but|although|,|since|if"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_511_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("none"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("lest"), regexp: None, inflected: false },
];
static ANTIPATTERN_512_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("my|opportunity"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("try|to"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_513_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("though"), regexp: None, inflected: false },
    AntipatternToken { text: Some("!"), regexp: None, inflected: false },
];
static ANTIPATTERN_514_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("let"), regexp: None, inflected: false },
    AntipatternToken { text: Some("i|he"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("be"), regexp: None, inflected: false },
];
static ANTIPATTERN_515_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("life"), regexp: None, inflected: false },
    AntipatternToken { text: Some("guards|time|savers?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_516_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("royal"), regexp: None, inflected: false },
    AntipatternToken { text: Some("life"), regexp: None, inflected: false },
];
static ANTIPATTERN_517_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
    AntipatternToken { text: Some("frames?|sheets?|lapses?|periods?|spans?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_518_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("gas"), regexp: None, inflected: false },
    AntipatternToken { text: Some("light"), regexp: None, inflected: false },
];
static ANTIPATTERN_519_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("gas"), regexp: None, inflected: false },
    AntipatternToken { text: Some("light"), regexp: None, inflected: false },
    AntipatternToken { text: Some("company"), regexp: None, inflected: false },
];
static ANTIPATTERN_520_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("street"), regexp: None, inflected: false },
];
static ANTIPATTERN_521_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("lights"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the|an?|my|y?our|their|his|her|its|these|those|this"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_522_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("lights?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
];
static ANTIPATTERN_523_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("main|wall"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("street"), regexp: None, inflected: false },
];
static ANTIPATTERN_524_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("association"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("oil"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pipe"), regexp: None, inflected: false },
];
static ANTIPATTERN_525_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("delaware"), regexp: None, inflected: false },
    AntipatternToken { text: Some("coast"), regexp: None, inflected: false },
    AntipatternToken { text: Some("line"), regexp: None, inflected: false },
];
static ANTIPATTERN_526_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("east|west"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("coast"), regexp: None, inflected: false },
];
static ANTIPATTERN_527_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("houston|olympic|canyon|oasis|panhandle|foothills|williams"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("pipe"), regexp: None, inflected: false },
    AntipatternToken { text: Some("lines?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_528_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("lines"), regexp: None, inflected: false },
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
];
static ANTIPATTERN_529_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("pipe"), regexp: None, inflected: false },
    AntipatternToken { text: Some("line"), regexp: None, inflected: false },
    AntipatternToken { text: Some("company|corp(oration)?|co"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_530_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("long"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ago"), regexp: None, inflected: false },
];
static ANTIPATTERN_531_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("long|long"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("island|island"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[ii]ced|iced"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[tt]ea|tea"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_532_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("los|los"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("angeles|angeles"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_533_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("cerro"), regexp: None, inflected: false },
    AntipatternToken { text: Some("de"), regexp: None, inflected: false },
    AntipatternToken { text: Some("los"), regexp: None, inflected: false },
];
static ANTIPATTERN_534_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("down"), regexp: None, inflected: false },
];
static ANTIPATTERN_535_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("off"), regexp: None, inflected: false },
];
static ANTIPATTERN_536_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("out"), regexp: None, inflected: false },
];
static ANTIPATTERN_537_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
];
static ANTIPATTERN_538_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(".*[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("mabey"), regexp: None, inflected: false },
];
static ANTIPATTERN_539_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mabey"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hire|group|logistic"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_540_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("_"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("mabey"), regexp: None, inflected: false },
];
static ANTIPATTERN_541_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mrs?|dr|prof|miss|sr|jr|[a-z]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("mabey"), regexp: None, inflected: false },
];
static ANTIPATTERN_542_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mrs?|dr|prof(essor)?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("mah"), regexp: None, inflected: false },
];
static ANTIPATTERN_543_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mah"), regexp: None, inflected: false },
    AntipatternToken { text: Some("jongg?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_544_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("film|movie"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("makers?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_545_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("manor"), regexp: None, inflected: false },
    AntipatternToken { text: Some("houses?|homes?|near(by)?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_546_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("various"), regexp: None, inflected: false },
];
static ANTIPATTERN_547_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("as"), regexp: None, inflected: false },
    AntipatternToken { text: Some("many"), regexp: None, inflected: false },
    AntipatternToken { text: Some("know"), regexp: None, inflected: false },
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
];
static ANTIPATTERN_548_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("due"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_549_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("familiar"), regexp: None, inflected: false },
    AntipatternToken { text: Some("with"), regexp: None, inflected: false },
];
static ANTIPATTERN_550_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("few"), regexp: None, inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
];
static ANTIPATTERN_551_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("hall"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("famers"), regexp: None, inflected: false },
];
static ANTIPATTERN_552_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("many"), regexp: None, inflected: false },
    AntipatternToken { text: Some("care"), regexp: None, inflected: false },
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
];
static ANTIPATTERN_553_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("many"), regexp: None, inflected: false },
    AntipatternToken { text: Some("debate"), regexp: None, inflected: false },
    AntipatternToken { text: Some("on"), regexp: None, inflected: false },
];
static ANTIPATTERN_554_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("many"), regexp: None, inflected: false },
    AntipatternToken { text: Some("fold"), regexp: None, inflected: false },
];
static ANTIPATTERN_555_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("many"), regexp: None, inflected: false },
    AntipatternToken { text: Some("miss"), regexp: None, inflected: false },
    AntipatternToken { text: Some("out"), regexp: None, inflected: false },
];
static ANTIPATTERN_556_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("many"), regexp: None, inflected: false },
    AntipatternToken { text: Some("regard"), regexp: None, inflected: false },
    AntipatternToken { text: Some("h(?:i[ms]|er)|its?|th(is|em)"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_557_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("many"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("many"), regexp: None, inflected: false },
];
static ANTIPATTERN_558_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("many"), regexp: None, inflected: false },
    AntipatternToken { text: Some("wonder"), regexp: None, inflected: false },
    AntipatternToken { text: Some("if"), regexp: None, inflected: false },
    AntipatternToken { text: Some("this|that|there|the|an?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_559_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("many|several|few"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("end"), regexp: None, inflected: false },
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
];
static ANTIPATTERN_560_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("many|several|few"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("like"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_561_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("many|several|few"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("sort|kind|couple"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("off?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_562_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("many|several|few|various"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("follow"), regexp: None, inflected: false },
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
];
static ANTIPATTERN_563_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("reality"), regexp: None, inflected: false },
    AntipatternToken { text: Some("show"), regexp: None, inflected: false },
    AntipatternToken { text: Some("contestants"), regexp: None, inflected: false },
];
static ANTIPATTERN_564_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("stay"), regexp: None, inflected: false },
    AntipatternToken { text: Some("at"), regexp: None, inflected: false },
    AntipatternToken { text: Some("home"), regexp: None, inflected: false },
];
static ANTIPATTERN_565_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("that|which|who|what"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("many"), regexp: None, inflected: false },
    AntipatternToken { text: Some("fear"), regexp: None, inflected: false },
];
static ANTIPATTERN_566_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("various"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z].*"), regexp: None, inflected: false },
];
static ANTIPATTERN_567_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("various"), regexp: None, inflected: false },
    AntipatternToken { text: Some("history"), regexp: None, inflected: false },
];
static ANTIPATTERN_568_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("various"), regexp: None, inflected: false },
    AntipatternToken { text: Some("wild"), regexp: None, inflected: false },
    AntipatternToken { text: Some("game"), regexp: None, inflected: false },
];
static ANTIPATTERN_569_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("stunt|weather"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_570_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("high|low"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("tide"), regexp: None, inflected: false },
    AntipatternToken { text: Some("marks?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_571_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("marks"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the|an?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_572_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("trade"), regexp: None, inflected: false },
    AntipatternToken { text: Some("marks"), regexp: None, inflected: false },
    AntipatternToken { text: Some("act"), regexp: None, inflected: false },
];
static ANTIPATTERN_573_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("s"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ma"), regexp: None, inflected: false },
];
static ANTIPATTERN_574_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("i"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ma"), regexp: None, inflected: false },
];
static ANTIPATTERN_575_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\.|_"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("ma"), regexp: None, inflected: false },
];
static ANTIPATTERN_576_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("dr|mrs?|prof"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("ma"), regexp: None, inflected: false },
];
static ANTIPATTERN_577_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("jack"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ma"), regexp: None, inflected: false },
];
static ANTIPATTERN_578_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ma"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_579_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ma"), regexp: None, inflected: false },
    AntipatternToken { text: Some("femme|wan|la|non|anand|bar?ker|bufang|boyong|buqing|chao|cheri[eéè]s?|qi|chengyuan|zh.*|ch[uiae]ng|dong.*|dehua|tianyu|qing|huateng|belle|yve|yvé"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_580_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ma"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ma"), regexp: None, inflected: false },
];
static ANTIPATTERN_581_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("my|y?our|his|her|their|its|the|an?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("ma"), regexp: None, inflected: false },
];
static ANTIPATTERN_582_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sur|vous|pour"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("ma"), regexp: None, inflected: false },
];
static ANTIPATTERN_583_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("no"), regexp: None, inflected: false },
    AntipatternToken { text: Some("longer"), regexp: None, inflected: false },
];
static ANTIPATTERN_584_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("meeds"), regexp: None, inflected: false },
    AntipatternToken { text: Some("llp|lake"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_585_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("mi"), regexp: None, inflected: false },
];
static ANTIPATTERN_586_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(".*[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("mi"), regexp: None, inflected: false },
];
static ANTIPATTERN_587_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(".*\\d.*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("mi"), regexp: None, inflected: false },
];
static ANTIPATTERN_588_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("/|\\?|="), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("mi"), regexp: None, inflected: false },
];
static ANTIPATTERN_589_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mi"), regexp: None, inflected: false },
    AntipatternToken { text: Some("home"), regexp: None, inflected: false },
];
static ANTIPATTERN_590_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[;,:]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("mi"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[;,:]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_591_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mi"), regexp: None, inflected: false },
    AntipatternToken { text: Some(".*[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_592_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mi"), regexp: None, inflected: false },
    AntipatternToken { text: Some("=|\\/"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_593_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mi"), regexp: None, inflected: false },
    AntipatternToken { text: Some("estas|amig[ao]|rancho|casa"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_594_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mi"), regexp: None, inflected: false },
    AntipatternToken { text: Some("familia"), regexp: None, inflected: false },
];
static ANTIPATTERN_595_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("mi"), regexp: None, inflected: false },
];
static ANTIPATTERN_596_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("xiaomi"), regexp: None, inflected: false },
    AntipatternToken { text: Some("mi"), regexp: None, inflected: false },
];
static ANTIPATTERN_597_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("miami"), regexp: None, inflected: false },
    AntipatternToken { text: Some("dade"), regexp: None, inflected: false },
    AntipatternToken { text: Some("college|fc"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_598_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mozart|bach|beethoven|boccherini|elaborate|stately|trumpet|orchestra"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("minuets?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_599_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("minuets?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("orchestra|keyboard|piano|flute|harpsichord|guitar|violin"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_600_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("minuets?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("piano|music|composed|style"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_601_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("prior"), regexp: None, inflected: false },
    AntipatternToken { text: Some("general|executive"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_602_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("due"), regexp: None, inflected: false },
    AntipatternToken { text: Some("east|west"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_603_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("due"), regexp: None, inflected: false },
    AntipatternToken { text: Some("every|each"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_604_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("miss"), regexp: None, inflected: false },
    AntipatternToken { text: Some("spelling"), regexp: None, inflected: false },
];
static ANTIPATTERN_605_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("motor"), regexp: None, inflected: false },
    AntipatternToken { text: Some("trucks?|cycles?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("company|ltd|corp(oration)?|inc|llc"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_606_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("how|this|that"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("much"), regexp: None, inflected: false },
    AntipatternToken { text: Some("money"), regexp: None, inflected: false },
];
static ANTIPATTERN_607_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("too|so+|as"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("much"), regexp: None, inflected: false },
    AntipatternToken { text: Some("money"), regexp: None, inflected: false },
];
static ANTIPATTERN_608_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("be"), regexp: None, inflected: false },
    AntipatternToken { text: Some("be"), regexp: None, inflected: false },
];
static ANTIPATTERN_609_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("an?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("absolute|total"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("must"), regexp: None, inflected: false },
];
static ANTIPATTERN_610_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[0-9].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("mu"), regexp: None, inflected: false },
];
static ANTIPATTERN_611_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mu"), regexp: None, inflected: false },
    AntipatternToken { text: Some("shu"), regexp: None, inflected: false },
];
static ANTIPATTERN_612_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("-"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ne"), regexp: None, inflected: false },
];
static ANTIPATTERN_613_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("je"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ne"), regexp: None, inflected: false },
    AntipatternToken { text: Some("sais"), regexp: None, inflected: false },
    AntipatternToken { text: Some("quoi"), regexp: None, inflected: false },
];
static ANTIPATTERN_614_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ne"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("er"), regexp: None, inflected: false },
];
static ANTIPATTERN_615_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ne"), regexp: None, inflected: false },
    AntipatternToken { text: Some("plus"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ultra"), regexp: None, inflected: false },
];
static ANTIPATTERN_616_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("nee"), regexp: None, inflected: false },
];
static ANTIPATTERN_617_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("aa?ron|adam|kevin"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("nee"), regexp: None, inflected: false },
];
static ANTIPATTERN_618_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("nee"), regexp: None, inflected: false },
    AntipatternToken { text: Some("brothers"), regexp: None, inflected: false },
];
static ANTIPATTERN_619_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("neither"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_620_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("news"), regexp: None, inflected: false },
    AntipatternToken { text: Some("group"), regexp: None, inflected: false },
];
static ANTIPATTERN_621_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("news"), regexp: None, inflected: false },
    AntipatternToken { text: Some("letter"), regexp: None, inflected: false },
];
static ANTIPATTERN_622_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("fox|mercury|cnn|bbc|nyt|abc|msnbc|nbc|rt|google|msn|yahoo"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("news"), regexp: None, inflected: false },
];
static ANTIPATTERN_623_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("new"), regexp: None, inflected: false },
    AntipatternToken { text: Some("guinea"), regexp: None, inflected: false },
];
static ANTIPATTERN_624_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("nit"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pick"), regexp: None, inflected: true },
];
static ANTIPATTERN_625_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("towers"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("brahma|hanoi"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_626_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("that"), regexp: None, inflected: false },
];
static ANTIPATTERN_627_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("gods?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("that"), regexp: None, inflected: false },
    AntipatternToken { text: Some("is|'s"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_628_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("that"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_629_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("that"), regexp: None, inflected: false },
    AntipatternToken { text: Some("is|'s"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("not"), regexp: None, inflected: true },
    AntipatternToken { text: Some("wh(y|en)"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_630_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("these|some"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("days|times"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_631_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("coleridge"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hundred"), regexp: None, inflected: false },
];
static ANTIPATTERN_632_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("million"), regexp: None, inflected: false },
    AntipatternToken { text: Some("dollar"), regexp: None, inflected: false },
    AntipatternToken { text: Some("baby"), regexp: None, inflected: false },
];
static ANTIPATTERN_633_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("six"), regexp: None, inflected: false },
    AntipatternToken { text: Some("million"), regexp: None, inflected: false },
    AntipatternToken { text: Some("dollar"), regexp: None, inflected: false },
    AntipatternToken { text: Some("man"), regexp: None, inflected: false },
];
static ANTIPATTERN_634_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("thousand"), regexp: None, inflected: false },
    AntipatternToken { text: Some("island"), regexp: None, inflected: false },
];
static ANTIPATTERN_635_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("thousand"), regexp: None, inflected: false },
    AntipatternToken { text: Some("oaks|islands"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_636_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("million"), regexp: None, inflected: false },
];
static ANTIPATTERN_637_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[x]+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("dozen|hundred|thousand|[bm]illion"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_638_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("couple|per|several"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("dozen|hundred|thousand|[bm]illion"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_639_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("dozen|hundred|thousand|[bm]illion"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_640_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("hundred"), regexp: None, inflected: false },
    AntipatternToken { text: Some("dozen|thousand|[bm]illion"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_641_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("jack"), regexp: None, inflected: false },
    AntipatternToken { text: Some("billion"), regexp: None, inflected: false },
];
static ANTIPATTERN_642_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mrs?|miss|dr|professor|doctor|[$€£¥฿₹₿]?\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("dozen|hundred|thousand|[bm]illion"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_643_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|twenty|thirty|fourty|fifty|sixty|seventy|eighty|ninety"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("dozen|hundred|thousand|[bm]illion"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_644_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bank"), regexp: None, inflected: false },
    AntipatternToken { text: Some("notes"), regexp: None, inflected: false },
    AntipatternToken { text: Some("act"), regexp: None, inflected: false },
];
static ANTIPATTERN_645_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("day|week|month|year|event|weekend|night"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("be|seem"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("not"), regexp: None, inflected: false },
    AntipatternToken { text: Some("longer"), regexp: None, inflected: false },
];
static ANTIPATTERN_646_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ai|do"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("n't"), regexp: None, inflected: false },
];
static ANTIPATTERN_647_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("if"), regexp: None, inflected: false },
    AntipatternToken { text: Some("not|yes|(im)?possible|any(thing)?|ever"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("then|please"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_648_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
    AntipatternToken { text: Some("know"), regexp: None, inflected: false },
    AntipatternToken { text: Some("what"), regexp: None, inflected: false },
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
];
static ANTIPATTERN_649_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("at"), regexp: None, inflected: false },
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
];
static ANTIPATTERN_650_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[:=]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("”"), regexp: None, inflected: false },
];
static ANTIPATTERN_651_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("”"), regexp: None, inflected: false },
];
static ANTIPATTERN_652_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("[\\p{ll}\\p{lu}]\\p{ll}*[0-9]\\p{ll}+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_653_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("axle|crunchy|salty|nougat|areca|oat|pine|beech|betel|brazil|cashew|kola|macadamia|monkey|palm|barrel|cage|clip-on|collar|coupling|flange|hex|split|wing"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("nut"), regexp: None, inflected: false },
];
static ANTIPATTERN_654_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("butter"), regexp: None, inflected: false },
    AntipatternToken { text: Some("nut"), regexp: None, inflected: false },
];
static ANTIPATTERN_655_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("date"), regexp: None, inflected: false },
    AntipatternToken { text: Some("nut"), regexp: None, inflected: false },
];
static ANTIPATTERN_656_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("nut"), regexp: None, inflected: false },
    AntipatternToken { text: Some("christmas"), regexp: None, inflected: false },
    AntipatternToken { text: Some("trees?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_657_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("nut"), regexp: None, inflected: false },
    AntipatternToken { text: Some("free|based|glazed"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_658_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("nut"), regexp: None, inflected: false },
    AntipatternToken { text: Some("in(side)?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_659_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("nut"), regexp: None, inflected: false },
    AntipatternToken { text: Some("not|dispens.*|parts?|crack.*|mix.*|cap.*|nougat|creme|munch.*|granola|sack|choc.*|eat.*|shop.*|store.*|filled|ink.*|out|on|off|heads?|butter.*|.*milk|drivers?|wrench(es)?|width|height|kits?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_660_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("do|does|did"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("s?he|you|we|they|i"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("mean"), regexp: None, inflected: false },
    AntipatternToken { text: Some("by"), regexp: None, inflected: false },
];
static ANTIPATTERN_661_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("hand"), regexp: None, inflected: false },
    AntipatternToken { text: Some("full?|cuff.*|bag.*|craft.*|luggage|brakes?|wash.*|down|jobs?|pick.*|knit.*|writ.*|over|puppets?|signals?|saniti.*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_662_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("hand"), regexp: None, inflected: false },
    AntipatternToken { text: Some("in|to"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("hand|glove"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_663_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("jump|pay|drop"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("off"), regexp: None, inflected: false },
];
static ANTIPATTERN_664_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("s[iau]ngs?|rap(ped|ping|s)?|rhym(e|es|ed|ing)"), regexp: None, inflected: false },
    AntipatternToken { text: Some("off"), regexp: None, inflected: false },
];
static ANTIPATTERN_665_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
    AntipatternToken { text: Some("process"), regexp: None, inflected: false },
];
static ANTIPATTERN_666_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[_.]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("no|ok"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_667_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ones"), regexp: None, inflected: false },
    AntipatternToken { text: Some("left"), regexp: None, inflected: false },
    AntipatternToken { text: Some("behind"), regexp: None, inflected: false },
];
static ANTIPATTERN_668_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("no"), regexp: None, inflected: false },
    AntipatternToken { text: Some("one"), regexp: None, inflected: false },
];
static ANTIPATTERN_669_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("another|no"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("one"), regexp: None, inflected: false },
];
static ANTIPATTERN_670_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ore"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bodies|body|mountains|carriers?|mining|mines?|,|\\.|!|\\?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_671_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sole"), regexp: None, inflected: false },
    AntipatternToken { text: Some("24"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ore"), regexp: None, inflected: false },
];
static ANTIPATTERN_672_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("1|one"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_673_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("1|one"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("(wo)?men"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_674_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("1|one"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("bedrooms|cycles|does|folks|gets|ins|pagers?|processes|sequences|shots?|timers?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_675_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("1|one"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("hours|days|nights|weeks|months|years"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("time|worth"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_676_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("1|one"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("ones"), regexp: None, inflected: false },
];
static ANTIPATTERN_677_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("1|one"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("to|on"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("1|one"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_678_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z]{3,4}"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("1|one"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_679_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[\\.\\/:#\\+-–−xx]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("1"), regexp: None, inflected: false },
];
static ANTIPATTERN_680_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("1|one"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_681_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
    AntipatternToken { text: Some("that"), regexp: None, inflected: false },
    AntipatternToken { text: Some("1|one"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("needs"), regexp: None, inflected: false },
];
static ANTIPATTERN_682_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("one"), regexp: None, inflected: false },
];
static ANTIPATTERN_683_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(".*[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("1|one"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_684_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(".*\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[\\.,]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("1"), regexp: None, inflected: false },
];
static ANTIPATTERN_685_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("on"), regexp: None, inflected: false },
    AntipatternToken { text: Some("behalf"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_686_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("on"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("day|night"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_687_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("opining"), regexp: None, inflected: false },
    AntipatternToken { text: Some("on|that|s?he|we|they|it|there"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_688_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("@.+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some(".+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("can|ca|have|should|could|will|wo|won|may|might"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("not|n't"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_689_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("not"), regexp: None, inflected: false },
    AntipatternToken { text: Some("only"), regexp: None, inflected: false },
    AntipatternToken { text: Some("can|ca|have|should|could|will|wo|may|might"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("not|n't"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_690_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("wise"), regexp: None, inflected: false },
    AntipatternToken { text: Some("old"), regexp: None, inflected: false },
    AntipatternToken { text: Some("m[ae]n|guys?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_691_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("be|do|have"), regexp: None, inflected: false },
    AntipatternToken { text: Some("out"), regexp: None, inflected: false },
];
static ANTIPATTERN_692_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("do"), regexp: None, inflected: false },
    AntipatternToken { text: Some("overdue"), regexp: None, inflected: false },
];
static ANTIPATTERN_693_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\p{p}"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("o"), regexp: None, inflected: false },
];
static ANTIPATTERN_694_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("papua"), regexp: None, inflected: false },
    AntipatternToken { text: Some("new"), regexp: None, inflected: false },
    AntipatternToken { text: Some("guinean?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_695_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("a|some|any|the|lesser"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("degree|extent"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_696_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("get"), regexp: None, inflected: true },
    AntipatternToken { text: Some("passed"), regexp: None, inflected: false },
    AntipatternToken { text: Some("an?"), regexp: None, inflected: false },
];
static ANTIPATTERN_697_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("get"), regexp: None, inflected: true },
    AntipatternToken { text: Some("passed"), regexp: None, inflected: false },
    AntipatternToken { text: Some("back"), regexp: None, inflected: false },
];
static ANTIPATTERN_698_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("messages?|legislations?|bills?|laws?|data|parameters?|variables?|arguments?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("get"), regexp: None, inflected: true },
    AntipatternToken { text: Some("passed"), regexp: None, inflected: false },
];
static ANTIPATTERN_699_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("passed"), regexp: None, inflected: false },
    AntipatternToken { text: Some("or"), regexp: None, inflected: false },
    AntipatternToken { text: Some("rejected|failed"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_700_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("long"), regexp: None, inflected: false },
    AntipatternToken { text: Some("past"), regexp: None, inflected: false },
];
static ANTIPATTERN_701_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("not"), regexp: None, inflected: false },
    AntipatternToken { text: Some("past"), regexp: None, inflected: false },
];
static ANTIPATTERN_702_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
    AntipatternToken { text: Some("zone|span|limit|period|expression|interval|difference"), regexp: Some("yes"), inflected: true },
];
static ANTIPATTERN_703_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("payed"), regexp: None, inflected: false },
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
];
static ANTIPATTERN_704_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("gives|lets|makes"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("us|you|them"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_705_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("how"), regexp: None, inflected: false },
    AntipatternToken { text: Some("many"), regexp: None, inflected: false },
    AntipatternToken { text: Some("people"), regexp: None, inflected: false },
];
static ANTIPATTERN_706_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("people"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_707_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("people"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ages"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d.*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_708_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("seems"), regexp: None, inflected: false },
    AntipatternToken { text: Some("about"), regexp: None, inflected: false },
    AntipatternToken { text: Some("right"), regexp: None, inflected: false },
];
static ANTIPATTERN_709_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("persona"), regexp: None, inflected: false },
    AntipatternToken { text: Some("non"), regexp: None, inflected: false },
    AntipatternToken { text: Some("grata"), regexp: None, inflected: false },
];
static ANTIPATTERN_710_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("block|carve"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("out"), regexp: None, inflected: false },
];
static ANTIPATTERN_711_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("free|make|set"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
];
static ANTIPATTERN_712_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("piece"), regexp: None, inflected: false },
    AntipatternToken { text: Some("by"), regexp: None, inflected: false },
    AntipatternToken { text: Some("piece"), regexp: None, inflected: false },
];
static ANTIPATTERN_713_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("puppet|tv"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("show"), regexp: None, inflected: false },
];
static ANTIPATTERN_714_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("show"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pieces"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_715_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("market"), regexp: None, inflected: false },
    AntipatternToken { text: Some("place"), regexp: None, inflected: false },
];
static ANTIPATTERN_716_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ground"), regexp: None, inflected: false },
    AntipatternToken { text: Some("level"), regexp: None, inflected: false },
];
static ANTIPATTERN_717_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ed"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
];
static ANTIPATTERN_718_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("please"), regexp: None, inflected: false },
    AntipatternToken { text: Some("do"), regexp: None, inflected: false },
    AntipatternToken { text: Some("n't"), regexp: None, inflected: false },
];
static ANTIPATTERN_719_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hundreds|thousands|millions|billions"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_720_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("how"), regexp: None, inflected: false },
    AntipatternToken { text: Some("many"), regexp: None, inflected: false },
    AntipatternToken { text: Some("times"), regexp: None, inflected: false },
];
static ANTIPATTERN_721_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("about"), regexp: None, inflected: false },
    AntipatternToken { text: Some("me"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pages?|(web)?sites?|posters?|descriptions?|presentations?|portfolios?|assignments?|sections?|box|links?|buttons?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_722_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("i"), regexp: None, inflected: false },
];
static ANTIPATTERN_723_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("i"), regexp: None, inflected: false },
    AntipatternToken { text: Some("corps"), regexp: None, inflected: false },
];
static ANTIPATTERN_724_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("i"), regexp: None, inflected: false },
    AntipatternToken { text: Some("so"), regexp: None, inflected: false },
];
static ANTIPATTERN_725_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("i"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_726_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
];
static ANTIPATTERN_727_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("apple|form|schedule"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("i"), regexp: None, inflected: false },
];
static ANTIPATTERN_728_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("be"), regexp: None, inflected: true },
    AntipatternToken { text: Some("it?|you|s?he|we|they"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_729_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("be"), regexp: None, inflected: true },
    AntipatternToken { text: Some("n't"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it?|you|s?he|we|they"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_730_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("huang|liu|zheng"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("he"), regexp: None, inflected: false },
];
static ANTIPATTERN_731_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("if"), regexp: None, inflected: false },
    AntipatternToken { text: Some("needed|required|desired"), regexp: Some("yes"), inflected: true },
];
static ANTIPATTERN_732_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
    AntipatternToken { text: Some("so"), regexp: None, inflected: false },
    AntipatternToken { text: Some("happen(s|ed)"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_733_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("social"), regexp: None, inflected: false },
    AntipatternToken { text: Some("distance"), regexp: None, inflected: true },
];
static ANTIPATTERN_734_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("video"), regexp: None, inflected: false },
    AntipatternToken { text: Some("chat"), regexp: None, inflected: false },
];
static ANTIPATTERN_735_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("we"), regexp: None, inflected: false },
    AntipatternToken { text: Some("elderly"), regexp: None, inflected: false },
    AntipatternToken { text: Some("people"), regexp: None, inflected: false },
];
static ANTIPATTERN_736_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
    AntipatternToken { text: Some("sir"), regexp: None, inflected: false },
];
static ANTIPATTERN_737_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("think|hope"), regexp: None, inflected: false },
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
];
static ANTIPATTERN_738_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("hope|think|assume|believe|doubt|guess"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it|you"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("ill"), regexp: None, inflected: false },
];
static ANTIPATTERN_739_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
    AntipatternToken { text: Some("right"), regexp: None, inflected: false },
];
static ANTIPATTERN_740_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
    AntipatternToken { text: Some("year|day|night|week|month|time|access|wise|knowing(ness)?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_741_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("me|her|him|you"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("n?either"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_742_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("you|s?he|anyone"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("again|especially|t?here|not"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[!?]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_743_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("art"), regexp: None, inflected: false },
    AntipatternToken { text: Some("thou"), regexp: None, inflected: false },
];
static ANTIPATTERN_744_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("who"), regexp: None, inflected: false },
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("me"), regexp: None, inflected: false },
    AntipatternToken { text: Some("?"), regexp: None, inflected: false },
];
static ANTIPATTERN_745_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("reasons?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_746_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("red-nosed"), regexp: None, inflected: false },
    AntipatternToken { text: Some("reindeers?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_747_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ringo|ringo"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("starr|starr"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_748_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("think|hope|believe"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it|you"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_749_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("other"), regexp: None, inflected: false },
    AntipatternToken { text: Some("roles"), regexp: None, inflected: false },
];
static ANTIPATTERN_750_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("rolls?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("out"), regexp: None, inflected: false },
];
static ANTIPATTERN_751_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("donald"), regexp: None, inflected: false },
    AntipatternToken { text: Some("regan"), regexp: None, inflected: false },
];
static ANTIPATTERN_752_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ronald|ronald"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("reagan|reagan"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_753_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("president"), regexp: None, inflected: false },
    AntipatternToken { text: Some("reagan|reagan"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_754_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z].+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("room"), regexp: None, inflected: false },
];
static ANTIPATTERN_755_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("show"), regexp: None, inflected: false },
    AntipatternToken { text: Some("rooms"), regexp: None, inflected: false },
];
static ANTIPATTERN_756_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("show|work"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("room"), regexp: None, inflected: false },
];
static ANTIPATTERN_757_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("show|work"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("room"), regexp: None, inflected: false },
];
static ANTIPATTERN_758_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("show|work"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("room"), regexp: None, inflected: false },
];
static ANTIPATTERN_759_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("show|work"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("room"), regexp: None, inflected: false },
];
static ANTIPATTERN_760_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("stock"), regexp: None, inflected: false },
    AntipatternToken { text: Some("room"), regexp: None, inflected: false },
    AntipatternToken { text: Some("photos?|images?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_761_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("royal"), regexp: None, inflected: false },
    AntipatternToken { text: Some("air"), regexp: None, inflected: false },
    AntipatternToken { text: Some("force|maroc"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_762_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("royal"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z].+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_763_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("knees"), regexp: None, inflected: false },
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to|high"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_764_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("my|your"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("knees"), regexp: None, inflected: false },
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
];
static ANTIPATTERN_765_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the|an?|towards?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_766_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("with"), regexp: None, inflected: false },
    AntipatternToken { text: Some("your|her|his|my|their"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("knees"), regexp: None, inflected: false },
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
];
static ANTIPATTERN_767_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("salvador|salvador"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("dalí|dalí"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_768_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("at|on"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("same"), regexp: None, inflected: false },
    AntipatternToken { text: Some("day"), regexp: None, inflected: false },
];
static ANTIPATTERN_769_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("santa|santa"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("claus|claus"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_770_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("santa"), regexp: None, inflected: false },
    AntipatternToken { text: Some("clause"), regexp: None, inflected: false },
];
static ANTIPATTERN_771_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("são"), regexp: None, inflected: false },
    AntipatternToken { text: Some("paulo"), regexp: None, inflected: false },
];
static ANTIPATTERN_772_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("schindler|schindler"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("'s|'s"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("list|list|list"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_773_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sea"), regexp: None, inflected: false },
    AntipatternToken { text: Some("gull"), regexp: None, inflected: false },
];
static ANTIPATTERN_774_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sea"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ports"), regexp: None, inflected: false },
    AntipatternToken { text: Some("organi[sz]ations?|authorit(y|ies)"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_775_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sea"), regexp: None, inflected: false },
    AntipatternToken { text: Some("water"), regexp: None, inflected: false },
    AntipatternToken { text: Some("conveyances?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_776_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("black|red|baltic|dead|deep|caspian|caribbean|arabian|mediter.*|greenland|andaman|sargasso|china|irish|philippine|laptev|tasman|yellow|chukchi|beaufort|siberian|ligurian|norwegian|celtic|bohol|salton|savu|myrtoan|alboran|solomon|bismarck|ceram|labrador|comsonauts|mawson|lazarev|archipelago|molucca|camotes|adolf|east"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("sea"), regexp: None, inflected: false },
];
static ANTIPATTERN_777_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("am"), regexp: None, inflected: false },
    AntipatternToken { text: Some("seeming"), regexp: None, inflected: false },
    AntipatternToken { text: Some("like"), regexp: None, inflected: false },
];
static ANTIPATTERN_778_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("ship|ship"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_779_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("friend"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ships"), regexp: None, inflected: false },
];
static ANTIPATTERN_780_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ship"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_781_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("proud"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_782_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("her|his|its|their"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("sleep"), regexp: None, inflected: false },
];
static ANTIPATTERN_783_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("(sign|log)-(in|up|off)"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_784_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("signs"), regexp: None, inflected: false },
    AntipatternToken { text: Some("into"), regexp: None, inflected: false },
];
static ANTIPATTERN_785_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("(some|any|no)thing"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("similar"), regexp: None, inflected: false },
];
static ANTIPATTERN_786_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("some|any|no"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("thing"), regexp: None, inflected: false },
    AntipatternToken { text: Some("similar"), regexp: None, inflected: false },
];
static ANTIPATTERN_787_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("singles|singles"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("day|day"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_788_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("never|sure"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("doubt|hope"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_789_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("that"), regexp: None, inflected: false },
];
static ANTIPATTERN_790_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("one"), regexp: None, inflected: false },
    AntipatternToken { text: Some("that"), regexp: None, inflected: false },
];
static ANTIPATTERN_791_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sky"), regexp: None, inflected: false },
    AntipatternToken { text: Some("box"), regexp: None, inflected: false },
];
static ANTIPATTERN_792_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("shame"), regexp: None, inflected: false },
    AntipatternToken { text: Some("on"), regexp: None, inflected: false },
    AntipatternToken { text: Some("you|her|him|them|us|me"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_793_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("société"), regexp: None, inflected: false },
    AntipatternToken { text: Some("générales?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_794_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sometime"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("next|previous|following|upcoming"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_795_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("and"), regexp: None, inflected: false },
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("extent"), regexp: None, inflected: false },
];
static ANTIPATTERN_796_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("at"), regexp: None, inflected: false },
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("point"), regexp: None, inflected: false },
];
static ANTIPATTERN_797_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("cell"), regexp: None, inflected: false },
    AntipatternToken { text: Some("phone"), regexp: None, inflected: false },
];
static ANTIPATTERN_798_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("reason"), regexp: None, inflected: false },
];
static ANTIPATTERN_799_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for|after"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
];
static ANTIPATTERN_800_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ground|next"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("level"), regexp: None, inflected: false },
];
static ANTIPATTERN_801_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("part"), regexp: None, inflected: false },
];
static ANTIPATTERN_802_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("way"), regexp: None, inflected: false },
    AntipatternToken { text: Some("close"), regexp: None, inflected: false },
];
static ANTIPATTERN_803_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("let"), regexp: None, inflected: false },
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
];
static ANTIPATTERN_804_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("day"), regexp: None, inflected: false },
];
static ANTIPATTERN_805_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("how|body|one"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_806_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("say|find"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_807_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
    AntipatternToken { text: Some("back"), regexp: None, inflected: false },
];
static ANTIPATTERN_808_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("think|hope|believe|doubt|guess|assume|expect"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it|him|her|this|them|that"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
];
static ANTIPATTERN_809_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("effect"), regexp: None, inflected: false },
];
static ANTIPATTERN_810_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("extent"), regexp: None, inflected: false },
];
static ANTIPATTERN_811_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("url"), regexp: None, inflected: false },
    AntipatternToken { text: Some("slug"), regexp: None, inflected: false },
];
static ANTIPATTERN_812_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("are|were|propose[sd]?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("times"), regexp: None, inflected: false },
    AntipatternToken { text: Some("that"), regexp: None, inflected: false },
];
static ANTIPATTERN_813_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("block|reserve|mark|confirm|book|schedule"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("times"), regexp: None, inflected: false },
];
static ANTIPATTERN_814_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("times"), regexp: None, inflected: false },
    AntipatternToken { text: Some("block|reserve|mark|confirm|book|schedule"), regexp: Some("yes"), inflected: true },
];
static ANTIPATTERN_815_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("take"), regexp: None, inflected: true },
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
];
static ANTIPATTERN_816_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("times"), regexp: None, inflected: false },
    AntipatternToken { text: Some("and|&|or"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("dates"), regexp: None, inflected: false },
];
static ANTIPATTERN_817_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("times"), regexp: None, inflected: false },
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("cst|cdt|mst|mdt|pst|pdt|est|edt"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_818_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("times"), regexp: None, inflected: false },
    AntipatternToken { text: Some("journalists?|newspapers?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_819_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("times"), regexp: None, inflected: false },
    AntipatternToken { text: Some("new"), regexp: None, inflected: false },
    AntipatternToken { text: Some("roman"), regexp: None, inflected: false },
];
static ANTIPATTERN_820_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("what"), regexp: None, inflected: false },
    AntipatternToken { text: Some("are|were"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("some"), regexp: None, inflected: false },
    AntipatternToken { text: Some("times"), regexp: None, inflected: false },
];
static ANTIPATTERN_821_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("too"), regexp: None, inflected: false },
    AntipatternToken { text: Some("soon"), regexp: None, inflected: false },
];
static ANTIPATTERN_822_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("geia|de"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("sou"), regexp: None, inflected: false },
];
static ANTIPATTERN_823_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mrs?|dr|prof"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("sou"), regexp: None, inflected: false },
];
static ANTIPATTERN_824_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sou"), regexp: None, inflected: false },
    AntipatternToken { text: Some("de|'s|chef|um"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_825_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the|a|eu"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("sou"), regexp: None, inflected: false },
];
static ANTIPATTERN_826_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("("), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z]{2,3}spd|c[sd]u|afd|fdp|ard|zdf|rtl|fbi|cia|usa|hsv|etf|nrw"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_827_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("("), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some(")"), regexp: None, inflected: false },
];
static ANTIPATTERN_828_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("an?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("("), regexp: None, inflected: false },
    AntipatternToken { text: Some("n?other"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_829_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("child"), regexp: None, inflected: false },
    AntipatternToken { text: Some("("), regexp: None, inflected: false },
    AntipatternToken { text: Some("ren"), regexp: None, inflected: false },
];
static ANTIPATTERN_830_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("east|west|south|north"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("("), regexp: None, inflected: false },
    AntipatternToken { text: Some("ern"), regexp: None, inflected: false },
];
static ANTIPATTERN_831_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sens"), regexp: None, inflected: false },
    AntipatternToken { text: Some("("), regexp: None, inflected: false },
    AntipatternToken { text: Some("or"), regexp: None, inflected: false },
];
static ANTIPATTERN_832_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("good|sure|(him|her|them|it)(self|selves)?|myself|yourself|me|you"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_833_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("fall"), regexp: None, inflected: true },
    AntipatternToken { text: Some("down"), regexp: None, inflected: false },
];
static ANTIPATTERN_834_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("torn"), regexp: None, inflected: false },
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
    AntipatternToken { text: Some("stairs"), regexp: None, inflected: false },
];
static ANTIPATTERN_835_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
    AntipatternToken { text: Some("&|and|or"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("down"), regexp: None, inflected: false },
];
static ANTIPATTERN_836_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("state"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[oo]f"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[tt]he"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("union"), regexp: None, inflected: false },
];
static ANTIPATTERN_837_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("step"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z].+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_838_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("if"), regexp: None, inflected: false },
    AntipatternToken { text: Some("factory"), regexp: None, inflected: false },
    AntipatternToken { text: Some("reset"), regexp: None, inflected: false },
];
static ANTIPATTERN_839_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("course"), regexp: None, inflected: false },
];
static ANTIPATTERN_840_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("law|body|counter|space|track|snow|jump|cat|sun"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("suits"), regexp: None, inflected: false },
    AntipatternToken { text: Some("my|y?our|the|an?|their|his|her"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_841_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("summery"), regexp: None, inflected: false },
    AntipatternToken { text: Some("outside"), regexp: None, inflected: false },
];
static ANTIPATTERN_842_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("very|really"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("summery"), regexp: None, inflected: false },
];
static ANTIPATTERN_843_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("was|is|'s|gets|got|becomes"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("summery"), regexp: None, inflected: false },
];
static ANTIPATTERN_844_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("make"), regexp: None, inflected: true },
    AntipatternToken { text: Some("sure"), regexp: None, inflected: false },
    AntipatternToken { text: Some("than"), regexp: None, inflected: false },
];
static ANTIPATTERN_845_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("two|2|three|3|four|4|five|5|six|6|seven|7|eight|8|nine|9|ten|10|many"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("times"), regexp: None, inflected: false },
    AntipatternToken { text: Some("as"), regexp: None, inflected: false },
    AntipatternToken { text: Some("much"), regexp: None, inflected: false },
];
static ANTIPATTERN_846_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("publix"), regexp: None, inflected: false },
    AntipatternToken { text: Some("super"), regexp: None, inflected: false },
    AntipatternToken { text: Some("markets"), regexp: None, inflected: false },
];
static ANTIPATTERN_847_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("super"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z].+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_848_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("marvel|dc|lego"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("super"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hero(es)?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_849_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("super"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hero"), regexp: None, inflected: false },
    AntipatternToken { text: Some("girls?|squads?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_850_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("let"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_851_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("tat"), regexp: None, inflected: false },
];
static ANTIPATTERN_852_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("tat"), regexp: None, inflected: false },
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("rev"), regexp: None, inflected: false },
];
static ANTIPATTERN_853_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("cheap|old|souvenir|de"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("tat"), regexp: None, inflected: false },
];
static ANTIPATTERN_854_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tit"), regexp: None, inflected: false },
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("tat"), regexp: None, inflected: false },
];
static ANTIPATTERN_855_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("than"), regexp: None, inflected: false },
    AntipatternToken { text: Some("than"), regexp: None, inflected: false },
];
static ANTIPATTERN_856_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_857_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("sure|cast.*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_858_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("here|hear"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("and|n?or|&|\\/|,"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("there"), regexp: None, inflected: false },
];
static ANTIPATTERN_859_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("there"), regexp: None, inflected: false },
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("while"), regexp: None, inflected: false },
];
static ANTIPATTERN_860_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("there"), regexp: None, inflected: false },
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("each"), regexp: None, inflected: false },
    AntipatternToken { text: Some("other"), regexp: None, inflected: false },
];
static ANTIPATTERN_861_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("there"), regexp: None, inflected: false },
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("them|him|her|me"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_862_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ones"), regexp: None, inflected: false },
    AntipatternToken { text: Some("and"), regexp: None, inflected: false },
    AntipatternToken { text: Some("zeroe?s"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_863_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("be|head|hold"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("there"), regexp: None, inflected: false },
    AntipatternToken { text: Some("where"), regexp: None, inflected: false },
];
static ANTIPATTERN_864_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("out|over|in|up"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("there"), regexp: None, inflected: false },
    AntipatternToken { text: Some("where"), regexp: None, inflected: false },
];
static ANTIPATTERN_865_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("there"), regexp: None, inflected: false },
    AntipatternToken { text: Some("where"), regexp: None, inflected: false },
    AntipatternToken { text: Some("there"), regexp: None, inflected: false },
    AntipatternToken { text: Some("is|are|were|was"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_866_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("be"), regexp: None, inflected: false },
    AntipatternToken { text: Some("there"), regexp: None, inflected: false },
    AntipatternToken { text: Some("where"), regexp: None, inflected: false },
];
static ANTIPATTERN_867_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("old"), regexp: None, inflected: false },
    AntipatternToken { text: Some("dutch"), regexp: None, inflected: false },
];
static ANTIPATTERN_868_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("how"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_869_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("who"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
    AntipatternToken { text: Some("who"), regexp: None, inflected: false },
];
static ANTIPATTERN_870_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("an|the"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("(?-i)it"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("girl"), regexp: None, inflected: true },
];
static ANTIPATTERN_871_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_872_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("b"), regexp: None, inflected: false },
];
static ANTIPATTERN_873_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("n"), regexp: None, inflected: false },
    AntipatternToken { text: Some("/"), regexp: None, inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_874_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("vowel|letter"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_875_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(":|\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_876_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ping"), regexp: None, inflected: false },
    AntipatternToken { text: Some("an"), regexp: None, inflected: false },
];
static ANTIPATTERN_877_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\p{ll}"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("−|-"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
];
static ANTIPATTERN_878_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("lit|paraphraph|literature|sect?|section|named|assigned|abbrev(iated)?|acr(onym)?|plan|versus|vs|modulo|ex(ample)?|part|pt|answer|annex"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_879_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("what"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[!?]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_880_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("get"), regexp: None, inflected: true },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("better"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_881_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("course"), regexp: None, inflected: false },
];
static ANTIPATTERN_882_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("off"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hand"), regexp: None, inflected: false },
];
static ANTIPATTERN_883_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("off"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_884_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("off"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("top"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_885_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("and|&|or|,"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("that|this|you|it"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_886_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("any.+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("like"), regexp: None, inflected: false },
    AntipatternToken { text: Some("this"), regexp: None, inflected: false },
];
static ANTIPATTERN_887_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("days"), regexp: None, inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("week"), regexp: None, inflected: false },
];
static ANTIPATTERN_888_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("next|last"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("years|months|weeks|weekends|nights"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_889_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("through"), regexp: None, inflected: false },
    AntipatternToken { text: Some("street|ticket|road|lane|line|hole|ball|pass|route"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_890_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("("), regexp: None, inflected: false },
    AntipatternToken { text: Some("th"), regexp: None, inflected: false },
    AntipatternToken { text: Some(")"), regexp: None, inflected: false },
];
static ANTIPATTERN_891_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("th"), regexp: None, inflected: false },
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z][a-z]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_892_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z][a-z]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("th"), regexp: None, inflected: false },
];
static ANTIPATTERN_893_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("tis"), regexp: None, inflected: false },
];
static ANTIPATTERN_894_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tis"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hazari"), regexp: None, inflected: false },
];
static ANTIPATTERN_895_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\p{l}\\p{l}+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("tis"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\p{l}\\p{l}+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_896_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tis"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
];
static ANTIPATTERN_897_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("due"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("unable"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_898_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("too"), regexp: None, inflected: false },
    AntipatternToken { text: Some("(:?[1-2]\\d)?\\d01000"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_899_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("t?here"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("too"), regexp: None, inflected: false },
    AntipatternToken { text: Some("this"), regexp: None, inflected: false },
    AntipatternToken { text: Some("morning|afternoon|evening"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_900_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tw?o"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("/"), regexp: None, inflected: false },
    AntipatternToken { text: Some("too"), regexp: None, inflected: false },
    AntipatternToken { text: Some("that"), regexp: None, inflected: false },
];
static ANTIPATTERN_901_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("be"), regexp: None, inflected: true },
    AntipatternToken { text: Some("towed"), regexp: None, inflected: false },
];
static ANTIPATTERN_902_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\p{p}"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("tr"), regexp: None, inflected: false },
];
static ANTIPATTERN_903_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tr"), regexp: None, inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
];
static ANTIPATTERN_904_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("none?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("less"), regexp: None, inflected: false },
];
static ANTIPATTERN_905_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("i|you|we|they|he|s?he|it"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("can"), regexp: None, inflected: false },
    AntipatternToken { text: Some("will"), regexp: None, inflected: false },
];
static ANTIPATTERN_906_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("docker.*|accept|drop|return|reject"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
    AntipatternToken { text: Some("-"), regexp: None, inflected: false },
    AntipatternToken { text: Some("-"), regexp: None, inflected: false },
];
static ANTIPATTERN_907_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\.|\\;|\\:"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("-"), regexp: None, inflected: false },
    AntipatternToken { text: Some("-"), regexp: None, inflected: false },
];
static ANTIPATTERN_908_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("end"), regexp: None, inflected: false },
    AntipatternToken { text: Some("tell|try|repeat|if"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("-"), regexp: None, inflected: false },
    AntipatternToken { text: Some("-"), regexp: None, inflected: false },
];
static ANTIPATTERN_909_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("don"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("s"), regexp: None, inflected: false },
];
static ANTIPATTERN_910_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("can|don|doesn|ha[ds]n|haven|[cw]ouldn|shouldn|wasn|weren|mustn|aren|isn"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("t"), regexp: None, inflected: false },
];
static ANTIPATTERN_911_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("we|they"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_912_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("t"), regexp: None, inflected: false },
];
static ANTIPATTERN_913_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("t"), regexp: None, inflected: false },
    AntipatternToken { text: Some("he"), regexp: None, inflected: false },
];
static ANTIPATTERN_914_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("he"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_915_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("he"), regexp: None, inflected: false },
    AntipatternToken { text: Some("is|was"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_916_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("t-rex"), regexp: None, inflected: false },
    AntipatternToken { text: Some("engineering"), regexp: None, inflected: false },
];
static ANTIPATTERN_917_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bam"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bam"), regexp: None, inflected: false },
];
static ANTIPATTERN_918_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bam"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bigelow"), regexp: None, inflected: false },
];
static ANTIPATTERN_919_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("boo"), regexp: None, inflected: false },
    AntipatternToken { text: Some("brazil"), regexp: None, inflected: false },
];
static ANTIPATTERN_920_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("huh"), regexp: None, inflected: false },
    AntipatternToken { text: Some("huh"), regexp: None, inflected: false },
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("hollywood"), regexp: None, inflected: false },
];
static ANTIPATTERN_921_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("under"), regexp: None, inflected: false },
    AntipatternToken { text: Some("cover"), regexp: None, inflected: false },
];
static ANTIPATTERN_922_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("cover"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_923_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("unicode"), regexp: None, inflected: false },
    AntipatternToken { text: Some("="), regexp: None, inflected: false },
];
static ANTIPATTERN_924_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("unicode"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[_.]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some(".*[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_925_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(":"), regexp: None, inflected: false },
    AntipatternToken { text: Some(":"), regexp: None, inflected: false },
    AntipatternToken { text: Some(":"), regexp: None, inflected: false },
];
static ANTIPATTERN_926_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(":"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some(":"), regexp: None, inflected: false },
];
static ANTIPATTERN_927_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(";"), regexp: None, inflected: false },
    AntipatternToken { text: Some(";"), regexp: None, inflected: false },
    AntipatternToken { text: Some(";"), regexp: None, inflected: false },
];
static ANTIPATTERN_928_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("stand|follow"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
];
static ANTIPATTERN_929_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("royal"), regexp: None, inflected: false },
    AntipatternToken { text: Some("game"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ur"), regexp: None, inflected: false },
];
static ANTIPATTERN_930_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("do|did|does"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("i|you|we|they|he|she|it"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("use"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_931_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("terms"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("use"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_932_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("'re"), regexp: None, inflected: false },
    AntipatternToken { text: Some("not"), regexp: None, inflected: false },
];
static ANTIPATTERN_933_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_934_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("copy"), regexp: None, inflected: false },
    AntipatternToken { text: Some("paste"), regexp: None, inflected: false },
];
static ANTIPATTERN_935_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("get"), regexp: None, inflected: false },
    AntipatternToken { text: Some("rid"), regexp: None, inflected: false },
];
static ANTIPATTERN_936_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("let"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_937_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("let"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_938_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("let"), regexp: None, inflected: false },
    AntipatternToken { text: Some("go"), regexp: None, inflected: false },
];
static ANTIPATTERN_939_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("plan"), regexp: None, inflected: false },
    AntipatternToken { text: Some("revenge"), regexp: None, inflected: false },
];
static ANTIPATTERN_940_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("play"), regexp: None, inflected: false },
    AntipatternToken { text: Some("catch"), regexp: None, inflected: false },
];
static ANTIPATTERN_941_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("turn"), regexp: None, inflected: false },
    AntipatternToken { text: Some("power"), regexp: None, inflected: false },
    AntipatternToken { text: Some("on|off"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_942_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(":|;"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("vitamin"), regexp: None, inflected: false },
];
static ANTIPATTERN_943_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("vitamin|vitamin"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_944_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\p{p}"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("needs?|likes?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_945_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("try|need|like"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_946_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("as|when|if|(al)?though"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("needed"), regexp: None, inflected: false },
];
static ANTIPATTERN_947_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("love"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("god"), regexp: None, inflected: false },
];
static ANTIPATTERN_948_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("love"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("all"), regexp: None, inflected: false },
];
static ANTIPATTERN_949_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("love"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("every(one|body)"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_950_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("make"), regexp: None, inflected: true },
    AntipatternToken { text: Some("love"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_951_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sort|kind"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("off?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_952_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some(".+-.+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_953_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z]{1,3}"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_954_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("deep"), regexp: None, inflected: false },
    AntipatternToken { text: Some("condition"), regexp: None, inflected: false },
];
static ANTIPATTERN_955_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("due"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
];
static ANTIPATTERN_956_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("google|neg"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_957_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("last|next"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("week(end)?|month|year|time|christmas|thanksgiving"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_958_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("less|more"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("than"), regexp: None, inflected: false },
];
static ANTIPATTERN_959_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("less|more|then|just|perhaps"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_960_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("more"), regexp: None, inflected: false },
    AntipatternToken { text: Some("or|and|&"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("less|more"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_961_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("much"), regexp: None, inflected: false },
];
static ANTIPATTERN_962_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("netflix"), regexp: None, inflected: false },
    AntipatternToken { text: Some("and|&"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("chill"), regexp: None, inflected: false },
];
static ANTIPATTERN_963_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("one"), regexp: None, inflected: false },
    AntipatternToken { text: Some("day|way"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_964_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("only|just"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("when|if|so"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_965_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("sooner"), regexp: None, inflected: false },
    AntipatternToken { text: Some("or"), regexp: None, inflected: false },
    AntipatternToken { text: Some("later"), regexp: None, inflected: false },
];
static ANTIPATTERN_966_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("white|black"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("list"), regexp: None, inflected: false },
];
static ANTIPATTERN_967_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(".*[,a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("wee"), regexp: None, inflected: false },
];
static ANTIPATTERN_968_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("has|is|was"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("n't"), regexp: None, inflected: false },
    AntipatternToken { text: Some("wee"), regexp: None, inflected: false },
];
static ANTIPATTERN_969_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("wernher|wernher"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("von|von"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("braun|braun"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_970_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[dd]id|[dd]oes"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[a-z]{2,5}"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_971_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("how"), regexp: None, inflected: false },
    AntipatternToken { text: Some("come"), regexp: None, inflected: false },
];
static ANTIPATTERN_972_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("reason"), regexp: None, inflected: false },
    AntipatternToken { text: Some("enough"), regexp: None, inflected: false },
];
static ANTIPATTERN_973_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("how"), regexp: None, inflected: false },
    AntipatternToken { text: Some("to"), regexp: None, inflected: false },
    AntipatternToken { text: Some("articles|documents|examples|scripts|manuals|tips"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_974_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("up"), regexp: None, inflected: false },
    AntipatternToken { text: Some("and"), regexp: None, inflected: false },
    AntipatternToken { text: Some("coming"), regexp: None, inflected: false },
];
static ANTIPATTERN_975_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("what"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_976_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("as"), regexp: None, inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("result|matter"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_977_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("as"), regexp: None, inflected: false },
    AntipatternToken { text: Some("previously|already"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("said|mentioned|described|told|explained"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_978_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("by"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_979_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("by"), regexp: None, inflected: false },
    AntipatternToken { text: Some("default|any|nature|heart|mistake|chance|choice|now"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_980_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("by"), regexp: None, inflected: false },
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
];
static ANTIPATTERN_981_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("suggestions"), regexp: None, inflected: false },
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("when|where|how|why"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_982_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("when|where"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("need"), regexp: None, inflected: false },
    AntipatternToken { text: Some("be"), regexp: None, inflected: false },
];
static ANTIPATTERN_983_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("i"), regexp: None, inflected: false },
];
static ANTIPATTERN_984_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("every|any"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("whit"), regexp: None, inflected: false },
];
static ANTIPATTERN_985_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("who"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_986_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("home"), regexp: None, inflected: false },
    AntipatternToken { text: Some("?"), regexp: None, inflected: false },
];
static ANTIPATTERN_987_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the"), regexp: None, inflected: false },
    AntipatternToken { text: Some("who"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_988_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("who"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
    AntipatternToken { text: Some("kind|sort|part"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("off?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_989_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("the|my"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("who"), regexp: None, inflected: false },
];
static ANTIPATTERN_990_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("who"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
    AntipatternToken { text: Some("who"), regexp: None, inflected: false },
];
static ANTIPATTERN_991_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("who"), regexp: None, inflected: false },
    AntipatternToken { text: Some("kind|sort"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_992_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("who"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_993_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ask|know|question|wonder"), regexp: None, inflected: false },
    AntipatternToken { text: Some("who"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
];
static ANTIPATTERN_994_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("#"), regexp: None, inflected: false },
    AntipatternToken { text: Some("wien"), regexp: None, inflected: false },
];
static ANTIPATTERN_995_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mrs?|dr|prof"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("wien"), regexp: None, inflected: false },
];
static ANTIPATTERN_996_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("wien"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'s"), regexp: None, inflected: false },
    AntipatternToken { text: Some("law"), regexp: None, inflected: false },
];
static ANTIPATTERN_997_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("wien"), regexp: None, inflected: false },
    AntipatternToken { text: Some("energie"), regexp: None, inflected: false },
];
static ANTIPATTERN_998_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("wien"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-zöäü].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_999_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("wien"), regexp: None, inflected: false },
];
static ANTIPATTERN_1000_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("out"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("course"), regexp: None, inflected: false },
];
static ANTIPATTERN_1001_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
    AntipatternToken { text: Some("course"), regexp: None, inflected: false },
];
static ANTIPATTERN_1002_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("firth|frith|holt|weald|wood|woodland"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("wold"), regexp: None, inflected: false },
];
static ANTIPATTERN_1003_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("wold"), regexp: None, inflected: false },
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("firth|frith|holt|weald|wood|woodland"), regexp: Some("yes"), inflected: true },
];
static ANTIPATTERN_1004_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("business"), regexp: None, inflected: false },
    AntipatternToken { text: Some("wom[ea]n"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1005_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("will"), regexp: None, inflected: false },
    AntipatternToken { text: Some("or|and"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("wo"), regexp: None, inflected: false },
    AntipatternToken { text: Some("n't"), regexp: None, inflected: false },
];
static ANTIPATTERN_1006_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("wordpress"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("com"), regexp: None, inflected: false },
];
static ANTIPATTERN_1007_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("["), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z-]+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z-]+"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1008_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z-]+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z0-9]{2,7}"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1009_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1010_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("id|token|name|type|title|category|price|number|timestamp|date|code|value|error|page"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1011_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("qty"), regexp: None, inflected: false },
];
static ANTIPATTERN_1012_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("client"), regexp: None, inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("secret"), regexp: None, inflected: false },
];
static ANTIPATTERN_1013_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("created|deleted|destroyed|deactivated|updated"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("at"), regexp: None, inflected: false },
];
static ANTIPATTERN_1014_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("date|time"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("created|updated"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1015_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("de"), regexp: None, inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("de|at|ch"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1016_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("en"), regexp: None, inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("us|gb|nz|ca|za|au"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1017_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("es"), regexp: None, inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("es|co|mx"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1018_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("get|retrieve"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
];
static ANTIPATTERN_1019_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("is"), regexp: None, inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("enabled|disabled|activated|deactivated"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1020_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("last"), regexp: None, inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("modified"), regexp: None, inflected: false },
];
static ANTIPATTERN_1021_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("nl"), regexp: None, inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("nl|be"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1022_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("pt"), regexp: None, inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("br|pt|mz"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1023_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("public"), regexp: None, inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("html"), regexp: None, inflected: false },
];
static ANTIPATTERN_1024_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("session"), regexp: None, inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("start"), regexp: None, inflected: false },
];
static ANTIPATTERN_1025_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("total"), regexp: None, inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("amount|quantaty"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1026_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("user"), regexp: None, inflected: false },
    AntipatternToken { text: Some("_"), regexp: None, inflected: false },
    AntipatternToken { text: Some("agent|language"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1027_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("auto|farm|iron"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("workers"), regexp: None, inflected: false },
];
static ANTIPATTERN_1028_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("part|full|real"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
    AntipatternToken { text: Some("workers?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1029_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sheet"), regexp: None, inflected: false },
    AntipatternToken { text: Some("metal"), regexp: None, inflected: false },
];
static ANTIPATTERN_1030_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("time"), regexp: None, inflected: false },
    AntipatternToken { text: Some("workers"), regexp: None, inflected: false },
    AntipatternToken { text: Some("spend"), regexp: None, inflected: true },
];
static ANTIPATTERN_1031_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("world"), regexp: None, inflected: false },
    AntipatternToken { text: Some("wide"), regexp: None, inflected: false },
    AntipatternToken { text: Some("web"), regexp: None, inflected: false },
];
static ANTIPATTERN_1032_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("for"), regexp: None, inflected: false },
    AntipatternToken { text: Some("now"), regexp: None, inflected: false },
];
static ANTIPATTERN_1033_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("let"), regexp: None, inflected: false },
    AntipatternToken { text: Some("me|us|them|him|her|it"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("know"), regexp: None, inflected: false },
];
static ANTIPATTERN_1034_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("thank"), regexp: None, inflected: false },
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
];
static ANTIPATTERN_1035_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ghost"), regexp: None, inflected: false },
    AntipatternToken { text: Some("writer"), regexp: None, inflected: false },
];
static ANTIPATTERN_1036_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("think|hope|believe"), regexp: None, inflected: false },
    AntipatternToken { text: Some("it"), regexp: None, inflected: false },
];
static ANTIPATTERN_1037_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[\\._#:\\/]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[tw]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1038_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("w"), regexp: None, inflected: false },
    AntipatternToken { text: Some("his"), regexp: None, inflected: false },
];
static ANTIPATTERN_1039_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("brooklyn"), regexp: None, inflected: false },
    AntipatternToken { text: Some("boat"), regexp: None, inflected: false },
    AntipatternToken { text: Some("yard"), regexp: None, inflected: false },
];
static ANTIPATTERN_1040_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("stock"), regexp: None, inflected: false },
    AntipatternToken { text: Some("yards"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bank"), regexp: None, inflected: false },
];
static ANTIPATTERN_1041_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("u?s"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("steel"), regexp: None, inflected: false },
    AntipatternToken { text: Some("yard"), regexp: None, inflected: false },
];
static ANTIPATTERN_1042_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("super"), regexp: None, inflected: false },
    AntipatternToken { text: Some("duper"), regexp: None, inflected: false },
];
static ANTIPATTERN_1043_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'re"), regexp: None, inflected: false },
    AntipatternToken { text: Some("(every|any)thing"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("to|for|i|s?he|you|we|they|it|that|what|which"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1044_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'re"), regexp: None, inflected: false },
    AntipatternToken { text: Some("drop"), regexp: None, inflected: false },
    AntipatternToken { text: Some("dead"), regexp: None, inflected: false },
];
static ANTIPATTERN_1045_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'re"), regexp: None, inflected: false },
    AntipatternToken { text: Some("friends"), regexp: None, inflected: false },
    AntipatternToken { text: Some("with"), regexp: None, inflected: false },
];
static ANTIPATTERN_1046_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'re"), regexp: None, inflected: false },
    AntipatternToken { text: Some("home"), regexp: None, inflected: false },
    AntipatternToken { text: Some("early|late|soon"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1047_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'re"), regexp: None, inflected: false },
    AntipatternToken { text: Some("most"), regexp: None, inflected: false },
    AntipatternToken { text: Some("welcome"), regexp: None, inflected: false },
];
static ANTIPATTERN_1048_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'re"), regexp: None, inflected: false },
    AntipatternToken { text: Some("only|just"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1049_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'re"), regexp: None, inflected: false },
    AntipatternToken { text: Some("something|nuts|fools|wimps|nothing|geniuses|on|under|already|just|now|welcome"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1050_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'re"), regexp: None, inflected: false },
    AntipatternToken { text: Some("sort"), regexp: None, inflected: false },
    AntipatternToken { text: Some("of"), regexp: None, inflected: false },
];
static ANTIPATTERN_1051_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'re"), regexp: None, inflected: false },
    AntipatternToken { text: Some("way"), regexp: None, inflected: false },
    AntipatternToken { text: Some("off|behind|too|ahead|out|before|after"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1052_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("you"), regexp: None, inflected: false },
    AntipatternToken { text: Some("'re"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ways?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("too"), regexp: None, inflected: false },
];

/// Antipatterns for EN (sorted by rule_id)
/// Total: 1053 antipatterns
pub static EN_ANTIPATTERNS: &[Antipattern] = &[
    Antipattern { rule_id: "ABOUT_ITS_NN", tokens: ANTIPATTERN_0_TOKENS },
    Antipattern { rule_id: "ABSORB_AT_IN", tokens: ANTIPATTERN_1_TOKENS },
    Antipattern { rule_id: "ACCORDING_TO", tokens: ANTIPATTERN_2_TOKENS },
    Antipattern { rule_id: "ACHE_COMPOUNDS", tokens: ANTIPATTERN_3_TOKENS },
    Antipattern { rule_id: "ADVERB_VERB_ADVERB_REPETITION", tokens: ANTIPATTERN_4_TOKENS },
    Antipattern { rule_id: "ADVERB_VERB_ADVERB_REPETITION", tokens: ANTIPATTERN_5_TOKENS },
    Antipattern { rule_id: "ADVERB_VERB_ADVERB_REPETITION", tokens: ANTIPATTERN_6_TOKENS },
    Antipattern { rule_id: "ADVERB_VERB_ADVERB_REPETITION", tokens: ANTIPATTERN_7_TOKENS },
    Antipattern { rule_id: "ADVERTISEMENT_OF_FOR", tokens: ANTIPATTERN_8_TOKENS },
    Antipattern { rule_id: "AI", tokens: ANTIPATTERN_9_TOKENS },
    Antipattern { rule_id: "AI", tokens: ANTIPATTERN_10_TOKENS },
    Antipattern { rule_id: "AI", tokens: ANTIPATTERN_11_TOKENS },
    Antipattern { rule_id: "AI", tokens: ANTIPATTERN_12_TOKENS },
    Antipattern { rule_id: "ALLOW_TO_DO", tokens: ANTIPATTERN_13_TOKENS },
    Antipattern { rule_id: "ALL_MOST_SOME_OF_NOUN", tokens: ANTIPATTERN_14_TOKENS },
    Antipattern { rule_id: "ALL_MOST_SOME_OF_NOUN", tokens: ANTIPATTERN_15_TOKENS },
    Antipattern { rule_id: "ALL_MOST_SOME_OF_NOUN", tokens: ANTIPATTERN_16_TOKENS },
    Antipattern { rule_id: "ALL_MOST_SOME_OF_NOUN", tokens: ANTIPATTERN_17_TOKENS },
    Antipattern { rule_id: "ALL_MOST_SOME_OF_NOUN", tokens: ANTIPATTERN_18_TOKENS },
    Antipattern { rule_id: "ALL_WAYS", tokens: ANTIPATTERN_19_TOKENS },
    Antipattern { rule_id: "ALL_WAYS", tokens: ANTIPATTERN_20_TOKENS },
    Antipattern { rule_id: "ALL_WAYS", tokens: ANTIPATTERN_21_TOKENS },
    Antipattern { rule_id: "AMERICANO", tokens: ANTIPATTERN_22_TOKENS },
    Antipattern { rule_id: "ANYMORE_ADVERB", tokens: ANTIPATTERN_23_TOKENS },
    Antipattern { rule_id: "ANYMORE_ADVERB", tokens: ANTIPATTERN_24_TOKENS },
    Antipattern { rule_id: "ANYMORE_ADVERB", tokens: ANTIPATTERN_25_TOKENS },
    Antipattern { rule_id: "ANYMORE_ADVERB", tokens: ANTIPATTERN_26_TOKENS },
    Antipattern { rule_id: "APOSTROPHE_UPPERCASE_LETTER", tokens: ANTIPATTERN_27_TOKENS },
    Antipattern { rule_id: "APPRECIATE_IF", tokens: ANTIPATTERN_28_TOKENS },
    Antipattern { rule_id: "APPSTORE", tokens: ANTIPATTERN_29_TOKENS },
    Antipattern { rule_id: "ARRIVE_ON_AT_THE_BEACH", tokens: ANTIPATTERN_30_TOKENS },
    Antipattern { rule_id: "ARRIVE_ON_AT_THE_BEACH", tokens: ANTIPATTERN_31_TOKENS },
    Antipattern { rule_id: "ARTICLE_UNNECESSARY", tokens: ANTIPATTERN_32_TOKENS },
    Antipattern { rule_id: "ASK_TO", tokens: ANTIPATTERN_33_TOKENS },
    Antipattern { rule_id: "ASK_TO", tokens: ANTIPATTERN_34_TOKENS },
    Antipattern { rule_id: "ASSASSINS_CREED", tokens: ANTIPATTERN_35_TOKENS },
    Antipattern { rule_id: "AS_ADJ_AS", tokens: ANTIPATTERN_36_TOKENS },
    Antipattern { rule_id: "AS_ADJ_AS", tokens: ANTIPATTERN_37_TOKENS },
    Antipattern { rule_id: "AS_SAD", tokens: ANTIPATTERN_38_TOKENS },
    Antipattern { rule_id: "AT_ANYTIME", tokens: ANTIPATTERN_39_TOKENS },
    Antipattern { rule_id: "AT_IN_THE_KITCHEN", tokens: ANTIPATTERN_40_TOKENS },
    Antipattern { rule_id: "AT_IN_THE_KITCHEN", tokens: ANTIPATTERN_41_TOKENS },
    Antipattern { rule_id: "AT_IN_THE_KITCHEN", tokens: ANTIPATTERN_42_TOKENS },
    Antipattern { rule_id: "AT_IN_THE_KITCHEN", tokens: ANTIPATTERN_43_TOKENS },
    Antipattern { rule_id: "AT_THE_JOB", tokens: ANTIPATTERN_44_TOKENS },
    Antipattern { rule_id: "AT_THE_JOB", tokens: ANTIPATTERN_45_TOKENS },
    Antipattern { rule_id: "AU", tokens: ANTIPATTERN_46_TOKENS },
    Antipattern { rule_id: "AU", tokens: ANTIPATTERN_47_TOKENS },
    Antipattern { rule_id: "AU", tokens: ANTIPATTERN_48_TOKENS },
    Antipattern { rule_id: "AU", tokens: ANTIPATTERN_49_TOKENS },
    Antipattern { rule_id: "AU", tokens: ANTIPATTERN_50_TOKENS },
    Antipattern { rule_id: "AU", tokens: ANTIPATTERN_51_TOKENS },
    Antipattern { rule_id: "AU", tokens: ANTIPATTERN_52_TOKENS },
    Antipattern { rule_id: "AU", tokens: ANTIPATTERN_53_TOKENS },
    Antipattern { rule_id: "AU", tokens: ANTIPATTERN_54_TOKENS },
    Antipattern { rule_id: "AU", tokens: ANTIPATTERN_55_TOKENS },
    Antipattern { rule_id: "AU", tokens: ANTIPATTERN_56_TOKENS },
    Antipattern { rule_id: "A_BIT_OF", tokens: ANTIPATTERN_57_TOKENS },
    Antipattern { rule_id: "A_BIT_OF", tokens: ANTIPATTERN_58_TOKENS },
    Antipattern { rule_id: "A_BIT_OF", tokens: ANTIPATTERN_59_TOKENS },
    Antipattern { rule_id: "A_BIT_OF", tokens: ANTIPATTERN_60_TOKENS },
    Antipattern { rule_id: "A_BIT_OF", tokens: ANTIPATTERN_61_TOKENS },
    Antipattern { rule_id: "A_CD_NNS", tokens: ANTIPATTERN_62_TOKENS },
    Antipattern { rule_id: "A_CD_NNS", tokens: ANTIPATTERN_63_TOKENS },
    Antipattern { rule_id: "A_CD_NNS", tokens: ANTIPATTERN_64_TOKENS },
    Antipattern { rule_id: "A_DISCOVER", tokens: ANTIPATTERN_65_TOKENS },
    Antipattern { rule_id: "A_FEEDBACK", tokens: ANTIPATTERN_66_TOKENS },
    Antipattern { rule_id: "A_LA_DIACRITIC", tokens: ANTIPATTERN_67_TOKENS },
    Antipattern { rule_id: "A_LA_DIACRITIC", tokens: ANTIPATTERN_68_TOKENS },
    Antipattern { rule_id: "A_MY", tokens: ANTIPATTERN_69_TOKENS },
    Antipattern { rule_id: "A_MY", tokens: ANTIPATTERN_70_TOKENS },
    Antipattern { rule_id: "A_MY", tokens: ANTIPATTERN_71_TOKENS },
    Antipattern { rule_id: "A_MY", tokens: ANTIPATTERN_72_TOKENS },
    Antipattern { rule_id: "A_MY", tokens: ANTIPATTERN_73_TOKENS },
    Antipattern { rule_id: "A_MY", tokens: ANTIPATTERN_74_TOKENS },
    Antipattern { rule_id: "A_MY", tokens: ANTIPATTERN_75_TOKENS },
    Antipattern { rule_id: "A_NNS_AND", tokens: ANTIPATTERN_76_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_77_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_78_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_79_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_80_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_81_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_82_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_83_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_84_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_85_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_86_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_87_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_88_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_89_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_90_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_91_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_92_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_93_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_94_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_95_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_96_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_97_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_98_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_99_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_100_TOKENS },
    Antipattern { rule_id: "A_RB_NN", tokens: ANTIPATTERN_101_TOKENS },
    Antipattern { rule_id: "A_SCISSOR", tokens: ANTIPATTERN_102_TOKENS },
    Antipattern { rule_id: "A_SCISSOR", tokens: ANTIPATTERN_103_TOKENS },
    Antipattern { rule_id: "A_THANK_YOU", tokens: ANTIPATTERN_104_TOKENS },
    Antipattern { rule_id: "A_TO", tokens: ANTIPATTERN_105_TOKENS },
    Antipattern { rule_id: "A_TO", tokens: ANTIPATTERN_106_TOKENS },
    Antipattern { rule_id: "A_TO", tokens: ANTIPATTERN_107_TOKENS },
    Antipattern { rule_id: "A_TO", tokens: ANTIPATTERN_108_TOKENS },
    Antipattern { rule_id: "BACK_COMPOUNDS", tokens: ANTIPATTERN_109_TOKENS },
    Antipattern { rule_id: "BACK_COMPOUNDS", tokens: ANTIPATTERN_110_TOKENS },
    Antipattern { rule_id: "BACK_COMPOUNDS", tokens: ANTIPATTERN_111_TOKENS },
    Antipattern { rule_id: "BACK_COMPOUNDS", tokens: ANTIPATTERN_112_TOKENS },
    Antipattern { rule_id: "BACK_COMPOUNDS", tokens: ANTIPATTERN_113_TOKENS },
    Antipattern { rule_id: "BACK_COMPOUNDS", tokens: ANTIPATTERN_114_TOKENS },
    Antipattern { rule_id: "BAYERN", tokens: ANTIPATTERN_115_TOKENS },
    Antipattern { rule_id: "BAYERN", tokens: ANTIPATTERN_116_TOKENS },
    Antipattern { rule_id: "BAYERN", tokens: ANTIPATTERN_117_TOKENS },
    Antipattern { rule_id: "BAYERN", tokens: ANTIPATTERN_118_TOKENS },
    Antipattern { rule_id: "BAYERN", tokens: ANTIPATTERN_119_TOKENS },
    Antipattern { rule_id: "BAY_AREA", tokens: ANTIPATTERN_120_TOKENS },
    Antipattern { rule_id: "BAY_AREA", tokens: ANTIPATTERN_121_TOKENS },
    Antipattern { rule_id: "BEAN_BEEN", tokens: ANTIPATTERN_122_TOKENS },
    Antipattern { rule_id: "BEAN_BEEN", tokens: ANTIPATTERN_123_TOKENS },
    Antipattern { rule_id: "BEGINNING_TO_ADDING_BROAD", tokens: ANTIPATTERN_124_TOKENS },
    Antipattern { rule_id: "BELIEF_BELIEVE", tokens: ANTIPATTERN_125_TOKENS },
    Antipattern { rule_id: "BELIEF_BELIEVE", tokens: ANTIPATTERN_126_TOKENS },
    Antipattern { rule_id: "BELIEVE_BELIEF", tokens: ANTIPATTERN_127_TOKENS },
    Antipattern { rule_id: "BELIEVE_TO_IN", tokens: ANTIPATTERN_128_TOKENS },
    Antipattern { rule_id: "BENDED", tokens: ANTIPATTERN_129_TOKENS },
    Antipattern { rule_id: "BESIDES_THE_POINT", tokens: ANTIPATTERN_130_TOKENS },
    Antipattern { rule_id: "BEWARE_PREPOSITION", tokens: ANTIPATTERN_131_TOKENS },
    Antipattern { rule_id: "BE_INTEREST_IN", tokens: ANTIPATTERN_132_TOKENS },
    Antipattern { rule_id: "BE_INTEREST_IN", tokens: ANTIPATTERN_133_TOKENS },
    Antipattern { rule_id: "BE_NO_VB", tokens: ANTIPATTERN_134_TOKENS },
    Antipattern { rule_id: "BE_NO_VB", tokens: ANTIPATTERN_135_TOKENS },
    Antipattern { rule_id: "BE_NO_VB", tokens: ANTIPATTERN_136_TOKENS },
    Antipattern { rule_id: "BE_NO_VB", tokens: ANTIPATTERN_137_TOKENS },
    Antipattern { rule_id: "BE_NO_VB", tokens: ANTIPATTERN_138_TOKENS },
    Antipattern { rule_id: "BE_NO_VB", tokens: ANTIPATTERN_139_TOKENS },
    Antipattern { rule_id: "BE_NO_VB", tokens: ANTIPATTERN_140_TOKENS },
    Antipattern { rule_id: "BE_RB_BE", tokens: ANTIPATTERN_141_TOKENS },
    Antipattern { rule_id: "BE_TO_VBG", tokens: ANTIPATTERN_142_TOKENS },
    Antipattern { rule_id: "BE_TO_VBG", tokens: ANTIPATTERN_143_TOKENS },
    Antipattern { rule_id: "BE_WILL", tokens: ANTIPATTERN_144_TOKENS },
    Antipattern { rule_id: "BIS_BUS", tokens: ANTIPATTERN_145_TOKENS },
    Antipattern { rule_id: "BLACK_SEA", tokens: ANTIPATTERN_146_TOKENS },
    Antipattern { rule_id: "BLACK_SEA", tokens: ANTIPATTERN_147_TOKENS },
    Antipattern { rule_id: "BOEING_737_MAX", tokens: ANTIPATTERN_148_TOKENS },
    Antipattern { rule_id: "BON_APPETITE", tokens: ANTIPATTERN_149_TOKENS },
    Antipattern { rule_id: "BREAKER_COMPOUNDS", tokens: ANTIPATTERN_150_TOKENS },
    Antipattern { rule_id: "BROUGHT_THEM_IN_THE_AIRPORT", tokens: ANTIPATTERN_151_TOKENS },
    Antipattern { rule_id: "BU", tokens: ANTIPATTERN_152_TOKENS },
    Antipattern { rule_id: "BU", tokens: ANTIPATTERN_153_TOKENS },
    Antipattern { rule_id: "BUY_VBG", tokens: ANTIPATTERN_154_TOKENS },
    Antipattern { rule_id: "BUY_VBG", tokens: ANTIPATTERN_155_TOKENS },
    Antipattern { rule_id: "CALL_OF_DUTY", tokens: ANTIPATTERN_156_TOKENS },
    Antipattern { rule_id: "CAN_BACKUP", tokens: ANTIPATTERN_157_TOKENS },
    Antipattern { rule_id: "CAN_CHECKOUT", tokens: ANTIPATTERN_158_TOKENS },
    Antipattern { rule_id: "CAN_MISSPELLING", tokens: ANTIPATTERN_159_TOKENS },
    Antipattern { rule_id: "CAREFUL_FOR_WITH", tokens: ANTIPATTERN_160_TOKENS },
    Antipattern { rule_id: "CARNEGIE_MELLON", tokens: ANTIPATTERN_161_TOKENS },
    Antipattern { rule_id: "CARRIES_CARIES", tokens: ANTIPATTERN_162_TOKENS },
    Antipattern { rule_id: "CATCH_ALL_HYPHEN", tokens: ANTIPATTERN_163_TOKENS },
    Antipattern { rule_id: "CA_COAST_TO_COAST", tokens: ANTIPATTERN_164_TOKENS },
    Antipattern { rule_id: "CC_IS_VBZ", tokens: ANTIPATTERN_165_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_166_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_167_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_168_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_169_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_170_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_171_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_172_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_173_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_174_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_175_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_176_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_177_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_178_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_179_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_180_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_181_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_182_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_183_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_184_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_185_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_186_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_187_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_188_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_189_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_190_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_191_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_192_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_193_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_194_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_195_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_196_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_197_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_198_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_199_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_200_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_201_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_202_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_203_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_204_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_205_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_206_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_207_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_208_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_209_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_210_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_211_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_212_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_213_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_214_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_215_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_216_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_217_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_218_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_219_TOKENS },
    Antipattern { rule_id: "CD_NN", tokens: ANTIPATTERN_220_TOKENS },
    Antipattern { rule_id: "CD_TH", tokens: ANTIPATTERN_221_TOKENS },
    Antipattern { rule_id: "CD_TH", tokens: ANTIPATTERN_222_TOKENS },
    Antipattern { rule_id: "CD_TH", tokens: ANTIPATTERN_223_TOKENS },
    Antipattern { rule_id: "CD_TH", tokens: ANTIPATTERN_224_TOKENS },
    Antipattern { rule_id: "CLICK_THROUGH_RATE", tokens: ANTIPATTERN_225_TOKENS },
    Antipattern { rule_id: "COLLECTIVE_NOUN_VERB_AGREEMENT_VBP", tokens: ANTIPATTERN_226_TOKENS },
    Antipattern { rule_id: "COLLECTIVE_NOUN_VERB_AGREEMENT_VBP", tokens: ANTIPATTERN_227_TOKENS },
    Antipattern { rule_id: "COME_IN_TO", tokens: ANTIPATTERN_228_TOKENS },
    Antipattern { rule_id: "COME_TO_PLANE", tokens: ANTIPATTERN_229_TOKENS },
    Antipattern { rule_id: "COMMA_PERIOD", tokens: ANTIPATTERN_230_TOKENS },
    Antipattern { rule_id: "COMMA_PERIOD", tokens: ANTIPATTERN_231_TOKENS },
    Antipattern { rule_id: "COMMA_PERIOD", tokens: ANTIPATTERN_232_TOKENS },
    Antipattern { rule_id: "COMMA_PERIOD", tokens: ANTIPATTERN_233_TOKENS },
    Antipattern { rule_id: "COMMA_PERIOD", tokens: ANTIPATTERN_234_TOKENS },
    Antipattern { rule_id: "COMMA_PERIOD", tokens: ANTIPATTERN_235_TOKENS },
    Antipattern { rule_id: "COMMA_WHICH", tokens: ANTIPATTERN_236_TOKENS },
    Antipattern { rule_id: "CONFUSION_OF_WHEN_WHAT", tokens: ANTIPATTERN_237_TOKENS },
    Antipattern { rule_id: "CONSIST_TO_OF", tokens: ANTIPATTERN_238_TOKENS },
    Antipattern { rule_id: "COUNTER_COMPOUNDS", tokens: ANTIPATTERN_239_TOKENS },
    Antipattern { rule_id: "COUPLE_OF_TIMES", tokens: ANTIPATTERN_240_TOKENS },
    Antipattern { rule_id: "CRAVE_FOR", tokens: ANTIPATTERN_241_TOKENS },
    Antipattern { rule_id: "CRAVE_FOR", tokens: ANTIPATTERN_242_TOKENS },
    Antipattern { rule_id: "CURRENCY_SPACE", tokens: ANTIPATTERN_243_TOKENS },
    Antipattern { rule_id: "CYBER_COMPOUNDS", tokens: ANTIPATTERN_244_TOKENS },
    Antipattern { rule_id: "CYBER_COMPOUNDS", tokens: ANTIPATTERN_245_TOKENS },
    Antipattern { rule_id: "DAMAGE_OF_TO", tokens: ANTIPATTERN_246_TOKENS },
    Antipattern { rule_id: "DEUS_EX_MACHINA", tokens: ANTIPATTERN_247_TOKENS },
    Antipattern { rule_id: "DID_FOUND_AMBIGUOUS", tokens: ANTIPATTERN_248_TOKENS },
    Antipattern { rule_id: "DOG_COMPOUNDS", tokens: ANTIPATTERN_249_TOKENS },
    Antipattern { rule_id: "DOG_EAT_DOG_HYPHEN", tokens: ANTIPATTERN_250_TOKENS },
    Antipattern { rule_id: "DOUBLE_AUX", tokens: ANTIPATTERN_251_TOKENS },
    Antipattern { rule_id: "DOUBT_FOR_IN", tokens: ANTIPATTERN_252_TOKENS },
    Antipattern { rule_id: "DOUBT_FOR_IN", tokens: ANTIPATTERN_253_TOKENS },
    Antipattern { rule_id: "DOUBT_FOR_IN", tokens: ANTIPATTERN_254_TOKENS },
    Antipattern { rule_id: "DOUBT_FOR_IN", tokens: ANTIPATTERN_255_TOKENS },
    Antipattern { rule_id: "DOWN_COMPOUNDS", tokens: ANTIPATTERN_256_TOKENS },
    Antipattern { rule_id: "DOWN_COMPOUNDS", tokens: ANTIPATTERN_257_TOKENS },
    Antipattern { rule_id: "DOWN_COMPOUNDS", tokens: ANTIPATTERN_258_TOKENS },
    Antipattern { rule_id: "DOWN_COMPOUNDS", tokens: ANTIPATTERN_259_TOKENS },
    Antipattern { rule_id: "DOWN_COMPOUNDS", tokens: ANTIPATTERN_260_TOKENS },
    Antipattern { rule_id: "DOWN_SIDE", tokens: ANTIPATTERN_261_TOKENS },
    Antipattern { rule_id: "DO_VBZ", tokens: ANTIPATTERN_262_TOKENS },
    Antipattern { rule_id: "DO_VBZ", tokens: ANTIPATTERN_263_TOKENS },
    Antipattern { rule_id: "DO_VBZ", tokens: ANTIPATTERN_264_TOKENS },
    Antipattern { rule_id: "DO_YOU_WHAT", tokens: ANTIPATTERN_265_TOKENS },
    Antipattern { rule_id: "DRESS_WITH_IN", tokens: ANTIPATTERN_266_TOKENS },
    Antipattern { rule_id: "DRESS_WITH_IN", tokens: ANTIPATTERN_267_TOKENS },
    Antipattern { rule_id: "DROP_DEAD_HYPHEN", tokens: ANTIPATTERN_268_TOKENS },
    Antipattern { rule_id: "DROP_DEAD_HYPHEN", tokens: ANTIPATTERN_269_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_270_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_271_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_272_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_273_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_274_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_275_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_276_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_277_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_278_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_279_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_280_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_281_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_282_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_283_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_284_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_285_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_286_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_287_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_288_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_289_TOKENS },
    Antipattern { rule_id: "DT_PRP", tokens: ANTIPATTERN_290_TOKENS },
    Antipattern { rule_id: "DT_RB_IN", tokens: ANTIPATTERN_291_TOKENS },
    Antipattern { rule_id: "DT_RB_IN", tokens: ANTIPATTERN_292_TOKENS },
    Antipattern { rule_id: "EARL_GREY", tokens: ANTIPATTERN_293_TOKENS },
    Antipattern { rule_id: "EDGAR_ALLAN_POE", tokens: ANTIPATTERN_294_TOKENS },
    Antipattern { rule_id: "EMMANUEL_MACRON", tokens: ANTIPATTERN_295_TOKENS },
    Antipattern { rule_id: "ENTER_IN", tokens: ANTIPATTERN_296_TOKENS },
    Antipattern { rule_id: "ENTER_IN", tokens: ANTIPATTERN_297_TOKENS },
    Antipattern { rule_id: "ENUMERATION_AND_DASHES", tokens: ANTIPATTERN_298_TOKENS },
    Antipattern { rule_id: "ER", tokens: ANTIPATTERN_299_TOKENS },
    Antipattern { rule_id: "ER", tokens: ANTIPATTERN_300_TOKENS },
    Antipattern { rule_id: "ER", tokens: ANTIPATTERN_301_TOKENS },
    Antipattern { rule_id: "EVEN_HANDED_HYPHEN", tokens: ANTIPATTERN_302_TOKENS },
    Antipattern { rule_id: "EVEN_HANDED_HYPHEN", tokens: ANTIPATTERN_303_TOKENS },
    Antipattern { rule_id: "EVERY_BODY", tokens: ANTIPATTERN_304_TOKENS },
    Antipattern { rule_id: "EVERY_BODY", tokens: ANTIPATTERN_305_TOKENS },
    Antipattern { rule_id: "EVERY_BODY", tokens: ANTIPATTERN_306_TOKENS },
    Antipattern { rule_id: "EVERY_BODY", tokens: ANTIPATTERN_307_TOKENS },
    Antipattern { rule_id: "EVERY_EACH_SINGULAR", tokens: ANTIPATTERN_308_TOKENS },
    Antipattern { rule_id: "EXCITED_FOR", tokens: ANTIPATTERN_309_TOKENS },
    Antipattern { rule_id: "EXCITED_FOR", tokens: ANTIPATTERN_310_TOKENS },
    Antipattern { rule_id: "EXCITED_FOR", tokens: ANTIPATTERN_311_TOKENS },
    Antipattern { rule_id: "EXCITED_FOR", tokens: ANTIPATTERN_312_TOKENS },
    Antipattern { rule_id: "EXCITED_FOR", tokens: ANTIPATTERN_313_TOKENS },
    Antipattern { rule_id: "EXCITED_FOR", tokens: ANTIPATTERN_314_TOKENS },
    Antipattern { rule_id: "EXCITED_FOR", tokens: ANTIPATTERN_315_TOKENS },
    Antipattern { rule_id: "FAIR_SURE", tokens: ANTIPATTERN_316_TOKENS },
    Antipattern { rule_id: "FAR_OF_FROM", tokens: ANTIPATTERN_317_TOKENS },
    Antipattern { rule_id: "FAR_OF_FROM", tokens: ANTIPATTERN_318_TOKENS },
    Antipattern { rule_id: "FEE_FREE", tokens: ANTIPATTERN_319_TOKENS },
    Antipattern { rule_id: "FEE_FREE", tokens: ANTIPATTERN_320_TOKENS },
    Antipattern { rule_id: "FIGHTER_COMPOUNDS", tokens: ANTIPATTERN_321_TOKENS },
    Antipattern { rule_id: "FIGURE_HYPHEN", tokens: ANTIPATTERN_322_TOKENS },
    Antipattern { rule_id: "FILE_EXTENSIONS_CASE", tokens: ANTIPATTERN_323_TOKENS },
    Antipattern { rule_id: "FILE_EXTENSIONS_CASE", tokens: ANTIPATTERN_324_TOKENS },
    Antipattern { rule_id: "FILE_EXTENSIONS_CASE", tokens: ANTIPATTERN_325_TOKENS },
    Antipattern { rule_id: "FILE_EXTENSIONS_CASE", tokens: ANTIPATTERN_326_TOKENS },
    Antipattern { rule_id: "FILE_EXTENSIONS_CASE", tokens: ANTIPATTERN_327_TOKENS },
    Antipattern { rule_id: "FILE_EXTENSIONS_CASE", tokens: ANTIPATTERN_328_TOKENS },
    Antipattern { rule_id: "FILL_OF_WITH", tokens: ANTIPATTERN_329_TOKENS },
    Antipattern { rule_id: "FILL_OF_WITH", tokens: ANTIPATTERN_330_TOKENS },
    Antipattern { rule_id: "FILL_OF_WITH", tokens: ANTIPATTERN_331_TOKENS },
    Antipattern { rule_id: "FINAL_ADVERB_COMMA", tokens: ANTIPATTERN_332_TOKENS },
    Antipattern { rule_id: "FINAL_ADVERB_COMMA", tokens: ANTIPATTERN_333_TOKENS },
    Antipattern { rule_id: "FINAL_ADVERB_COMMA", tokens: ANTIPATTERN_334_TOKENS },
    Antipattern { rule_id: "FINAL_ADVERB_COMMA", tokens: ANTIPATTERN_335_TOKENS },
    Antipattern { rule_id: "FINAL_ADVERB_COMMA", tokens: ANTIPATTERN_336_TOKENS },
    Antipattern { rule_id: "FINAL_ADVERB_COMMA", tokens: ANTIPATTERN_337_TOKENS },
    Antipattern { rule_id: "FOLLOW_A_COURSE", tokens: ANTIPATTERN_338_TOKENS },
    Antipattern { rule_id: "FOR_FRO", tokens: ANTIPATTERN_339_TOKENS },
    Antipattern { rule_id: "FREQUENT_ASKED_QUESTIONS", tokens: ANTIPATTERN_340_TOKENS },
    Antipattern { rule_id: "FROM_X_Y", tokens: ANTIPATTERN_341_TOKENS },
    Antipattern { rule_id: "FROM_X_Y", tokens: ANTIPATTERN_342_TOKENS },
    Antipattern { rule_id: "FULL_FILL", tokens: ANTIPATTERN_343_TOKENS },
    Antipattern { rule_id: "FULL_TIME", tokens: ANTIPATTERN_344_TOKENS },
    Antipattern { rule_id: "FULL_TIME", tokens: ANTIPATTERN_345_TOKENS },
    Antipattern { rule_id: "FULL_TIME", tokens: ANTIPATTERN_346_TOKENS },
    Antipattern { rule_id: "FULL_WITH_OF", tokens: ANTIPATTERN_347_TOKENS },
    Antipattern { rule_id: "GOD_COMMA", tokens: ANTIPATTERN_348_TOKENS },
    Antipattern { rule_id: "GOD_COMMA", tokens: ANTIPATTERN_349_TOKENS },
    Antipattern { rule_id: "GOING_TO_VACATION", tokens: ANTIPATTERN_350_TOKENS },
    Antipattern { rule_id: "GOOD_IN_AT_GERUND", tokens: ANTIPATTERN_351_TOKENS },
    Antipattern { rule_id: "GOT_IT_DONE", tokens: ANTIPATTERN_352_TOKENS },
    Antipattern { rule_id: "GO_FOR_IT_GIRLS_COMMA", tokens: ANTIPATTERN_353_TOKENS },
    Antipattern { rule_id: "GO_GERUND", tokens: ANTIPATTERN_354_TOKENS },
    Antipattern { rule_id: "GO_TO_HOME", tokens: ANTIPATTERN_355_TOKENS },
    Antipattern { rule_id: "GO_TO_HOME", tokens: ANTIPATTERN_356_TOKENS },
    Antipattern { rule_id: "GREYS_ANATOMY", tokens: ANTIPATTERN_357_TOKENS },
    Antipattern { rule_id: "GUILTY_FOR_OF", tokens: ANTIPATTERN_358_TOKENS },
    Antipattern { rule_id: "GUILTY_FOR_OF", tokens: ANTIPATTERN_359_TOKENS },
    Antipattern { rule_id: "G_MAIL", tokens: ANTIPATTERN_360_TOKENS },
    Antipattern { rule_id: "HAD_VBP", tokens: ANTIPATTERN_361_TOKENS },
    Antipattern { rule_id: "HAD_VBP", tokens: ANTIPATTERN_362_TOKENS },
    Antipattern { rule_id: "HARPERS_BAZAAR", tokens: ANTIPATTERN_363_TOKENS },
    Antipattern { rule_id: "HART_HEART", tokens: ANTIPATTERN_364_TOKENS },
    Antipattern { rule_id: "HAS_IT_NNS__IT_ITS", tokens: ANTIPATTERN_365_TOKENS },
    Antipattern { rule_id: "HEAD_COMPOUNDS", tokens: ANTIPATTERN_366_TOKENS },
    Antipattern { rule_id: "HEAD_COMPOUNDS", tokens: ANTIPATTERN_367_TOKENS },
    Antipattern { rule_id: "HEAD_COMPOUNDS", tokens: ANTIPATTERN_368_TOKENS },
    Antipattern { rule_id: "HEAD_COMPOUNDS", tokens: ANTIPATTERN_369_TOKENS },
    Antipattern { rule_id: "HELP_NP_VBZ", tokens: ANTIPATTERN_370_TOKENS },
    Antipattern { rule_id: "HEP", tokens: ANTIPATTERN_371_TOKENS },
    Antipattern { rule_id: "HE_BE", tokens: ANTIPATTERN_372_TOKENS },
    Antipattern { rule_id: "HE_NEED", tokens: ANTIPATTERN_373_TOKENS },
    Antipattern { rule_id: "HE_NEED", tokens: ANTIPATTERN_374_TOKENS },
    Antipattern { rule_id: "HE_NEED", tokens: ANTIPATTERN_375_TOKENS },
    Antipattern { rule_id: "HE_NEED", tokens: ANTIPATTERN_376_TOKENS },
    Antipattern { rule_id: "HE_QUESTION", tokens: ANTIPATTERN_377_TOKENS },
    Antipattern { rule_id: "HE_QUESTION", tokens: ANTIPATTERN_378_TOKENS },
    Antipattern { rule_id: "HE_QUESTION", tokens: ANTIPATTERN_379_TOKENS },
    Antipattern { rule_id: "HE_QUESTION", tokens: ANTIPATTERN_380_TOKENS },
    Antipattern { rule_id: "HING", tokens: ANTIPATTERN_381_TOKENS },
    Antipattern { rule_id: "HING", tokens: ANTIPATTERN_382_TOKENS },
    Antipattern { rule_id: "HI_TIME", tokens: ANTIPATTERN_383_TOKENS },
    Antipattern { rule_id: "HOLDER_COMPOUNDS", tokens: ANTIPATTERN_384_TOKENS },
    Antipattern { rule_id: "HOLDER_COMPOUNDS", tokens: ANTIPATTERN_385_TOKENS },
    Antipattern { rule_id: "HOME_COMPOUNDS", tokens: ANTIPATTERN_386_TOKENS },
    Antipattern { rule_id: "HOME_COMPOUNDS", tokens: ANTIPATTERN_387_TOKENS },
    Antipattern { rule_id: "HOME_COMPOUNDS", tokens: ANTIPATTERN_388_TOKENS },
    Antipattern { rule_id: "HOME_COMPOUNDS", tokens: ANTIPATTERN_389_TOKENS },
    Antipattern { rule_id: "HOME_COMPOUNDS", tokens: ANTIPATTERN_390_TOKENS },
    Antipattern { rule_id: "HOME_COMPOUNDS", tokens: ANTIPATTERN_391_TOKENS },
    Antipattern { rule_id: "HOOD_COMPOUNDS", tokens: ANTIPATTERN_392_TOKENS },
    Antipattern { rule_id: "HOUR_HYPHEN", tokens: ANTIPATTERN_393_TOKENS },
    Antipattern { rule_id: "HOUR_HYPHEN", tokens: ANTIPATTERN_394_TOKENS },
    Antipattern { rule_id: "HOUSE_COMPOUNDS", tokens: ANTIPATTERN_395_TOKENS },
    Antipattern { rule_id: "HOUSE_COMPOUNDS", tokens: ANTIPATTERN_396_TOKENS },
    Antipattern { rule_id: "HOUSE_COMPOUNDS", tokens: ANTIPATTERN_397_TOKENS },
    Antipattern { rule_id: "HOUSE_COMPOUNDS", tokens: ANTIPATTERN_398_TOKENS },
    Antipattern { rule_id: "HWY_WHY", tokens: ANTIPATTERN_399_TOKENS },
    Antipattern { rule_id: "HWY_WHY", tokens: ANTIPATTERN_400_TOKENS },
    Antipattern { rule_id: "HWY_WHY", tokens: ANTIPATTERN_401_TOKENS },
    Antipattern { rule_id: "HWY_WHY", tokens: ANTIPATTERN_402_TOKENS },
    Antipattern { rule_id: "HWY_WHY", tokens: ANTIPATTERN_403_TOKENS },
    Antipattern { rule_id: "IF_VB_PCT", tokens: ANTIPATTERN_404_TOKENS },
    Antipattern { rule_id: "IMITATION_FROM_OF", tokens: ANTIPATTERN_405_TOKENS },
    Antipattern { rule_id: "IM_I_M", tokens: ANTIPATTERN_406_TOKENS },
    Antipattern { rule_id: "IM_I_M", tokens: ANTIPATTERN_407_TOKENS },
    Antipattern { rule_id: "IM_I_M", tokens: ANTIPATTERN_408_TOKENS },
    Antipattern { rule_id: "INTERESTED_BY", tokens: ANTIPATTERN_409_TOKENS },
    Antipattern { rule_id: "INTERESTED_BY", tokens: ANTIPATTERN_410_TOKENS },
    Antipattern { rule_id: "INTERESTED_BY", tokens: ANTIPATTERN_411_TOKENS },
    Antipattern { rule_id: "INTEREST_FOR_IN", tokens: ANTIPATTERN_412_TOKENS },
    Antipattern { rule_id: "INTEREST_FOR_IN", tokens: ANTIPATTERN_413_TOKENS },
    Antipattern { rule_id: "INTEREST_FOR_IN", tokens: ANTIPATTERN_414_TOKENS },
    Antipattern { rule_id: "INVOICE_OF_FOR", tokens: ANTIPATTERN_415_TOKENS },
    Antipattern { rule_id: "IN_A_TROUBLE", tokens: ANTIPATTERN_416_TOKENS },
    Antipattern { rule_id: "IN_A_TROUBLE", tokens: ANTIPATTERN_417_TOKENS },
    Antipattern { rule_id: "IN_CHARGE_OF_FROM", tokens: ANTIPATTERN_418_TOKENS },
    Antipattern { rule_id: "IN_FACEBOOK", tokens: ANTIPATTERN_419_TOKENS },
    Antipattern { rule_id: "IN_FACEBOOK", tokens: ANTIPATTERN_420_TOKENS },
    Antipattern { rule_id: "IN_FRONT_OF", tokens: ANTIPATTERN_421_TOKENS },
    Antipattern { rule_id: "IN_JANUARY", tokens: ANTIPATTERN_422_TOKENS },
    Antipattern { rule_id: "IN_ON_AN_ALBUM", tokens: ANTIPATTERN_423_TOKENS },
    Antipattern { rule_id: "IN_TERM_OF_PHRASE", tokens: ANTIPATTERN_424_TOKENS },
    Antipattern { rule_id: "IN_THE_INTERNET", tokens: ANTIPATTERN_425_TOKENS },
    Antipattern { rule_id: "IN_THE_INTERNET", tokens: ANTIPATTERN_426_TOKENS },
    Antipattern { rule_id: "IN_THE_INTERNET", tokens: ANTIPATTERN_427_TOKENS },
    Antipattern { rule_id: "IN_THE_LONG_TERMS", tokens: ANTIPATTERN_428_TOKENS },
    Antipattern { rule_id: "IN_WEBSITE", tokens: ANTIPATTERN_429_TOKENS },
    Antipattern { rule_id: "IN_WEBSITE", tokens: ANTIPATTERN_430_TOKENS },
    Antipattern { rule_id: "IN_WINDOWS", tokens: ANTIPATTERN_431_TOKENS },
    Antipattern { rule_id: "IN_WINDOWS", tokens: ANTIPATTERN_432_TOKENS },
    Antipattern { rule_id: "IS_AND_ARE", tokens: ANTIPATTERN_433_TOKENS },
    Antipattern { rule_id: "IS_SHOULD", tokens: ANTIPATTERN_434_TOKENS },
    Antipattern { rule_id: "ITS_JJ_NNSNN", tokens: ANTIPATTERN_435_TOKENS },
    Antipattern { rule_id: "ITS_JJ_NNSNN", tokens: ANTIPATTERN_436_TOKENS },
    Antipattern { rule_id: "IT_IS_JJ_TO_VBG", tokens: ANTIPATTERN_437_TOKENS },
    Antipattern { rule_id: "IT_TIME_TO", tokens: ANTIPATTERN_438_TOKENS },
    Antipattern { rule_id: "IT_TIME_TO", tokens: ANTIPATTERN_439_TOKENS },
    Antipattern { rule_id: "IT_TIME_TO", tokens: ANTIPATTERN_440_TOKENS },
    Antipattern { rule_id: "IT_TIME_TO", tokens: ANTIPATTERN_441_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_442_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_443_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_444_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_445_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_446_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_447_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_448_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_449_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_450_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_451_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_452_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_453_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_454_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_455_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_456_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_457_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_458_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_459_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_460_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_461_TOKENS },
    Antipattern { rule_id: "IT_VBZ", tokens: ANTIPATTERN_462_TOKENS },
    Antipattern { rule_id: "IVE_CONTRACTION", tokens: ANTIPATTERN_463_TOKENS },
    Antipattern { rule_id: "I_AM_WORRY", tokens: ANTIPATTERN_464_TOKENS },
    Antipattern { rule_id: "I_AS_LOOKING", tokens: ANTIPATTERN_465_TOKENS },
    Antipattern { rule_id: "I_AS_LOOKING", tokens: ANTIPATTERN_466_TOKENS },
    Antipattern { rule_id: "I_PERSONAL", tokens: ANTIPATTERN_467_TOKENS },
    Antipattern { rule_id: "JAPAN", tokens: ANTIPATTERN_468_TOKENS },
    Antipattern { rule_id: "JAPAN", tokens: ANTIPATTERN_469_TOKENS },
    Antipattern { rule_id: "JAPAN", tokens: ANTIPATTERN_470_TOKENS },
    Antipattern { rule_id: "JENNIFER_ANISTON", tokens: ANTIPATTERN_471_TOKENS },
    Antipattern { rule_id: "KEEPER_COMPOUNDS", tokens: ANTIPATTERN_472_TOKENS },
    Antipattern { rule_id: "KEEPER_COMPOUNDS", tokens: ANTIPATTERN_473_TOKENS },
    Antipattern { rule_id: "KEEPER_COMPOUNDS", tokens: ANTIPATTERN_474_TOKENS },
    Antipattern { rule_id: "KEEPER_COMPOUNDS", tokens: ANTIPATTERN_475_TOKENS },
    Antipattern { rule_id: "KEEPER_COMPOUNDS", tokens: ANTIPATTERN_476_TOKENS },
    Antipattern { rule_id: "KEEP_SEEING", tokens: ANTIPATTERN_477_TOKENS },
    Antipattern { rule_id: "KEY_STOKE", tokens: ANTIPATTERN_478_TOKENS },
    Antipattern { rule_id: "KIND_OF_A", tokens: ANTIPATTERN_479_TOKENS },
    Antipattern { rule_id: "KIND_OF_A", tokens: ANTIPATTERN_480_TOKENS },
    Antipattern { rule_id: "KIND_WITH_TO", tokens: ANTIPATTERN_481_TOKENS },
    Antipattern { rule_id: "LAW_COMPOUNDS", tokens: ANTIPATTERN_482_TOKENS },
    Antipattern { rule_id: "LA_PAZ", tokens: ANTIPATTERN_483_TOKENS },
    Antipattern { rule_id: "LA_PAZ", tokens: ANTIPATTERN_484_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_485_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_486_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_487_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_488_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_489_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_490_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_491_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_492_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_493_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_494_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_495_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_496_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_497_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_498_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_499_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_500_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_501_TOKENS },
    Antipattern { rule_id: "LC_AFTER_PERIOD", tokens: ANTIPATTERN_502_TOKENS },
    Antipattern { rule_id: "LEARN_NNNNS_ON_DO", tokens: ANTIPATTERN_503_TOKENS },
    Antipattern { rule_id: "LEHMANN_BROTHERS", tokens: ANTIPATTERN_504_TOKENS },
    Antipattern { rule_id: "LEROY_SANE", tokens: ANTIPATTERN_505_TOKENS },
    Antipattern { rule_id: "LESS_COMPARATIVE", tokens: ANTIPATTERN_506_TOKENS },
    Antipattern { rule_id: "LESS_MORE_THEN", tokens: ANTIPATTERN_507_TOKENS },
    Antipattern { rule_id: "LESS_MORE_THEN", tokens: ANTIPATTERN_508_TOKENS },
    Antipattern { rule_id: "LESS_MORE_THEN", tokens: ANTIPATTERN_509_TOKENS },
    Antipattern { rule_id: "LESS_MORE_THEN", tokens: ANTIPATTERN_510_TOKENS },
    Antipattern { rule_id: "LEST_LAST", tokens: ANTIPATTERN_511_TOKENS },
    Antipattern { rule_id: "LET_IT_INFINITIVE", tokens: ANTIPATTERN_512_TOKENS },
    Antipattern { rule_id: "LET_ME_TROUGH", tokens: ANTIPATTERN_513_TOKENS },
    Antipattern { rule_id: "LET_OBJECT", tokens: ANTIPATTERN_514_TOKENS },
    Antipattern { rule_id: "LIFE_COMPOUNDS", tokens: ANTIPATTERN_515_TOKENS },
    Antipattern { rule_id: "LIFE_COMPOUNDS", tokens: ANTIPATTERN_516_TOKENS },
    Antipattern { rule_id: "LIFE_COMPOUNDS", tokens: ANTIPATTERN_517_TOKENS },
    Antipattern { rule_id: "LIGHT_COMPOUNDS", tokens: ANTIPATTERN_518_TOKENS },
    Antipattern { rule_id: "LIGHT_COMPOUNDS", tokens: ANTIPATTERN_519_TOKENS },
    Antipattern { rule_id: "LIGHT_COMPOUNDS", tokens: ANTIPATTERN_520_TOKENS },
    Antipattern { rule_id: "LIGHT_COMPOUNDS", tokens: ANTIPATTERN_521_TOKENS },
    Antipattern { rule_id: "LIGHT_COMPOUNDS", tokens: ANTIPATTERN_522_TOKENS },
    Antipattern { rule_id: "LIGHT_COMPOUNDS", tokens: ANTIPATTERN_523_TOKENS },
    Antipattern { rule_id: "LINE_COMPOUNDS", tokens: ANTIPATTERN_524_TOKENS },
    Antipattern { rule_id: "LINE_COMPOUNDS", tokens: ANTIPATTERN_525_TOKENS },
    Antipattern { rule_id: "LINE_COMPOUNDS", tokens: ANTIPATTERN_526_TOKENS },
    Antipattern { rule_id: "LINE_COMPOUNDS", tokens: ANTIPATTERN_527_TOKENS },
    Antipattern { rule_id: "LINE_COMPOUNDS", tokens: ANTIPATTERN_528_TOKENS },
    Antipattern { rule_id: "LINE_COMPOUNDS", tokens: ANTIPATTERN_529_TOKENS },
    Antipattern { rule_id: "LONG_COMPOUNDS", tokens: ANTIPATTERN_530_TOKENS },
    Antipattern { rule_id: "LONG_ISLAND_ICED_TEA", tokens: ANTIPATTERN_531_TOKENS },
    Antipattern { rule_id: "LOS_ANGELS", tokens: ANTIPATTERN_532_TOKENS },
    Antipattern { rule_id: "LOS_ANGELS", tokens: ANTIPATTERN_533_TOKENS },
    Antipattern { rule_id: "LOTS_OF_NN", tokens: ANTIPATTERN_534_TOKENS },
    Antipattern { rule_id: "LOTS_OF_NN", tokens: ANTIPATTERN_535_TOKENS },
    Antipattern { rule_id: "LOTS_OF_NN", tokens: ANTIPATTERN_536_TOKENS },
    Antipattern { rule_id: "LOTS_OF_NN", tokens: ANTIPATTERN_537_TOKENS },
    Antipattern { rule_id: "MABEY_MAYBE", tokens: ANTIPATTERN_538_TOKENS },
    Antipattern { rule_id: "MABEY_MAYBE", tokens: ANTIPATTERN_539_TOKENS },
    Antipattern { rule_id: "MABEY_MAYBE", tokens: ANTIPATTERN_540_TOKENS },
    Antipattern { rule_id: "MABEY_MAYBE", tokens: ANTIPATTERN_541_TOKENS },
    Antipattern { rule_id: "MAH", tokens: ANTIPATTERN_542_TOKENS },
    Antipattern { rule_id: "MAH", tokens: ANTIPATTERN_543_TOKENS },
    Antipattern { rule_id: "MAKER_COMPOUNDS", tokens: ANTIPATTERN_544_TOKENS },
    Antipattern { rule_id: "MANOR_MANNER", tokens: ANTIPATTERN_545_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_546_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_547_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_548_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_549_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_550_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_551_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_552_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_553_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_554_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_555_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_556_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_557_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_558_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_559_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_560_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_561_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_562_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_563_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_564_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_565_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_566_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_567_TOKENS },
    Antipattern { rule_id: "MANY_NN", tokens: ANTIPATTERN_568_TOKENS },
    Antipattern { rule_id: "MAN_COMPOUNDS", tokens: ANTIPATTERN_569_TOKENS },
    Antipattern { rule_id: "MARK_COMPOUNDS", tokens: ANTIPATTERN_570_TOKENS },
    Antipattern { rule_id: "MARK_COMPOUNDS", tokens: ANTIPATTERN_571_TOKENS },
    Antipattern { rule_id: "MARK_COMPOUNDS", tokens: ANTIPATTERN_572_TOKENS },
    Antipattern { rule_id: "MA_MY", tokens: ANTIPATTERN_573_TOKENS },
    Antipattern { rule_id: "MA_MY", tokens: ANTIPATTERN_574_TOKENS },
    Antipattern { rule_id: "MA_MY", tokens: ANTIPATTERN_575_TOKENS },
    Antipattern { rule_id: "MA_MY", tokens: ANTIPATTERN_576_TOKENS },
    Antipattern { rule_id: "MA_MY", tokens: ANTIPATTERN_577_TOKENS },
    Antipattern { rule_id: "MA_MY", tokens: ANTIPATTERN_578_TOKENS },
    Antipattern { rule_id: "MA_MY", tokens: ANTIPATTERN_579_TOKENS },
    Antipattern { rule_id: "MA_MY", tokens: ANTIPATTERN_580_TOKENS },
    Antipattern { rule_id: "MA_MY", tokens: ANTIPATTERN_581_TOKENS },
    Antipattern { rule_id: "MA_MY", tokens: ANTIPATTERN_582_TOKENS },
    Antipattern { rule_id: "MD_NO_VB", tokens: ANTIPATTERN_583_TOKENS },
    Antipattern { rule_id: "MEED_MEET", tokens: ANTIPATTERN_584_TOKENS },
    Antipattern { rule_id: "MI", tokens: ANTIPATTERN_585_TOKENS },
    Antipattern { rule_id: "MI", tokens: ANTIPATTERN_586_TOKENS },
    Antipattern { rule_id: "MI", tokens: ANTIPATTERN_587_TOKENS },
    Antipattern { rule_id: "MI", tokens: ANTIPATTERN_588_TOKENS },
    Antipattern { rule_id: "MI", tokens: ANTIPATTERN_589_TOKENS },
    Antipattern { rule_id: "MI", tokens: ANTIPATTERN_590_TOKENS },
    Antipattern { rule_id: "MI", tokens: ANTIPATTERN_591_TOKENS },
    Antipattern { rule_id: "MI", tokens: ANTIPATTERN_592_TOKENS },
    Antipattern { rule_id: "MI", tokens: ANTIPATTERN_593_TOKENS },
    Antipattern { rule_id: "MI", tokens: ANTIPATTERN_594_TOKENS },
    Antipattern { rule_id: "MI", tokens: ANTIPATTERN_595_TOKENS },
    Antipattern { rule_id: "MI", tokens: ANTIPATTERN_596_TOKENS },
    Antipattern { rule_id: "MIAMI_DADE_HYPHEN", tokens: ANTIPATTERN_597_TOKENS },
    Antipattern { rule_id: "MINUETS", tokens: ANTIPATTERN_598_TOKENS },
    Antipattern { rule_id: "MINUETS", tokens: ANTIPATTERN_599_TOKENS },
    Antipattern { rule_id: "MINUETS", tokens: ANTIPATTERN_600_TOKENS },
    Antipattern { rule_id: "MISSING_PREPOSITION", tokens: ANTIPATTERN_601_TOKENS },
    Antipattern { rule_id: "MISSING_PREPOSITION", tokens: ANTIPATTERN_602_TOKENS },
    Antipattern { rule_id: "MISSING_PREPOSITION", tokens: ANTIPATTERN_603_TOKENS },
    Antipattern { rule_id: "MISS_SPELLING", tokens: ANTIPATTERN_604_TOKENS },
    Antipattern { rule_id: "MOTOR_COMPOUNDS", tokens: ANTIPATTERN_605_TOKENS },
    Antipattern { rule_id: "MUCH_MONEY", tokens: ANTIPATTERN_606_TOKENS },
    Antipattern { rule_id: "MUCH_MONEY", tokens: ANTIPATTERN_607_TOKENS },
    Antipattern { rule_id: "MUST_BE_DO", tokens: ANTIPATTERN_608_TOKENS },
    Antipattern { rule_id: "MUST_HAVE_TO", tokens: ANTIPATTERN_609_TOKENS },
    Antipattern { rule_id: "MY_NOT_MU", tokens: ANTIPATTERN_610_TOKENS },
    Antipattern { rule_id: "MY_NOT_MU", tokens: ANTIPATTERN_611_TOKENS },
    Antipattern { rule_id: "NE", tokens: ANTIPATTERN_612_TOKENS },
    Antipattern { rule_id: "NE", tokens: ANTIPATTERN_613_TOKENS },
    Antipattern { rule_id: "NE", tokens: ANTIPATTERN_614_TOKENS },
    Antipattern { rule_id: "NE", tokens: ANTIPATTERN_615_TOKENS },
    Antipattern { rule_id: "NEE", tokens: ANTIPATTERN_616_TOKENS },
    Antipattern { rule_id: "NEE", tokens: ANTIPATTERN_617_TOKENS },
    Antipattern { rule_id: "NEE", tokens: ANTIPATTERN_618_TOKENS },
    Antipattern { rule_id: "NEITHER_NOR", tokens: ANTIPATTERN_619_TOKENS },
    Antipattern { rule_id: "NEWS_COMPOUNDS", tokens: ANTIPATTERN_620_TOKENS },
    Antipattern { rule_id: "NEWS_COMPOUNDS", tokens: ANTIPATTERN_621_TOKENS },
    Antipattern { rule_id: "NEWS_COMPOUNDS", tokens: ANTIPATTERN_622_TOKENS },
    Antipattern { rule_id: "NEW_GUINEA", tokens: ANTIPATTERN_623_TOKENS },
    Antipattern { rule_id: "NIT_NOT", tokens: ANTIPATTERN_624_TOKENS },
    Antipattern { rule_id: "NNS_IN_NNP_VBZ", tokens: ANTIPATTERN_625_TOKENS },
    Antipattern { rule_id: "NNS_THAT_AGREEMENT", tokens: ANTIPATTERN_626_TOKENS },
    Antipattern { rule_id: "NNS_THAT_AGREEMENT", tokens: ANTIPATTERN_627_TOKENS },
    Antipattern { rule_id: "NNS_THAT_AGREEMENT", tokens: ANTIPATTERN_628_TOKENS },
    Antipattern { rule_id: "NNS_THAT_AGREEMENT", tokens: ANTIPATTERN_629_TOKENS },
    Antipattern { rule_id: "NNS_THAT_AGREEMENT", tokens: ANTIPATTERN_630_TOKENS },
    Antipattern { rule_id: "NODT_DOZEN", tokens: ANTIPATTERN_631_TOKENS },
    Antipattern { rule_id: "NODT_DOZEN", tokens: ANTIPATTERN_632_TOKENS },
    Antipattern { rule_id: "NODT_DOZEN", tokens: ANTIPATTERN_633_TOKENS },
    Antipattern { rule_id: "NODT_DOZEN", tokens: ANTIPATTERN_634_TOKENS },
    Antipattern { rule_id: "NODT_DOZEN", tokens: ANTIPATTERN_635_TOKENS },
    Antipattern { rule_id: "NODT_DOZEN", tokens: ANTIPATTERN_636_TOKENS },
    Antipattern { rule_id: "NODT_DOZEN", tokens: ANTIPATTERN_637_TOKENS },
    Antipattern { rule_id: "NODT_DOZEN", tokens: ANTIPATTERN_638_TOKENS },
    Antipattern { rule_id: "NODT_DOZEN", tokens: ANTIPATTERN_639_TOKENS },
    Antipattern { rule_id: "NODT_DOZEN", tokens: ANTIPATTERN_640_TOKENS },
    Antipattern { rule_id: "NODT_DOZEN", tokens: ANTIPATTERN_641_TOKENS },
    Antipattern { rule_id: "NODT_DOZEN", tokens: ANTIPATTERN_642_TOKENS },
    Antipattern { rule_id: "NODT_DOZEN", tokens: ANTIPATTERN_643_TOKENS },
    Antipattern { rule_id: "NOTE_COMPOUNDS", tokens: ANTIPATTERN_644_TOKENS },
    Antipattern { rule_id: "NOT_LONGER", tokens: ANTIPATTERN_645_TOKENS },
    Antipattern { rule_id: "NOT_NEVER", tokens: ANTIPATTERN_646_TOKENS },
    Antipattern { rule_id: "NO_COMMA_BEFORE_INDIRECT_QUESTION", tokens: ANTIPATTERN_647_TOKENS },
    Antipattern { rule_id: "NO_COMMA_BEFORE_INDIRECT_QUESTION", tokens: ANTIPATTERN_648_TOKENS },
    Antipattern { rule_id: "NO_PROBLEM_ET_AL", tokens: ANTIPATTERN_649_TOKENS },
    Antipattern { rule_id: "NO_SPACE_CLOSING_QUOTE", tokens: ANTIPATTERN_650_TOKENS },
    Antipattern { rule_id: "NO_SPACE_CLOSING_QUOTE", tokens: ANTIPATTERN_651_TOKENS },
    Antipattern { rule_id: "NUMBERS_IN_WORDS", tokens: ANTIPATTERN_652_TOKENS },
    Antipattern { rule_id: "NUT_NOT", tokens: ANTIPATTERN_653_TOKENS },
    Antipattern { rule_id: "NUT_NOT", tokens: ANTIPATTERN_654_TOKENS },
    Antipattern { rule_id: "NUT_NOT", tokens: ANTIPATTERN_655_TOKENS },
    Antipattern { rule_id: "NUT_NOT", tokens: ANTIPATTERN_656_TOKENS },
    Antipattern { rule_id: "NUT_NOT", tokens: ANTIPATTERN_657_TOKENS },
    Antipattern { rule_id: "NUT_NOT", tokens: ANTIPATTERN_658_TOKENS },
    Antipattern { rule_id: "NUT_NOT", tokens: ANTIPATTERN_659_TOKENS },
    Antipattern { rule_id: "OBJECTIVE_CASE", tokens: ANTIPATTERN_660_TOKENS },
    Antipattern { rule_id: "OFF_HAND_COMPOUND", tokens: ANTIPATTERN_661_TOKENS },
    Antipattern { rule_id: "OFF_HAND_COMPOUND", tokens: ANTIPATTERN_662_TOKENS },
    Antipattern { rule_id: "OFF_HAND_COMPOUND", tokens: ANTIPATTERN_663_TOKENS },
    Antipattern { rule_id: "OFF_KEY_HYPHEN", tokens: ANTIPATTERN_664_TOKENS },
    Antipattern { rule_id: "OF_ALL_PLURAL", tokens: ANTIPATTERN_665_TOKENS },
    Antipattern { rule_id: "OK_OK_COMMA", tokens: ANTIPATTERN_666_TOKENS },
    Antipattern { rule_id: "ONES", tokens: ANTIPATTERN_667_TOKENS },
    Antipattern { rule_id: "ONE_HANDED_HYPHEN", tokens: ANTIPATTERN_668_TOKENS },
    Antipattern { rule_id: "ONE_IN_THE_SAME", tokens: ANTIPATTERN_669_TOKENS },
    Antipattern { rule_id: "ONE_ORE", tokens: ANTIPATTERN_670_TOKENS },
    Antipattern { rule_id: "ONE_ORE", tokens: ANTIPATTERN_671_TOKENS },
    Antipattern { rule_id: "ONE_PLURAL", tokens: ANTIPATTERN_672_TOKENS },
    Antipattern { rule_id: "ONE_PLURAL", tokens: ANTIPATTERN_673_TOKENS },
    Antipattern { rule_id: "ONE_PLURAL", tokens: ANTIPATTERN_674_TOKENS },
    Antipattern { rule_id: "ONE_PLURAL", tokens: ANTIPATTERN_675_TOKENS },
    Antipattern { rule_id: "ONE_PLURAL", tokens: ANTIPATTERN_676_TOKENS },
    Antipattern { rule_id: "ONE_PLURAL", tokens: ANTIPATTERN_677_TOKENS },
    Antipattern { rule_id: "ONE_PLURAL", tokens: ANTIPATTERN_678_TOKENS },
    Antipattern { rule_id: "ONE_PLURAL", tokens: ANTIPATTERN_679_TOKENS },
    Antipattern { rule_id: "ONE_PLURAL", tokens: ANTIPATTERN_680_TOKENS },
    Antipattern { rule_id: "ONE_PLURAL", tokens: ANTIPATTERN_681_TOKENS },
    Antipattern { rule_id: "ONE_PLURAL", tokens: ANTIPATTERN_682_TOKENS },
    Antipattern { rule_id: "ONE_YEARS_OLD", tokens: ANTIPATTERN_683_TOKENS },
    Antipattern { rule_id: "ONE_YEARS_OLD", tokens: ANTIPATTERN_684_TOKENS },
    Antipattern { rule_id: "ON_THE_NOVEL", tokens: ANTIPATTERN_685_TOKENS },
    Antipattern { rule_id: "ON_THE_NOVEL", tokens: ANTIPATTERN_686_TOKENS },
    Antipattern { rule_id: "OPINING_OPENING", tokens: ANTIPATTERN_687_TOKENS },
    Antipattern { rule_id: "ORDER_OF_WORDS_WITH_NOT", tokens: ANTIPATTERN_688_TOKENS },
    Antipattern { rule_id: "ORDER_OF_WORDS_WITH_NOT", tokens: ANTIPATTERN_689_TOKENS },
    Antipattern { rule_id: "OTHER_WISE_COMPOUND", tokens: ANTIPATTERN_690_TOKENS },
    Antipattern { rule_id: "OUT_PERFORM_COMPOUND", tokens: ANTIPATTERN_691_TOKENS },
    Antipattern { rule_id: "OVERDUE_OVERDO", tokens: ANTIPATTERN_692_TOKENS },
    Antipattern { rule_id: "O_CONNOR", tokens: ANTIPATTERN_693_TOKENS },
    Antipattern { rule_id: "PAPUA_NEW_GUINEA", tokens: ANTIPATTERN_694_TOKENS },
    Antipattern { rule_id: "PARTICIPATE_TO_IN", tokens: ANTIPATTERN_695_TOKENS },
    Antipattern { rule_id: "PASSED_PAST", tokens: ANTIPATTERN_696_TOKENS },
    Antipattern { rule_id: "PASSED_PAST", tokens: ANTIPATTERN_697_TOKENS },
    Antipattern { rule_id: "PASSED_PAST", tokens: ANTIPATTERN_698_TOKENS },
    Antipattern { rule_id: "PASSED_PAST", tokens: ANTIPATTERN_699_TOKENS },
    Antipattern { rule_id: "PAST_TIME", tokens: ANTIPATTERN_700_TOKENS },
    Antipattern { rule_id: "PAST_TIME", tokens: ANTIPATTERN_701_TOKENS },
    Antipattern { rule_id: "PAST_TIME", tokens: ANTIPATTERN_702_TOKENS },
    Antipattern { rule_id: "PAYED", tokens: ANTIPATTERN_703_TOKENS },
    Antipattern { rule_id: "PEOPLE_VBZ", tokens: ANTIPATTERN_704_TOKENS },
    Antipattern { rule_id: "PEOPLE_VBZ", tokens: ANTIPATTERN_705_TOKENS },
    Antipattern { rule_id: "PEOPLE_VBZ", tokens: ANTIPATTERN_706_TOKENS },
    Antipattern { rule_id: "PEOPLE_VBZ", tokens: ANTIPATTERN_707_TOKENS },
    Antipattern { rule_id: "PEOPLE_VBZ", tokens: ANTIPATTERN_708_TOKENS },
    Antipattern { rule_id: "PERSONA_NON_GRATA", tokens: ANTIPATTERN_709_TOKENS },
    Antipattern { rule_id: "PHRASAL_VERB_SOMETIME", tokens: ANTIPATTERN_710_TOKENS },
    Antipattern { rule_id: "PHRASAL_VERB_SOMETIME", tokens: ANTIPATTERN_711_TOKENS },
    Antipattern { rule_id: "PIECE_COMPOUNDS", tokens: ANTIPATTERN_712_TOKENS },
    Antipattern { rule_id: "PIECE_COMPOUNDS", tokens: ANTIPATTERN_713_TOKENS },
    Antipattern { rule_id: "PIECE_COMPOUNDS", tokens: ANTIPATTERN_714_TOKENS },
    Antipattern { rule_id: "PLACE_COMPOUNDS", tokens: ANTIPATTERN_715_TOKENS },
    Antipattern { rule_id: "PLAY_COMPOUNDS", tokens: ANTIPATTERN_716_TOKENS },
    Antipattern { rule_id: "PLAY_ED", tokens: ANTIPATTERN_717_TOKENS },
    Antipattern { rule_id: "PLEASE_DO_NOT_THE_CAT", tokens: ANTIPATTERN_718_TOKENS },
    Antipattern { rule_id: "PLURAL_MODIFIER", tokens: ANTIPATTERN_719_TOKENS },
    Antipattern { rule_id: "PLURAL_THAT_AGREEMENT", tokens: ANTIPATTERN_720_TOKENS },
    Antipattern { rule_id: "POSSESSIVE_CASE", tokens: ANTIPATTERN_721_TOKENS },
    Antipattern { rule_id: "PRONOUN_NOUN", tokens: ANTIPATTERN_722_TOKENS },
    Antipattern { rule_id: "PRONOUN_NOUN", tokens: ANTIPATTERN_723_TOKENS },
    Antipattern { rule_id: "PRONOUN_NOUN", tokens: ANTIPATTERN_724_TOKENS },
    Antipattern { rule_id: "PRONOUN_NOUN", tokens: ANTIPATTERN_725_TOKENS },
    Antipattern { rule_id: "PRONOUN_NOUN", tokens: ANTIPATTERN_726_TOKENS },
    Antipattern { rule_id: "PRONOUN_NOUN", tokens: ANTIPATTERN_727_TOKENS },
    Antipattern { rule_id: "PRONOUN_NOUN", tokens: ANTIPATTERN_728_TOKENS },
    Antipattern { rule_id: "PRONOUN_NOUN", tokens: ANTIPATTERN_729_TOKENS },
    Antipattern { rule_id: "PRONOUN_NOUN", tokens: ANTIPATTERN_730_TOKENS },
    Antipattern { rule_id: "PRONOUN_NOUN", tokens: ANTIPATTERN_731_TOKENS },
    Antipattern { rule_id: "PRONOUN_NOUN", tokens: ANTIPATTERN_732_TOKENS },
    Antipattern { rule_id: "PRONOUN_NOUN", tokens: ANTIPATTERN_733_TOKENS },
    Antipattern { rule_id: "PRONOUN_NOUN", tokens: ANTIPATTERN_734_TOKENS },
    Antipattern { rule_id: "PRONOUN_NOUN", tokens: ANTIPATTERN_735_TOKENS },
    Antipattern { rule_id: "PRONOUN_NOUN", tokens: ANTIPATTERN_736_TOKENS },
    Antipattern { rule_id: "PRP_AREA", tokens: ANTIPATTERN_737_TOKENS },
    Antipattern { rule_id: "PRP_ILL_VB", tokens: ANTIPATTERN_738_TOKENS },
    Antipattern { rule_id: "PRP_RB_NO_VB", tokens: ANTIPATTERN_739_TOKENS },
    Antipattern { rule_id: "PRP_RB_NO_VB", tokens: ANTIPATTERN_740_TOKENS },
    Antipattern { rule_id: "PRP_RB_NO_VB", tokens: ANTIPATTERN_741_TOKENS },
    Antipattern { rule_id: "PRP_RB_NO_VB", tokens: ANTIPATTERN_742_TOKENS },
    Antipattern { rule_id: "QUESTION_WITHOUT_VERB", tokens: ANTIPATTERN_743_TOKENS },
    Antipattern { rule_id: "QUESTION_WITHOUT_VERB", tokens: ANTIPATTERN_744_TOKENS },
    Antipattern { rule_id: "REASON_WHY_NO_COMMA", tokens: ANTIPATTERN_745_TOKENS },
    Antipattern { rule_id: "RED_NOSED_REINDEER", tokens: ANTIPATTERN_746_TOKENS },
    Antipattern { rule_id: "RINGO_STARR", tokens: ANTIPATTERN_747_TOKENS },
    Antipattern { rule_id: "ROAD_RODE", tokens: ANTIPATTERN_748_TOKENS },
    Antipattern { rule_id: "ROLE_ROLL", tokens: ANTIPATTERN_749_TOKENS },
    Antipattern { rule_id: "ROLL_COMPOUNDS", tokens: ANTIPATTERN_750_TOKENS },
    Antipattern { rule_id: "RONALD_REAGAN", tokens: ANTIPATTERN_751_TOKENS },
    Antipattern { rule_id: "RONALD_REAGAN", tokens: ANTIPATTERN_752_TOKENS },
    Antipattern { rule_id: "RONALD_REAGAN", tokens: ANTIPATTERN_753_TOKENS },
    Antipattern { rule_id: "ROOM_COMPOUNDS", tokens: ANTIPATTERN_754_TOKENS },
    Antipattern { rule_id: "ROOM_COMPOUNDS", tokens: ANTIPATTERN_755_TOKENS },
    Antipattern { rule_id: "ROOM_COMPOUNDS", tokens: ANTIPATTERN_756_TOKENS },
    Antipattern { rule_id: "ROOM_COMPOUNDS", tokens: ANTIPATTERN_757_TOKENS },
    Antipattern { rule_id: "ROOM_COMPOUNDS", tokens: ANTIPATTERN_758_TOKENS },
    Antipattern { rule_id: "ROOM_COMPOUNDS", tokens: ANTIPATTERN_759_TOKENS },
    Antipattern { rule_id: "ROOM_COMPOUNDS", tokens: ANTIPATTERN_760_TOKENS },
    Antipattern { rule_id: "ROYAL_AIR_FORCE", tokens: ANTIPATTERN_761_TOKENS },
    Antipattern { rule_id: "ROYAL_MAIL", tokens: ANTIPATTERN_762_TOKENS },
    Antipattern { rule_id: "RUNNER_UP_HYPHEN", tokens: ANTIPATTERN_763_TOKENS },
    Antipattern { rule_id: "RUNNER_UP_HYPHEN", tokens: ANTIPATTERN_764_TOKENS },
    Antipattern { rule_id: "RUNNER_UP_HYPHEN", tokens: ANTIPATTERN_765_TOKENS },
    Antipattern { rule_id: "RUNNER_UP_HYPHEN", tokens: ANTIPATTERN_766_TOKENS },
    Antipattern { rule_id: "SALVADOR_DALI", tokens: ANTIPATTERN_767_TOKENS },
    Antipattern { rule_id: "SAME_DAY_DELIVERY_HYPHEN", tokens: ANTIPATTERN_768_TOKENS },
    Antipattern { rule_id: "SANTA_CLAUS", tokens: ANTIPATTERN_769_TOKENS },
    Antipattern { rule_id: "SANTA_CLAUS", tokens: ANTIPATTERN_770_TOKENS },
    Antipattern { rule_id: "SAO_PAOLO", tokens: ANTIPATTERN_771_TOKENS },
    Antipattern { rule_id: "SCHINDLERS_LIST", tokens: ANTIPATTERN_772_TOKENS },
    Antipattern { rule_id: "SEA_COMPOUNDS", tokens: ANTIPATTERN_773_TOKENS },
    Antipattern { rule_id: "SEA_COMPOUNDS", tokens: ANTIPATTERN_774_TOKENS },
    Antipattern { rule_id: "SEA_COMPOUNDS", tokens: ANTIPATTERN_775_TOKENS },
    Antipattern { rule_id: "SEA_COMPOUNDS", tokens: ANTIPATTERN_776_TOKENS },
    Antipattern { rule_id: "SEEMING_SEEMS", tokens: ANTIPATTERN_777_TOKENS },
    Antipattern { rule_id: "SHIP_COMPOUNDS", tokens: ANTIPATTERN_778_TOKENS },
    Antipattern { rule_id: "SHIP_COMPOUNDS", tokens: ANTIPATTERN_779_TOKENS },
    Antipattern { rule_id: "SHIP_COMPOUNDS", tokens: ANTIPATTERN_780_TOKENS },
    Antipattern { rule_id: "SHORT_SUPERLATIVES", tokens: ANTIPATTERN_781_TOKENS },
    Antipattern { rule_id: "SIGH_SIGN", tokens: ANTIPATTERN_782_TOKENS },
    Antipattern { rule_id: "SIGN_IN", tokens: ANTIPATTERN_783_TOKENS },
    Antipattern { rule_id: "SIGN_INTO", tokens: ANTIPATTERN_784_TOKENS },
    Antipattern { rule_id: "SIMILAR_LIKE", tokens: ANTIPATTERN_785_TOKENS },
    Antipattern { rule_id: "SIMILAR_LIKE", tokens: ANTIPATTERN_786_TOKENS },
    Antipattern { rule_id: "SINGLES_DAY", tokens: ANTIPATTERN_787_TOKENS },
    Antipattern { rule_id: "SINGULAR_NOUN_ADV_AGREEMENT", tokens: ANTIPATTERN_788_TOKENS },
    Antipattern { rule_id: "SINGULAR_NOUN_THAT_AGREEMENT", tokens: ANTIPATTERN_789_TOKENS },
    Antipattern { rule_id: "SINGULAR_NOUN_THAT_AGREEMENT", tokens: ANTIPATTERN_790_TOKENS },
    Antipattern { rule_id: "SKY_COMPOUNDS", tokens: ANTIPATTERN_791_TOKENS },
    Antipattern { rule_id: "SLUT_SHAME_HYPHEN", tokens: ANTIPATTERN_792_TOKENS },
    Antipattern { rule_id: "SOCIETE_GENERALE", tokens: ANTIPATTERN_793_TOKENS },
    Antipattern { rule_id: "SOMETIME_SOMETIMES", tokens: ANTIPATTERN_794_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_795_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_796_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_797_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_798_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_799_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_800_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_801_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_802_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_803_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_804_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_805_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_806_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_807_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_808_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_809_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_810_TOKENS },
    Antipattern { rule_id: "SOME_NN_VBP", tokens: ANTIPATTERN_811_TOKENS },
    Antipattern { rule_id: "SOME_TIMES", tokens: ANTIPATTERN_812_TOKENS },
    Antipattern { rule_id: "SOME_TIMES", tokens: ANTIPATTERN_813_TOKENS },
    Antipattern { rule_id: "SOME_TIMES", tokens: ANTIPATTERN_814_TOKENS },
    Antipattern { rule_id: "SOME_TIMES", tokens: ANTIPATTERN_815_TOKENS },
    Antipattern { rule_id: "SOME_TIMES", tokens: ANTIPATTERN_816_TOKENS },
    Antipattern { rule_id: "SOME_TIMES", tokens: ANTIPATTERN_817_TOKENS },
    Antipattern { rule_id: "SOME_TIMES", tokens: ANTIPATTERN_818_TOKENS },
    Antipattern { rule_id: "SOME_TIMES", tokens: ANTIPATTERN_819_TOKENS },
    Antipattern { rule_id: "SOME_TIMES", tokens: ANTIPATTERN_820_TOKENS },
    Antipattern { rule_id: "SOON_OR_LATER", tokens: ANTIPATTERN_821_TOKENS },
    Antipattern { rule_id: "SOU_YOU", tokens: ANTIPATTERN_822_TOKENS },
    Antipattern { rule_id: "SOU_YOU", tokens: ANTIPATTERN_823_TOKENS },
    Antipattern { rule_id: "SOU_YOU", tokens: ANTIPATTERN_824_TOKENS },
    Antipattern { rule_id: "SOU_YOU", tokens: ANTIPATTERN_825_TOKENS },
    Antipattern { rule_id: "SPACE_BEFORE_PARENTHESIS", tokens: ANTIPATTERN_826_TOKENS },
    Antipattern { rule_id: "SPACE_BEFORE_PARENTHESIS", tokens: ANTIPATTERN_827_TOKENS },
    Antipattern { rule_id: "SPACE_BEFORE_PARENTHESIS", tokens: ANTIPATTERN_828_TOKENS },
    Antipattern { rule_id: "SPACE_BEFORE_PARENTHESIS", tokens: ANTIPATTERN_829_TOKENS },
    Antipattern { rule_id: "SPACE_BEFORE_PARENTHESIS", tokens: ANTIPATTERN_830_TOKENS },
    Antipattern { rule_id: "SPACE_BEFORE_PARENTHESIS", tokens: ANTIPATTERN_831_TOKENS },
    Antipattern { rule_id: "SPEND_IT_FOR", tokens: ANTIPATTERN_832_TOKENS },
    Antipattern { rule_id: "STAIRS_COMPOUNDS", tokens: ANTIPATTERN_833_TOKENS },
    Antipattern { rule_id: "STAIRS_COMPOUNDS", tokens: ANTIPATTERN_834_TOKENS },
    Antipattern { rule_id: "STAIRS_COMPOUNDS", tokens: ANTIPATTERN_835_TOKENS },
    Antipattern { rule_id: "STATE_OF_THE_UNION", tokens: ANTIPATTERN_836_TOKENS },
    Antipattern { rule_id: "STEP_COMPOUNDS", tokens: ANTIPATTERN_837_TOKENS },
    Antipattern { rule_id: "SUBJECT_NUMBER", tokens: ANTIPATTERN_838_TOKENS },
    Antipattern { rule_id: "SUFFER_OF_WITH", tokens: ANTIPATTERN_839_TOKENS },
    Antipattern { rule_id: "SUIT_COMPOUNDS", tokens: ANTIPATTERN_840_TOKENS },
    Antipattern { rule_id: "SUMMERY_SUMMARY", tokens: ANTIPATTERN_841_TOKENS },
    Antipattern { rule_id: "SUMMERY_SUMMARY", tokens: ANTIPATTERN_842_TOKENS },
    Antipattern { rule_id: "SUMMERY_SUMMARY", tokens: ANTIPATTERN_843_TOKENS },
    Antipattern { rule_id: "SUPERLATIVE_THAN", tokens: ANTIPATTERN_844_TOKENS },
    Antipattern { rule_id: "SUPERLATIVE_THAN", tokens: ANTIPATTERN_845_TOKENS },
    Antipattern { rule_id: "SUPER_COMPOUNDS", tokens: ANTIPATTERN_846_TOKENS },
    Antipattern { rule_id: "SUPER_COMPOUNDS", tokens: ANTIPATTERN_847_TOKENS },
    Antipattern { rule_id: "SUPER_COMPOUNDS", tokens: ANTIPATTERN_848_TOKENS },
    Antipattern { rule_id: "SUPER_COMPOUNDS", tokens: ANTIPATTERN_849_TOKENS },
    Antipattern { rule_id: "TAKEAWAY", tokens: ANTIPATTERN_850_TOKENS },
    Antipattern { rule_id: "TAT", tokens: ANTIPATTERN_851_TOKENS },
    Antipattern { rule_id: "TAT", tokens: ANTIPATTERN_852_TOKENS },
    Antipattern { rule_id: "TAT", tokens: ANTIPATTERN_853_TOKENS },
    Antipattern { rule_id: "TAT", tokens: ANTIPATTERN_854_TOKENS },
    Antipattern { rule_id: "THAN_THANK", tokens: ANTIPATTERN_855_TOKENS },
    Antipattern { rule_id: "THAN_THANK", tokens: ANTIPATTERN_856_TOKENS },
    Antipattern { rule_id: "THERE_FOR", tokens: ANTIPATTERN_857_TOKENS },
    Antipattern { rule_id: "THERE_FOR", tokens: ANTIPATTERN_858_TOKENS },
    Antipattern { rule_id: "THERE_FOR", tokens: ANTIPATTERN_859_TOKENS },
    Antipattern { rule_id: "THERE_FOR", tokens: ANTIPATTERN_860_TOKENS },
    Antipattern { rule_id: "THERE_FOR", tokens: ANTIPATTERN_861_TOKENS },
    Antipattern { rule_id: "THESE_ONES", tokens: ANTIPATTERN_862_TOKENS },
    Antipattern { rule_id: "THEY_WHERE", tokens: ANTIPATTERN_863_TOKENS },
    Antipattern { rule_id: "THEY_WHERE", tokens: ANTIPATTERN_864_TOKENS },
    Antipattern { rule_id: "THEY_WHERE", tokens: ANTIPATTERN_865_TOKENS },
    Antipattern { rule_id: "THEY_WHERE", tokens: ANTIPATTERN_866_TOKENS },
    Antipattern { rule_id: "THE_DUTCH", tokens: ANTIPATTERN_867_TOKENS },
    Antipattern { rule_id: "THE_HOW", tokens: ANTIPATTERN_868_TOKENS },
    Antipattern { rule_id: "THE_HOW", tokens: ANTIPATTERN_869_TOKENS },
    Antipattern { rule_id: "THE_IT", tokens: ANTIPATTERN_870_TOKENS },
    Antipattern { rule_id: "THE_PUNCT", tokens: ANTIPATTERN_871_TOKENS },
    Antipattern { rule_id: "THE_PUNCT", tokens: ANTIPATTERN_872_TOKENS },
    Antipattern { rule_id: "THE_PUNCT", tokens: ANTIPATTERN_873_TOKENS },
    Antipattern { rule_id: "THE_PUNCT", tokens: ANTIPATTERN_874_TOKENS },
    Antipattern { rule_id: "THE_SENT_END", tokens: ANTIPATTERN_875_TOKENS },
    Antipattern { rule_id: "THE_SENT_END", tokens: ANTIPATTERN_876_TOKENS },
    Antipattern { rule_id: "THE_SENT_END", tokens: ANTIPATTERN_877_TOKENS },
    Antipattern { rule_id: "THE_SENT_END", tokens: ANTIPATTERN_878_TOKENS },
    Antipattern { rule_id: "THE_SENT_END", tokens: ANTIPATTERN_879_TOKENS },
    Antipattern { rule_id: "THE_WORSE_OF", tokens: ANTIPATTERN_880_TOKENS },
    Antipattern { rule_id: "THE_WORSE_OF", tokens: ANTIPATTERN_881_TOKENS },
    Antipattern { rule_id: "THINK_OFF", tokens: ANTIPATTERN_882_TOKENS },
    Antipattern { rule_id: "THINK_OFF", tokens: ANTIPATTERN_883_TOKENS },
    Antipattern { rule_id: "THINK_OFF", tokens: ANTIPATTERN_884_TOKENS },
    Antipattern { rule_id: "THIS_IS_HAVE", tokens: ANTIPATTERN_885_TOKENS },
    Antipattern { rule_id: "THIS_TWO_MEN", tokens: ANTIPATTERN_886_TOKENS },
    Antipattern { rule_id: "THIS_TWO_MEN", tokens: ANTIPATTERN_887_TOKENS },
    Antipattern { rule_id: "THIS_YEARS_POSSESSIVE_APOSTROPHE", tokens: ANTIPATTERN_888_TOKENS },
    Antipattern { rule_id: "THROUGH_THOROUGH", tokens: ANTIPATTERN_889_TOKENS },
    Antipattern { rule_id: "TH_THORIUM", tokens: ANTIPATTERN_890_TOKENS },
    Antipattern { rule_id: "TH_THORIUM", tokens: ANTIPATTERN_891_TOKENS },
    Antipattern { rule_id: "TH_THORIUM", tokens: ANTIPATTERN_892_TOKENS },
    Antipattern { rule_id: "TIS", tokens: ANTIPATTERN_893_TOKENS },
    Antipattern { rule_id: "TIS", tokens: ANTIPATTERN_894_TOKENS },
    Antipattern { rule_id: "TIS", tokens: ANTIPATTERN_895_TOKENS },
    Antipattern { rule_id: "TIS", tokens: ANTIPATTERN_896_TOKENS },
    Antipattern { rule_id: "TOO_ADJECTIVE_TO", tokens: ANTIPATTERN_897_TOKENS },
    Antipattern { rule_id: "TOO_CARDINAL_NUMBER", tokens: ANTIPATTERN_898_TOKENS },
    Antipattern { rule_id: "TOO_DETERMINER", tokens: ANTIPATTERN_899_TOKENS },
    Antipattern { rule_id: "TOO_DETERMINER", tokens: ANTIPATTERN_900_TOKENS },
    Antipattern { rule_id: "TOW_THE_LINE", tokens: ANTIPATTERN_901_TOKENS },
    Antipattern { rule_id: "TR", tokens: ANTIPATTERN_902_TOKENS },
    Antipattern { rule_id: "TR", tokens: ANTIPATTERN_903_TOKENS },
    Antipattern { rule_id: "TRUE_TRUTH", tokens: ANTIPATTERN_904_TOKENS },
    Antipattern { rule_id: "TWO_CONNECTED_MODAL_VERBS", tokens: ANTIPATTERN_905_TOKENS },
    Antipattern { rule_id: "TWO_HYPHENS", tokens: ANTIPATTERN_906_TOKENS },
    Antipattern { rule_id: "TWO_HYPHENS", tokens: ANTIPATTERN_907_TOKENS },
    Antipattern { rule_id: "TWO_HYPHENS", tokens: ANTIPATTERN_908_TOKENS },
    Antipattern { rule_id: "TYPO_CONTRACTION", tokens: ANTIPATTERN_909_TOKENS },
    Antipattern { rule_id: "TYPO_CONTRACTION", tokens: ANTIPATTERN_910_TOKENS },
    Antipattern { rule_id: "TYPO_THEY_S", tokens: ANTIPATTERN_911_TOKENS },
    Antipattern { rule_id: "T_HE", tokens: ANTIPATTERN_912_TOKENS },
    Antipattern { rule_id: "T_HE", tokens: ANTIPATTERN_913_TOKENS },
    Antipattern { rule_id: "T_HE", tokens: ANTIPATTERN_914_TOKENS },
    Antipattern { rule_id: "T_HE", tokens: ANTIPATTERN_915_TOKENS },
    Antipattern { rule_id: "T_REX", tokens: ANTIPATTERN_916_TOKENS },
    Antipattern { rule_id: "UH_UH_COMMA", tokens: ANTIPATTERN_917_TOKENS },
    Antipattern { rule_id: "UH_UH_COMMA", tokens: ANTIPATTERN_918_TOKENS },
    Antipattern { rule_id: "UH_UH_COMMA", tokens: ANTIPATTERN_919_TOKENS },
    Antipattern { rule_id: "UH_UH_COMMA", tokens: ANTIPATTERN_920_TOKENS },
    Antipattern { rule_id: "UNDER_COVER_COMPOUND", tokens: ANTIPATTERN_921_TOKENS },
    Antipattern { rule_id: "UNDER_COVER_COMPOUND", tokens: ANTIPATTERN_922_TOKENS },
    Antipattern { rule_id: "UNICODE_CASING", tokens: ANTIPATTERN_923_TOKENS },
    Antipattern { rule_id: "UNICODE_CASING", tokens: ANTIPATTERN_924_TOKENS },
    Antipattern { rule_id: "UNLIKELY_OPENING_PUNCTUATION", tokens: ANTIPATTERN_925_TOKENS },
    Antipattern { rule_id: "UNLIKELY_OPENING_PUNCTUATION", tokens: ANTIPATTERN_926_TOKENS },
    Antipattern { rule_id: "UNLIKELY_OPENING_PUNCTUATION", tokens: ANTIPATTERN_927_TOKENS },
    Antipattern { rule_id: "UP_AND_COMING_HYPHEN", tokens: ANTIPATTERN_928_TOKENS },
    Antipattern { rule_id: "UR", tokens: ANTIPATTERN_929_TOKENS },
    Antipattern { rule_id: "USE_TO_VERB", tokens: ANTIPATTERN_930_TOKENS },
    Antipattern { rule_id: "USE_TO_VERB", tokens: ANTIPATTERN_931_TOKENS },
    Antipattern { rule_id: "VBG_THEYRE", tokens: ANTIPATTERN_932_TOKENS },
    Antipattern { rule_id: "VBP_VBP", tokens: ANTIPATTERN_933_TOKENS },
    Antipattern { rule_id: "VBP_VBP", tokens: ANTIPATTERN_934_TOKENS },
    Antipattern { rule_id: "VBP_VBP", tokens: ANTIPATTERN_935_TOKENS },
    Antipattern { rule_id: "VBP_VBP", tokens: ANTIPATTERN_936_TOKENS },
    Antipattern { rule_id: "VBP_VBP", tokens: ANTIPATTERN_937_TOKENS },
    Antipattern { rule_id: "VBP_VBP", tokens: ANTIPATTERN_938_TOKENS },
    Antipattern { rule_id: "VBP_VBP", tokens: ANTIPATTERN_939_TOKENS },
    Antipattern { rule_id: "VBP_VBP", tokens: ANTIPATTERN_940_TOKENS },
    Antipattern { rule_id: "VBP_VBP", tokens: ANTIPATTERN_941_TOKENS },
    Antipattern { rule_id: "VITAMIN_C", tokens: ANTIPATTERN_942_TOKENS },
    Antipattern { rule_id: "VITAMIN_C", tokens: ANTIPATTERN_943_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_944_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_945_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_946_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_947_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_948_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_949_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_950_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_951_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_952_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_953_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_954_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_955_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_956_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_957_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_958_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_959_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_960_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_961_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_962_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_963_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_964_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_965_TOKENS },
    Antipattern { rule_id: "WANT_TO_NN", tokens: ANTIPATTERN_966_TOKENS },
    Antipattern { rule_id: "WEE_WE", tokens: ANTIPATTERN_967_TOKENS },
    Antipattern { rule_id: "WEE_WE", tokens: ANTIPATTERN_968_TOKENS },
    Antipattern { rule_id: "WERNHER_VON_BRAUN", tokens: ANTIPATTERN_969_TOKENS },
    Antipattern { rule_id: "WHAT_DID_VBD", tokens: ANTIPATTERN_970_TOKENS },
    Antipattern { rule_id: "WHAT_DO_THAT", tokens: ANTIPATTERN_971_TOKENS },
    Antipattern { rule_id: "WHAT_IS_REASON", tokens: ANTIPATTERN_972_TOKENS },
    Antipattern { rule_id: "WHAT_TO_VBD", tokens: ANTIPATTERN_973_TOKENS },
    Antipattern { rule_id: "WHAT_VBZ", tokens: ANTIPATTERN_974_TOKENS },
    Antipattern { rule_id: "WHAT_VBZ", tokens: ANTIPATTERN_975_TOKENS },
    Antipattern { rule_id: "WHERE_AS", tokens: ANTIPATTERN_976_TOKENS },
    Antipattern { rule_id: "WHERE_AS", tokens: ANTIPATTERN_977_TOKENS },
    Antipattern { rule_id: "WHERE_AS", tokens: ANTIPATTERN_978_TOKENS },
    Antipattern { rule_id: "WHERE_AS", tokens: ANTIPATTERN_979_TOKENS },
    Antipattern { rule_id: "WHERE_AS", tokens: ANTIPATTERN_980_TOKENS },
    Antipattern { rule_id: "WHERE_MD_VB", tokens: ANTIPATTERN_981_TOKENS },
    Antipattern { rule_id: "WHERE_MD_VB", tokens: ANTIPATTERN_982_TOKENS },
    Antipattern { rule_id: "WHICH_WISH", tokens: ANTIPATTERN_983_TOKENS },
    Antipattern { rule_id: "WHIT_WITH", tokens: ANTIPATTERN_984_TOKENS },
    Antipattern { rule_id: "WHOS_NN", tokens: ANTIPATTERN_985_TOKENS },
    Antipattern { rule_id: "WHOS_NN", tokens: ANTIPATTERN_986_TOKENS },
    Antipattern { rule_id: "WHOS_NN", tokens: ANTIPATTERN_987_TOKENS },
    Antipattern { rule_id: "WHOS_NN", tokens: ANTIPATTERN_988_TOKENS },
    Antipattern { rule_id: "WHO_NOUN", tokens: ANTIPATTERN_989_TOKENS },
    Antipattern { rule_id: "WHO_NOUN", tokens: ANTIPATTERN_990_TOKENS },
    Antipattern { rule_id: "WHO_NOUN", tokens: ANTIPATTERN_991_TOKENS },
    Antipattern { rule_id: "WHO_S_NN_VB", tokens: ANTIPATTERN_992_TOKENS },
    Antipattern { rule_id: "WHO_S_NN_VB", tokens: ANTIPATTERN_993_TOKENS },
    Antipattern { rule_id: "WIEN", tokens: ANTIPATTERN_994_TOKENS },
    Antipattern { rule_id: "WIEN", tokens: ANTIPATTERN_995_TOKENS },
    Antipattern { rule_id: "WIEN", tokens: ANTIPATTERN_996_TOKENS },
    Antipattern { rule_id: "WIEN", tokens: ANTIPATTERN_997_TOKENS },
    Antipattern { rule_id: "WIEN", tokens: ANTIPATTERN_998_TOKENS },
    Antipattern { rule_id: "WIEN", tokens: ANTIPATTERN_999_TOKENS },
    Antipattern { rule_id: "WITHOUT_OUT", tokens: ANTIPATTERN_1000_TOKENS },
    Antipattern { rule_id: "WITH_EXCEPTION_OF", tokens: ANTIPATTERN_1001_TOKENS },
    Antipattern { rule_id: "WOLD", tokens: ANTIPATTERN_1002_TOKENS },
    Antipattern { rule_id: "WOLD", tokens: ANTIPATTERN_1003_TOKENS },
    Antipattern { rule_id: "WOMAN_COMPOUNDS", tokens: ANTIPATTERN_1004_TOKENS },
    Antipattern { rule_id: "WON_T_TO", tokens: ANTIPATTERN_1005_TOKENS },
    Antipattern { rule_id: "WORDPRESS", tokens: ANTIPATTERN_1006_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1007_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1008_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1009_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1010_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1011_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1012_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1013_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1014_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1015_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1016_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1017_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1018_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1019_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1020_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1021_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1022_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1023_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1024_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1025_TOKENS },
    Antipattern { rule_id: "WORD_CONTAINS_UNDERSCORE", tokens: ANTIPATTERN_1026_TOKENS },
    Antipattern { rule_id: "WORKER_COMPOUNDS", tokens: ANTIPATTERN_1027_TOKENS },
    Antipattern { rule_id: "WORKER_COMPOUNDS", tokens: ANTIPATTERN_1028_TOKENS },
    Antipattern { rule_id: "WORKER_COMPOUNDS", tokens: ANTIPATTERN_1029_TOKENS },
    Antipattern { rule_id: "WORKER_COMPOUNDS", tokens: ANTIPATTERN_1030_TOKENS },
    Antipattern { rule_id: "WORLD_WIDE", tokens: ANTIPATTERN_1031_TOKENS },
    Antipattern { rule_id: "WORRY_FOR", tokens: ANTIPATTERN_1032_TOKENS },
    Antipattern { rule_id: "WOULD_BE_JJ_VB", tokens: ANTIPATTERN_1033_TOKENS },
    Antipattern { rule_id: "WOULD_BE_JJ_VB", tokens: ANTIPATTERN_1034_TOKENS },
    Antipattern { rule_id: "WRITER_COMPOUNDS", tokens: ANTIPATTERN_1035_TOKENS },
    Antipattern { rule_id: "WRITS_WRITES", tokens: ANTIPATTERN_1036_TOKENS },
    Antipattern { rule_id: "W_HAT", tokens: ANTIPATTERN_1037_TOKENS },
    Antipattern { rule_id: "W_HAT", tokens: ANTIPATTERN_1038_TOKENS },
    Antipattern { rule_id: "YARD_COMPOUNDS", tokens: ANTIPATTERN_1039_TOKENS },
    Antipattern { rule_id: "YARD_COMPOUNDS", tokens: ANTIPATTERN_1040_TOKENS },
    Antipattern { rule_id: "YARD_COMPOUNDS", tokens: ANTIPATTERN_1041_TOKENS },
    Antipattern { rule_id: "YOUR_NN", tokens: ANTIPATTERN_1042_TOKENS },
    Antipattern { rule_id: "YOUR_NN", tokens: ANTIPATTERN_1043_TOKENS },
    Antipattern { rule_id: "YOUR_NN", tokens: ANTIPATTERN_1044_TOKENS },
    Antipattern { rule_id: "YOUR_NN", tokens: ANTIPATTERN_1045_TOKENS },
    Antipattern { rule_id: "YOUR_NN", tokens: ANTIPATTERN_1046_TOKENS },
    Antipattern { rule_id: "YOUR_NN", tokens: ANTIPATTERN_1047_TOKENS },
    Antipattern { rule_id: "YOUR_NN", tokens: ANTIPATTERN_1048_TOKENS },
    Antipattern { rule_id: "YOUR_NN", tokens: ANTIPATTERN_1049_TOKENS },
    Antipattern { rule_id: "YOUR_NN", tokens: ANTIPATTERN_1050_TOKENS },
    Antipattern { rule_id: "YOUR_NN", tokens: ANTIPATTERN_1051_TOKENS },
    Antipattern { rule_id: "YOUR_NN", tokens: ANTIPATTERN_1052_TOKENS },
];

use std::collections::HashMap;
use std::sync::LazyLock;

/// Lookup antipatterns by rule ID
pub static EN_ANTIPATTERNS_BY_RULE: LazyLock<HashMap<&'static str, Vec<&'static Antipattern>>> = LazyLock::new(|| {
	let mut map: HashMap<&'static str, Vec<&'static Antipattern>> = HashMap::new();
	for ap in EN_ANTIPATTERNS {
		map.entry(ap.rule_id).or_default().push(ap);
	}
	map
});

/// Get antipatterns for a rule ID
pub fn get_en_antipatterns(rule_id: &str) -> Option<&'static Vec<&'static Antipattern>> {
	EN_ANTIPATTERNS_BY_RULE.get(rule_id)
}
