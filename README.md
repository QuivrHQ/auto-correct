# AutoCorrect - Extension Chrome

Extension Chrome pour la correction grammaticale et orthographique en temps réel, utilisant LanguageTool.

## Fonctionnalités

- Détection automatique des erreurs d'orthographe et de grammaire
- Soulignements visuels des erreurs (rouge pour orthographe, orange pour grammaire)
- Suggestions de correction en un clic
- Fonctionne sur tous les champs de texte (input, textarea, contenteditable)
- Compatible avec les éditeurs riches comme CKEditor (Zendesk, etc.)

## Installation depuis le ZIP

### Télécharger et installer

1. **Téléchargez** le fichier `autocorrect-extension.zip` depuis les releases
2. **Décompressez** le fichier ZIP dans un dossier de votre choix
3. **Ouvrez Chrome** et allez à `chrome://extensions`
4. **Activez le mode développeur** (toggle en haut à droite)
5. **Cliquez sur "Charger l'extension non empaquetée"**
6. **Sélectionnez** le dossier `dist` décompressé

### Configuration

1. Cliquez sur l'icône de l'extension dans la barre d'outils
2. L'extension est activée par défaut avec le serveur LanguageTool public
3. Vous pouvez changer la langue (FR/EN/Auto) dans le popup

## Développement

### Prérequis

- Node.js 18+
- npm ou yarn

### Installation des dépendances

```bash
npm install
```

### Développement

```bash
npm run dev
```

### Build

```bash
npm run build
```

Le build génère les fichiers dans le dossier `dist/`.

### Tests

```bash
# Lancer les tests
npm test

# Lancer les tests avec UI
npm run test:ui

# Lancer les tests en mode visible
npm run test:headed
```

## Structure du projet

```
auto-correct/
├── src/
│   ├── content/           # Content scripts (injection dans les pages)
│   │   ├── index.ts       # Point d'entrée
│   │   ├── text-field-manager.ts  # Gestion des champs de texte
│   │   ├── underline-renderer.ts  # Rendu des soulignements
│   │   └── language-tool-client.ts # Client API LanguageTool
│   ├── popup/             # Interface popup
│   │   ├── Popup.tsx      # Composant React principal
│   │   └── index.html     # Page popup
│   ├── background/        # Service worker
│   └── shared/            # Types et utilitaires partagés
├── public/
│   └── manifest.json      # Manifest Chrome Extension v3
├── tests/
│   └── e2e/              # Tests Playwright
├── dist/                  # Build output
└── test/                  # Page de test locale
```

## API LanguageTool

L'extension utilise par défaut le serveur LanguageTool hébergé sur Fly.io:
- URL: `https://languagetool-autocorrect.fly.dev`

Vous pouvez configurer votre propre serveur LanguageTool dans les paramètres.

## Compatibilité

- Chrome 88+
- Manifest V3
- Fonctionne sur:
  - Champs input standard
  - Textareas
  - Éléments contenteditable
  - CKEditor 5 (Zendesk, etc.)
  - La plupart des éditeurs riches

## Licence

MIT
