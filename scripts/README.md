# Scripts

## GitHub Activity Data Fetcher

The `github-activity.ts` script fetches GitHub activity data and saves it to `assets/github-activity.json`.

### Setup

To fetch real contribution data, you need a GitHub Personal Access Token:

1. **Create a token**: See [GITHUB_TOKEN_SETUP.md](./GITHUB_TOKEN_SETUP.md) for detailed instructions
2. **Set the environment variable**:
   ```bash
   export GITHUB_TOKEN=your_github_token_here
   ```

### Usage

```bash
# With token (fetches all data including contributions)
GITHUB_TOKEN=your_token_here bun scripts/github-activity.ts

# Without token (fetches basic data only)
bun scripts/github-activity.ts
```

### What Gets Fetched

- **User Profile**: Followers, repository count, bio
- **Recent Activity**: Latest commits, stars, issues, PRs
- **Top Repositories**: Your 6 most recently updated repos
- **Contribution Graph**: 365 days of contribution data (requires token)

Example GraphQL query for real contribution data:
```graphql
query {
  user(login: "hmziqrs") {
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
```

The current implementation uses pseudo-random data for demonstration purposes.

## Assets Copy Script

The `assets.ts` script copies favicon assets to the release directory during builds.

### Usage

```bash
bun scripts/assets.ts
```