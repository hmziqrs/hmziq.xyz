# Code Style and Conventions

## Rust Code Style
- **Edition**: Rust 2021
- **Formatting**: Use `cargo fmt` for automatic formatting
- **Linting**: Use `cargo clippy` for code quality checks
- **Naming**: Standard Rust naming conventions (snake_case, PascalCase)

## Dioxus Conventions
- **Components**: PascalCase function names with `#[component]` attribute
- **Props**: Define props with `#[derive(Props)]` for complex components
- **Hooks**: Prefix with `use_` (e.g. `use_form`, `use_previous`)
- **Signals**: Use `use_signal()` for reactive state
- **Global State**: Use `GlobalSignal` with `OnceLock` pattern

## Project Patterns

### State Management
- **StateFrame**: Wrapper for async operations with Loading/Success/Failed states
- **GlobalSignal**: For application-wide state
- **Store Pattern**: Separate state definition and actions

### Component Organization
- **components/**: Pure UI components, reusable
- **containers/**: Components with business logic
- **screens/**: Full page components
- **hooks/**: Custom reactive hooks

### File Structure
- Each module has `mod.rs` for public interface
- Related functionality grouped in dedicated directories
- Asset references use `asset!()` macro

## Form Handling
- **OxForm**: Custom form state management pattern
- **Validation**: Using `validator` crate with derive macros
- **Field State**: Track touched, dirty, error states per field