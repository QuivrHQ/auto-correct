# Load Tests - LanguageTool API

Tests de charge pour l'API LanguageTool déployée sur Fly.io.

## Prérequis

```bash
# Installer k6
brew install k6
```

## Utilisation

```bash
cd docker/load-tests

# Vérifier la connectivité
./run.sh check

# Lancer un test spécifique
./run.sh baseline   # 10 users, 5 min
./run.sh target     # 100 users, 10 min
./run.sh stress     # Jusqu'à 500 users

# Lancer tous les tests
./run.sh all
```

## Scénarios

| Scénario     | Users  | Durée  | Objectif                       |
| ------------ | ------ | ------ | ------------------------------ |
| **baseline** | 10     | 5 min  | Métriques de référence         |
| **target**   | 100    | 10 min | Valider l'objectif (100 users) |
| **stress**   | 50→500 | 6 min  | Trouver le point de rupture    |

## Seuils de succès

- **Latence p95** : < 2s (baseline/target), < 5s (stress)
- **Taux d'erreur** : < 1% (baseline/target), < 10% (stress)
- **Throughput** : > 10 req/s stable

## Stratégie de génération de texte

Pour éviter que le cache serveur fausse les résultats :

- **70% textes uniques** : suffixe timestamp + random pour forcer cache miss
- **30% textes répétés** : simule le comportement réel (cache client)
- **60% avec typos** : fautes réalistes (accents, doublons, inversions)
- **70% français, 30% anglais** : distribution linguistique réaliste

## Configuration

```bash
# Utiliser une autre URL
API_URL=http://localhost:8010 ./run.sh baseline

# URL par défaut
https://languagetool-autocorrect.fly.dev
```

## Résultats

Les résultats sont sauvegardés dans `./results/` :

- `baseline_YYYYMMDD_HHMMSS.json`
- `target_YYYYMMDD_HHMMSS.json`
- `stress_YYYYMMDD_HHMMSS.json`

### Interpréter les résultats

```json
{
  "scenario": "target",
  "metrics": {
    "requests": 12500,
    "rps": 20.8,
    "latency": {
      "avg": 450,
      "p50": 380,
      "p95": 890,
      "p99": 1250
    },
    "errors": 0.002
  }
}
```

## Monitoring pendant les tests

Ouvrir le dashboard Fly.io pour surveiller :

- CPU / RAM usage
- Connexions actives
- Latence réseau

```bash
fly dashboard -a languagetool-autocorrect
```

## Recommandations post-test

### Si p95 > 2s avec 100 users :

1. **Scaling vertical** : Augmenter CPU/RAM sur Fly.io
2. **Scaling horizontal** : Ajouter des machines
3. **Cache serveur** : Augmenter TTL ou taille du cache

### Si erreurs > 5% :

1. Vérifier les logs : `fly logs -a languagetool-autocorrect`
2. Vérifier la mémoire Java (heap overflow)
3. Considérer migration vers Porter/K8s pour auto-scaling

## Coûts estimés (Fly.io)

| Config   | CPU | RAM  | Prix/mois |
| -------- | --- | ---- | --------- |
| Actuelle | 4   | 8GB  | ~$50      |
| Scaled   | 4x2 | 4GB  | ~$60      |
| Premium  | 8   | 16GB | ~$100     |
