# Déploiement grammar-rs sur Fly.io

## Configuration

- **App**: `grammar-rs-autocorrect`
- **Région**: `cdg` (Paris)
- **Resources**: 1 CPU, 1GB RAM
- **Volume**: 30GB pour N-grams (EN: 23GB, FR: 2GB)
- **Auto-download**: Activé via `GRAMMAR_RS_AUTO_DOWNLOAD=1`

## Premier déploiement

Le volume sera créé automatiquement par Fly.io lors du premier déploiement.

```bash
cd /Users/stan/Dev/auto-correct/grammar-rs
fly deploy --config fly.toml
```

## Premier lancement

**⚠️ Important**: Au premier lancement, l'application va télécharger automatiquement les N-grams depuis R2 :
- EN: ~23GB (~5-10 minutes selon la connexion)
- FR: ~2GB (~1-2 minutes)

Le grace period est configuré à 300s (5 minutes) pour permettre le téléchargement EN.

Vous pouvez suivre la progression dans les logs :
```bash
fly logs -a grammar-rs-autocorrect
```

## Lancements suivants

Les N-grams sont persistés dans le volume `grammar_ngrams`, donc les lancements suivants seront instantanés.

## Endpoints

Une fois déployé :
- **Health**: `https://grammar-rs-autocorrect.fly.dev/`
- **Languages**: `https://grammar-rs-autocorrect.fly.dev/v2/languages`
- **Check**: `POST https://grammar-rs-autocorrect.fly.dev/v2/check`

## Test

```bash
# Vérifier les langues disponibles
curl https://grammar-rs-autocorrect.fly.dev/v2/languages

# Tester la vérification grammaticale
curl -X POST https://grammar-rs-autocorrect.fly.dev/v2/check \
  -d "text=I have a apple&language=en"
```

## Monitoring

```bash
# Status de l'app
fly status -a grammar-rs-autocorrect

# Logs en temps réel
fly logs -a grammar-rs-autocorrect --tail

# Volume info
fly volumes list -a grammar-rs-autocorrect

# SSH dans la machine
fly ssh console -a grammar-rs-autocorrect
```

## Coûts estimés

- Machine (1 CPU, 1GB RAM): ~$7/mois (avec auto-suspend)
- Volume (30GB): ~$3/mois
- **Total**: ~$10/mois

## Comparaison vs LanguageTool

Le script de comparaison est disponible pour benchmarker :

```bash
node /Users/stan/Dev/auto-correct/scripts/compare-performance.js
```
