# Grammar-RS: Features Manquantes

> **État actuel:** ~70% des règles grammar.xml extraites (patterns simples + complexes)
>
> **Performance:** grammar-rs ~9ms vs LanguageTool ~1.4s (~150x plus rapide)
>
> **Principale lacune:** Unification + Suggestions dynamiques (~30% des règles restantes)
>
> **Dernière mise à jour:** 2026-01-24 - Ajout documentation Complex Pattern Rules

---

## 1. Disambiguation / POS Tagging Avancé - 🔶 Partiellement implémenté

**Description:** Résolution d'ambiguïté grammaticale pour identifier la fonction des mots.

**Exemple:** "I saw the saw" → saw₁ = verbe (VBD), saw₂ = nom (NN)

**État actuel:**
- ✅ Skip patterns extraits (24 EN + 1 FR mots, 36 EN + 3 FR regex)
- ✅ POS single-token rules extraits (24 EN + 28 FR)
- ⏸️ Règles contextuelles multi-tokens: non implémentées
- ⏸️ Modèle statistique HMM/Perceptron: non implémenté

**Stats extraction sync-lt:**
- EN: 547 règles parsées → 24 skip words + 36 regex + 24 POS rules
- FR: 461 règles parsées → 1 skip word + 3 regex + 28 POS rules

**LanguageTool complet:** ~2,000 règles disambiguation.xml + modèle HMM/Perceptron

**Sources LT:**
- `languagetool/org/languagetool/resource/en/disambiguation.xml`

**Priorité:** BASSE (règles contextuelles complexes)

---

## 2. N-gram Language Models - ✅ COMPLÉTÉ

**Description:** Modèles statistiques pour détecter erreurs de choix de mots basés sur le contexte.

**Exemple:** "I went to there house" → "their" (basé sur fréquence n-gram)

**État:** ✅ Implémenté

**Fichiers:**
- `src/language_model/mod.rs` - Module N-gram
- `src/language_model/compact_model.rs` - Format compact avec mmap
- `src/language_model/builder.rs` - Construction des modèles
- `src/checker/ngram_confusion_checker.rs` - Checker confusion N-gram

**Fonctionnalités:**
- ✅ Stupid Backoff (trigram → bigram → unigram)
- ✅ Format compact avec memory-mapping (memmap2)
- ✅ Binary search O(log n) sur arrays triés
- ✅ Support EN + FR
- ✅ Facteurs calibrés de LanguageTool (confusion_sets.txt)

**Taille données:**
- EN: ~1.5-2 GB compressé (vs 9 GB raw)
- FR: ~500 MB compressé (vs 2 GB raw)

**Usage:**
```bash
# Télécharger les données N-gram
./scripts/download_ngrams.sh en

# Extraire au format compact
cargo run --bin sync-lt -- --extract-ngrams --language en
```

**Priorité:** ~~BASSE~~ TERMINÉ

---

## 3. Pipeline Français - ✅ COMPLÉTÉ

**Description:** Le pipeline FR intègre maintenant les checkers principaux.

| Checker | Données | Pipeline EN | Pipeline FR |
|---------|---------|-------------|-------------|
| PosPatternChecker | 25 règles FR | ✅ | ✅ |
| StyleChecker | 51 règles FR | ✅ | ✅ |
| CompoundWordChecker | 1,345 règles FR | ✅ | ✅ |
| CoherencyChecker | EN only | ✅ | N/A |
| DiacriticsChecker | EN only | ✅ | N/A |
| ContractionChecker | EN only | ✅ | N/A |

**Note:** CoherencyChecker, DiacriticsChecker, ContractionChecker sont spécifiques EN.

**Priorité:** ~~HAUTE~~ TERMINÉ

---

## 4. L2 Learner Confusion Pairs - ✅ FR COMPLÉTÉ

**Description:** Paires de confusion spécifiques aux apprenants L2 selon leur langue maternelle.

**État:** FR intégré, autres langues disponibles mais non intégrées.

