# K6 Load Tests - grammar-rs vs LanguageTool

Test de charge complet comparant les performances des deux services sous différentes conditions de charge.

## Installation K6

### macOS
```bash
brew install k6
```

### Linux
```bash
sudo gpg -k
sudo gpg --no-default-keyring --keyring /usr/share/keyrings/k6-archive-keyring.gpg \
  --keyserver hkp://keyserver.ubuntu.com:80 \
  --recv-keys C5AD17C747E3415A3642D57D77C6C491D6AC1D69
echo "deb [signed-by=/usr/share/keyrings/k6-archive-keyring.gpg] https://dl.k6.io/deb stable main" | \
  sudo tee /etc/apt/sources.list.d/k6.list
sudo apt-get update
sudo apt-get install k6
```

### Windows (Chocolatey)
```bash
choco install k6
```

### Vérification
```bash
k6 version
```

---

## Lancer les tests

### Tests recommandés (nouveaux)

**Test avec cache ~30% hit rate (8 minutes):**
```bash
cd tests/k6
k6 run load-test-cached.js
```

**Test sans cache - 100% unique (8 minutes):**
```bash
k6 run load-test-nocache.js
```

### Test original (baseline, 7 textes)
```bash
k6 run load-test.js
```

### Tester un seul service (4 minutes)
```bash
# Grammar-rs uniquement (cached)
k6 run --env BASE_URL=https://grammar-rs-autocorrect.fly.dev load-test-cached.js

# LanguageTool uniquement (no cache)
k6 run --env BASE_URL=https://languagetool-autocorrect.fly.dev load-test-nocache.js
```

### Options avancées

#### Export JSON pour analyse
```bash
k6 run --out json=results.json load-test.js
```

#### Export CSV
```bash
k6 run --out csv=results.csv load-test.js
```

#### Export vers InfluxDB (pour Grafana)
```bash
k6 run --out influxdb=http://localhost:8086/k6 load-test.js
```

#### Ajuster les VUs max
```bash
# Stress test plus léger (max 20 VUs au lieu de 50)
k6 run --stage 30s:5,1m:10,1m:20,1m:20,30s:0 load-test.js
```

---

## Profil de charge

Le test utilise un ramping progressif qui teste différents niveaux de charge sur 4 minutes par service.

### Timeline

**Grammar-rs (0-4 min):**
```
VUs
50 |                    ╭────╮
40 |                   ╱      ╲
30 |                  ╱        ╲
20 |            ╭────╯          ╲
10 |      ╭────╯                 ╲
 5 | ╭───╯                        ╲
 0 ─┴──────────────────────────────╰──
   0   30s  1m30  2m30  3m30   4m
```

**LanguageTool (4-8 min):**
Même profil, décalé de 4 minutes.

### Phases

| Phase | Durée | VUs | Objectif |
|-------|-------|-----|----------|
| **Warm-up** | 0-30s | 0→5 | Vérification basique, réveil machines |
| **Charge normale** | 30s-1m30s | 5→10 | Usage typique (10 requêtes concurrentes) |
| **Charge élevée** | 1m30s-2m30s | 10→20 | Pic de trafic (20 requêtes) |
| **Stress test** | 2m30s-3m30s | 20→50 | Limites du système (50 requêtes) |
| **Ramp-down** | 3m30s-4m | 50→0 | Récupération |

### Ce que ça teste

- ✅ **Performance sous charge normale** (10 VUs)
- ✅ **Comportement sous charge élevée** (20 VUs)
- ✅ **Limites et points de rupture** (50 VUs)
- ✅ **Latence à différents niveaux de concurrence**
- ✅ **Taux d'erreur sous stress**
- ✅ **Stabilité et récupération**

---

## Tests disponibles

### `load-test-cached.js` ✨ (RECOMMANDÉ)
- **Corpus:** 1,000 textes générés dynamiquement
- **Stratégie:** 30% cache hits, 70% cache misses
- **Objectif:** Tester performance réaliste avec cache
- **Distribution:** 5% short, 75% medium, 20% long
- **Erreurs:** 70% des textes ont des erreurs intentionnelles

### `load-test-nocache.js` ✨ (RECOMMANDÉ)
- **Corpus:** 10,000 textes complètement uniques
- **Stratégie:** 0% cache hits (tous différents)
- **Objectif:** Tester vraie performance sans cache
- **Distribution:** 5% short, 75% medium, 20% long
- **Erreurs:** 70% des textes ont des erreurs intentionnelles

