# Grammar-RS: Écarts avec LanguageTool

> **Important:** Toujours synchroniser les nouvelles règles depuis LanguageTool avec `cargo run --bin sync-lt`

## État actuel: ~25-30% de parité fonctionnelle

### Résumé des performances
- **grammar-rs:** ~9ms par requête
- **LanguageTool:** ~1.4s par requête
- **Ratio:** ~150x plus rapide

---

## 1. Antipatterns (Extraction: ✅ 100% | Intégration: 0%)

### Description
Les antipatterns sont des exceptions aux règles - des patterns qui ressemblent à des erreurs mais sont corrects.

### État actuel
- **Extraits:** 1,269 antipatterns (1,053 EN + 216 FR)
- **Fichiers générés:**
  - `src/checker/data/en_antipatterns.rs`
  - `src/checker/data/fr_antipatterns.rs`
- **Prochaine étape:** Intégrer dans `AhoPatternRuleChecker` pour filtrer les faux positifs

### Exemple
```xml
<rule id="A_AN">
  <pattern>
    <token>a</token>
    <token regexp="yes">[aeiou].*</token>
  </pattern>
  <!-- Antipattern: "a one-time" est correct car "one" se prononce /wʌn/ -->
  <antipattern>
    <token>a</token>
    <token>one</token>
  </antipattern>
</rule>
```

### Impact
Sans antipatterns, grammar-rs génère des faux positifs sur:
- "a one-time event" → signale à tort "an one-time"
- "a union" → signale à tort "an union"
- "a European" → signale à tort "an European"

### Fichiers sources LanguageTool
- `languagetool/org/languagetool/rules/en/grammar.xml`
- Chercher: `<antipattern>...</antipattern>`

### Implémentation requise
1. Étendre `sync_lt.rs` avec `parse_antipatterns()`
2. Générer `data/antipatterns.rs`
3. Modifier `AhoPatternRuleChecker` pour filtrer avec antipatterns

---

## 2. Règles conditionnelles (0% implémenté)

### Description
Règles avec logique complexe: filtres POS, exceptions, tokens avec attributs. LanguageTool en a ~800.

### Exemple
```xml
<rule>
  <pattern>
    <token postag="VB">have</token>  <!-- Seulement si verbe -->
    <token>went</token>
  </pattern>
  <filter class="PostagFilter">
    <args postag="VBD"/>  <!-- Seulement si passé -->
  </filter>
  <message>Use "had gone" instead</message>
</rule>
```

### Impact
Sans règles conditionnelles:
- Détection incorrecte des temps verbaux
- Faux positifs sur constructions grammaticales valides
- Manque de précision sur l'accord sujet-verbe

### Fichiers sources LanguageTool
- `languagetool/org/languagetool/rules/en/grammar.xml`
- Chercher: `<filter>`, `<exception>`, `postag=`

### Implémentation requise
1. Parser les attributs `postag`, `chunk`, `inflected`
2. Créer `ConditionalRuleChecker`
3. Intégrer avec le POS tagger

---

## 3. Hunspell / Morphologie (0% implémenté)

### Description
Spell-checking morphologique avec lemmatisation et suggestions intelligentes.

### Capacités manquantes
- Détection fautes d'orthographe (typos)
- Suggestions basées sur la distance d'édition
- Lemmatisation (running → run)
- Formes fléchies (run, runs, ran, running)

### Fichiers dictionnaires LanguageTool
```
languagetool/org/languagetool/resource/en/
├── hunspell/
│   ├── en_US.dic    # ~60,000 mots
│   ├── en_US.aff    # Règles affixes
│   ├── en_GB.dic
│   └── en_GB.aff
└── spelling.txt     # Exceptions orthographe
```

### Implémentation requise
1. Ajouter dépendance `hunspell-rs = "0.4"`
2. Créer `HunspellChecker`
3. Synchroniser les dictionnaires .dic/.aff

---

## 4. Confusion pairs étendus (22% implémenté)

### État actuel
- **Extraits:** ~330 paires (L2 learners: DE, ES, FR, NL)
- **Manquants:** ~1,170 paires (locuteurs natifs)

### Exemples manquants (natifs)
```
accept/except
advice/advise
affect/effect
allusion/illusion
brake/break
complement/compliment
principal/principle
stationary/stationery
```

### Fichiers sources LanguageTool
- `languagetool/org/languagetool/resource/en/confusion_sets.txt`

### Implémentation requise
1. Étendre `parse_confusion_sets()` pour format complet
2. Générer `data/confusion_native.rs`

---

## 5. Disambiguation / POS Tagging (2.5% implémenté)

### Description
Résolution d'ambiguïté grammaticale pour identifier la fonction des mots.

### Exemple
"I saw the saw" →
- saw₁ = verbe (VBD: past tense of "see")
- saw₂ = nom (NN: tool)

### État actuel
- ~50 règles basiques
- Pas de modèle statistique

### LanguageTool
- ~2,000 règles disambiguation.xml
- Modèle HMM/Perceptron

### Fichiers sources
- `languagetool/org/languagetool/resource/en/disambiguation.xml`

---

## 6. N-gram Language Models (0% implémenté)

### Description
Modèles statistiques pour détecter les erreurs de choix de mots basés sur le contexte.

### Exemple
"I went to there house" → "their" (basé sur fréquence n-gram)

### Problème
Les modèles n-gram pèsent ~1GB par langue. Pas prioritaire pour une solution légère.

### Alternative
Utiliser les confusion pairs avec scoring de fréquence plus léger.

---

## 7. Style rules (15% implémenté)

### État actuel
- ~30 règles de style
- Wordiness, passive voice, clichés basiques

### Manquants
- Détection de jargon
- Readability scoring
- Sentence variety analysis
- Gender-neutral language suggestions

### Fichiers sources
- `languagetool/org/languagetool/rules/en/style.xml`

---

## Roadmap prioritaire

| Priorité | Phase | Fonctionnalité | Parité estimée |
|----------|-------|----------------|----------------|
| HAUTE | 5 | Antipatterns | 25% → 35% |
| HAUTE | 6 | Règles conditionnelles | 35% → 50% |
| MOYENNE | 7 | Hunspell | 50% → 65% |
| MOYENNE | 8 | Confusion pairs natifs | 65% → 75% |
| BASSE | 9 | Disambiguation | 75% → 80% |
| BASSE | 10 | Style étendu | 80% → 85% |

---

## Commande de synchronisation

```bash
# Toujours utiliser sync-lt pour extraire les nouvelles ressources
cd grammar-rs
cargo run --bin sync-lt -- --languagetool-path ../languagetool

# Après modification de sync_lt.rs, régénérer tout:
cargo run --bin sync-lt -- --languagetool-path ../languagetool --force
```

---

## Fichiers clés grammar-rs

| Fichier | Description |
|---------|-------------|
| `src/bin/sync_lt.rs` | Extracteur de ressources LanguageTool |
| `src/data/` | Données générées par sync-lt |
| `src/checker/` | Implémentations des checkers |
| `src/bin/api/` | API HTTP compatible LanguageTool |

---

## Tests de validation

```bash
# Comparer avec LanguageTool de référence
curl -s -X POST http://localhost:8081/v2/check \
  -d "text=I have a apple&language=en" | jq .

curl -s -X POST https://languagetool-autocorrect.fly.dev/v2/check \
  -d "text=I have a apple&language=en" | jq .
```
