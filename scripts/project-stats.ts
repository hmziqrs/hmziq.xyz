#!/usr/bin/env bun

/**
 * Project Stats Fetcher
 *
 * Fetches stars, forks, and last commit date for all projects listed in projects.rs.
 * Writes the result to assets/project-stats.json for compile-time inclusion.
 *
 * Environment Variables:
 *   GITHUB_TOKEN - Optional, increases rate limits
 *
 * Usage:
 *   bun run scripts/project-stats.ts
 */

import { writeFileSync } from "fs";
import { join } from "path";

const GITHUB_USERNAME = "hmziqrs";
const OUTPUT_FILE = join("assets", "project-stats.json");

const REPO_SLUGS = [
  "flutter-ui-designs",
  "invmovieconcept1",
  "react-native-loop-game",
  "Flutter-Experiments",
  "ruxlog-frontend",
  "go-cross-screenshot",
  "fhgllaunch",
  "claude-multi",
  "ruxlog-backend",
  "gandalf-sax",
  "ruxlog",
  "go-minesweeper",
];

interface RepoStats {
  stars: number;
  forks: number;
  last_commit: string;
}

interface ProjectStats {
  [slug: string]: RepoStats;
}

async function fetchJson(endpoint: string): Promise<any> {
  const token = process.env.GITHUB_TOKEN;
  const headers: Record<string, string> = {
    Accept: "application/vnd.github.v3+json",
    "User-Agent": "hmziq-xyz-website",
    "Cache-Control": "no-cache",
  };

  if (token) {
    headers.Authorization = `Bearer ${token}`;
  }

  const response = await fetch(`https://api.github.com${endpoint}`, { headers });

  if (!response.ok) {
    throw new Error(`GitHub API error: ${response.status} ${response.statusText} for ${endpoint}`);
  }

  return response.json();
}

async function fetchRepoStats(slug: string): Promise<RepoStats> {
  const [repo, commits] = await Promise.all([
    fetchJson(`/repos/${GITHUB_USERNAME}/${slug}`),
    fetchJson(`/repos/${GITHUB_USERNAME}/${slug}/commits?per_page=1`),
  ]);

  const lastCommit = commits.length > 0 ? commits[0].commit.committer.date.split("T")[0] : "unknown";

  return {
    stars: repo.stargazers_count,
    forks: repo.forks_count,
    last_commit: lastCommit,
  };
}

async function main(): Promise<void> {
  try {
    console.log(`Fetching stats for ${REPO_SLUGS.length} repositories...`);

    const results: ProjectStats = {};

    for (const slug of REPO_SLUGS) {
      try {
        results[slug] = await fetchRepoStats(slug);
        console.log(`  ✓ ${slug}: ${results[slug].stars}★ ${results[slug].forks}⎇ last: ${results[slug].last_commit}`);
      } catch (error) {
        console.error(`  ✗ ${slug}: ${error instanceof Error ? error.message : error}`);
        results[slug] = { stars: 0, forks: 0, last_commit: "unknown" };
      }
    }

    writeFileSync(OUTPUT_FILE, JSON.stringify(results, null, 2));
    console.log(`\n✅ Wrote project stats to ${OUTPUT_FILE}`);
  } catch (error) {
    console.error("Error:", error instanceof Error ? error.message : error);
    process.exit(1);
  }
}

if (import.meta.main) {
  main();
}