### `load-test.js` (BASELINE)
- **Corpus:** 7 textes fixes (3 short, 2 medium, 2 long)
- **Stratégie:** Répétitions massives (idéal pour tester cache)
- **Objectif:** Baseline, test de cache extrême

---

## Corpus de textes

### Nouveaux tests (cached/nocache)

Les textes sont **générés dynamiquement** via templates avec variations:
- **Templates:** 25+ EN + 20+ FR patterns de phrases
- **Vocabulaire:** 50+ verbes, 50+ noms, 40+ adjectifs par langue
- **Variations:** Nombres, dates, noms propres, entreprises
- **Erreurs intentionnelles:** confusion pairs (their/there), pluriels, temps verbaux

**Exemple de génération:**
```javascript
Template: "I {verb} {article} {noun} yesterday."
→ "I saw a dog yesterday."
→ "I bought the book yesterday."
→ "I found an answer yesterday."
```

### Test original (baseline)

Les textes sont répartis en 3 tiers avec distribution réaliste:

| Tier | Taille | Distribution | Exemples |
|------|--------|--------------|----------|
| **Short** | ~100 mots | 5% | Phrases simples, quick checks |
| **Medium** | ~500 mots | 75% | Emails, paragraphes (usage typique) |
| **Long** | ~1500 mots | 20% | Articles, documents |

Chaque tier contient:
- Textes propres (0 erreurs)
- Textes avec erreurs intentionnelles (8-18 erreurs)
- Variantes EN + FR

**Erreurs types:**
- Fautes d'orthographe (`sentance` → `sentence`)
- Confusion homophones (`then` vs `than`, `it's` vs `its`)
- Accord sujet-verbe (`was` vs `were`)
- Pluriels irréguliers (`centurys` → `centuries`)
- Temps verbaux (`leaded` → `led`)

---

## Métriques

### Métriques K6 standard

| Métrique | Description |
|----------|-------------|
| `http_req_duration` | Temps de réponse total (p50, p95, p99) |
| `http_req_waiting` | Temps d'attente serveur (TTFB - Time To First Byte) |
| `http_req_sending` | Temps d'envoi de la requête |
| `http_req_receiving` | Temps de réception de la réponse |
| `http_req_blocked` | Temps bloqué (DNS + connexion TCP) |
| `http_req_connecting` | Temps d'établissement de la connexion TCP |
| `http_req_failed` | Pourcentage de requêtes échouées |
| `http_reqs` | Nombre total de requêtes |
| `iterations` | Nombre d'itérations complètes |
| `vus` | Virtual users actifs |
| `vus_max` | Maximum de VUs atteint |

### Métriques custom

| Métrique | Description |
|----------|-------------|
| `errors` | Taux d'erreur des checks (Rate) |
| `match_count` | Nombre de corrections détectées (Trend) |
| `requests_by_tier` | Nombre de requêtes par tier de texte (Counter) |

### Thresholds (critères de succès)

Les thresholds définissent les critères de pass/fail du test:

```javascript
thresholds: {
  // Latence globale
  'http_req_duration': ['p(95)<1000', 'p(99)<2000'],

  // Latence par service
  'http_req_duration{service:grammar-rs}': ['p(95)<500', 'p(99)<1000'],
  'http_req_duration{service:languagetool}': ['p(95)<1500', 'p(99)<3000'],

  // Taux d'erreur
  'errors': ['rate<0.10'],          // <10% d'erreurs
  'http_req_failed': ['rate<0.10'], // <10% de requêtes échouées
  'checks': ['rate>0.85'],          // >85% de checks passés
}
```

**Interprétation:**
- ✅ **Threshold passé** = métrique respecte le critère
- ❌ **Threshold échoué** = métrique dépasse le seuil (pas bon)

---

## Interpréter les résultats

### Console output

K6 affiche en temps réel:
- Progression du test (VUs actifs)
- Métriques par seconde
- Résumé final avec toutes les métriques
- Status des thresholds (✓ ou ✗)

### Exemple de sortie finale

