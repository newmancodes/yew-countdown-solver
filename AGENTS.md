# AGENTS.md

Guidance for AI coding agents working in this repository.

## Project Overview

Rust/Yew WebAssembly app that solves the "numbers round" from the British game show Countdown. Users input 6 numbers and a target (1-999); the solver finds arithmetic solutions using +, -, *, /. Built with Yew 0.22 (CSR mode), compiled to WASM via Trunk, styled with Tailwind CSS v4.

## Build & Development Commands

```bash
# Prerequisites (one-time setup)
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
npm install

# Run all unit tests
cargo test

# Run a single test by name (substring match)
cargo test test_name

# Run tests in a specific module
cargo test board::tests

# Check formatting (CI enforces this — must pass before merge)
cargo fmt --check

# Fix formatting
cargo fmt

# Build WASM for production
trunk build --release

# Local development server with hot-reload
trunk serve
```

Tailwind CSS compiles automatically via a Trunk pre-build hook (`npm run build` runs before each `trunk build`/`trunk serve`). Source: `styles/main.css`. Output: `styles/output.css` (generated and committed).

### E2E Tests (Playwright + pytest)

```bash
# Install dependencies (first time)
cd tests/e2e && uv sync
uv run playwright install chromium --with-deps

# Start the server for E2E tests (requires a built dist/)
python3 -m http.server 8080 --directory ./dist

# Run all E2E tests (server must be running on port 8080)
cd tests/e2e && uv run pytest tests/

# Run a single E2E test file
cd tests/e2e && uv run pytest tests/test_solver.py

# Run a single E2E test by name
cd tests/e2e && uv run pytest tests/test_solver.py -k "test_name"
```

### CI Pipeline

Push to `main` runs: `cargo fmt --check` -> `cargo test` (with coverage) -> `trunk build --release` -> E2E tests -> deploy to Azure Static Web Apps. All checks must pass.

## Architecture

### Module Structure

```
src/
  main.rs              # WASM entry point, tracing setup
  app.rs               # Root Yew component, screen routing via use_state
  lib.rs               # Public library crate (game, solver modules)
  components.rs        # Module re-exports for GameBoard, GameProvider, ManualEntry
  components/
    game_board.rs      # Game display, solve/compete, solution rendering
    game_provider.rs   # Game setup screen (random, custom split, manual entry)
    manual_entry.rs    # Number picker with validation, use_reducer state
  game.rs              # Module file
  game/
    board.rs           # Board type, BoardBuilder (validated), BoardAdjuster (solver)
    model.rs           # Game type wrapping Board + target, Problem impl
  solver.rs            # Module file
  solver/
    traits.rs          # Problem, Solver traits; Operation, Solution, Instruction types
    iterative_deepening.rs  # IDDFS solver with arena allocation
```

**Dual-crate pattern**: `src/main.rs` is the binary crate (UI components in `src/components/`). `src/lib.rs` is the library crate (`game`, `solver` modules). Components import from the library crate using `yew_countdown_solver::game::...` and `yew_countdown_solver::solver::...`.

### Key Domain Types

- **Board**: Exactly 6 numbers, sorted ascending. Built via `BoardBuilder` (validates game rules) or `BoardAdjuster` (solver use, no rule validation).
- **Game**: Board + target (1-999). Implements `Problem` trait.
- **IterativeDeepeningSolver**: IDDFS over board states, depth 1-6. Uses `typed_arena::Arena` for allocation.
- **Operation**: `{ left, operator, right, result }` -- subtraction/division always expressed as larger op smaller.

## Code Style Guidelines

### Formatting

- **Use `cargo fmt`** (rustfmt default settings). CI enforces `cargo fmt --check`.
- No custom rustfmt configuration exists; standard defaults apply.

### Imports

- Group imports in this order: (1) `std` library, (2) external crates, (3) crate/local imports.
- Use `use crate::` for library-internal imports within `src/game/` and `src/solver/`.
- Use `use yew_countdown_solver::` for library imports from the binary crate (`src/components/`, `src/app.rs`, `src/main.rs`).
- Use glob imports sparingly — `yew::prelude::*` is acceptable; avoid elsewhere.
- Conditional test imports use `#[cfg(test)]` on the `use` statement (see `iterative_deepening.rs:1-2`).

### Naming Conventions

- Types: `PascalCase` (e.g., `BoardBuilder`, `SolutionState`, `GameBoardProps`).
- Functions/methods: `snake_case` (e.g., `generate_children`, `is_solved`).
- Constants: `SCREAMING_SNAKE_CASE` for associated consts (e.g., `Board::SMALL_NUMBERS`).
- Yew components: `PascalCase` function name with `#[component]` attribute (e.g., `fn GameBoard`).
- Yew props: struct named `{ComponentName}Props` with `#[derive(Properties, PartialEq)]`.
- Test functions: descriptive `snake_case` names (e.g., `small_numbers_can_be_reused_twice`).

### Type Conventions

- Use `u32` for board numbers, `u32` for targets (validated to 1-999 range).
- Use `Vec<u32>` for board number storage, returned as `&[u32]` via accessors.
- Use `Option<T>` for nullable values; `Result<T, E>` for fallible operations.
- Derive `Debug, Clone, PartialEq, Eq` on domain types. Add `Hash` where needed for `HashSet` usage.
- Use `thiserror::Error` for error enums with `#[error("...")]` display messages.

### Error Handling

- Domain errors use dedicated enums: `BoardError`, `GameError` (both derive `thiserror::Error`).
- Builder methods return `Result<Self, Error>` to allow chaining with `?`.
- `.unwrap()` is acceptable in `Default` implementations and test code, but production paths should propagate errors.
- Tests verify both error variant (`matches!(err, ErrorType::Variant(_))`) and display message (`format!("{}", err)`).

### Yew Component Patterns

- Components use function-style with `#[component]` attribute.
- State management: `use_state` for simple values, `use_reducer` for complex state (see `ManualEntry`).
- Callbacks: clone state handle into a block, then create `Callback::from(move |..| { ... })`.
- Effects: `use_effect_with` for side effects keyed on state changes.
- Props: passed via `#[derive(Properties, PartialEq)]` structs.

### Testing Patterns

- Tests live in `#[cfg(test)] mod tests` at the bottom of each source file.
- Use `use super::*;` in test modules.
- Test helper macros (e.g., `game!` macro in `iterative_deepening.rs`) are defined inside test modules.
- Tests returning `Result<(), Error>` use `?` for builder chains; assertion-only tests return `()`.
- Descriptive assertion messages are required: `assert!(condition, "context message", args)`.

### CSS / Styling

- All styling uses Tailwind CSS v4 utility classes directly in Rust `html!` macro strings.
- Source CSS: `styles/main.css` (imports Tailwind). Output: `styles/output.css` (committed).
- After changing CSS classes in `.rs` files, the Trunk pre-build hook regenerates `output.css` automatically.
- Buttons include focus-visible ring styles for accessibility: `focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2`.
- Use semantic HTML and ARIA attributes (`aria-label`, `aria-busy`, `role`).

### Logging

- Use `tracing` macros (`tracing::info!`, `tracing::debug!`) for runtime logging, not `println!`.
- Tracing outputs to browser console via `tracing-web`.
