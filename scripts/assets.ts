#!/usr/bin/env bun

import { mkdirSync, readdirSync, copyFileSync, statSync } from "fs";
import { join, basename } from "path";

async function main(): Promise<void> {
  try {
    // Define source and destination paths
    const sourceDir = "assets/fav";
    const releaseDir = "target/dx/hmziq-xyz/release/web/public/assets/fav";

    // Create the destination directory structure if it doesn't exist
    mkdirSync(releaseDir, { recursive: true });

    // Copy all files from source to destination
    await copyDirContents(sourceDir, releaseDir);

    console.log("Successfully copied favicon assets to release directory");
  } catch (error) {
    console.error("Error:", error instanceof Error ? error.message : error);
    process.exit(1);
  }
}

async function copyDirContents(src: string, dst: string): Promise<void> {
  // Ensure destination exists
  mkdirSync(dst, { recursive: true });

  // Read directory contents
  const entries = readdirSync(src);

  for (const entry of entries) {
    const srcPath = join(src, entry);
    const dstPath = join(dst, entry);
    const stats = statSync(srcPath);

    if (stats.isDirectory()) {
      // Recursively copy subdirectories
      await copyDirContents(srcPath, dstPath);
    } else {
      // Copy file
      copyFileSync(srcPath, dstPath);
      console.log(`Copied: ${srcPath} -> ${dstPath}`);
    }
  }
}

// Run the main function
if (import.meta.main) {
  main();
}
