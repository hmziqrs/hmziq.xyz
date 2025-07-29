# GitHub Personal Access Token Setup Guide

This guide will help you create a fine-grained personal access token to fetch your GitHub contribution data.

## Steps to Create a Fine-Grained Personal Access Token

1. **Navigate to GitHub Settings**
   - Go to [https://github.com/settings/tokens?type=beta](https://github.com/settings/tokens?type=beta)
   - Or manually: GitHub → Settings → Developer settings → Personal access tokens → Fine-grained tokens

2. **Click "Generate new token"**

3. **Configure Token Details**
   - **Token name**: `hmziq-xyz-contributions` (or any descriptive name)
   - **Expiration**: Select your preferred expiration (90 days recommended)
   - **Description**: `Token for fetching GitHub contribution data for hmziq.xyz`

4. **Repository Access**
   - Select **"Public Repositories (read)"**
   - This is sufficient for reading public contribution data

5. **Permissions**
   Under "Account permissions", set:
   - **Metadata**: Read (automatically selected)
   - **Profile**: Read (if not automatically selected)
   
   That's all you need! No repository permissions are required.

6. **Generate Token**
   - Click "Generate token" at the bottom
   - **IMPORTANT**: Copy the token immediately - you won't be able to see it again!

## Using the Token

### Option 1: Environment Variable (Recommended for Development)
```bash
# One-time use
GITHUB_TOKEN=github_pat_xxxxx bun scripts/github-activity.ts

# Or export for current session
export GITHUB_TOKEN=github_pat_xxxxx
bun scripts/github-activity.ts
```

### Option 2: .env File (Local Development)
Create a `.env` file in your project root:
```bash
GITHUB_TOKEN=github_pat_xxxxx
```

**Note**: Make sure `.env` is in your `.gitignore` file!

### Option 3: System Environment Variable
Add to your shell configuration file (`~/.zshrc`, `~/.bashrc`, etc.):
```bash
export GITHUB_TOKEN="github_pat_xxxxx"
```

Then reload your shell or run:
```bash
source ~/.zshrc  # or ~/.bashrc
```

## Security Notes

- **Never commit tokens to version control**
- Keep tokens in environment variables or secure secret management systems
- Use the minimum required permissions
- Rotate tokens regularly
- Delete tokens you no longer use

## Troubleshooting

If you get authentication errors:
1. Ensure the token has not expired
2. Verify you copied the complete token (starts with `github_pat_`)
3. Check that the token has the required permissions
4. Make sure you're using the token in the correct environment variable name (`GITHUB_TOKEN`)

## Token Permissions Reference

For this project, you only need:
- **Metadata**: Read (automatic)
- **Profile**: Read (for user data)

You do NOT need:
- Repository permissions
- Organization permissions
- Write access of any kind

This minimal permission set ensures your token is as secure as possible while still allowing the script to fetch your contribution data.