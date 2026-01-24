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
cargo test              # Tous les tests
cargo test --test api   # Tests E2E (pipelines complets)
cargo test --lib        # Tests unitaires uniquement
```

## Tester les features
**IMPORTANT:** Pour tester une nouvelle feature, ajouter des tests dans `tests/api.rs` (E2E) plutôt que des scripts ad-hoc ou des appels curl/python. Les tests E2E vérifient le comportement réel des pipelines.

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

## ✅ COMPLETED - L2 Confusion Checker (FR)
**Description:** Faux amis pour francophones apprenant l'anglais.
**Stats:** 325 paires de faux amis
**Fichiers:** `l2_confusion_checker.rs`, `en_confusion_l2_fr.rs`
**Intégration:** `L2ConfusionChecker` + API paramètre `motherTongue=fr`
**Exemples:**
- "lecture" → "reading" (FR "lecture" = lecture)
- "fabric" → "factory" (FR "fabrique" = usine)
- "pretend" → "claim" (FR "prétendre" = prétendre)

## ✅ COMPLETED - Spell Checker
**Description:** Vérification orthographique avec dictionnaire FST et suggestions Levenshtein.
**Stats:**
- EN: 370,105 mots (FST) + 16,590 skip words (EN_IGNORE + EN_PROPER_NOUNS + EN_DISAMBIG_SKIP)
- FR: 34,099 mots (HashSet) + 1,507 skip words (FR_IGNORE + FR_DISAMBIG_SKIP)
**Fichiers:** `spell.rs`, `en_US.fst`, `en_ignore.rs`, `en_proper_nouns.rs`, `fr_spelling.rs`, `fr_ignore.rs`
**Intégration:** `SpellChecker` dans les pipelines API EN et FR
**Features:**
- Dictionnaire FST pour EN (1.7MB, 370K mots)
- Skip lists pour éviter faux positifs (acronymes, noms propres, patterns disambiguation)
- Suggestions via distance de Levenshtein

## ✅ COMPLETED - Disambiguation Skip Patterns
**Description:** Patterns extraits de disambiguation.xml pour ignorer certains mots du spell checker.
**Stats:**
- EN: 24 mots + 36 regex patterns
- FR: 1 mot + 3 regex patterns
**Fichiers:** `en_disambig_skip.rs`, `fr_disambig_skip.rs`, `en_disambig_pos.rs`, `fr_disambig_pos.rs`
**Intégration:** Skip words ajoutés au SpellChecker via `EN_DISAMBIG_SKIP` / `FR_DISAMBIG_SKIP`
**Extraction:** Programmatique via `sync-lt.rs` (Phase 6)
**Features:**
- Patterns ignore_spelling de disambiguation.xml
- Évite les faux positifs sur contractions partielles, emprunts linguistiques
- Règles POS single-token extraites (24 EN + 28 FR) pour future amélioration du POS tagger

## ✅ COMPLETED - N-gram Language Model
**Description:** Modèle de langue N-gram pour calcul de probabilités contextuelles.
**Fichiers:** `src/language_model/mod.rs`, `ngram_model.rs`, `probability.rs`
**Algorithm:** Stupid Backoff (comme LanguageTool)
- Trigram → Bigram (×0.4) → Unigram (×0.16)
- Probabilité contextuelle P(mot|contexte)
**Features:**
- `NgramLanguageModel`: chargement depuis fichier binaire (bincode)
- `Probability`: résultat avec coverage et occurrence count
- `compare_words()`: ratio de probabilités pour comparaison
**Utilisation:** Base pour détection de confusion via N-gram (voir NgramConfusionChecker)

## ✅ COMPLETED - N-gram Confusion Checker
**Description:** Détection de mots confondus basée sur probabilités N-gram.
**Stats:** Support pour 1,363 paires EN (basic) + 3,571 paires EN (extended) + 101 FR
**Fichiers:** `ngram_confusion_checker.rs`
**Intégration:** `NgramConfusionChecker` (optionnel, requiert données N-gram)
**Features:**
- Utilise facteurs calibrés de LanguageTool (confusion_sets.txt)
- Compare P(mot_actuel|contexte) vs P(alternative|contexte)
- Support basic + extended confusion pairs
- Requiert données N-gram préalablement extraites
**Note:** Nécessite téléchargement et extraction des données N-gram (~9GB EN, ~2GB FR)

## ✅ COMPLETED - Lucene Index Reader (Minimal)
**Description:** Lecteur d'index Lucene 4.x en Rust pur pour extraction N-gram.
**Fichiers:** `src/lucene/mod.rs`, `vint.rs`, `codec.rs`, `compound.rs`, `stored.rs`, `reader.rs`
**Features:**
- VInt decoder (base-128 variable integers)
- Compound file parser (.cfs/.cfe)
- Term dictionary extraction
- Compatible format Lucene 4.1 (format N-gram LanguageTool)
**Utilisation:** Extraction programmatique via `sync-lt.rs --extract-ngrams`

## ✅ COMPLETED - N-gram Confusion Words List
**Description:** Liste des mots à surveiller pour extraction N-gram sélective.
**Stats:** 6,335 mots uniques extraits des confusion pairs
**Fichiers:** `en_ngram_words.rs`
**Extraction:** Automatique via `sync-lt.rs` depuis confusion_sets.txt + confusion_sets_extended.txt
**Utilisation:** Filtre pour extraire uniquement les N-grams pertinents (~150MB au lieu de 9GB)

## ✅ COMPLETED - Compact N-gram Storage Format
**Description:** Format de stockage compressé avec memory-mapping pour N-grams.
**Fichiers:** `src/language_model/compact_model.rs`, `src/language_model/builder.rs`
**Features:**
- Format binaire compact avec header + sections triées
- Memory-mapped file (memmap2) pour chargement instantané
- Binary search O(log n) sur arrays triés
- ~0 GB RAM au runtime (tout en mmap)
- Support EN + FR
**Taille estimée:** ~1.5-2 GB EN, ~500 MB FR (vs 9 GB + 2 GB raw)

## ✅ COMPLETED - N-gram Extraction Tool
**Description:** Outil d'extraction de N-grams depuis données LanguageTool.
**Fichiers:** `src/bin/sync_lt.rs` (flag `--extract-ngrams`)
**Usage:**
```bash
# Télécharger les données N-gram
./scripts/download_ngrams.sh en