```
     ✓ http_req_duration..............: avg=245ms  min=89ms  med=198ms  max=1.2s  p(95)=450ms p(99)=680ms
     ✓ http_req_duration{service:grammar-rs}....: avg=180ms  p(95)=280ms  p(99)=420ms
     ✗ http_req_duration{service:languagetool}..: avg=820ms  p(95)=1.6s   p(99)=2.1s
       http_reqs......................: 12450  20.75/s
       iterations.....................: 12450  20.75/s
     ✓ errors.........................: 3.2%   (398/12450)
     ✓ match_count...................: avg=4.2 min=0 max=18
       requests_by_tier{tier:short}..: 622    (5%)
       requests_by_tier{tier:medium}.: 9338   (75%)
       requests_by_tier{tier:long}...: 2490   (20%)
```

### Analyse de la sortie

**✅ Grammar-rs:**
- p95 = 280ms → Rapide, respecte le threshold (<500ms)
- p99 = 420ms → Très bon, bien en dessous de 1s
- Conclusion: Performant même sous charge élevée

**❌ LanguageTool:**
- p95 = 1.6s → Dépasse le threshold (1.5s)
- p99 = 2.1s → Dépasse aussi (3s)
- Conclusion: Ralentit sous stress, mais acceptable

**📊 Throughput:**
- 20.75 req/s → Débit moyen sur toute la durée
- 12,450 requêtes totales en 10 min (600s)

**🎯 Error rate:**
- 3.2% d'erreurs → OK (< seuil de 10%)
- Probablement des timeouts sous stress à 50 VUs

**📝 Match count:**
- Moyenne: 4.2 corrections par texte
- Distribution: 0-18 corrections (selon texte et erreurs)

### Comparaison grammar-rs vs LanguageTool

Pour comparer directement, regarder les métriques taggées `{service:...}`:

```
grammar-rs    p95=280ms  throughput=11 req/s
LanguageTool  p95=1.6s   throughput=9 req/s

Speedup: 5.7x plus rapide (p95)
```

---

## Exemples d'utilisation

### Smoke test rapide (30s)

Pour vérifier que les services fonctionnent:

```bash
k6 run --duration 30s --vus 1 load-test.js
```

### Test de charge léger (2 min)

```bash
k6 run --stage 30s:5,1m:10,30s:0 load-test.js
```

### Comparer différentes configurations

Tester avec différents niveaux de VUs:

```bash
# Test 1: Charge légère (max 10 VUs)
k6 run --stage 1m:10,1m:10,30s:0 load-test.js --out json=light.json

# Test 2: Charge moyenne (max 20 VUs)
k6 run --stage 1m:20,1m:20,30s:0 load-test.js --out json=medium.json

# Test 3: Stress (max 50 VUs)
k6 run --stage 1m:50,1m:50,30s:0 load-test.js --out json=heavy.json

# Comparer les résultats
jq '.metrics.http_req_duration.values' *.json
```

---

## Troubleshooting

### ❌ "Connection refused"

**Problème:** Les services Fly.io ne sont pas accessibles.

**Solution:**
```bash
# Vérifier manuellement
curl https://grammar-rs-autocorrect.fly.dev/v2/languages
curl https://languagetool-autocorrect.fly.dev/v2/languages

# Vérifier le status Fly.io
fly status -a grammar-rs-autocorrect
fly status -a languagetool-autocorrect
```

### ❌ Trop de timeouts (>10%)

**Problème:** Les services sont surchargés.

**Solutions:**
- Réduire le nombre de VUs max (20 au lieu de 50)
- Augmenter le timeout dans `load-test.js` (actuellement 30s)
- Augmenter les ressources Fly.io (plus de CPUs/RAM)

**Exemple:**
```javascript
// Dans load-test.js, ligne ~72
timeout: '60s', // au lieu de 30s
```

### ❌ Machines Fly.io suspendues

**Problème:** Les machines sont en auto-suspend, première requête lente.

**Symptômes:**
- Premier VU très lent (>5s)
- Puis performance normale

**Solutions:**
- Configurer `min_machines_running = 1` dans `fly.toml`
- Envoyer une requête de warm-up avant le test:

```bash
# Warm-up avant test
curl https://grammar-rs-autocorrect.fly.dev/v2/languages
curl https://languagetool-autocorrect.fly.dev/v2/languages

# Puis lancer K6
k6 run load-test.js
```

### ❌ "Module resolution error"

**Problème:** K6 ne trouve pas `corpus.js`.

