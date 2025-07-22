# Project Purpose and Overview

This is a personal homepage project for hmziq (hmziqrs@gmail.com) built with Rust and Dioxus framework.

## Tech Stack
- **Framework**: Dioxus 0.7.0-alpha.3 (modern Rust UI framework)
- **Language**: Rust (2021 edition)
- **Styling**: Tailwind CSS (automatic integration with Dioxus 0.7+)
- **Routing**: Dioxus Router
- **Build Tool**: Dioxus CLI (dx)

## Project Purpose
Personal homepage for showcasing experimental projects and serving as a portfolio/landing page.

## Key Dependencies
- `dioxus` with router features
- `serde` and `serde_json` for serialization
- `validator` for form validation
- `reqwest` for HTTP requests
- Web-specific: `wasm-bindgen`, `web-sys`, `gloo-*` crates
- Currently commented out: UI icons and toast notifications

## Platforms Supported
- Web (default)
- Desktop
- Mobile