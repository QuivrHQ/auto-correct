# Grammar-RS: Features Manquantes

> **État actuel:** ~70-80% de parité fonctionnelle avec LanguageTool
>
> **Performance:** grammar-rs ~9ms vs LanguageTool ~1.4s (~150x plus rapide)

---

## 1. Disambiguation / POS Tagging Avancé - ❌ Non implémenté

**Description:** Résolution d'ambiguïté grammaticale pour identifier la fonction des mots.

**Exemple:** "I saw the saw" → saw₁ = verbe (VBD), saw₂ = nom (NN)

**État:** ~2.5% - POS tagger basique (441 mots + heuristiques suffixes), pas de modèle statistique.

**LanguageTool:** ~2,000 règles disambiguation.xml + modèle HMM/Perceptron

**Sources LT:**
- `languagetool/org/languagetool/resource/en/disambiguation.xml`

**Priorité:** BASSE

---

## 2. N-gram Language Models - ❌ Non implémenté

**Description:** Modèles statistiques pour détecter erreurs de choix de mots basés sur le contexte.

**Exemple:** "I went to there house" → "their" (basé sur fréquence n-gram)

**État:** 0%

**Problème:** Modèles ~1GB par langue. Pas prioritaire pour solution légère.

**Alternative:** Confusion pairs avec scoring de fréquence (partiellement implémenté).

**Sources LT:**
- `languagetool/org/languagetool/resource/en/ngram-index/`

**Priorité:** BASSE

---

## 3. Pipeline Français Incomplet - 🔶 Partiel

**Description:** Le pipeline FR n'intègre pas tous les checkers disponibles.

**État:** Données générées mais non intégrées dans le pipeline API FR.

| Checker | Données | Pipeline EN | Pipeline FR |
|---------|---------|-------------|-------------|
| PosPatternChecker | 25 règles FR | ✅ | ❌ |
| StyleChecker | 51 règles FR | ✅ | ❌ |
| CompoundWordChecker | 1,346 règles FR | ✅ | ❌ |
| CoherencyChecker | - | ✅ | ❌ |
| DiacriticsChecker | - | ✅ | ❌ |
| ContractionChecker | - | ✅ | ❌ |

**Action requise:** Ajouter les checkers au pipeline FR dans `src/bin/api/main.rs`

**Priorité:** HAUTE

---

## 4. L2 Learner Confusion Pairs - 🔶 Données non intégrées

**Description:** Paires de confusion spécifiques aux apprenants L2 selon leur langue maternelle.

**État:** Données extraites, non intégrées dans le pipeline.

| Fichier | Paires | Intégré |
|---------|--------|---------|
| `en_confusion_l2_de.rs` | 75 | ❌ |
| `en_confusion_l2_es.rs` | ? | ❌ |
| `en_confusion_l2_fr.rs` | ? | ❌ |
| `en_confusion_l2_nl.rs` | ? | ❌ |

**Sources LT:**
- `languagetool/org/languagetool/resource/en/confusion_sets_l2_*.txt`

**Priorité:** MOYENNE

---

## 5. Spelling Suggestions - 🔶 Données non intégrées

**Description:** Suggestions de corrections orthographiques.

**État:** Données disponibles, non intégrées.

| Fichier | Entrées | Usage |
|---------|---------|-------|
| `en_spelling.rs` | 468 | ❌ Non intégré |
| `fr_spelling.rs` | 34,099 | ❌ Non intégré |
| `en_ignore.rs` | 11,029 | ❌ Skip list |
| `fr_ignore.rs` | 1,506 | ❌ Skip list |

**Action requise:** Créer SpellingSuggestionChecker utilisant ces données.

**Priorité:** MOYENNE

---

## 6. Proper Nouns Skip List - 🔶 Données non intégrées

**Description:** Liste de noms propres à ignorer lors du spell-check.

**État:** 5,537 noms propres EN extraits (`en_proper_nouns.rs`), non utilisés.

**Action requise:** Intégrer dans SpellChecker pour éviter faux positifs.

**Priorité:** BASSE

---

## 7. Multiword Expressions - 🔶 Données non intégrées

**Description:** Expressions multi-mots avec traitement spécial.

**État:** Données extraites (`en_multiwords.rs`, `fr_multiwords.rs`), non intégrées.

**Sources LT:**
- `languagetool/org/languagetool/resource/en/multiwords.txt`

**Priorité:** BASSE

---

## 8. Prohibited Words - 🔶 Données non intégrées

**Description:** Mots/patterns à signaler systématiquement.

**État:** Données extraites (`en_prohibit.rs`), non intégrées.

**Sources LT:**
- `languagetool/org/languagetool/resource/en/prohibit.txt`

**Priorité:** BASSE

---

## 9. Numbers Rules - 🔶 Données non intégrées

**Description:** Règles spécifiques aux nombres (format, cohérence).

**État:** Données extraites (`en_numbers.rs`), non intégrées.

**Priorité:** BASSE

---

## Résumé

| Catégorie | Features | Priorité |
|-----------|----------|----------|
| Non implémenté | Disambiguation, N-gram | BASSE |
| Partiel (pipeline) | FR checkers | HAUTE |
| Données non intégrées | L2 confusion, Spelling, Proper nouns, Multiwords, Prohibit, Numbers | MOYENNE/BASSE |

---

## Commande de synchronisation

```bash
cd grammar-rs
cargo run --bin sync-lt -- --languagetool-path ../languagetool
```
