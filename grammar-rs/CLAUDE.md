# Grammar-RS - Instructions Claude

Tu es un wrapper Rust pour les capacités de vérification grammaticale de LanguageTool.

## Principes
- Implémenter exactement les mêmes features que LanguageTool
- Synchroniser les règles via `sync-lt.rs` (LanguageTool = source de vérité)
- Correctness > Performance (pour l'instant)

## Workflow documentation
- `missing.md` = features **non implémentées uniquement**
- `CLAUDE.md` = instructions + features complètes/en cours/différées

### Tags de statut
| Tag | Signification |
|-----|---------------|
| ✅ COMPLETED | Feature terminée et intégrée |
| ⏸️ IN PROGRESS | En cours de développement |
| ⏸️ DEFERRED | Reporté (justification requise) |

Quand tu termines une feature → déplace de `missing.md` vers ici avec le tag approprié.

## Commandes utiles
```bash
# Synchroniser depuis LanguageTool
cargo run --bin sync-lt -- --languagetool-path ../languagetool

# Régénérer tout
cargo run --bin sync-lt -- --languagetool-path ../languagetool --force

# Tests
cargo test
```

---

# Features Complètes

## ✅ COMPLETED - Antipatterns
**Description:** Exceptions aux règles - patterns qui ressemblent à des erreurs mais sont corrects.
**Stats:** 1,054 EN + 216 FR antipatterns
**Fichiers:** `en_antipatterns.rs`, `fr_antipatterns.rs`
**Intégration:** `AhoPatternRuleChecker.with_antipatterns()` filtre les faux positifs

## ✅ COMPLETED - POS Pattern Rules (EN)
**Description:** Règles nécessitant le Part-of-Speech tagging.
**Stats:** 94 règles EN avec POS tagger (441 mots dictionnaire + heuristiques suffixes)
**Fichiers:** `en_pos_patterns.rs`, `en_added.rs`
**Intégration:** `PosPatternChecker` dans le pipeline API EN

## ✅ COMPLETED - Style Checker
**Description:** Détection de phrases verbeuses et redondantes.
**Stats:** 1,399 règles EN (692 wordiness + 707 redundancy) + 51 FR
**Fichiers:** `en_style.rs`, `fr_style.rs`
**Intégration:** `StyleChecker` dans le pipeline API EN

## ✅ COMPLETED - Confusion Pairs
**Description:** Mots fréquemment confondus (their/there/they're, etc).
**Stats:** 1,363 paires EN + 101 FR
**Fichiers:** `en_confusion.rs`, `fr_confusion.rs`
**Intégration:** Via `RuleChecker` (EnglishConfusionRule, FrenchConfusionRule)

## ✅ COMPLETED - Coherency Checker
**Description:** Cohérence orthographique dans un document (color vs colour).
**Stats:** 2,480 paires de variantes
**Fichiers:** `en_coherency.rs`
**Intégration:** `CoherencyChecker` dans le pipeline API EN

## ✅ COMPLETED - Compound Word Checker
**Description:** Formatage des mots composés (spacing/hyphenation).
**Stats:** 8,541 règles EN + 1,346 FR
**Fichiers:** `en_compounds.rs`, `fr_compounds.rs`
**Intégration:** `CompoundWordChecker` dans le pipeline API EN

## ✅ COMPLETED - Uncountable Noun Checker
**Description:** Erreurs de pluralisation sur noms indénombrables.
**Stats:** 5,579 noms disponibles (~50 communs actifs par défaut)
**Fichiers:** `en_uncountable.rs`
**Intégration:** `UncountableNounChecker` dans le pipeline API EN

## ✅ COMPLETED - Diacritics Checker
**Description:** Accents corrects pour mots empruntés (cafe → café).
**Stats:** ~1,200+ règles
**Fichiers:** `en_diacritics.rs`
**Intégration:** `DiacriticsChecker` dans le pipeline API EN

## ✅ COMPLETED - Contraction Checker
**Description:** Apostrophes manquantes (dont → don't).
**Stats:** 180 règles
**Fichiers:** `en_contractions.rs`
**Intégration:** `ContractionChecker` dans le pipeline API EN

## ✅ COMPLETED - Context Checker
**Description:** Confusion de mots sensible au contexte (prescribe vs proscribe).
**Stats:** 12 règles contextuelles
**Fichiers:** `en_context_words.rs`
**Intégration:** `ContextChecker` dans le pipeline API EN

## ✅ COMPLETED - Pattern Rules
**Description:** Règles basées sur patterns textuels via Aho-Corasick.
**Stats:** 394 règles EN + 170 FR
**Fichiers:** `en_patterns.rs`, `fr_patterns.rs`
**Intégration:** `AhoPatternRuleChecker` dans les pipelines API

## ✅ COMPLETED - Basic Grammar Rules
**Description:** Règles de base (a/an, subject-verb, repeated words, etc).
**Stats:** 17+ règles EN + 9 FR
**Fichiers:** `src/rules/` (ImprovedAAnRule, SubjectVerbAgreementRule, etc)
**Intégration:** `RuleChecker` dans les pipelines API

---

# Features Différées

## ⏸️ DEFERRED - Hunspell / Morphologie
**Description:** Spell-checking morphologique avec lemmatisation.
**Raison:** Nécessite dépendances système (libhunspell). LanguageTool utilise Morfologik (FST) pour l'anglais.
**Alternative:** SpellChecker existant avec FST/HashSet + edit distance. Données extraites disponibles (EN_COMMON_WORDS, EN_IGNORE, EN_SPELLING).
