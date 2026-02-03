# MED: Meus Explorator Documentorum - AI Agent Instructions

## Project Overview

**MED** is a Rust-based file browser application. As of the current state, this is a greenfield Rust project with basic repository structure but no implementation yet. AI agents should establish patterns from the start.

## Architecture & Key Decisions

### Language & Tooling
- **Language**: Rust (confirmed by `.gitignore` with Cargo patterns)
- **Build System**: Cargo (standard Rust package manager)
- **Project Type**: Likely a TUI (Terminal User Interface) or CLI file browser, based on the name "File Browser" and Latin description

### Expected Components (to be established)
- **Core filesystem module**: Directory traversal, file metadata operations
- **UI/Display layer**: TUI rendering (suggest using libraries like `ratatui` or `crossterm`)
- **Navigation state**: Current directory, selection, filtering logic
- **Configuration**: User settings for display preferences

## Development Workflow

### Build & Run
```bash
cargo build           # Debug build
cargo build --release # Optimized release build
cargo run            # Run the application
cargo run -- <args>  # Pass command-line arguments
```

### Testing & Quality
```bash
cargo test           # Run all tests
cargo test -- --nocapture  # Show test output
cargo clippy         # Lint checks (run regularly)
cargo fmt            # Auto-format code (check with --check)
```

### Mutation Testing
- Project has `mutants.out/` in gitignore; if adding mutation testing, use `cargo mutants`

## Code Conventions & Patterns

### Rust Best Practices for This Project
1. **Error Handling**: Use `Result<T, E>` types; prefer custom error types over string errors
2. **Module Organization**: Group related functionality; create `mod.rs` files for clear boundaries
3. **Testing**: Write unit tests alongside code; integration tests in `tests/` directory
4. **File Paths**: Use `std::path::Path` and `PathBuf` for all filesystem operations (avoid strings)
5. **Unicode Support**: Ensure proper UTF-8 handling for filenames that may contain non-ASCII characters

### Naming Conventions
- Modules: `snake_case`
- Types/Structs: `PascalCase`
- Functions/Methods: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`

## Key Files & Directories (when created)

| Path | Purpose |
|------|---------|
| `src/main.rs` | Application entry point |
| `src/lib.rs` | Core library functionality |
| `src/fs/` | Filesystem operations module |
| `src/ui/` | User interface/display logic |
| `src/nav/` | Navigation state management |
| `Cargo.toml` | Dependency management |

## Dependencies (when adding)

- **TUI frameworks**: `ratatui` or `termui` for interactive UI
- **File operations**: Use stdlib `std::fs`, `walkdir` for recursive traversal
- **Error handling**: Consider `anyhow` or custom error types
- **Async**: Only if implementing concurrent operations; prefer tokio if needed

## Testing Strategy

- **Unit tests**: Colocate with implementation in `src/`
- **Integration tests**: Place in `tests/` directory for end-to-end scenarios
- **Test file operations**: Use temporary directories (`tempfile` crate) to avoid side effects
- **Mock filesystem**: Consider `mockito` or similar for testing UI without real filesystem

## Common Patterns to Avoid

❌ **String-based file paths** → Use `PathBuf`  
❌ **Panics in production code** → Use `Result` types and propagate errors  
❌ **Unwrap()** → Only in tests; use proper error handling in main code  

## Documentation

- Add module-level documentation comments (`//!`)
- Document public APIs with `///` doc comments
- Generate docs with `cargo doc --open`

---

**Last Updated**: January 2026 | **Status**: Scaffolding phase
