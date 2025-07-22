# Suggested Commands

## Development Commands
- `dx serve` - Start development server with hot reload (default web platform)
- `dx serve --platform desktop` - Run on desktop platform
- `dx serve --platform mobile` - Run on mobile platform
- `dx build` - Build the project for production
- `dx build --release` - Build optimized release version

## Rust/Cargo Commands
- `cargo check` - Check code for errors without building
- `cargo clippy` - Run Rust linter
- `cargo fmt` - Format Rust code
- `cargo test` - Run tests
- `cargo update` - Update dependencies

## System Commands (Darwin/macOS)
- `ls` - List directory contents
- `cd <path>` - Change directory
- `grep <pattern> <files>` - Search for patterns in files
- `find <path> -name <pattern>` - Find files by name pattern
- `git status` - Check git status
- `git add .` - Stage all changes
- `git commit -m "message"` - Commit changes
- `git push` - Push to remote repository

## Tailwind CSS
- Automatic Tailwind compilation with Dioxus 0.7+ (no manual setup needed)
- Custom tailwind.css file in project root for input
- Output goes to assets/ directory automatically