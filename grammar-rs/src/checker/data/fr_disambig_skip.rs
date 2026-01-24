//! Auto-generated disambiguation skip patterns for FR from LanguageTool
//! Synced: 2026-01-24T12:08:49.459028+00:00
//! Total: 1 words + 3 regex patterns
//! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update
//!
//! Source: LanguageTool disambiguation.xml (action="ignore_spelling")
//! License: LGPL 2.1+
//!
//! These patterns should be ignored by the spell checker.

/// Skip words for FR spell checker (from disambiguation ignore_spelling rules)
pub const FR_DISAMBIG_SKIP: &[&str] = &[
    "xd",
];

/// Skip regex patterns for FR spell checker (from disambiguation ignore_spelling rules)
pub const FR_DISAMBIG_SKIP_REGEX: &[&str] = &[
    r"N°\\d?",
    r"[_\\^]+",
    r"the|of|and|to|your?|by|one|end|park|world|time|with|bank|goodies",
];
