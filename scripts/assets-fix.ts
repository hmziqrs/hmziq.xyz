#!/usr/bin/env bun

import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

const TARGET_DIR = "target/dx/hmziq-xyz/release/web/public";
const ASSETS_DIR = join(TARGET_DIR, "assets");
const INDEX_PATH = join(TARGET_DIR, "index.html");

async function main(): Promise<void> {
  try {
    // Read all files in assets directory (excluding subdirectories)
    const assetFiles = readdirSync(ASSETS_DIR).filter(file => {
      const filePath = join(ASSETS_DIR, file);
      return statSync(filePath).isFile();
    });

    console.log("Found assets:", assetFiles);

    // Find specific asset files
    const cssFile = assetFiles.find(f => f.startsWith("tailwind-") && f.endsWith(".css"));
    const appJsFile = assetFiles.find(f => f.startsWith("app-") && f.endsWith(".js"));

    if (!cssFile) {
      console.error("Warning: No tailwind CSS file found");
    }
    if (!appJsFile) {
      console.error("Warning: No app JS file found");
    }

    // Read index.html
    let html = readFileSync(INDEX_PATH, "utf-8");
    console.log("Read index.html");

    // Track changes
    let changesMade = 0;

    // Fix CSS placeholder
    if (cssFile) {
      const cssPlaceholder = /href="\/assets\/this is a placeholder path which will be replaced by the linker"/;
      if (cssPlaceholder.test(html)) {
        html = html.replace(cssPlaceholder, `href="/assets/${cssFile}"`);
        console.log(`✓ Replaced CSS placeholder with: /assets/${cssFile}`);
        changesMade++;
      }
    }

    // Fix JS placeholder
    if (appJsFile) {
      const jsPlaceholder = /src="\/assets\/this is a placeholder path which will be replaced by the linker"/;
      if (jsPlaceholder.test(html)) {
        html = html.replace(jsPlaceholder, `src="/assets/${appJsFile}"`);
        console.log(`✓ Replaced JS placeholder with: /assets/${appJsFile}`);
        changesMade++;
      }
    }

    // Fix /./assets/ prefixes
    const doubleDotPattern = /\/\.\/assets\//g;
    const doubleDotMatches = html.match(doubleDotPattern);
    if (doubleDotMatches) {
      html = html.replace(doubleDotPattern, "/assets/");
      console.log(`✓ Fixed ${doubleDotMatches.length} instances of /./assets/ to /assets/`);
      changesMade += doubleDotMatches.length;
    }

    // Write updated HTML
    if (changesMade > 0) {
      writeFileSync(INDEX_PATH, html);
      console.log(`\n✅ Successfully updated index.html with ${changesMade} changes`);
    } else {
      console.log("\n✅ No changes needed - index.html is already correct");
    }

  } catch (error) {
    console.error("Error:", error instanceof Error ? error.message : error);
    process.exit(1);
  }
}

// Run the script
if (import.meta.main) {
  main();
}