| Fichier | Paires | Intégré |
|---------|--------|---------|
| `en_confusion_l2_de.rs` | 75 | ❌ |
| `en_confusion_l2_es.rs` | 26 | ❌ |
| `en_confusion_l2_fr.rs` | 325 | ✅ `L2ConfusionChecker` |
| `en_confusion_l2_nl.rs` | 11 | ❌ |

**API:** `motherTongue=fr` active la détection de faux amis pour francophones.

**Priorité:** ~~MOYENNE~~ FR TERMINÉ

---

## 5. Spelling Infrastructure - ✅ COMPLÉTÉ

**Description:** Spell-checking complet avec suggestions.

**État:** Intégré aux pipelines EN et FR.

| Langue | Dictionnaire | Skip List | État |
|--------|--------------|-----------|------|
| EN | FST 370K mots | 16,590 mots (EN_IGNORE + EN_PROPER_NOUNS + EN_DISAMBIG_SKIP) | ✅ Intégré |
| FR | 34K mots (FR_SPELLING) | 1,507 mots (FR_IGNORE + FR_DISAMBIG_SKIP) | ✅ Intégré |

**Fichiers modifiés:**
- `src/checker/spell.rs` - Ajout support skip_words
- `src/bin/api/state.rs` - Intégration aux pipelines

**Note:** Le dictionnaire FR est limité (34K mots vs 370K EN) mais fonctionnel.

**Priorité:** ~~MOYENNE~~ TERMINÉ

---

## 6. Proper Nouns Skip List - ✅ COMPLÉTÉ

**Description:** Liste de noms propres à ignorer lors du spell-check.

**État:** 5,537 noms propres EN intégrés au SpellChecker (`en_proper_nouns.rs`).

**Intégration:** `SpellChecker.with_skip_words(EN_PROPER_NOUNS.iter().copied())`

**Priorité:** ~~BASSE~~ TERMINÉ

---

## 7. Multiword Expressions - 🔶 Données non intégrées

**Description:** Expressions multi-mots avec traitement spécial.

**État:** Données extraites (`en_multiwords.rs`, `fr_multiwords.rs`), non intégrées.

**Sources LT:**
- `languagetool/org/languagetool/resource/en/multiwords.txt`

**Priorité:** BASSE

---

## 8. Prohibited Words - ✅ COMPLÉTÉ

**Description:** Mots/patterns à signaler systématiquement.

**État:** `ProhibitChecker` intégré au pipeline EN (330 mots).

**Exemples:** "Christoper" → "Christopher", "GDPR-complaint" → "GDPR-compliant"

**Priorité:** ~~BASSE~~ TERMINÉ

---

## 9. Numbers POS Tagging - ✅ COMPLÉTÉ

**Description:** Mots numériques composés taggés comme CD (cardinal number) dans le POS tagger.

**État:** Intégré aux pipelines EN et FR.

| Langue | Fichier | Mots | Exemples |
|--------|---------|------|----------|
| EN | `en_numbers.rs` | 72 | twenty-one, thirty-five, ninety-nine |
| FR | `fr_numbers.rs` | 79 | vingt-et-un, quatre-vingts, soixante-dix-sept |

**Intégration:** `tagger.load_from_lines(EN_NUMBERS.iter().copied())`

**Impact:** Améliore la précision des règles POS comme `TOO_CARDINAL_NUMBER` et `NUMBER_OF_NNS`.

**Priorité:** ~~BASSE~~ TERMINÉ

---

## 10. Complex Pattern Rules - 🔶 Partiellement implémenté

**Description:** Règles grammar.xml utilisant des fonctionnalités avancées (regex, skip, unification, suggestions dynamiques).

