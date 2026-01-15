<p align="center">
  <img src="public/icons/icon128.svg" width="80" height="80" alt="AutoCorrect Logo">
</p>

<h1 align="center">AutoCorrect</h1>

<p align="center">
  <strong>Real-time grammar & spelling correction for Chrome</strong>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#how-it-works">How It Works</a> •
  <a href="#development">Development</a> •
  <a href="#api">API</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Chrome-Extension-4285F4?style=for-the-badge&logo=googlechrome&logoColor=white" alt="Chrome Extension">
  <img src="https://img.shields.io/badge/Manifest-V3-success?style=for-the-badge" alt="Manifest V3">
  <img src="https://img.shields.io/badge/TypeScript-5.0-3178C6?style=for-the-badge&logo=typescript&logoColor=white" alt="TypeScript">
  <img src="https://img.shields.io/badge/React-19-61DAFB?style=for-the-badge&logo=react&logoColor=black" alt="React">
</p>

---

## ✨ Features

🔍 **Real-time Detection** — Automatically detects spelling and grammar errors as you type

🎯 **Smart Underlines** — Visual indicators with color coding:
- 🔴 **Red** for spelling errors
- 🟠 **Orange** for grammar mistakes
- 🔵 **Blue** for style suggestions

⚡ **One-Click Fix** — Click any underlined word to see suggestions and apply corrections instantly

🚀 **Fix All** — Apply all corrections at once with a single button

📊 **Score Dashboard** — Real-time quality score with detailed breakdown by error type

🌍 **Multi-language** — Supports French, English, and auto-detection

🔌 **Universal Compatibility** — Works everywhere:
- Standard inputs & textareas
- Rich text editors (CKEditor, TinyMCE)
- Contenteditable elements
- Zendesk, Gmail, Notion, and more

---

## 📦 Installation

### From Release (Recommended)

1. **Download** the latest `autocorrect-extension.zip` from [Releases](https://github.com/StanGirard/auto-correct/releases)
2. **Unzip** the file to a folder
3. **Open Chrome** and navigate to `chrome://extensions`
4. **Enable** "Developer mode" (toggle in top right)
5. **Click** "Load unpacked" and select the `dist` folder
6. **Done!** The extension icon appears in your toolbar

### From Source

```bash
# Clone the repository
git clone https://github.com/StanGirard/auto-correct.git
cd auto-correct

# Install dependencies
npm install

# Build the extension
npm run build

# Load the dist/ folder in Chrome
```

---

## 🎯 How It Works

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│   You type: "Je vais au marche aujourdui"              │
│                      ~~~~~~~  ~~~~~~~~~                 │
│                         │         │                     │
│                         ▼         ▼                     │
│                     [marché]  [aujourd'hui]             │
│                                                         │
│   Click the underline → See suggestions → Apply fix    │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

1. **Detection** — The extension monitors text fields on any webpage
2. **Analysis** — Text is sent to LanguageTool API for grammar/spelling check
3. **Display** — Errors are highlighted with colored wavy underlines
4. **Correction** — Click to see suggestions, one more click to apply

---

## 🖥️ Popup Interface

```
┌─────────────────────────────────┐
│  🔵 AutoCorrect        [FR ▼]  │
├─────────────────────────────────┤
│  Score  ████████░░░  78/100    │
│                                 │
│  🔴 2 spelling  🟠 1 grammar   │
│                                 │
│  Recommendation:                │
│  aujourdui → aujourd'hui        │
│                                 │
│  [    ✓ Fix All (3)    ]       │
├─────────────────────────────────┤
│  • Error cards with details... │
└─────────────────────────────────┘
```

---

## 🛠️ Development

### Prerequisites

- Node.js 18+
- npm or yarn

### Commands

```bash
# Development mode with hot reload
npm run dev

# Production build
npm run build

# Run E2E tests (Playwright)
npm test

# Run tests with UI
npm run test:ui
```

### Project Structure

```
auto-correct/
├── src/
│   ├── content/              # Content scripts (injected into pages)
│   │   ├── text-field-manager.ts   # Field detection & correction
│   │   ├── underline-renderer.ts   # Visual underlines & tooltips
│   │   └── language-tool-client.ts # API client
│   ├── popup/                # Extension popup UI
│   │   └── Popup.tsx         # React component
│   ├── background/           # Service worker
│   └── shared/               # Shared types & utilities
├── tests/
│   └── e2e/                  # Playwright tests
├── dist/                     # Built extension
└── docker/                   # LanguageTool server config
```

---

## 🌐 API

The extension uses [LanguageTool](https://languagetool.org/) for grammar checking.

**Default server:** `https://languagetool-autocorrect.fly.dev`

You can self-host your own LanguageTool server using the provided Docker configuration:

```bash
cd docker
fly launch  # Deploy to Fly.io
```

Or use the official LanguageTool API at `https://api.languagetool.org`

---

## 🧪 Tested Platforms

| Platform | Status | Notes |
|----------|--------|-------|
| Standard inputs | ✅ | Full support |
| Textareas | ✅ | Full support |
| Contenteditable | ✅ | Full support |
| CKEditor 5 | ✅ | Zendesk, etc. |
| Gmail | ✅ | Compose window |
| Google Docs | ⚠️ | Canvas-based, limited |
| Notion | ✅ | Works with contenteditable |

---

## 📄 License

MIT © [Stan Girard](https://github.com/StanGirard)

---

<p align="center">
  <sub>Built with ❤️ using TypeScript, React, and Tailwind CSS</sub>
</p>
