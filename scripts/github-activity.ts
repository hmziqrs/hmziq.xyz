#!/usr/bin/env bun

/**
 * GitHub Activity Fetcher
 *
 * Fetches GitHub user data, recent activity, top repositories, and contribution calendar.
 *
 * Environment Variables:
 *   GITHUB_TOKEN - Required for fetching contribution data via GraphQL API
 *                  Create a token at: https://github.com/settings/tokens
 *                  Required scopes: read:user
 *
 * Usage:
 *   GITHUB_TOKEN=your_token_here bun run scripts/github-activity.ts
 */

import { writeFileSync } from "fs";
import { join } from "path";

const GITHUB_USERNAME = "hmziqrs";
const OUTPUT_FILE = join("assets", "github-activity.json");

interface GitHubUser {
  login: string;
  name: string | null;
  avatar_url: string;
  bio: string | null;
  followers: number;
  public_repos: number;
  html_url: string;
}

interface GitHubRepo {
  name: string;
  description: string | null;
  language: string | null;
  stargazers_count: number;
  html_url: string;
  topics: string[];
}

interface GitHubEvent {
  type: string;
  repo: {
    name: string;
  };
  payload: any;
  created_at: string;
}

interface ContributionDay {
  contributionCount: number;
  date: string;
  color: string;
}

interface ContributionWeek {
  contributionDays: ContributionDay[];
}

interface ContributionCalendar {
  totalContributions: number;
  weeks: ContributionWeek[];
}

interface ActivityData {
  user: {
    login: string;
    name: string | null;
    avatarUrl: string;
    bio: string | null;
    followers: number;
    publicRepos: number;
    profileUrl: string;
  };
  recentActivity: Array<{
    type: string;
    repo: string;
    message?: string;
    action?: string;
    createdAt: string;
  }>;
  topRepositories: Array<{
    name: string;
    description: string | null;
    language: string | null;
    stars: number;
    url: string;
    topics: string[];
  }>;
  contributions: {
    totalContributions: number;
    calendar: ContributionCalendar;
  };
  lastUpdated: string;
}

async function fetchGitHubData(endpoint: string): Promise<any> {
  const response = await fetch(`https://api.github.com${endpoint}`, {
    headers: {
      Accept: "application/vnd.github.v3+json",
      "User-Agent": "hmziq-xyz-website",
    },
  });

  if (!response.ok) {
    throw new Error(
      `GitHub API error: ${response.status} ${response.statusText}`,
    );
  }

  return response.json();
}

async function fetchContributionData(
  username: string,
): Promise<ContributionCalendar> {
  const token = process.env.GITHUB_TOKEN;

  if (!token) {
    throw new Error(
      "GITHUB_TOKEN environment variable is required for fetching contribution data",
    );
  }

  const query = `
    query($username: String!) {
      user(login: $username) {
        contributionsCollection {
          contributionCalendar {
            totalContributions
            weeks {
              contributionDays {
                contributionCount
                date
                color
              }
            }
          }
        }
      }
    }
  `;

  const response = await fetch("https://api.github.com/graphql", {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
      "User-Agent": "hmziq-xyz-website",
    },
    body: JSON.stringify({
      query,
      variables: { username },
    }),
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(
      `GitHub GraphQL API error: ${response.status} ${response.statusText} - ${errorText}`,
    );
  }

  const data = await response.json();

  if (data.errors) {
    throw new Error(`GraphQL errors: ${JSON.stringify(data.errors)}`);
  }

  if (!data.data?.user?.contributionsCollection?.contributionCalendar) {
    throw new Error(
      "Failed to fetch contribution data: Invalid response structure",
    );
  }

  return data.data.user.contributionsCollection.contributionCalendar;
}

async function processEvents(
  events: GitHubEvent[],
): Promise<ActivityData["recentActivity"]> {
  const processedEvents = events.slice(0, 10).map((event) => {
    const base = {
      type: event.type,
      repo: event.repo.name,
      createdAt: event.created_at,
    };

    switch (event.type) {
      case "PushEvent":
        const commits = event.payload.commits || [];
        const latestCommit = commits[commits.length - 1];
        return {
          ...base,
          message: latestCommit?.message || "Pushed commits",
        };

      case "WatchEvent":
        return {
          ...base,
          action: "starred",
        };

      case "CreateEvent":
        return {
          ...base,
          action: `created ${event.payload.ref_type}`,
          message: event.payload.ref || event.payload.ref_type,
        };

      case "PullRequestEvent":
        return {
          ...base,
          action: event.payload.action,
          message: event.payload.pull_request?.title,
        };

      case "IssuesEvent":
        return {
          ...base,
          action: event.payload.action,
          message: event.payload.issue?.title,
        };

      default:
        return base;
    }
  });

  return processedEvents;
}

async function main(): Promise<void> {
  try {
    console.log("Fetching GitHub data...");

    // Fetch user data
    const userData: GitHubUser = await fetchGitHubData(
      `/users/${GITHUB_USERNAME}`,
    );
    console.log("✓ Fetched user data");

    // Fetch recent events
    const events: GitHubEvent[] = await fetchGitHubData(
      `/users/${GITHUB_USERNAME}/events?per_page=30`,
    );
    console.log(`✓ Fetched ${events.length} recent events`);

    // Fetch top repositories
    const repos: GitHubRepo[] = await fetchGitHubData(
      `/users/${GITHUB_USERNAME}/repos?sort=updated&per_page=6`,
    );
    console.log(`✓ Fetched ${repos.length} repositories`);

    // Fetch contribution data
    let contributionCalendar: ContributionCalendar;
    try {
      contributionCalendar = await fetchContributionData(GITHUB_USERNAME);
      console.log(
        `✓ Fetched contribution data (${contributionCalendar.totalContributions} contributions)`,
      );
    } catch (error) {
      console.warn(
        "⚠ Failed to fetch contribution data:",
        error instanceof Error ? error.message : error,
      );
      console.warn(
        "  To fetch contribution data, set the GITHUB_TOKEN environment variable",
      );
      console.warn(
        "  You can create a token at: https://github.com/settings/tokens",
      );
      console.warn("  Required scopes: read:user");

      // Provide fallback empty contribution data
      contributionCalendar = {
        totalContributions: 0,
        weeks: [],
      };
    }

    // Process and structure data
    const activityData: ActivityData = {
      user: {
        login: userData.login,
        name: userData.name,
        avatarUrl: userData.avatar_url,
        bio: userData.bio,
        followers: userData.followers,
        publicRepos: userData.public_repos,
        profileUrl: userData.html_url,
      },
      recentActivity: await processEvents(events),
      topRepositories: repos.map((repo) => ({
        name: repo.name,
        description: repo.description,
        language: repo.language,
        stars: repo.stargazers_count,
        url: repo.html_url,
        topics: repo.topics || [],
      })),
      contributions: {
        totalContributions: contributionCalendar.totalContributions,
        calendar: contributionCalendar,
      },
      lastUpdated: new Date().toISOString(),
    };

    // Write to file
    writeFileSync(OUTPUT_FILE, JSON.stringify(activityData, null, 2));
    console.log(`✓ Successfully wrote GitHub activity data to ${OUTPUT_FILE}`);
  } catch (error) {
    console.error("Error:", error instanceof Error ? error.message : error);
    process.exit(1);
  }
}

// Run the main function
if (import.meta.main) {
  main();
}
