//! Auto-generated context-sensitive word rules for EN from LanguageTool
//! Synced: 2026-01-24T12:08:47.231545+00:00
//! Total rules: 11
//! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update
//!
//! Source: LanguageTool wrongWordInContext.txt
//! License: LGPL 2.1+
//!
//! These rules detect commonly confused words based on surrounding context.
//! For example: "affect" vs "effect" depending on whether verbs or nouns are nearby.

/// A context-sensitive word rule
#[derive(Debug, Clone)]
pub struct ContextRule {
    /// First word in the confused pair
    pub word1: &'static str,
    /// Second word in the confused pair
    pub word2: &'static str,
    /// Pattern to match in word1 (for partial matching)
    pub match1: &'static str,
    /// Pattern to match in word2 (for partial matching)
    pub match2: &'static str,
    /// Regex pattern for context where word1 is correct
    pub context1_regex: &'static str,
    /// Regex pattern for context where word2 is correct
    pub context2_regex: &'static str,
    /// Explanation for word1 usage
    pub explanation1: Option<&'static str>,
    /// Explanation for word2 usage
    pub explanation2: Option<&'static str>,
}

/// Context-sensitive word rules for EN
pub const EN_CONTEXT_RULES: &[ContextRule] = &[
    ContextRule {
		word1: "prescribe[ds]?",
		word2: "proscribe[sd]?",
		match1: "prescr",
		match2: "proscr",
		context1_regex: "medication|medicine|antibiotics|doctors?",
		context2_regex: "theft|murder",
		explanation1: Some("to recommend or authorize"),
		explanation2: Some("to forbid, to limit or to banish"),
	},
    ContextRule {
		word1: "heroin",
		word2: "heroine",
		match1: "oin",
		match2: "oine",
		context1_regex: "addict.*|morphine|drug|narcotics?|withdrawal",
		context2_regex: "literature|novels?|author|story|poems?|play|female|dramatic|statue|heroes?",
		explanation1: Some("highly addictive narcotic"),
		explanation2: Some("the chief female character; a woman admired for great deeds"),
	},
    ContextRule {
		word1: "bazaar",
		word2: "bizarre",
		match1: "azaar",
		match2: "izarre",
		context1_regex: "oriental|marketplace|rummage|goods|buy|bought|sell|sold",
		context2_regex: "moment|odd|unusual|incident|behaviou?r|situations?|stor(y|ies)|reasons?|moments?",
		explanation1: Some("a marketplace (noun)"),
		explanation2: Some("unusual, odd, or whimsically strange (adjective)"),
	},
    ContextRule {
		word1: "bridal",
		word2: "bridle",
		match1: "ridal",
		match2: "ridle",
		context1_regex: "weddings?|brides?|boutique|gowns?|church|party|shower|shop|jewellery|dress|fashion",
		context2_regex: "horses?|cavalier|harness|headgear|headstall|rider?s?",
		explanation1: Some("related to a bride"),
		explanation2: Some("a part of a horse's harness"),
	},
    ContextRule {
		word1: "desserts?",
		word2: "deserts?",
		match1: "esse",
		match2: "ese",
		context1_regex: "chocolate|menu|wait(ers?|ress)|cookies|cakes?",
		context2_regex: "bec[ao]me|dry|arid|vegetation|camels?|precipitation|rain",
		explanation1: Some("a confectionery course that concludes a main meal"),
		explanation2: Some("a barren area of landscape with little precipitation"),
	},
    ContextRule {
		word1: "statutes?",
		word2: "statues?",
		match1: "ute",
		match2: "ue",
		context1_regex: "government|legislat(ure|ive)|laws?|senators?|regulat(ions?|ed?|ing)|Republicans?|Democrats?|limitations",
		context2_regex: "life-size|figur(ative|ine)|sculptur(ing|ed?)|modeling|art(ist)|temples?|marble|bronze|Gree(k|ce)",
		explanation1: Some("a formal written enactment of a legislative authority"),
		explanation2: Some("a sculpture; a carved or shaped imitation of an object"),
	},
    ContextRule {
		word1: "gorilla",
		word2: "guerilla",
		match1: "o",
		match2: "ue",
		context1_regex: "apes?|chimpanzees?|bonobos?|[Ss]ilverback|zoos?",
		context2_regex: "marketing|war(fares?)?|insurgents?|soldiers?|army|(para)?military|ambush(ed)?",
		explanation1: Some("animal in ape family"),
		explanation2: Some("soldier specializing surprise attacks"),
	},
    ContextRule {
		word1: "massage",
		word2: "message",
		match1: "ma",
		match2: "me",
		context1_regex: "Thai|relaxing|naked|therap(eutic|y|ist)|prostate|erotic|tantric|lymphatic|muscles?",
		context2_regex: "error|send(ing)?|sent|receive[dr]?|important|take|writ(e|ten)",
		explanation1: Some("action of rubbing, kneading or hitting someone's body"),
		explanation2: Some("communication, or what is communicated"),
	},
    ContextRule {
		word1: "sing",
		word2: "sign",
		match1: "ing",
		match2: "ign",
		context1_regex: "songs?|music|choirs?",
		context2_regex: "contracts?",
		explanation1: Some("to produce musical or harmonious sounds with one’s voice"),
		explanation2: Some("to write one's signature"),
	},
    ContextRule {
		word1: "neutrons?",
		word2: "neurons?",
		match1: "utr",
		match2: "ur",
		context1_regex: "[fF]ermion(ic)?|[pP]rotons?|[Aa]tom(s?|ic)|[Ee]lectrons?|[nN]uclear|[Ss]pins?|hadron(ic)?|[tT]hermal|bound",
		context2_regex: "ganglia|axons?|dendrits?|nervous|synapses?|stimul(i|us)|nerves?|neurotransmitters?|cerebral|(bi|uni)polar",
		explanation1: Some("subatomic particle"),
		explanation2: Some("nerve cell"),
	},
    ContextRule {
		word1: "hangars?",
		word2: "hangers?",
		match1: "gar",
		match2: "ger",
		context1_regex: "air(craft|plain)s?|aeroplains?|planes?",
		context2_regex: "cloth(es?|ing)|coats?|garments?",
		explanation1: Some("shed or shelter"),
		explanation2: Some("a frame for hanging clothes"),
	},
];
