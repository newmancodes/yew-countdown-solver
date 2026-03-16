# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust/Yew WebAssembly application that solves the numbers round from the British game show "Countdown". Users input 6 numbers and a target (1-999), and the solver finds solutions using basic arithmetic operations (+, -, *, /).

## Build & Development Commands

```bash
# Run all unit tests
cargo test

# Run a single test by name
cargo test test_name

# Check formatting (CI enforces this)
cargo fmt --check

# Fix formatting
cargo fmt

# Build WASM (requires trunk and wasm32 target)
trunk build --release

# Local development server
trunk serve
```

**Note:** Tailwind CSS is automatically compiled via Trunk pre-build hook — `npm run build` runs before each `trunk build` or `trunk serve`. The source CSS is `styles/main.css`; `styles/output.css` is generated and committed.

### E2E Tests (Playwright/pytest)

```bash
# Install dependencies (first time)
cd tests/e2e && uv sync
uv run playwright install chromium --with-deps

# Run all E2E tests (requires a running server on port 8080)
cd tests/e2e && uv run pytest tests/

# Run a single E2E test file
cd tests/e2e && uv run pytest tests/test_solver.py
```

Start the server for E2E tests with: `python3 -m http.server 8080 --directory ./dist`

### Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
npm install
```

## Architecture

### UI Flow

The app has two screens managed by `App` (`src/app.rs`) via a single `use_state`:

1. **`GameProvider`** (`src/components/game_provider.rs`) — shown when no game is active. Presents buttons to start a game (Random, Custom Split, Manual Entry). Only Random Game is currently wired up; it calls `Game::default()` and emits the game upward.

2. **`GameBoard`** (`src/components/game_board.rs`) — shown once a game is active. Displays the target and 6 board tiles, a Solve button, and a Reset button. Solving is synchronous and updates a `SolutionState` enum (`NotAttempted` → `Solving` → `Solved(solution)` / `NotFound`). The solution is rendered as a step-by-step list with the resulting board tiles shown after each operation, the newly produced number highlighted in green.

### Domain Model

**`Board`** (`src/game/board.rs`)
- Exactly 6 numbers, stored sorted ascending as `Vec<u32>`
- Valid numbers: 1–10 (each up to twice), 25/50/75/100 (each once)
- `BoardBuilder` validates rules on construction; `BoardAdjuster` is used by the solver to produce child board states (remove two operands, add result) without re-validating rules.

**`Game`** (`src/game/game.rs`)
- Wraps a `Board` + target (`u16`, valid range 1–999)
- Implements `Problem`; `is_solved()` returns true if the target is already on the board

### Solver Architecture

**`solver/solver.rs`** defines the core traits and types:
- `Problem` trait: `is_solved() -> bool`
- `Solver<P, S>` trait: `solve() -> Option<Solution<P, S>>`
- `Solution<P, S>`: holds the original `Problem` and a `Vec<Instruction<S>>`
- `Instruction<S>`: holds a board `state: S` and `operation: Option<Operation>`. The first instruction always has `operation = None` (initial board state); subsequent instructions each carry the `Operation` that produced that state.
- `Operation`: `{ left, operator, right, result }` — always expressed as larger op smaller for subtraction/division

**`IterativeDeepeningSolver`** (`src/solver/iterative_deepening.rs`)
- Iterative deepening DFS over board states, depth limit 1–6 (max 6 operations)
- `generate_children(board)` returns `Vec<(Board, Operation)>` — all valid single-operation successors
- Uses a `HashSet<Board>` for the explored set and a `Vec<StateTraversal>` frontier; `StateTraversal` forms a linked list back to the root for path reconstruction
- Subtraction skips equal operands (result would be 0); division only applies when the result is a whole number

### CSS

`styles/main.css` is the Tailwind source. `styles/output.css` is compiled and committed — update CSS classes in `.rs` files and let the Trunk pre-build hook regenerate it, or run `npm run build` manually.

## CI Pipeline

Runs on push to main: `cargo fmt --check` → `cargo test` → `trunk build --release` → E2E tests against the built dist → deploy to Azure Static Web Apps via Bicep deployment stack.
