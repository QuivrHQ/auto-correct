//! Auto-generated antipatterns for FR from LanguageTool
//! Synced: 2026-01-23T14:23:46.773210646+00:00
//! Total antipatterns: 216
//! DO NOT EDIT MANUALLY - Run `cargo run --bin sync-lt` to update
//!
//! Source: LanguageTool grammar.xml
//! License: LGPL 2.1+
//!
//! Antipatterns are exceptions to grammar rules.
//! When text matches an antipattern, the rule should NOT fire.

use super::en_antipatterns::{Antipattern, AntipatternToken};

static ANTIPATTERN_0_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("milliards"), regexp: None, inflected: false },
    AntipatternToken { text: Some("de"), regexp: None, inflected: true },
    AntipatternToken { text: Some("économie|bénéfice|augmentation|hausse|baisse|diminution|indemnisation"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_1_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("nombre"), regexp: None, inflected: false },
    AntipatternToken { text: Some("de"), regexp: None, inflected: true },
    AntipatternToken { text: Some("masse|neuf|(?-i)mach|onde|or|oxydation"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_2_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("numéros?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("de"), regexp: None, inflected: false },
    AntipatternToken { text: Some("série"), regexp: None, inflected: false },
];
static ANTIPATTERN_3_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("série"), regexp: None, inflected: true },
    AntipatternToken { text: Some("d'"), regexp: None, inflected: false },
    AntipatternToken { text: Some("animation"), regexp: None, inflected: false },
];
static ANTIPATTERN_4_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("série"), regexp: None, inflected: false },
    AntipatternToken { text: Some("de"), regexp: None, inflected: true },
    AntipatternToken { text: Some("(?-i)[a-z].*|été"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_5_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("série"), regexp: None, inflected: true },
    AntipatternToken { text: Some("de"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bande"), regexp: None, inflected: false },
    AntipatternToken { text: Some("dessinée?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_6_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("spécial"), regexp: None, inflected: false },
    AntipatternToken { text: Some("rentrée"), regexp: None, inflected: false },
];
static ANTIPATTERN_7_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\+|-|×|x"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[0-9]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_8_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("magnétiques?|zeeman|monismes?|diffusions?|dispersions?|diffractions?|commutations?|produits?|biens?|biréfringences?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("anoma(l(es?)?|aux)"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_9_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("autant"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pour"), regexp: None, inflected: false },
    AntipatternToken { text: Some("moi"), regexp: None, inflected: false },
    AntipatternToken { text: Some("que"), regexp: None, inflected: false },
];
static ANTIPATTERN_10_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("avent"), regexp: None, inflected: false },
    AntipatternToken { text: Some("("), regexp: None, inflected: false },
    AntipatternToken { text: Some("un"), regexp: None, inflected: false },
    AntipatternToken { text: Some(")"), regexp: None, inflected: false },
];
static ANTIPATTERN_11_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("avent"), regexp: None, inflected: false },
    AntipatternToken { text: Some("à"), regexp: None, inflected: false },
    AntipatternToken { text: Some("met"), regexp: None, inflected: false },
];
static ANTIPATTERN_12_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("calendrier|couronne"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("de"), regexp: None, inflected: false },
    AntipatternToken { text: Some("l'"), regexp: None, inflected: false },
    AntipatternToken { text: Some("avent"), regexp: None, inflected: false },
];
static ANTIPATTERN_13_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("en"), regexp: None, inflected: false },
    AntipatternToken { text: Some("vélo|byciclette|moto|cheval|skis"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("(?-i)[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_14_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("en"), regexp: None, inflected: false },
    AntipatternToken { text: Some("vélo|byciclette|moto|cheval|skis"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("de|d'|cross|neige|trial|freeride|freestyle|rando|racing"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_15_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("en"), regexp: None, inflected: false },
    AntipatternToken { text: Some("vélo|byciclette|moto|cheval|skis"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("tout|à"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("chemin|terrain|cailloux"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_16_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("transformer|déguiser|métamorphoser|changer"), regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("en"), regexp: None, inflected: false },
    AntipatternToken { text: Some("vélo|byciclette|moto|cheval|skis"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_17_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("à"), regexp: None, inflected: false },
    AntipatternToken { text: Some("peu"), regexp: None, inflected: false },
    AntipatternToken { text: Some("près"), regexp: None, inflected: false },
];
static ANTIPATTERN_18_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("à"), regexp: None, inflected: false },
    AntipatternToken { text: Some("toutes"), regexp: None, inflected: false },
    AntipatternToken { text: Some("fins"), regexp: None, inflected: false },
    AntipatternToken { text: Some("utiles"), regexp: None, inflected: false },
];
static ANTIPATTERN_19_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bénéficier"), regexp: None, inflected: true },
    AntipatternToken { text: Some("à|aux?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("plus|moins"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("des?|d'"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_20_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bon|meilleur"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("marché"), regexp: None, inflected: false },
];
static ANTIPATTERN_21_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("de"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bouche"), regexp: None, inflected: false },
];
static ANTIPATTERN_22_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("au"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pire"), regexp: None, inflected: false },
    AntipatternToken { text: Some("des"), regexp: None, inflected: false },
    AntipatternToken { text: Some("cas"), regexp: None, inflected: false },
];
static ANTIPATTERN_23_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("cest"), regexp: None, inflected: false },
];
static ANTIPATTERN_24_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\d+.\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("cest"), regexp: None, inflected: false },
];
static ANTIPATTERN_25_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("en|durant|pendant"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("\\d+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("siècle"), regexp: None, inflected: false },
];
static ANTIPATTERN_26_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("china"), regexp: None, inflected: false },
    AntipatternToken { text: Some("post|eastern"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_27_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("made"), regexp: None, inflected: false },
    AntipatternToken { text: Some("in"), regexp: None, inflected: false },
    AntipatternToken { text: Some("china"), regexp: None, inflected: false },
];
static ANTIPATTERN_28_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("le|du|un"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("chine"), regexp: None, inflected: false },
];
static ANTIPATTERN_29_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("on"), regexp: None, inflected: false },
    AntipatternToken { text: Some("aura"), regexp: None, inflected: false },
    AntipatternToken { text: Some("voulu"), regexp: None, inflected: false },
];
static ANTIPATTERN_30_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("il"), regexp: None, inflected: false },
    AntipatternToken { text: Some("se"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pouvoir"), regexp: None, inflected: true },
];
static ANTIPATTERN_31_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(","), regexp: None, inflected: false },
    AntipatternToken { text: Some("or"), regexp: None, inflected: false },
];
static ANTIPATTERN_32_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("plutôt"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: true },
];
static ANTIPATTERN_33_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: None, regexp: Some("yes"), inflected: true },
    AntipatternToken { text: Some("plutôt"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pas"), regexp: None, inflected: false },
    AntipatternToken { text: Some("mal"), regexp: None, inflected: false },
];
static ANTIPATTERN_34_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("et"), regexp: None, inflected: false },
    AntipatternToken { text: Some("au"), regexp: None, inflected: false },
    AntipatternToken { text: Some("su"), regexp: None, inflected: false },
];
static ANTIPATTERN_35_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("su"), regexp: None, inflected: false },
    AntipatternToken { text: Some("y|à|au|ni|elle|il|on"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_36_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("débuter"), regexp: None, inflected: true },
    AntipatternToken { text: Some("le"), regexp: None, inflected: false },
    AntipatternToken { text: Some("long"), regexp: None, inflected: false },
    AntipatternToken { text: Some("d'|de|du"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_37_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("débuter"), regexp: None, inflected: true },
    AntipatternToken { text: Some("le"), regexp: None, inflected: false },
    AntipatternToken { text: Some("mieux"), regexp: None, inflected: false },
];
static ANTIPATTERN_38_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("débuter"), regexp: None, inflected: true },
    AntipatternToken { text: Some("le"), regexp: None, inflected: false },
    AntipatternToken { text: Some("plus|moins"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("à|au"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_39_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("débuter"), regexp: None, inflected: true },
    AntipatternToken { text: Some("quelque"), regexp: None, inflected: false },
    AntipatternToken { text: Some("part"), regexp: None, inflected: false },
];
static ANTIPATTERN_40_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("démarrer"), regexp: None, inflected: true },
    AntipatternToken { text: Some("le"), regexp: None, inflected: false },
    AntipatternToken { text: Some("long"), regexp: None, inflected: false },
    AntipatternToken { text: Some("d'|de|du"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_41_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("démarrer"), regexp: None, inflected: true },
    AntipatternToken { text: Some("le"), regexp: None, inflected: false },
    AntipatternToken { text: Some("mieux"), regexp: None, inflected: false },
];
static ANTIPATTERN_42_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("démarrer"), regexp: None, inflected: true },
    AntipatternToken { text: Some("le"), regexp: None, inflected: false },
    AntipatternToken { text: Some("plus|moins"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("à|au"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_43_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("démarrer"), regexp: None, inflected: true },
    AntipatternToken { text: Some("quelque"), regexp: None, inflected: false },
    AntipatternToken { text: Some("part"), regexp: None, inflected: false },
];
static ANTIPATTERN_44_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("dernier"), regexp: None, inflected: false },
    AntipatternToken { text: Some("cri"), regexp: None, inflected: false },
];
static ANTIPATTERN_45_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("-|en"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("car"), regexp: None, inflected: false },
];
static ANTIPATTERN_46_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bene"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ma"), regexp: None, inflected: false },
];
static ANTIPATTERN_47_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bon"), regexp: None, inflected: false },
    AntipatternToken { text: Some("gré"), regexp: None, inflected: false },
    AntipatternToken { text: Some("malgré"), regexp: None, inflected: false },
];
static ANTIPATTERN_48_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("des?|aux?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("leurs?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_49_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("et"), regexp: None, inflected: false },
    AntipatternToken { text: Some("quelques?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_50_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mais"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pas"), regexp: None, inflected: false },
    AntipatternToken { text: Some("que"), regexp: None, inflected: false },
];
static ANTIPATTERN_51_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("non"), regexp: None, inflected: false },
    AntipatternToken { text: Some("mais"), regexp: None, inflected: false },
];
static ANTIPATTERN_52_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tout"), regexp: None, inflected: false },
    AntipatternToken { text: Some("comme"), regexp: None, inflected: false },
];
static ANTIPATTERN_53_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("-|en"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("car"), regexp: None, inflected: false },
];
static ANTIPATTERN_54_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bene"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ma"), regexp: None, inflected: false },
];
static ANTIPATTERN_55_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("bon"), regexp: None, inflected: false },
    AntipatternToken { text: Some("gré"), regexp: None, inflected: false },
    AntipatternToken { text: Some("malgré"), regexp: None, inflected: false },
];
static ANTIPATTERN_56_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("des?|aux?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("leurs?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_57_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("et"), regexp: None, inflected: false },
    AntipatternToken { text: Some("quelques?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_58_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mais"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pas"), regexp: None, inflected: false },
    AntipatternToken { text: Some("que"), regexp: None, inflected: false },
];
static ANTIPATTERN_59_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("non"), regexp: None, inflected: false },
    AntipatternToken { text: Some("mais"), regexp: None, inflected: false },
];
static ANTIPATTERN_60_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tout"), regexp: None, inflected: false },
    AntipatternToken { text: Some("comme"), regexp: None, inflected: false },
];
static ANTIPATTERN_61_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("en"), regexp: None, inflected: false },
    AntipatternToken { text: Some("tant"), regexp: None, inflected: false },
    AntipatternToken { text: Some("que"), regexp: None, inflected: false },
    AntipatternToken { text: Some("dieu"), regexp: None, inflected: false },
];
static ANTIPATTERN_62_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("du"), regexp: None, inflected: false },
    AntipatternToken { text: Some("x[ivx]?es"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("siècle"), regexp: None, inflected: false },
];
static ANTIPATTERN_63_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("faire"), regexp: None, inflected: true },
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
    AntipatternToken { text: Some("moue?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_64_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
    AntipatternToken { text: Some("grand"), regexp: None, inflected: false },
    AntipatternToken { text: Some("rue"), regexp: None, inflected: false },
];
static ANTIPATTERN_65_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("la|les|une|des"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("pers"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
];
static ANTIPATTERN_66_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("les"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pro-.*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_67_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ses"), regexp: None, inflected: false },
    AntipatternToken { text: Some("abusé"), regexp: None, inflected: false },
];
static ANTIPATTERN_68_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("toutes"), regexp: None, inflected: false },
    AntipatternToken { text: Some("et"), regexp: None, inflected: false },
    AntipatternToken { text: Some("tous"), regexp: None, inflected: false },
];
static ANTIPATTERN_69_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("être"), regexp: None, inflected: true },
    AntipatternToken { text: Some("plusieurs"), regexp: None, inflected: false },
];
static ANTIPATTERN_70_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("du"), regexp: None, inflected: false },
    AntipatternToken { text: Some("(?-i)[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_71_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ma"), regexp: None, inflected: false },
    AntipatternToken { text: Some("lin|yuan"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_72_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sa"), regexp: None, inflected: false },
    AntipatternToken { text: Some("(?-i)[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_73_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sa"), regexp: None, inflected: false },
    AntipatternToken { text: Some("(?-i)[a-z][a-z].*"), regexp: None, inflected: false },
];
static ANTIPATTERN_74_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[\\/]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("(?-i)ce"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_75_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("bonjour|salut|coucou"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_76_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[aà]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
    AntipatternToken { text: Some("con"), regexp: None, inflected: false },
];
static ANTIPATTERN_77_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("au"), regexp: None, inflected: false },
    AntipatternToken { text: Some("prés"), regexp: None, inflected: false },
    AntipatternToken { text: Some("des?|du"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_78_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("aux?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("vue?s?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("des?|d'"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_79_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("au|spin|mille"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("pieds|off|merci"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_80_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("des"), regexp: None, inflected: false },
    AntipatternToken { text: Some("top"), regexp: None, inflected: false },
];
static ANTIPATTERN_81_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("divers"), regexp: None, inflected: false },
    AntipatternToken { text: Some("gauche|droite"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_82_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("du"), regexp: None, inflected: false },
    AntipatternToken { text: Some(".*e$"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_83_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("du"), regexp: None, inflected: false },
    AntipatternToken { text: Some("maria|queen"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_84_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("faire"), regexp: None, inflected: true },
    AntipatternToken { text: Some("tous|toute?s?|tout"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("partie"), regexp: None, inflected: false },
];
static ANTIPATTERN_85_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("grandes?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("p[èe]res?|oncles?|cousins?|neuveux?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_86_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("grands?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("m[èe]res?|tantes?|cousines?|nièces?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_87_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z][a-z]|noël|mi|mm|suède|tac|veto|visa"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_88_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("le"), regexp: None, inflected: false },
    AntipatternToken { text: Some("mer"), regexp: None, inflected: false },
    AntipatternToken { text: Some("\\d.*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_89_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ma"), regexp: None, inflected: false },
    AntipatternToken { text: Some("permis"), regexp: None, inflected: false },
];
static ANTIPATTERN_90_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("pareil"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pour"), regexp: None, inflected: false },
];
static ANTIPATTERN_91_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("plusieurs|certains"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("dure"), regexp: None, inflected: false },
];
static ANTIPATTERN_92_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("point"), regexp: None, inflected: false },
    AntipatternToken { text: Some("du"), regexp: None, inflected: false },
    AntipatternToken { text: Some("vue"), regexp: None, inflected: false },
];
static ANTIPATTERN_93_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("putain"), regexp: None, inflected: false },
    AntipatternToken { text: Some("de"), regexp: None, inflected: true },
];
static ANTIPATTERN_94_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("que"), regexp: None, inflected: false },
    AntipatternToken { text: Some("certains?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("traine"), regexp: None, inflected: false },
];
static ANTIPATTERN_95_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("quelle"), regexp: None, inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_96_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("quelque"), regexp: None, inflected: false },
    AntipatternToken { text: Some("choses"), regexp: None, inflected: false },
];
static ANTIPATTERN_97_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("quelques"), regexp: None, inflected: false },
    AntipatternToken { text: Some("chose"), regexp: None, inflected: false },
];
static ANTIPATTERN_98_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("qui"), regexp: None, inflected: false },
    AntipatternToken { text: Some("les?|la"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("sortie"), regexp: None, inflected: false },
];
static ANTIPATTERN_99_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sa"), regexp: None, inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_100_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sa"), regexp: None, inflected: false },
    AntipatternToken { text: Some("fait"), regexp: None, inflected: false },
];
static ANTIPATTERN_101_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sa"), regexp: None, inflected: false },
    AntipatternToken { text: Some("grand|hyper|super|aller|bip|bug"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_102_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("saint"), regexp: None, inflected: false },
    AntipatternToken { text: Some("-"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_103_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tes"), regexp: None, inflected: false },
    AntipatternToken { text: Some("mortes?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_104_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tou[ts]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("(?-i)[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_105_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tou[ts]|toutes?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_106_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tou[ts]|toutes?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("autours?|rendu"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_107_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tous"), regexp: None, inflected: false },
    AntipatternToken { text: Some("merci"), regexp: None, inflected: false },
];
static ANTIPATTERN_108_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("toute"), regexp: None, inflected: false },
    AntipatternToken { text: Some("faon"), regexp: None, inflected: false },
];
static ANTIPATTERN_109_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("triple"), regexp: None, inflected: false },
    AntipatternToken { text: Some("foyers?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_110_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("trouver"), regexp: None, inflected: true },
    AntipatternToken { text: Some("sa"), regexp: None, inflected: false },
];
static ANTIPATTERN_111_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("vingt|trente|quarante|cinquante|soixante"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("et"), regexp: None, inflected: false },
    AntipatternToken { text: Some("un"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_112_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("vos"), regexp: None, inflected: false },
    AntipatternToken { text: Some("nom"), regexp: None, inflected: false },
    AntipatternToken { text: Some("et"), regexp: None, inflected: false },
    AntipatternToken { text: Some("prénoms?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_113_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("la|les|une|des"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("pers"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
];
static ANTIPATTERN_114_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("être"), regexp: None, inflected: true },
    AntipatternToken { text: Some("plusieurs"), regexp: None, inflected: false },
];
static ANTIPATTERN_115_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("emmanuel|emmanuel"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("macron|macron"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_116_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("en"), regexp: None, inflected: false },
    AntipatternToken { text: Some("activités"), regexp: None, inflected: false },
    AntipatternToken { text: Some("et"), regexp: None, inflected: false },
    AntipatternToken { text: Some("techniques"), regexp: None, inflected: false },
];
static ANTIPATTERN_117_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("riche|scindée?s?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("en"), regexp: None, inflected: false },
    AntipatternToken { text: Some("activités"), regexp: None, inflected: false },
];
static ANTIPATTERN_118_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("au"), regexp: None, inflected: false },
    AntipatternToken { text: Some("cas"), regexp: None, inflected: false },
    AntipatternToken { text: Some("où"), regexp: None, inflected: false },
];
static ANTIPATTERN_119_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("avoir"), regexp: None, inflected: true },
    AntipatternToken { text: Some("tu"), regexp: None, inflected: false },
];
static ANTIPATTERN_120_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("s'"), regexp: None, inflected: false },
    AntipatternToken { text: Some("est"), regexp: None, inflected: false },
];
static ANTIPATTERN_121_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ça"), regexp: None, inflected: false },
    AntipatternToken { text: Some("y"), regexp: None, inflected: false },
    AntipatternToken { text: Some("est"), regexp: None, inflected: false },
];
static ANTIPATTERN_122_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("de"), regexp: None, inflected: false },
    AntipatternToken { text: Some("fait"), regexp: None, inflected: false },
    AntipatternToken { text: Some("parti"), regexp: None, inflected: false },
];
static ANTIPATTERN_123_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("saint"), regexp: None, inflected: false },
    AntipatternToken { text: Some("gilles"), regexp: None, inflected: false },
];
static ANTIPATTERN_124_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sociétés"), regexp: None, inflected: false },
    AntipatternToken { text: Some("de"), regexp: None, inflected: false },
    AntipatternToken { text: Some("gilles"), regexp: None, inflected: false },
];
static ANTIPATTERN_125_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("je|il|elle"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("google"), regexp: None, inflected: false },
];
static ANTIPATTERN_126_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("lien|page|site|web|recherche|clé|mot|compte|entreprise|solution|e-mail|email|adresse|courrier|poste|messagerie"), regexp: None, inflected: false },
    AntipatternToken { text: Some("google"), regexp: None, inflected: false },
];
static ANTIPATTERN_127_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("grand"), regexp: None, inflected: false },
    AntipatternToken { text: Some("public"), regexp: None, inflected: false },
];
static ANTIPATTERN_128_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("heur"), regexp: None, inflected: false },
    AntipatternToken { text: Some("et|ou"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("malheur"), regexp: None, inflected: false },
];
static ANTIPATTERN_129_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("heurs"), regexp: None, inflected: false },
    AntipatternToken { text: Some("et|ou"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("malheurs"), regexp: None, inflected: false },
];
static ANTIPATTERN_130_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("lundis?|mardis?|mercredis?|jeudis?|vendredis?|samedis?|dimanches?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("saints?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_131_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mardi"), regexp: None, inflected: false },
    AntipatternToken { text: Some("gras"), regexp: None, inflected: false },
];
static ANTIPATTERN_132_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("del?|la|do|el"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("mar"), regexp: None, inflected: false },
];
static ANTIPATTERN_133_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("dim"), regexp: None, inflected: false },
    AntipatternToken { text: Some("et"), regexp: None, inflected: false },
];
static ANTIPATTERN_134_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("jeu"), regexp: None, inflected: false },
    AntipatternToken { text: Some("soir|après-midi|matin|\\d.*"), regexp: None, inflected: false },
];
static ANTIPATTERN_135_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mer|jeu"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("soir|après-midi|matin|\\d.*"), regexp: None, inflected: false },
];
static ANTIPATTERN_136_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("de"), regexp: None, inflected: false },
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
    AntipatternToken { text: Some("voire"), regexp: None, inflected: false },
];
static ANTIPATTERN_137_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("il|elle|on"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
];
static ANTIPATTERN_138_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
    AntipatternToken { text: Some("qui"), regexp: None, inflected: false },
];
static ANTIPATTERN_139_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
    AntipatternToken { text: Some("très"), regexp: None, inflected: false },
    AntipatternToken { text: Some("forte"), regexp: None, inflected: false },
];
static ANTIPATTERN_140_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
    AntipatternToken { text: Some("tête"), regexp: None, inflected: false },
    AntipatternToken { text: Some("aux"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pieds"), regexp: None, inflected: false },
];
static ANTIPATTERN_141_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("par"), regexp: None, inflected: false },
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
    AntipatternToken { text: Some("suite"), regexp: None, inflected: false },
];
static ANTIPATTERN_142_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("à"), regexp: None, inflected: false },
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
    AntipatternToken { text: Some("une"), regexp: None, inflected: false },
];
static ANTIPATTERN_143_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
    AntipatternToken { text: Some("tête"), regexp: None, inflected: false },
    AntipatternToken { text: Some("aux"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pieds"), regexp: None, inflected: false },
];
static ANTIPATTERN_144_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("par"), regexp: None, inflected: false },
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
    AntipatternToken { text: Some("suite"), regexp: None, inflected: false },
];
static ANTIPATTERN_145_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("à"), regexp: None, inflected: false },
    AntipatternToken { text: Some("la"), regexp: None, inflected: false },
    AntipatternToken { text: Some("une"), regexp: None, inflected: false },
];
static ANTIPATTERN_146_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("quelqu'"), regexp: None, inflected: false },
    AntipatternToken { text: Some("un"), regexp: None, inflected: false },
    AntipatternToken { text: Some("leur"), regexp: None, inflected: false },
];
static ANTIPATTERN_147_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("sa"), regexp: None, inflected: false },
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
];
static ANTIPATTERN_148_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("media|media"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("\\p{lu}.*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_149_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("\\p{lu}.*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("media|media"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_150_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mass|social"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("media"), regexp: None, inflected: false },
];
static ANTIPATTERN_151_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("media"), regexp: None, inflected: false },
    AntipatternToken { text: Some("centers?|centres?|box|players?|files?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_152_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("merci|désolée?s?|navrée?s?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("pour"), regexp: None, inflected: false },
    AntipatternToken { text: Some("le"), regexp: None, inflected: false },
    AntipatternToken { text: Some("dîner"), regexp: None, inflected: false },
];
static ANTIPATTERN_153_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("a"), regexp: None, inflected: false },
    AntipatternToken { text: Some("mes"), regexp: None, inflected: false },
    AntipatternToken { text: Some("cotés"), regexp: None, inflected: false },
];
static ANTIPATTERN_154_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[\\d,\\. ]+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("milles"), regexp: None, inflected: false },
];
static ANTIPATTERN_155_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("dizaines|centaines|milliers|millions"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("de"), regexp: None, inflected: false },
    AntipatternToken { text: Some("milles"), regexp: None, inflected: false },
];
static ANTIPATTERN_156_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("les"), regexp: None, inflected: false },
    AntipatternToken { text: Some("mille-et-une"), regexp: None, inflected: false },
    AntipatternToken { text: Some("nuits"), regexp: None, inflected: false },
];
static ANTIPATTERN_157_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[tsm]on"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("cote"), regexp: None, inflected: false },
];
static ANTIPATTERN_158_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("au|du"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[ts]on"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[ahâeiouéèîœ].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_159_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ny"), regexp: None, inflected: false },
    AntipatternToken { text: Some("c|city|county|counties|state"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_160_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("parton"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ne"), regexp: None, inflected: false },
];
static ANTIPATTERN_161_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some(".*(?-i)[a-z].*als$"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_162_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some(".*(?-i)[a-z].*als$"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_163_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("are|these|port|#|-|the"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some(".*als"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_164_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("journals"), regexp: None, inflected: false },
    AntipatternToken { text: Some("online"), regexp: None, inflected: false },
];
static ANTIPATTERN_165_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("professionnal"), regexp: None, inflected: false },
    AntipatternToken { text: Some("jury"), regexp: None, inflected: false },
];
static ANTIPATTERN_166_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
];
static ANTIPATTERN_167_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("…"), regexp: None, inflected: false },
];
static ANTIPATTERN_168_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("…"), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
];
static ANTIPATTERN_169_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("stare|legnickie|žižkovo|trnové|tehelné|svaté|slivo|královo|ryńskie|dolne|dobré|totem|jill|single"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("[pp]ole"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_170_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("poles?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("positions?|dance"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_171_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("ne"), regexp: None, inflected: false },
    AntipatternToken { text: Some("puis"), regexp: None, inflected: false },
];
static ANTIPATTERN_172_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("que"), regexp: None, inflected: false },
    AntipatternToken { text: Some("sais"), regexp: None, inflected: false },
    AntipatternToken { text: Some("-je"), regexp: None, inflected: false },
];
static ANTIPATTERN_173_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[«‹»›„“‟”’\"❝❞❮❯⹂〝〞〟＂‚‘‛❛❜❟]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("quand"), regexp: None, inflected: false },
    AntipatternToken { text: Some("[«‹»›„“‟”’\"❝❞❮❯⹂〝〞〟＂‚‘‛❛❜❟]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_174_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("quand"), regexp: None, inflected: false },
    AntipatternToken { text: Some("à"), regexp: None, inflected: false },
    AntipatternToken { text: Some("peu"), regexp: None, inflected: false },
    AntipatternToken { text: Some("près"), regexp: None, inflected: false },
];
static ANTIPATTERN_175_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("quasi|non"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("(?-i)[a-z].*"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_176_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("quasi|non"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("points?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("de"), regexp: None, inflected: false },
    AntipatternToken { text: Some("vue"), regexp: None, inflected: false },
];
static ANTIPATTERN_177_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("éditions?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("quasi|non"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("lieu"), regexp: None, inflected: false },
];
static ANTIPATTERN_178_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("vous"), regexp: None, inflected: false },
    AntipatternToken { text: Some("saurez"), regexp: None, inflected: false },
    AntipatternToken { text: Some("quelque"), regexp: None, inflected: false },
    AntipatternToken { text: Some("jour"), regexp: None, inflected: false },
];
static ANTIPATTERN_179_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("que"), regexp: None, inflected: true },
    AntipatternToken { text: Some("en"), regexp: None, inflected: false },
    AntipatternToken { text: Some("on"), regexp: None, inflected: false },
    AntipatternToken { text: Some(".*(ais|ait|ée?s?|er)"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_180_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("3[2-9]|[4-9][0-9]|[1-9][0-9][0-9]+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some(":"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("le"), regexp: None, inflected: false },
    AntipatternToken { text: Some("réveillon"), regexp: None, inflected: false },
];
static ANTIPATTERN_181_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("deus"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ex"), regexp: None, inflected: false },
    AntipatternToken { text: Some("machina"), regexp: None, inflected: false },
];
static ANTIPATTERN_182_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mi"), regexp: None, inflected: false },
    AntipatternToken { text: Some("bémol|majeur|mineur|aigu|grave"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_183_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("faire"), regexp: None, inflected: true },
    AntipatternToken { text: Some("se"), regexp: None, inflected: false },
    AntipatternToken { text: Some("preuve"), regexp: None, inflected: false },
];
static ANTIPATTERN_184_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("mouvement|fin"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("se"), regexp: None, inflected: false },
    AntipatternToken { text: Some("solidarité|saison"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_185_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("prendre"), regexp: None, inflected: true },
    AntipatternToken { text: Some("se"), regexp: None, inflected: false },
    AntipatternToken { text: Some("sympathie"), regexp: None, inflected: false },
];
static ANTIPATTERN_186_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("se"), regexp: None, inflected: false },
    AntipatternToken { text: Some("marrie|nymphose|chrysalide|rédie|ballade|craie|poursuite"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_187_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tout"), regexp: None, inflected: false },
    AntipatternToken { text: Some("se"), regexp: None, inflected: false },
    AntipatternToken { text: Some("suite"), regexp: None, inflected: false },
];
static ANTIPATTERN_188_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("patrouilles?|familles?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("se"), regexp: None, inflected: false },
    AntipatternToken { text: Some("soldats"), regexp: None, inflected: false },
];
static ANTIPATTERN_189_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("se"), regexp: None, inflected: false },
    AntipatternToken { text: Some(".*ante?s?"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_190_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("je|il|elle"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("skype"), regexp: None, inflected: false },
];
static ANTIPATTERN_191_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("statu"), regexp: None, inflected: false },
    AntipatternToken { text: Some("quo"), regexp: None, inflected: false },
];
static ANTIPATTERN_192_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("en"), regexp: None, inflected: false },
    AntipatternToken { text: Some("même"), regexp: None, inflected: false },
    AntipatternToken { text: Some("temps"), regexp: None, inflected: false },
];
static ANTIPATTERN_193_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
];
static ANTIPATTERN_194_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z].*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("environ"), regexp: None, inflected: false },
];
static ANTIPATTERN_195_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[mts]e"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some(".*ant"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_196_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("de"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ne"), regexp: None, inflected: false },
    AntipatternToken { text: Some("se"), regexp: None, inflected: false },
    AntipatternToken { text: Some("pas"), regexp: None, inflected: false },
];
static ANTIPATTERN_197_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("en"), regexp: None, inflected: false },
    AntipatternToken { text: Some("se"), regexp: None, inflected: false },
    AntipatternToken { text: Some("moment"), regexp: None, inflected: false },
];
static ANTIPATTERN_198_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("me"), regexp: None, inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_199_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("se"), regexp: None, inflected: false },
    AntipatternToken { text: Some("court|produit|marrie|nymphose|chrysalide|rédie|ballade|craie|poursuite|in"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_200_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("se"), regexp: None, inflected: false },
    AntipatternToken { text: Some("gars"), regexp: None, inflected: false },
];
static ANTIPATTERN_201_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("te"), regexp: None, inflected: false },
    AntipatternToken { text: Some("taux"), regexp: None, inflected: false },
];
static ANTIPATTERN_202_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tout|joueurs?|joueuses?"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("se"), regexp: None, inflected: false },
    AntipatternToken { text: Some("suite|tennis"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_203_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("en"), regexp: None, inflected: false },
    AntipatternToken { text: Some("grande"), regexp: None, inflected: false },
    AntipatternToken { text: Some("partie"), regexp: None, inflected: false },
];
static ANTIPATTERN_204_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("['‘\"“]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("['‘\"“]"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_205_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[\\#]"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: None, regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_206_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("[a-z-]+_[a-z-]+"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("."), regexp: None, inflected: false },
    AntipatternToken { text: Some("[a-z0-9]{2,7}"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_207_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("`"), regexp: None, inflected: false },
    AntipatternToken { text: Some(".*_.*"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("`"), regexp: None, inflected: false },
];
static ANTIPATTERN_208_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("tord"), regexp: None, inflected: false },
    AntipatternToken { text: Some("boyau"), regexp: None, inflected: false },
];
static ANTIPATTERN_209_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("à"), regexp: None, inflected: false },
    AntipatternToken { text: Some("tous"), regexp: None, inflected: false },
];
static ANTIPATTERN_210_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("aller"), regexp: None, inflected: true },
    AntipatternToken { text: Some("vous"), regexp: None, inflected: false },
    AntipatternToken { text: Some(".*ez$"), regexp: Some("yes"), inflected: false },
];
static ANTIPATTERN_211_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("l'"), regexp: None, inflected: false },
    AntipatternToken { text: Some("on"), regexp: None, inflected: false },
    AntipatternToken { text: Some("ne"), regexp: None, inflected: true },
];
static ANTIPATTERN_212_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("on"), regexp: None, inflected: false },
    AntipatternToken { text: Some("taxes"), regexp: None, inflected: false },
];
static ANTIPATTERN_213_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("s'|t'|m'|nous|vous"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("avoir"), regexp: None, inflected: true },
    AntipatternToken { text: Some("avoir"), regexp: None, inflected: true },
];
static ANTIPATTERN_214_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("se|te|me|nous|vous"), regexp: Some("yes"), inflected: false },
    AntipatternToken { text: Some("voir"), regexp: None, inflected: true },
];
static ANTIPATTERN_215_TOKENS: &[AntipatternToken] = &[
    AntipatternToken { text: Some("x"), regexp: None, inflected: false },
    AntipatternToken { text: Some("window"), regexp: None, inflected: false },
];

