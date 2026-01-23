# Grammar-RS: Features Manquantes

> **État actuel:** ~92% de parité fonctionnelle avec LanguageTool
>
> **Performance:** grammar-rs ~9ms vs LanguageTool ~1.4s (~150x plus rapide)
>
> **Dernière mise à jour:** SpellChecker intégré (370K mots EN, 34K mots FR)

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
| EN | FST 370K mots | 16,566 mots (EN_IGNORE + EN_PROPER_NOUNS) | ✅ Intégré |
| FR | 34K mots (FR_SPELLING) | 1,506 mots (FR_IGNORE) | ✅ Intégré |

**Fichiers modifiés:**
- `src/checker/spell.rs` - Ajout support skip_words
- `src/bin/api/state.rs` - Intégration aux pipelines

**Note:** Le dictionnaire FR est limité (34K mots vs 370K EN) mais fonctionnel.

**Priorité:** ~~MOYENNE~~ TERMINÉ

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

## 8. Prohibited Words - ✅ COMPLÉTÉ

**Description:** Mots/patterns à signaler systématiquement.

**État:** `ProhibitChecker` intégré au pipeline EN (330 mots).

**Exemples:** "Christoper" → "Christopher", "GDPR-complaint" → "GDPR-compliant"

**Priorité:** ~~BASSE~~ TERMINÉ

---

## 9. Numbers Rules - 🔶 Données non intégrées

**Description:** Règles spécifiques aux nombres (format, cohérence).

**État:** Données extraites (`en_numbers.rs`), non intégrées.

**Priorité:** BASSE

---

## Résumé

| Catégorie | Features | Priorité | État |
|-----------|----------|----------|------|
| ✅ Complété | FR pipeline, ProhibitChecker, L2ConfusionChecker FR, SpellChecker | - | Intégré |
| ❌ Complexe | Disambiguation/POS avancé, N-gram models | BASSE | Nécessite ML/données volumineuses |
| ⏸️ Différé | Multiwords, Numbers | BASSE | Nécessite POS avancé |

**Note:**
- **Disambiguation:** Nécessite ~2,000 règles + modèle HMM/Perceptron (~10-50MB)
- **N-gram:** Nécessite modèles statistiques (~1GB par langue)
- **SpellChecker:** ✅ Intégré avec FST 370K mots EN + 34K mots FR

---

## Commande de synchronisation

```bash
cd grammar-rs
cargo run --bin sync-lt -- --languagetool-path ../languagetool
```
