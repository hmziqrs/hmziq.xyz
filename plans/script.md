# Isolated Rust Scripts with GitHub Workflows Guide

A comprehensive guide for setting up isolated Rust scripts as standalone binaries with automated GitHub workflows for cross-platform builds and releases.

## Table of Contents
1. [Overview](#overview)
2. [Project Structure](#project-structure)
3. [Creating Rust Scripts](#creating-rust-scripts)
4. [Cargo Configuration](#cargo-configuration)
5. [GitHub Workflow Setup](#github-workflow-setup)
6. [Using Scripts in Workflows](#using-scripts-in-workflows)
7. [Best Practices](#best-practices)
8. [Security Considerations](#security-considerations)

## Overview

This guide demonstrates how to:
- Create isolated Rust scripts that compile to standalone binaries
- Set up GitHub Actions to automatically build these scripts for multiple platforms
- Create releases with pre-built binaries
- Use these binaries in other workflows without rebuilding

## Project Structure

Organize your Rust project with a dedicated scripts directory:
```
project-root/
├── scripts/
│   ├── tool_one.rs
│   ├── tool_two.rs
│   └── utility.rs
├── src/
│   └── main.rs
├── Cargo.toml
└── .github/
    └── workflows/
        ├── build-tool-one.yml
        ├── build-tool-two.yml
        └── deploy.yml
```

## Creating Rust Scripts

### Basic Script Template

```rust
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Parse command line arguments
    let config = parse_args(&args)?;

    // Execute main logic
    run(config)?;

    Ok(())
}

struct Config {
    input: String,
    verbose: bool,
}

fn parse_args(args: &[String]) -> Result<Config, Box<dyn Error>> {
    if args.len() < 2 {
        return Err("Usage: tool_name <input> [--verbose]".into());
    }

    Ok(Config {
        input: args[1].clone(),
        verbose: args.contains(&"--verbose".to_string()),
    })
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.verbose {
        println!("Processing input: {}", config.input);
    }

    // Your tool's logic here
    println!("Result: Processed {}", config.input);

    Ok(())
}
```

### Advanced Script with Dependencies

```rust
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct ProcessedData {
    original: String,
    processed: String,
    timestamp: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = std::env::args()
        .nth(1)
        .ok_or("Please provide input file path")?;

    // Read input file
    let content = fs::read_to_string(&input_path)?;

    // Process data
    let processed = ProcessedData {
        original: content.clone(),
        processed: content.to_uppercase(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs(),
    };

    // Write output
    let output_path = Path::new(&input_path).with_extension("json");
    let json = serde_json::to_string_pretty(&processed)?;
    fs::write(output_path, json)?;

    println!("Processing complete!");
    Ok(())
}
```

## Cargo Configuration

Configure your `Cargo.toml` to define scripts as binary targets:

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

# Main application dependencies
[dependencies]
# Your main app dependencies here

# Define each script as a binary
[[bin]]
name = "tool_one"
path = "scripts/tool_one.rs"

[[bin]]
name = "tool_two"
path = "scripts/tool_two.rs"

[[bin]]
name = "utility"
path = "scripts/utility.rs"

# Dependencies only for non-WASM targets (scripts)
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
clap = { version = "4.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }

# Optimize release builds
[profile.release]
opt-level = "z"     # Optimize for size
debug = false       # No debug info
lto = true         # Link-time optimization
codegen-units = 1  # Single codegen unit
panic = "abort"    # Smaller binary
strip = true       # Strip symbols
incremental = false # Clean builds
```

## GitHub Workflow Setup

### Build Workflow Template

Create `.github/workflows/build-[tool-name].yml`:

```yaml
name: Build and Release [Tool Name] Binary

on:
  push:
    paths:
      - "scripts/[tool_name].rs"
      - ".github/workflows/build-[tool-name].yml"
  workflow_dispatch:

jobs:
  build:
    name: Build on ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        include:
          - os: ubuntu-latest
            artifact_name: [tool-name]-linux
            asset_name: [tool-name]-linux-amd64
          - os: macos-latest
            artifact_name: [tool-name]-macos
            asset_name: [tool-name]-macos-amd64
          - os: windows-latest
            artifact_name: [tool-name]-windows.exe
            asset_name: [tool-name]-windows-amd64.exe

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: stable

      # Optional: Install system dependencies
      # - name: Install dependencies (Ubuntu)
      #   if: matrix.os == 'ubuntu-latest'
      #   run: |
      #     sudo apt-get update
      #     sudo apt-get install -y libssl-dev pkg-config

      - name: Build binary in release mode
        run: |
          cargo build --release --bin [tool_name]

      - name: Copy binary to artifact location
        shell: bash
        run: |
          mkdir -p ./artifacts/
          if [ "${{ matrix.os }}" = "windows-latest" ]; then
            cp target/release/[tool_name].exe ./artifacts/${{ matrix.artifact_name }}
          else
            cp target/release/[tool_name] ./artifacts/${{ matrix.artifact_name }}
          fi

      - name: Upload build artifacts
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.asset_name }}
          path: ./artifacts/${{ matrix.artifact_name }}

  release:
    name: Create Release
    needs: build
    runs-on: ubuntu-latest
    permissions:
      contents: write
    if: github.ref == 'refs/heads/main' || github.ref == 'refs/heads/master' || github.event_name == 'workflow_dispatch'

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Set version
        id: version
        run: |
          echo "version=[tool-name]-$(date +'%Y%m%d%H%M%S')" >> $GITHUB_OUTPUT

      - name: Download all artifacts
        uses: actions/download-artifact@v4
        with:
          path: ./artifacts

      - name: Create Release
        uses: softprops/action-gh-release@v2
        with:
          tag_name: ${{ steps.version.outputs.version }}
          name: [Tool Name] Binary ${{ steps.version.outputs.version }}
          draft: false
          prerelease: false
          files: |
            ./artifacts/[tool-name]-linux-amd64/[tool-name]-linux
            ./artifacts/[tool-name]-macos-amd64/[tool-name]-macos
            ./artifacts/[tool-name]-windows-amd64.exe/[tool-name]-windows.exe
          token: ${{ secrets.GITHUB_TOKEN }}
```

### Multiple Scripts Build Workflow

For building multiple scripts in one workflow:

```yaml
name: Build All Tools

on:
  push:
    paths:
      - "scripts/**/*.rs"
      - "Cargo.toml"
  workflow_dispatch:

jobs:
  build:
    name: Build ${{ matrix.tool }} on ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        tool: [tool_one, tool_two, utility]
        include:
          - os: ubuntu-latest
            platform: linux
            ext: ""
          - os: macos-latest
            platform: macos
            ext: ""
          - os: windows-latest
            platform: windows
            ext: ".exe"

    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable

      - name: Build ${{ matrix.tool }}
        run: cargo build --release --bin ${{ matrix.tool }}

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.tool }}-${{ matrix.platform }}
          path: target/release/${{ matrix.tool }}${{ matrix.ext }}

  release:
    needs: build
    runs-on: ubuntu-latest
    permissions:
      contents: write

    steps:
      - uses: actions/checkout@v4

      - name: Download all artifacts
        uses: actions/download-artifact@v4

      - name: Create Release
        uses: softprops/action-gh-release@v2
        with:
          tag_name: tools-v${{ github.run_number }}
          name: Tools Release v${{ github.run_number }}
          files: |
            */*
          token: ${{ secrets.GITHUB_TOKEN }}
```

## Using Scripts in Other Workflows

### Download and Use Pre-built Binaries

```yaml
name: Deploy Application

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Download tool binary
        run: |
          mkdir -p ./bin

          # Find latest release
          LATEST_RELEASE=$(curl -s https://api.github.com/repos/${{ github.repository }}/releases | \
            jq -r '.[] | select(.name | contains("[Tool Name] Binary")) | .tag_name' | \
            sort -V | tail -n 1)

          if [ -z "$LATEST_RELEASE" ]; then
            echo "Error: No release found. Ensure build workflow has run."
            exit 1
          fi

          echo "Using release: $LATEST_RELEASE"

          # Download binary for current platform
          curl -L \
            "https://github.com/${{ github.repository }}/releases/download/$LATEST_RELEASE/[tool-name]-linux" \
            -o ./bin/tool_name

          chmod +x ./bin/tool_name

      - name: Use the tool
        run: ./bin/tool_name --input data.txt --verbose
```

### Download Multiple Tools

```yaml
- name: Setup tools
  run: |
    mkdir -p ./bin

    # Define tools to download
    declare -A tools=(
      ["tool_one"]="Tool One Binary"
      ["tool_two"]="Tool Two Binary"
      ["utility"]="Utility Binary"
    )

    # Download each tool
    for tool in "${!tools[@]}"; do
      echo "Downloading $tool..."

      RELEASE=$(curl -s https://api.github.com/repos/${{ github.repository }}/releases | \
        jq -r --arg name "${tools[$tool]}" \
        '.[] | select(.name | contains($name)) | .tag_name' | \
        sort -V | tail -n 1)

      if [ -n "$RELEASE" ]; then
        curl -L \
          "https://github.com/${{ github.repository }}/releases/download/$RELEASE/$tool-linux" \
          -o "./bin/$tool"
        chmod +x "./bin/$tool"
      else
        echo "Warning: No release found for $tool"
      fi
    done

- name: Run processing pipeline
  run: |
    ./bin/tool_one --prepare data/
    ./bin/tool_two --process data/
    ./bin/utility --cleanup
```

### Fallback to Local Build

```yaml
- name: Get or build tool
  run: |
    mkdir -p ./bin

    # Try to download pre-built binary
    if curl -f -s -L \
      "https://github.com/${{ github.repository }}/releases/latest/download/tool_name-linux" \
      -o ./bin/tool_name; then
      echo "Downloaded pre-built binary"
      chmod +x ./bin/tool_name
    else
      echo "Building from source..."
      cargo build --release --bin tool_name
      cp target/release/tool_name ./bin/
    fi

- name: Execute tool
  run: ./bin/tool_name --help
```

### Platform-Specific Downloads

```yaml
- name: Download platform-specific binary
  run: |
    mkdir -p ./bin

    # Determine platform
    case "${{ runner.os }}" in
      Linux)   PLATFORM="linux" ;;
      macOS)   PLATFORM="macos" ;;
      Windows) PLATFORM="windows"; EXT=".exe" ;;
    esac

    # Get latest release
    RELEASE_URL=$(curl -s https://api.github.com/repos/${{ github.repository }}/releases/latest | \
      jq -r ".assets[] | select(.name | contains(\"$PLATFORM\")) | .browser_download_url")

    if [ -n "$RELEASE_URL" ]; then
      curl -L "$RELEASE_URL" -o "./bin/tool_name${EXT:-}"
      chmod +x "./bin/tool_name${EXT:-}"
    else
      echo "No binary found for platform: $PLATFORM"
      exit 1
    fi
```

## Best Practices

### Path-Based Triggers

Optimize workflow runs by triggering builds only when relevant files change:

```yaml
on:
  push:
    paths:
      - "scripts/**.rs"
      - ".github/workflows/build-*.yml"
      - "Cargo.toml"
      - "Cargo.lock"
```

### Versioning Strategy

```yaml
# Semantic versioning
VERSION="v$(grep version Cargo.toml | head -1 | cut -d'"' -f2)"

# Date-based versioning
VERSION="v$(date +'%Y.%m.%d')-${{ github.run_number }}"

# Git-based versioning
VERSION="v$(git describe --tags --always --dirty)"
```

### Binary Size Optimization

```toml
# Cargo.toml optimization profiles
[profile.release]
opt-level = "z"       # Optimize for size
lto = true           # Enable Link Time Optimization
codegen-units = 1    # Compile crates one after another
panic = "abort"      # Remove panic unwinding code
strip = true         # Strip debug symbols
debug = false        # Disable debug info

# Further size reduction
[profile.release.package."*"]
opt-level = "z"
strip = "symbols"
```

### Cross-Platform Considerations

```rust
// Handle platform differences in scripts
use std::path::PathBuf;

#[cfg(target_os = "windows")]
const PATH_SEPARATOR: char = '\\';

#[cfg(not(target_os = "windows"))]
const PATH_SEPARATOR: char = '/';

fn get_config_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    let base = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());

    #[cfg(target_os = "macos")]
    let base = std::env::var("HOME")
        .map(|h| format!("{}/Library/Application Support", h))
        .unwrap_or_else(|_| ".".to_string());

    #[cfg(target_os = "linux")]
    let base = std::env::var("XDG_CONFIG_HOME")
        .or_else(|_| std::env::var("HOME").map(|h| format!("{}/.config", h)))
        .unwrap_or_else(|_| ".".to_string());

    PathBuf::from(base).join("my_tool")
}
```

### Error Handling Best Practices

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ToolError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Processing failed: {0}")]
    Processing(String),
}

type Result<T> = std::result::Result<T, ToolError>;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    // Tool implementation
    Ok(())
}
```

## Security Considerations

### GitHub Token Management

```yaml
# Use GITHUB_TOKEN when possible (automatically provided)
token: ${{ secrets.GITHUB_TOKEN }}

# For cross-repository access, use PAT with minimal permissions
# Settings → Developer settings → Personal access tokens → Fine-grained tokens
# Select only required repositories and permissions:
# - Contents: Read/Write (for releases)
# - Actions: Read (optional, for workflow status)
```

### Secure Workflow Permissions

```yaml
# Top-level permissions (most restrictive)
permissions:
  contents: read

jobs:
  build:
    permissions:
      contents: read
    # Build job only needs read

  release:
    permissions:
      contents: write  # Only release job needs write
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
```

### Binary Integrity

```yaml
- name: Generate checksums
  run: |
    cd artifacts/
    sha256sum * > SHA256SUMS

    # Sign checksums (optional)
    # gpg --detach-sign SHA256SUMS

- name: Upload checksums with release
  uses: softprops/action-gh-release@v2
  with:
    files: |
      artifacts/*
      artifacts/SHA256SUMS
```

### Dependency Security

```yaml
- name: Security audit
  run: |
    cargo install cargo-audit
    cargo audit

- name: License check
  run: |
    cargo install cargo-license
    cargo license --json > licenses.json
```

## Advanced Patterns

### Conditional Compilation

```rust
// Feature flags for optional functionality
#[cfg(feature = "advanced")]
mod advanced_features;

// Platform-specific code
#[cfg(target_os = "linux")]
fn platform_specific() {
    println!("Linux-specific code");
}

// Architecture-specific optimizations
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
```

### Multi-Binary Workspace

```toml
# Cargo.toml for workspace
[workspace]
members = ["tools/*"]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Your Name"]

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

### Caching Dependencies

```yaml
- name: Cache Cargo registry
  uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    restore-keys: |
      ${{ runner.os }}-cargo-
```

## Troubleshooting

### Diagnostic Commands

```bash
# Check release artifacts
gh release view --repo $GITHUB_REPOSITORY

# Download and test specific release
gh release download v1.0.0 --pattern "*-linux"

# Verify binary properties
file ./tool_name
ldd ./tool_name || otool -L ./tool_name
./tool_name --version

# Debug workflow runs
gh run list --workflow build.yml
gh run view <run-id> --log
```

### Common Solutions

1. **"Binary not executable on target platform"**
   ```yaml
   # Ensure correct target in build
   - run: rustup target add x86_64-unknown-linux-musl
   - run: cargo build --release --target x86_64-unknown-linux-musl
   ```

2. **"Dynamic library dependencies missing"**
   ```toml
   # Build static binaries
   [target.x86_64-unknown-linux-musl]
   rustflags = ["-C", "target-feature=+crt-static"]
   ```

3. **"Release upload fails"**
   ```yaml
   # Ensure unique filenames in matrix builds
   - name: Rename artifact
     run: |
       mv target/release/tool tool-${{ matrix.os }}-${{ matrix.arch }}
   ```

## Summary

This approach enables:
- **Modularity**: Scripts remain independent of main application
- **Reusability**: Pre-built binaries available across workflows
- **Performance**: No rebuild overhead in dependent workflows
- **Reliability**: Versioned releases ensure consistency
- **Portability**: Cross-platform binaries for diverse environments

The combination of Rust's compilation model and GitHub's release system creates a robust infrastructure for tool distribution and usage in CI/CD pipelines.
