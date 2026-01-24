//! Auto-generated disambiguation skip patterns for EN from LanguageTool
//! Synced: 2026-01-24T12:08:47.729232+00:00
//! Total: 24 words + 36 regex patterns
//! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update
//!
//! Source: LanguageTool disambiguation.xml (action="ignore_spelling")
//! License: LGPL 2.1+
//!
//! These patterns should be ignored by the spell checker.

/// Skip words for EN spell checker (from disambiguation ignore_spelling rules)
pub const EN_DISAMBIG_SKIP: &[&str] = &[
    "-",
    "acid",
    "aspartic",
    "blanche",
    "carte",
    "de",
    "es",
    "fu",
    "in",
    "ish",
    "keto",
    "kung",
    "nother",
    "por",
    "praia",
    "r$",
    "salut",
    "us$",
    "van",
    "visite",
    "vivo",
    "von",
    "whys",
    "y",
];

/// Skip regex patterns for EN spell checker (from disambiguation ignore_spelling rules)
pub const EN_DISAMBIG_SKIP_REGEX: &[&str] = &[
    r"(Y|Z|E|P|T|G|M|k|h|da|d|c|m|µ|n|p|f|a|z|y)?(m|g|s|A|K|mol|cd|rad|sr|Hz|N|Pa|J|W|C|V|F|Ω|Sv?|Wb|T|H|l[mx]|Bq|Gy|kat|min|h|d|L|t|Np|B|eV|ua?)(⁻)?(¹|²|³)",
    r"([a-z]+)[™®]",
    r"([a-z]|mc|mi|[dkm]m|sp)[²³]",
    r"(cc|CC)[’'](d|ing)",
    r".*[a-z].*",
    r".+\\$",
    r"CO₂-([Ff]ree|[Bb]ased|[Nn]eutral|[Cc]ompensat(ing|ed)|[Dd]ependent|[Ii]ndependent)",
    r"[A-Z].+",
    r"[A-Z]{1,2}&[A-Z]{1,2}",
    r"[AaOo]h{1,20}|[OA]H{1,20}",
    r"[Dd]e",
    r"[Dd]el",
    r"[Dd]el|du|di",
    r"[Ff]leek|FLEEK",
    r"[Ll]as?|[Ll]os",
    r"[Ll]a|[Ll][eoa]s",
    r"[Nn]other|NOTHER",
    r"[Tt]hou|THOU",
    r"[Ww]ast|WAST",
    r"[\\p{L}\\p{Nd}_\\-.]+[@_][\\p{L}\\p{Nd}_\\-.]+",
    r"[ae]l",
    r"[~►◄%]+",
    r"\\$[A-Z]{2,5}",
    r"\\^+",
    r"acids?|diets?",
    r"cartes?",
    r"d[oa]",
    r"de[nr]?",
    r"eaux?",
    r"el|los|las?",
    r"ing|ed",
    r"l[ao]s",
    r"pre|PRE|Pre",
    r"toilettes?|parfums?|javel|javelle|nile|colognes?",
    r"un|dis",
    r"§[A-Z]",
];