# Extraire au format compact
cargo run --bin sync-lt -- --extract-ngrams --language en
cargo run --bin sync-lt -- --extract-ngrams --language fr
```
**Options:**
- `--ngram-path <path>`: Chemin vers données N-gram extraites
- `--output <path>`: Répertoire de sortie
- `--language <lang>`: en ou fr

## ✅ COMPLETED - Numbers POS Tagging (EN + FR)
**Description:** Mots numériques composés taggés comme CD (cardinal number) dans le POS tagger.
**Stats:**
- EN: 72 mots (twenty-one, thirty-five, ninety-nine, etc.)
- FR: 79 mots (vingt-et-un, quatre-vingts, soixante-dix-sept, etc.)
**Fichiers:** `en_numbers.rs`, `fr_numbers.rs`
**Intégration:** Chargés dans les POS taggers via `tagger.load_from_lines()`
**Impact:** Améliore la précision des règles POS comme `TOO_CARDINAL_NUMBER` et `NUMBER_OF_NNS`.

## ✅ COMPLETED - Dynamic Pattern Checker (Complex Rules)
**Description:** Checker pour règles complexes qui ne peuvent pas être compilées en code Rust statique.
Supporte les fonctionnalités avancées de grammar.xml:
- `regexp="yes"` - matching regex sur le texte des tokens
- `postag_regexp="yes"` - matching regex sur les POS tags
- `min/max` - tokens optionnels (min=0) ou répétés
- `skip` - gap matching (N tokens entre deux éléments)
- `antipatterns` - exceptions aux règles
- `<match no="N">` - suggestions dynamiques avec références aux tokens matchés
- `<unify>` - validation genre/nombre (accord grammatical français)

**Stats:**
- EN: 2,345 règles complexes (grammar.xml + style.xml)
- FR: 1,852 règles complexes (grammar.xml + style.xml, 14 avec unification)
- Total: 4,197 règles supplémentaires (en plus des patterns simples)

**Fichiers:**
- `src/checker/dynamic_pattern_checker.rs` - Checker avec compilation regex au chargement
- `src/checker/data/en_complex_patterns.json` - Règles EN sérialisées (~8 MB)
- `src/checker/data/fr_complex_patterns.json` - Règles FR sérialisées (~3.5 MB)

**Intégration:** `DynamicPatternChecker` dans les pipelines API EN et FR

**Features:**
- Chargement paresseux via `OnceLock` (compilation regex une seule fois)
- Matching avec backtracking pour tokens optionnels
- Support complet des antipatterns pour éviter faux positifs
- **Suggestions dynamiques:** Références `\N` converties en tokens matchés
- **Transformations regex:** `regexp_match`/`regexp_replace` sur texte des tokens
- **Conversion de casse:** `alllower`, `allupper`, `startlower`, `startupper`
- **Unification (FR):** Validation accord genre/nombre via `<unify>` et `<feature>`
- Génération automatique via `sync-lt.rs` (Phase 8)

**Limitations connues:**
- Unification: Dépend du POS tagger pour fournir genre/nombre complets

**Exemples de règles supportées:**
- `CONFUSION_EST_ET` (FR) - Confusion est/et avec contexte POS
- `TU_VERBE` (FR) - Conjugaison 2ème personne (tu pense → tu penses)
- Règles avec regex sur tokens (mots multiples via `|`)
- Suggestions calculées dynamiquement à partir des tokens matchés

**Impact:** Augmente la couverture des règles grammar.xml de ~35% à ~70%

---

# Features Différées

## ⏸️ DEFERRED - Hunspell / Morphologie Avancée
**Description:** Spell-checking morphologique avec lemmatisation et affixes.
**Raison:** LanguageTool utilise Morfologik (binaire) non disponible. Notre SpellChecker utilise FST + Levenshtein.
**Alternative actuelle:** SpellChecker avec dictionnaire FST 370K mots (EN) fonctionne bien pour la plupart des cas.