/// Antipatterns for FR (sorted by rule_id)
/// Total: 216 antipatterns
pub static FR_ANTIPATTERNS: &[Antipattern] = &[
    Antipattern { rule_id: "ACCORD_CENTAINE_DE", tokens: ANTIPATTERN_0_TOKENS },
    Antipattern { rule_id: "ACCORD_CENTAINE_DE", tokens: ANTIPATTERN_1_TOKENS },
    Antipattern { rule_id: "ACCORD_CENTAINE_DE", tokens: ANTIPATTERN_2_TOKENS },
    Antipattern { rule_id: "ACCORD_CENTAINE_DE", tokens: ANTIPATTERN_3_TOKENS },
    Antipattern { rule_id: "ACCORD_CENTAINE_DE", tokens: ANTIPATTERN_4_TOKENS },
    Antipattern { rule_id: "ACCORD_CENTAINE_DE", tokens: ANTIPATTERN_5_TOKENS },
    Antipattern { rule_id: "AGREEMENT_ADJ", tokens: ANTIPATTERN_6_TOKENS },
    Antipattern { rule_id: "AGREEMENT_NUMERAL_SINGULAR", tokens: ANTIPATTERN_7_TOKENS },
    Antipattern { rule_id: "ANOMAL_ANORMAL", tokens: ANTIPATTERN_8_TOKENS },
    Antipattern { rule_id: "AUTANT_POUR_MOI", tokens: ANTIPATTERN_9_TOKENS },
    Antipattern { rule_id: "AVANT_AVENT", tokens: ANTIPATTERN_10_TOKENS },
    Antipattern { rule_id: "AVANT_AVENT", tokens: ANTIPATTERN_11_TOKENS },
    Antipattern { rule_id: "AVANT_AVENT", tokens: ANTIPATTERN_12_TOKENS },
    Antipattern { rule_id: "A_OU_EN_MOYEN_DE_TRANSPORT", tokens: ANTIPATTERN_13_TOKENS },
    Antipattern { rule_id: "A_OU_EN_MOYEN_DE_TRANSPORT", tokens: ANTIPATTERN_14_TOKENS },
    Antipattern { rule_id: "A_OU_EN_MOYEN_DE_TRANSPORT", tokens: ANTIPATTERN_15_TOKENS },
    Antipattern { rule_id: "A_OU_EN_MOYEN_DE_TRANSPORT", tokens: ANTIPATTERN_16_TOKENS },
    Antipattern { rule_id: "A_PEU_PRES", tokens: ANTIPATTERN_17_TOKENS },
    Antipattern { rule_id: "A_TOUTE_FIN_UTILE", tokens: ANTIPATTERN_18_TOKENS },
    Antipattern { rule_id: "BENEFICIER_A", tokens: ANTIPATTERN_19_TOKENS },
    Antipattern { rule_id: "BON_MARCHE", tokens: ANTIPATTERN_20_TOKENS },
    Antipattern { rule_id: "BOUCHE_A_OREILLE", tokens: ANTIPATTERN_21_TOKENS },
    Antipattern { rule_id: "CAS_CONTACT", tokens: ANTIPATTERN_22_TOKENS },
    Antipattern { rule_id: "CEST_TYPO", tokens: ANTIPATTERN_23_TOKENS },
    Antipattern { rule_id: "CEST_TYPO", tokens: ANTIPATTERN_24_TOKENS },
    Antipattern { rule_id: "CHIFFRE_SIECLE", tokens: ANTIPATTERN_25_TOKENS },
    Antipattern { rule_id: "CHINA", tokens: ANTIPATTERN_26_TOKENS },
    Antipattern { rule_id: "CHINA", tokens: ANTIPATTERN_27_TOKENS },
    Antipattern { rule_id: "CHINE_AVEC_MAJUSCULE", tokens: ANTIPATTERN_28_TOKENS },
    Antipattern { rule_id: "COND_FUTUR_AVOIR_AIMERAI", tokens: ANTIPATTERN_29_TOKENS },
    Antipattern { rule_id: "CONFUSION_NE_SE", tokens: ANTIPATTERN_30_TOKENS },
    Antipattern { rule_id: "CONFUSION_OR_HORS", tokens: ANTIPATTERN_31_TOKENS },
    Antipattern { rule_id: "CONFUSION_PLUS_TOT_PLUTOT", tokens: ANTIPATTERN_32_TOKENS },
    Antipattern { rule_id: "CONFUSION_PLUS_TOT_PLUTOT", tokens: ANTIPATTERN_33_TOKENS },
    Antipattern { rule_id: "CONFUSION_SU_SUR", tokens: ANTIPATTERN_34_TOKENS },
    Antipattern { rule_id: "CONFUSION_SU_SUR", tokens: ANTIPATTERN_35_TOKENS },
    Antipattern { rule_id: "DEBUTER_COMMENCER", tokens: ANTIPATTERN_36_TOKENS },
    Antipattern { rule_id: "DEBUTER_COMMENCER", tokens: ANTIPATTERN_37_TOKENS },
    Antipattern { rule_id: "DEBUTER_COMMENCER", tokens: ANTIPATTERN_38_TOKENS },
    Antipattern { rule_id: "DEBUTER_COMMENCER", tokens: ANTIPATTERN_39_TOKENS },
    Antipattern { rule_id: "DEMARRER_LANCER", tokens: ANTIPATTERN_40_TOKENS },
    Antipattern { rule_id: "DEMARRER_LANCER", tokens: ANTIPATTERN_41_TOKENS },
    Antipattern { rule_id: "DEMARRER_LANCER", tokens: ANTIPATTERN_42_TOKENS },
    Antipattern { rule_id: "DEMARRER_LANCER", tokens: ANTIPATTERN_43_TOKENS },
    Antipattern { rule_id: "DERNIER_CRI", tokens: ANTIPATTERN_44_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END", tokens: ANTIPATTERN_45_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END", tokens: ANTIPATTERN_46_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END", tokens: ANTIPATTERN_47_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END", tokens: ANTIPATTERN_48_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END", tokens: ANTIPATTERN_49_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END", tokens: ANTIPATTERN_50_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END", tokens: ANTIPATTERN_51_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END", tokens: ANTIPATTERN_52_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END2", tokens: ANTIPATTERN_53_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END2", tokens: ANTIPATTERN_54_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END2", tokens: ANTIPATTERN_55_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END2", tokens: ANTIPATTERN_56_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END2", tokens: ANTIPATTERN_57_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END2", tokens: ANTIPATTERN_58_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END2", tokens: ANTIPATTERN_59_TOKENS },
    Antipattern { rule_id: "DETERMINER_SENT_END2", tokens: ANTIPATTERN_60_TOKENS },
    Antipattern { rule_id: "DIEU1", tokens: ANTIPATTERN_61_TOKENS },
    Antipattern { rule_id: "D_J", tokens: ANTIPATTERN_62_TOKENS },
    Antipattern { rule_id: "D_J", tokens: ANTIPATTERN_63_TOKENS },
    Antipattern { rule_id: "D_J", tokens: ANTIPATTERN_64_TOKENS },
    Antipattern { rule_id: "D_J", tokens: ANTIPATTERN_65_TOKENS },
    Antipattern { rule_id: "D_J", tokens: ANTIPATTERN_66_TOKENS },
    Antipattern { rule_id: "D_J", tokens: ANTIPATTERN_67_TOKENS },
    Antipattern { rule_id: "D_J", tokens: ANTIPATTERN_68_TOKENS },
    Antipattern { rule_id: "D_J", tokens: ANTIPATTERN_69_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_70_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_71_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_72_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_73_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_74_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_75_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_76_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_77_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_78_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_79_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_80_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_81_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_82_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_83_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_84_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_85_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_86_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_87_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_88_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_89_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_90_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_91_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_92_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_93_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_94_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_95_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_96_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_97_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_98_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_99_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_100_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_101_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_102_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_103_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_104_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_105_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_106_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_107_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_108_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_109_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_110_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_111_TOKENS },
    Antipattern { rule_id: "D_N", tokens: ANTIPATTERN_112_TOKENS },
    Antipattern { rule_id: "D_VPPA", tokens: ANTIPATTERN_113_TOKENS },
    Antipattern { rule_id: "D_VPPA", tokens: ANTIPATTERN_114_TOKENS },
    Antipattern { rule_id: "EMMANUEL_MACRON", tokens: ANTIPATTERN_115_TOKENS },
    Antipattern { rule_id: "EN_ACTIVITES", tokens: ANTIPATTERN_116_TOKENS },
    Antipattern { rule_id: "EN_ACTIVITES", tokens: ANTIPATTERN_117_TOKENS },
    Antipattern { rule_id: "EN_CAS_OU", tokens: ANTIPATTERN_118_TOKENS },
    Antipattern { rule_id: "ES_TU", tokens: ANTIPATTERN_119_TOKENS },
    Antipattern { rule_id: "ES_TU", tokens: ANTIPATTERN_120_TOKENS },
    Antipattern { rule_id: "ES_TU", tokens: ANTIPATTERN_121_TOKENS },
    Antipattern { rule_id: "FAIRE_PARTI", tokens: ANTIPATTERN_122_TOKENS },
    Antipattern { rule_id: "GILLES", tokens: ANTIPATTERN_123_TOKENS },
    Antipattern { rule_id: "GILLES", tokens: ANTIPATTERN_124_TOKENS },
    Antipattern { rule_id: "GOOGLE", tokens: ANTIPATTERN_125_TOKENS },
    Antipattern { rule_id: "GOOGLE", tokens: ANTIPATTERN_126_TOKENS },
    Antipattern { rule_id: "GRAND_PUBLIC", tokens: ANTIPATTERN_127_TOKENS },
    Antipattern { rule_id: "HEUR_HEURE", tokens: ANTIPATTERN_128_TOKENS },
    Antipattern { rule_id: "HEUR_HEURE", tokens: ANTIPATTERN_129_TOKENS },
    Antipattern { rule_id: "JOURS", tokens: ANTIPATTERN_130_TOKENS },
    Antipattern { rule_id: "JOURS", tokens: ANTIPATTERN_131_TOKENS },
    Antipattern { rule_id: "JOURS", tokens: ANTIPATTERN_132_TOKENS },
    Antipattern { rule_id: "JOURS", tokens: ANTIPATTERN_133_TOKENS },
    Antipattern { rule_id: "JOURS", tokens: ANTIPATTERN_134_TOKENS },
    Antipattern { rule_id: "JOURS", tokens: ANTIPATTERN_135_TOKENS },
    Antipattern { rule_id: "LA_LA2", tokens: ANTIPATTERN_136_TOKENS },
    Antipattern { rule_id: "LA_LA2", tokens: ANTIPATTERN_137_TOKENS },
    Antipattern { rule_id: "LA_LA2", tokens: ANTIPATTERN_138_TOKENS },
    Antipattern { rule_id: "LA_LA2", tokens: ANTIPATTERN_139_TOKENS },
    Antipattern { rule_id: "LA_LA2", tokens: ANTIPATTERN_140_TOKENS },
    Antipattern { rule_id: "LA_LA2", tokens: ANTIPATTERN_141_TOKENS },
    Antipattern { rule_id: "LA_LA2", tokens: ANTIPATTERN_142_TOKENS },
    Antipattern { rule_id: "LA_LA_IMP", tokens: ANTIPATTERN_143_TOKENS },
    Antipattern { rule_id: "LA_LA_IMP", tokens: ANTIPATTERN_144_TOKENS },
    Antipattern { rule_id: "LA_LA_IMP", tokens: ANTIPATTERN_145_TOKENS },
    Antipattern { rule_id: "LEUR_LEURRE", tokens: ANTIPATTERN_146_TOKENS },
    Antipattern { rule_id: "MA_VOYELLE", tokens: ANTIPATTERN_147_TOKENS },
    Antipattern { rule_id: "MEDIA", tokens: ANTIPATTERN_148_TOKENS },
    Antipattern { rule_id: "MEDIA", tokens: ANTIPATTERN_149_TOKENS },
    Antipattern { rule_id: "MEDIA", tokens: ANTIPATTERN_150_TOKENS },
    Antipattern { rule_id: "MEDIA", tokens: ANTIPATTERN_151_TOKENS },
    Antipattern { rule_id: "MERCI_POUR_INF", tokens: ANTIPATTERN_152_TOKENS },
    Antipattern { rule_id: "MES_M_AIT", tokens: ANTIPATTERN_153_TOKENS },
    Antipattern { rule_id: "MILLES", tokens: ANTIPATTERN_154_TOKENS },
    Antipattern { rule_id: "MILLES", tokens: ANTIPATTERN_155_TOKENS },
    Antipattern { rule_id: "MILLE_ET_UNE_NUITS", tokens: ANTIPATTERN_156_TOKENS },
    Antipattern { rule_id: "MON_NFS", tokens: ANTIPATTERN_157_TOKENS },
    Antipattern { rule_id: "MON_NFS", tokens: ANTIPATTERN_158_TOKENS },
    Antipattern { rule_id: "NY", tokens: ANTIPATTERN_159_TOKENS },
    Antipattern { rule_id: "PARTON_VERBE", tokens: ANTIPATTERN_160_TOKENS },
    Antipattern { rule_id: "PLURIEL_AL2", tokens: ANTIPATTERN_161_TOKENS },
    Antipattern { rule_id: "PLURIEL_AL2", tokens: ANTIPATTERN_162_TOKENS },
    Antipattern { rule_id: "PLURIEL_AL2", tokens: ANTIPATTERN_163_TOKENS },
    Antipattern { rule_id: "PLURIEL_AL2", tokens: ANTIPATTERN_164_TOKENS },
    Antipattern { rule_id: "PLURIEL_AL2", tokens: ANTIPATTERN_165_TOKENS },
    Antipattern { rule_id: "POINTS_SUSPENSION", tokens: ANTIPATTERN_166_TOKENS },
    Antipattern { rule_id: "POINTS_SUSPENSION", tokens: ANTIPATTERN_167_TOKENS },
    Antipattern { rule_id: "POINTS_SUSPENSION", tokens: ANTIPATTERN_168_TOKENS },
    Antipattern { rule_id: "POLE", tokens: ANTIPATTERN_169_TOKENS },
    Antipattern { rule_id: "POLE", tokens: ANTIPATTERN_170_TOKENS },
    Antipattern { rule_id: "PUIS_QUE", tokens: ANTIPATTERN_171_TOKENS },
    Antipattern { rule_id: "PUIS_QUE", tokens: ANTIPATTERN_172_TOKENS },
    Antipattern { rule_id: "QUAND_QUANT", tokens: ANTIPATTERN_173_TOKENS },
    Antipattern { rule_id: "QUAND_QUANT", tokens: ANTIPATTERN_174_TOKENS },
    Antipattern { rule_id: "QUASI_NOM", tokens: ANTIPATTERN_175_TOKENS },
    Antipattern { rule_id: "QUASI_NOM", tokens: ANTIPATTERN_176_TOKENS },
    Antipattern { rule_id: "QUASI_NOM", tokens: ANTIPATTERN_177_TOKENS },
    Antipattern { rule_id: "QUELQUE_TRUC", tokens: ANTIPATTERN_178_TOKENS },
    Antipattern { rule_id: "QU_EN_QUAND", tokens: ANTIPATTERN_179_TOKENS },
    Antipattern { rule_id: "REVEILLON_MAJ", tokens: ANTIPATTERN_180_TOKENS },
    Antipattern { rule_id: "SEMI", tokens: ANTIPATTERN_181_TOKENS },
    Antipattern { rule_id: "SEMI", tokens: ANTIPATTERN_182_TOKENS },
    Antipattern { rule_id: "SE_SA", tokens: ANTIPATTERN_183_TOKENS },
    Antipattern { rule_id: "SE_SA", tokens: ANTIPATTERN_184_TOKENS },
    Antipattern { rule_id: "SE_SA", tokens: ANTIPATTERN_185_TOKENS },
    Antipattern { rule_id: "SE_SA", tokens: ANTIPATTERN_186_TOKENS },
    Antipattern { rule_id: "SE_SA", tokens: ANTIPATTERN_187_TOKENS },
    Antipattern { rule_id: "SE_SES", tokens: ANTIPATTERN_188_TOKENS },
    Antipattern { rule_id: "SE_SES", tokens: ANTIPATTERN_189_TOKENS },
    Antipattern { rule_id: "SKYPE", tokens: ANTIPATTERN_190_TOKENS },
    Antipattern { rule_id: "STATU_QUO", tokens: ANTIPATTERN_191_TOKENS },
    Antipattern { rule_id: "TANT_TEMPS", tokens: ANTIPATTERN_192_TOKENS },
    Antipattern { rule_id: "TE_NV", tokens: ANTIPATTERN_193_TOKENS },
    Antipattern { rule_id: "TE_NV", tokens: ANTIPATTERN_194_TOKENS },
    Antipattern { rule_id: "TE_NV", tokens: ANTIPATTERN_195_TOKENS },
    Antipattern { rule_id: "TE_NV", tokens: ANTIPATTERN_196_TOKENS },
    Antipattern { rule_id: "TE_NV", tokens: ANTIPATTERN_197_TOKENS },
    Antipattern { rule_id: "TE_NV", tokens: ANTIPATTERN_198_TOKENS },
    Antipattern { rule_id: "TE_NV", tokens: ANTIPATTERN_199_TOKENS },
    Antipattern { rule_id: "TE_NV", tokens: ANTIPATTERN_200_TOKENS },
    Antipattern { rule_id: "TE_NV", tokens: ANTIPATTERN_201_TOKENS },
    Antipattern { rule_id: "TE_NV", tokens: ANTIPATTERN_202_TOKENS },
    Antipattern { rule_id: "TIRER_PARTI", tokens: ANTIPATTERN_203_TOKENS },
    Antipattern { rule_id: "TIRET_BAS", tokens: ANTIPATTERN_204_TOKENS },
    Antipattern { rule_id: "TIRET_BAS", tokens: ANTIPATTERN_205_TOKENS },
    Antipattern { rule_id: "TIRET_BAS", tokens: ANTIPATTERN_206_TOKENS },
    Antipattern { rule_id: "TIRET_BAS", tokens: ANTIPATTERN_207_TOKENS },
    Antipattern { rule_id: "TORD_TORT", tokens: ANTIPATTERN_208_TOKENS },
    Antipattern { rule_id: "TOUT_CE", tokens: ANTIPATTERN_209_TOKENS },
    Antipattern { rule_id: "VIRG_NON_TROUVEE", tokens: ANTIPATTERN_210_TOKENS },
    Antipattern { rule_id: "VIRG_NON_TROUVEE", tokens: ANTIPATTERN_211_TOKENS },
    Antipattern { rule_id: "VIRG_NON_TROUVEE", tokens: ANTIPATTERN_212_TOKENS },
    Antipattern { rule_id: "VOIR_INFINITIF", tokens: ANTIPATTERN_213_TOKENS },
    Antipattern { rule_id: "VOIR_INFINITIF", tokens: ANTIPATTERN_214_TOKENS },
    Antipattern { rule_id: "X_WINDOW", tokens: ANTIPATTERN_215_TOKENS },
];

use std::collections::HashMap;
use std::sync::LazyLock;

/// Lookup antipatterns by rule ID
pub static FR_ANTIPATTERNS_BY_RULE: LazyLock<HashMap<&'static str, Vec<&'static Antipattern>>> = LazyLock::new(|| {
	let mut map: HashMap<&'static str, Vec<&'static Antipattern>> = HashMap::new();
	for ap in FR_ANTIPATTERNS {
		map.entry(ap.rule_id).or_default().push(ap);
	}
	map
});

/// Get antipatterns for a rule ID
pub fn get_fr_antipatterns(rule_id: &str) -> Option<&'static Vec<&'static Antipattern>> {
	FR_ANTIPATTERNS_BY_RULE.get(rule_id)
}
