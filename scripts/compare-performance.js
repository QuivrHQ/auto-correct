#!/usr/bin/env node

const GRAMMAR_RS_URL = "https://grammar-rs-autocorrect.fly.dev";
const LANGUAGETOOL_URL = "https://languagetool-autocorrect.fly.dev";

const TEST_TEXTS = [
  "I have a apple and a orange.",
  "She dont like speling mistakes.",
  "There house is beatiful.",
  "He go to school every day.",
  "The dog are running in the park.",
  "I seen that movie yesterday.",
  "In my personal opinion, I think that...",
  "The reason is because I was late.",
  "Free gift for all customers.",
  "Je vais à la plage avec mes amis.",
  "Il y a une faute d'orthographe dans ce texte.",
  "This is a longer text with multiple sentences to test performance under realistic conditions with various types of errors.",
];

async function checkText(url, text, language = "auto") {
  const startTime = Date.now();

  try {
    const response = await fetch(`${url}/v2/check`, {
      method: "POST",
      headers: { "Content-Type": "application/x-www-form-urlencoded" },
      body: new URLSearchParams({ text, language }),
    });

    const elapsed = Date.now() - startTime;

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }

    const data = await response.json();

    return {
      success: true,
      elapsed,
      matchCount: data.matches?.length || 0,
      matches: data.matches || [],
    };
  } catch (error) {
    return {
      success: false,
      elapsed: Date.now() - startTime,
      error: error.message,
    };
  }
}

async function runComparison() {
  console.log("🔬 Comparaison: LanguageTool vs grammar-rs\n");

  const results = {
    grammarRs: { latencies: [], matchCounts: [], errors: 0 },
    languageTool: { latencies: [], matchCounts: [], errors: 0 },
  };

  for (let i = 0; i < TEST_TEXTS.length; i++) {
    const text = TEST_TEXTS[i];
    console.log(`Test ${i + 1}/${TEST_TEXTS.length}: "${text.substring(0, 50)}..."`);

    const grResult = await checkText(GRAMMAR_RS_URL, text);
    if (grResult.success) {
      results.grammarRs.latencies.push(grResult.elapsed);
      results.grammarRs.matchCounts.push(grResult.matchCount);
    } else {
      results.grammarRs.errors++;
      console.log(`  ❌ grammar-rs error: ${grResult.error}`);
    }

    const ltResult = await checkText(LANGUAGETOOL_URL, text);
    if (ltResult.success) {
      results.languageTool.latencies.push(ltResult.elapsed);
      results.languageTool.matchCounts.push(ltResult.matchCount);
    } else {
      results.languageTool.errors++;
      console.log(`  ❌ LanguageTool error: ${ltResult.error}`);
    }

    console.log(`  grammar-rs: ${grResult.elapsed}ms (${grResult.matchCount} matches)`);
    console.log(`  LanguageTool: ${ltResult.elapsed}ms (${ltResult.matchCount} matches)\n`);
  }

  // Stats
  const stats = {};
  for (const [name, data] of Object.entries(results)) {
    const latencies = data.latencies.sort((a, b) => a - b);
    stats[name] = {
      p50: latencies[Math.floor(latencies.length * 0.5)] || 0,
      p95: latencies[Math.floor(latencies.length * 0.95)] || 0,
      p99: latencies[Math.floor(latencies.length * 0.99)] || 0,
      avg: latencies.reduce((a, b) => a + b, 0) / latencies.length || 0,
      min: Math.min(...latencies) || 0,
      max: Math.max(...latencies) || 0,
      avgMatches: data.matchCounts.reduce((a, b) => a + b, 0) / data.matchCounts.length || 0,
      errors: data.errors,
    };
  }

  // Rapport
  console.log("\n" + "=".repeat(70));
  console.log("📊 RÉSULTATS");
  console.log("=".repeat(70) + "\n");

  console.log("## Latence (ms)\n");
  console.log("| Métrique | LanguageTool | grammar-rs | Ratio |");
  console.log("|----------|--------------|------------|-------|");
  console.log(`| p50 | ${stats.languageTool.p50.toFixed(0)}ms | ${stats.grammarRs.p50.toFixed(0)}ms | ${(stats.languageTool.p50 / stats.grammarRs.p50).toFixed(1)}x |`);
  console.log(`| p95 | ${stats.languageTool.p95.toFixed(0)}ms | ${stats.grammarRs.p95.toFixed(0)}ms | ${(stats.languageTool.p95 / stats.grammarRs.p95).toFixed(1)}x |`);
  console.log(`| Moyenne | ${stats.languageTool.avg.toFixed(0)}ms | ${stats.grammarRs.avg.toFixed(0)}ms | ${(stats.languageTool.avg / stats.grammarRs.avg).toFixed(1)}x |`);

  console.log("\n## Précision\n");
  console.log("| Métrique | LanguageTool | grammar-rs |");
  console.log("|----------|--------------|------------|");
  console.log(`| Matches moyens | ${stats.languageTool.avgMatches.toFixed(1)} | ${stats.grammarRs.avgMatches.toFixed(1)} |`);
  console.log(`| Erreurs | ${stats.languageTool.errors} | ${stats.grammarRs.errors} |`);

  console.log("\n" + "=".repeat(70) + "\n");
}

runComparison().catch(console.error);
