# Task Completion Workflow

## Before Committing Code
1. **Format Code**: `cargo fmt`
2. **Lint Code**: `cargo clippy` (fix any warnings)
3. **Check Compilation**: `cargo check`
4. **Test**: `cargo test` (if tests exist)
5. **Build Verification**: `dx build` (ensure production build works)

## Development Workflow
1. **Start Development**: `dx serve`
2. **Make Changes**: Edit source files in src/
3. **Hot Reload**: Changes automatically refresh in browser
4. **Test in Browser**: Verify functionality works as expected

## Quality Checks
- **Rust Standards**: Code must pass `cargo clippy` without warnings
- **Formatting**: All code must be formatted with `cargo fmt`
- **Compilation**: Must compile without errors (`cargo check`)
- **Build**: Production build must succeed (`dx build`)

## Git Workflow
- Stage changes: `git add .`
- Commit with descriptive message: `git commit -m "feat: description"`
- Push to repository: `git push`

## Platform Testing
- Test primary platform: `dx serve` (web)
- For multi-platform features, test desktop: `dx serve --platform desktop`