**État actuel:**
- ✅ Patterns simples (2-6 tokens, texte littéral): 170 FR, 394 EN
- ✅ Patterns regex (`regexp="yes"`): 2,161 EN + 845 FR via DynamicPatternChecker
- ✅ Patterns postag_regexp (`postag_regexp="yes"`): supporté
- ✅ Tokens optionnels (`min="0"`): supporté
- ✅ Skip gaps (`skip="N"`): supporté (base)
- ✅ Antipatterns: supporté dans DynamicPatternChecker
- ✅ Suggestions dynamiques (`<match no="N">`): **IMPLÉMENTÉ** (599 EN + 484 FR)
  - Références `\N` aux tokens matchés
  - Transformations regex (`regexp_match`/`regexp_replace`)
  - Conversion de casse (`alllower`, `startupper`, etc.)
- 🔶 Unification (`<unify>`): **PARTIELLEMENT IMPLÉMENTÉ** (14 règles FR)
  - Parser `<unify>` et `<feature>` dans sync-lt
  - Validation genre/nombre dans DynamicPatternChecker
  - Limitation: POS tagger ne fournit pas toujours genre+nombre complets
- ⏸️ Transformations POS (`postag_replace`): non supporté (nécessite morphologie)

**Couverture actuelle:**
| Source | Règles FR | Règles EN | Couverture |
|--------|-----------|-----------|------------|
| grammar.xml total | 4,653 | ~3,500 | - |
| Patterns simples (AhoPatternRuleChecker) | 170 | 394 | ~8% |
| POS patterns (PosPatternChecker) | 25 | 94 | ~3% |
| **Complex patterns (DynamicPatternChecker)** | **845** | **2,161** | **~60%** |
| **Suggestions dynamiques** | **484** | **599** | ✅ Base implémentée |
| Confusion pairs | 101 | 1,363 | ✅ Complet |
| Style rules | 51 | 1,399 | ✅ Complet |
| Antipatterns | 216 | 1,054 | ✅ Complet |
| **Couverture règles pattern** | **~75%** | **~70%** | - |

**Fichiers:**
- `src/checker/dynamic_pattern_checker.rs` - Checker runtime
- `src/checker/data/en_complex_patterns.json` - 2,161 règles EN (~8 MB)
- `src/checker/data/fr_complex_patterns.json` - 845 règles FR (~3.5 MB)

**Fonctionnalités manquantes:**

1. **Transformations POS** (`postag_replace`):
   ```xml
   <suggestion><match no="1" postag="V.*:3s" postag_replace="V.*:2s"/></suggestion>
   <!-- Nécessite un lemmatizer/morphological generator -->
   ```

**Priorité:** MOYENNE (suggestions dynamiques implémentées, unification et transformations POS manquants)

---

## Résumé

| Catégorie | Features | Priorité | État |
|-----------|----------|----------|------|
| ✅ Complété | FR pipeline, ProhibitChecker, L2ConfusionChecker FR, SpellChecker, Proper Nouns, Disambig Skip, Numbers POS, DynamicPatternChecker, **Suggestions dynamiques** | - | Intégré |
| 🔶 Partiel | Disambiguation/POS (skip patterns OK, contexte manquant) | BASSE | Skip patterns intégrés |
| 🔶 Partiel | Complex Pattern Rules (regex/skip/suggestions/unification OK, POS transform manquant) | MOYENNE | 3,006 règles + 14 unification |
| ❌ Complexe | Disambiguation contextuelles | BASSE | Nécessite ML |
| ⏸️ Différé | Multiwords | BASSE | Nécessite POS avancé |

**Note:**
- **Disambiguation:** Skip patterns extraits et intégrés, règles contextuelles non implémentées
- **N-gram:** ✅ Implémenté avec format compact et memory-mapping
- **SpellChecker:** ✅ Intégré avec FST 370K mots EN + 34K mots FR + skip patterns disambiguation
- **Complex Pattern Rules:** 🔶 DynamicPatternChecker implémenté (2,161 EN + 845 FR) avec suggestions dynamiques. Unification et transformations POS manquants.

---

## Commande de synchronisation

```bash
cd grammar-rs
cargo run --bin sync-lt -- --languagetool-path ../languagetool
```