**Solution:**
```bash
# S'assurer d'être dans le bon répertoire
cd tests/k6
k6 run load-test.js

# OU utiliser un chemin absolu
k6 run /Users/stan/Dev/auto-correct/grammar-rs/tests/k6/load-test.js
```

### ❌ Résultats incohérents

**Problème:** Les métriques varient beaucoup entre runs.

**Causes possibles:**
- Auto-scaling Fly.io (machines suspendues/réveillées)
- Traffic réseau variable
- Cold start de l'API

**Solutions:**
- Lancer plusieurs runs et faire la moyenne
- Augmenter la durée du test (plus stable)
- Fixer `min_machines_running = 1` pour éviter cold starts

---

## Métriques attendues

### Avec cache (load-test-cached.js)

**Grammar-rs:**
- **p50:** ~20-30ms (mix cache hits + misses)
- **p95:** ~100-150ms
- **p99:** ~300-500ms
- **Cache hit:** <5ms
- **Cache miss:** ~80-100ms
- **Throughput:** 15-20 req/s @ 10 VUs

**LanguageTool:**
- **p50:** ~30-40ms (avec cache)
- **p95:** ~150-300ms
- **p99:** ~500-800ms

### Sans cache (load-test-nocache.js)

**Grammar-rs:**
- **p50:** ~80-100ms (pure compute)
- **p95:** ~200-300ms
- **p99:** ~500-700ms
- **Throughput:** 8-10 req/s @ 10 VUs
- **Error rate:** <2%

**LanguageTool:**
- **p50:** ~150-200ms
- **p95:** ~400-600ms
- **p99:** ~800-1200ms
- **Throughput:** 5-7 req/s @ 10 VUs
- **Error rate:** <3%

### Ratio attendu
- **Avec cache:** grammar-rs ~1.5-2x plus rapide que LanguageTool
- **Sans cache:** grammar-rs ~2-3x plus rapide
- **Cache impact:** ~10x speedup pour cache hits

**Note:** Sous stress (50 VUs), les performances peuvent dégrader. C'est normal et attendu.

---

## Next Steps

### Export et analyse

1. **Exporter en JSON:**
```bash
k6 run --out json=results.json load-test.js
```

2. **Analyser avec jq:**
```bash
# P95 latency par service
jq '.metrics["http_req_duration{service:grammar-rs}"].values.p95' results.json
jq '.metrics["http_req_duration{service:languagetool}"].values.p95' results.json

# Throughput
jq '.metrics.http_reqs.values.rate' results.json
```

3. **Visualiser avec Grafana:**
```bash
# Setup InfluxDB + Grafana
docker run -d -p 8086:8086 influxdb:1.8
docker run -d -p 3000:3000 grafana/grafana

# Run avec export
k6 run --out influxdb=http://localhost:8086/k6 load-test.js

# Ouvrir Grafana: http://localhost:3000
# Ajouter datasource InfluxDB
# Importer dashboard K6
```

### CI/CD Integration

```yaml
# .github/workflows/load-test.yml
name: Load Test
on:
  schedule:
    - cron: '0 2 * * *' # Daily at 2am
  workflow_dispatch:

jobs:
  k6-load-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup K6
        run: |
          sudo gpg --no-default-keyring --keyring /usr/share/keyrings/k6-archive-keyring.gpg \
            --keyserver hkp://keyserver.ubuntu.com:80 \
            --recv-keys C5AD17C747E3415A3642D57D77C6C491D6AC1D69
          echo "deb [signed-by=/usr/share/keyrings/k6-archive-keyring.gpg] https://dl.k6.io/deb stable main" | \
            sudo tee /etc/apt/sources.list.d/k6.list
          sudo apt-get update
          sudo apt-get install k6

      - name: Run load test
        run: |
          cd tests/k6
          k6 run --out json=results.json load-test.js

      - name: Upload results
        uses: actions/upload-artifact@v3
        with:
          name: k6-results
          path: tests/k6/results.json
```

---

## Resources

- **K6 Documentation:** https://k6.io/docs/
- **K6 Examples:** https://k6.io/docs/examples/
- **Grafana K6 Dashboard:** https://grafana.com/grafana/dashboards/2587
- **InfluxDB + K6:** https://k6.io/docs/results-output/real-time/influxdb/

---

## License

MIT
