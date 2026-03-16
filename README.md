# Countdown Solver

[![CI/CD](https://github.com/newmancodes/yew-countdown-solver/actions/workflows/main.yml/badge.svg)](https://github.com/newmancodes/yew-countdown-solver/actions/workflows/main.yml)

A WebAssembly application that solves the numbers round from the British game show [Countdown](https://en.wikipedia.org/wiki/Countdown_(game_show)). Given 6 numbers and a target (1--999), the solver finds a sequence of arithmetic operations (+, -, *, /) that reaches the target. Built with Rust and the [Yew](https://yew.rs/) framework, compiled to WASM and deployed as a static site.

**Live at:** [cds.newman.digital](https://cds.newman.digital)

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (Edition 2021), compiled to `wasm32-unknown-unknown` |
| UI Framework | [Yew](https://yew.rs/) 0.22 (client-side rendering) |
| Styling | [Tailwind CSS](https://tailwindcss.com/) v4 (utility classes in Rust `html!` macros) |
| Build Tool | [Trunk](https://trunkrs.dev/) (WASM bundler + dev server) |
| Arena Allocator | [`typed-arena`](https://docs.rs/typed-arena) for solver state nodes |
| Error Handling | [`thiserror`](https://docs.rs/thiserror) for domain error types |
| Logging | [`tracing`](https://docs.rs/tracing) + [`tracing-web`](https://docs.rs/tracing-web) (browser console output) |
| Browser APIs | [`web-sys`](https://docs.rs/web-sys), [`gloo-timers`](https://docs.rs/gloo-timers), [`wasm-bindgen-futures`](https://docs.rs/wasm-bindgen-futures) |
| Unit Tests | Rust `#[test]` with inline test modules |
| Benchmarks | [Criterion](https://bheisler.github.io/criterion.rs/) + [`stats_alloc`](https://docs.rs/stats_alloc) for allocation tracking |
| E2E Tests | [Playwright](https://playwright.dev/) + [pytest](https://docs.pytest.org/) (Python 3.13, managed with [uv](https://docs.astral.sh/uv/)) |
| Infrastructure | Azure Static Web Apps via [Bicep](https://learn.microsoft.com/en-us/azure/azure-resource-manager/bicep/overview) |
| CI/CD | GitHub Actions (fmt check, unit tests, WASM build, E2E tests, deploy) |

## Architecture

### Component Tree

```
App (src/app.rs)
│   use_state: Option<Game>
│
├── None → GameProvider (src/components/game_provider.rs)
│   ├── "Random Game"      → Game::default()
│   ├── "Simulate Round"   → custom large/small number split picker
│   └── "Manual Entry"     → ManualEntry (src/components/manual_entry.rs)
│                              use_reducer: number selection + target input
│
└── Some(game) → GameBoard (src/components/game_board.rs)
    ├── Target display, 6 board tiles, action buttons
    ├── Solve → runs IterativeDeepeningSolver synchronously
    ├── Compete → 30-second countdown timer, then auto-solves
    └── Solution display: step-by-step operations with board state
```

The root `App` component holds a single `Option<Game>` state. When `None`, the game setup screen (`GameProvider`) is rendered. Once a game is created, the `GameBoard` takes over. A reset callback clears the state back to `None`.

### Domain Model

**Board** (`src/game/board.rs`) -- Exactly 6 numbers, stored sorted ascending. Valid numbers are the small set (1--10, each usable up to twice) and the large set (25, 50, 75, 100, each usable once). `BoardBuilder` enforces these constraints at construction time. `BoardAdjuster` is used by the solver to create child board states (remove two operands, insert result) without re-validating the full rule set.

**Game** (`src/game/model.rs`) -- Wraps a `Board` and a target (1--999). Implements the `Problem` trait (`is_solved()` returns true if the target already exists on the board).

### Solver

**Algorithm:** Iterative deepening depth-first search (IDDFS), defined in `src/solver/iterative_deepening.rs`.

The solver searches for a sequence of up to 5 arithmetic operations (depth limit 1 through 6) that produces the target number from the 6 board numbers.

```
For each depth_limit in 1..=6:
  1. Allocate a fresh typed_arena::Arena for this iteration
  2. Push initial board state onto a Vec-based stack (DFS frontier)
  3. Pop a candidate:
     a. Check if the target is on the board → reconstruct path and return
     b. Generate all valid single-operation children (every pair, up to 4 ops each)
     c. Filter by depth limit and explored set, push new states
  4. If frontier exhausted, increase depth limit and retry
```

**Key design decisions:**
- **Arena allocation** -- All `StateTraversal` nodes for a given depth iteration are bump-allocated into a `typed_arena::Arena` and freed in bulk when the iteration ends. Benchmarks showed this reduces heap allocations by up to 49% and improves wall-clock time by up to 24% versus the naive deep-clone baseline. See [BENCHMARK_RESULTS.md](BENCHMARK_RESULTS.md) for the full comparison of three allocation strategies (Box, Rc, Arena).
- **Path reconstruction** -- `StateTraversal` nodes form a linked list via borrowed references back to the root. When a solution is found, the path is walked and reversed to produce the instruction sequence.
- **Operation normalization** -- Subtraction and division are always expressed as `larger op smaller`. Subtraction of equal operands is skipped (would produce 0). Division only applies when the result is a whole number.
- **Explored set** -- A `HashSet<Board>` prevents revisiting equivalent board states within a depth iteration.

### Data Flow

```
User selects game mode
        │
        ▼
GameProvider / ManualEntry
        │
        │  Callback<Game>
        ▼
   App (state update)
        │
        ▼
    GameBoard
        │
        │  User clicks "Solve" or "Compete" timer expires
        ▼
IterativeDeepeningSolver::solve()
        │
        │  Option<Solution<Game, Board>>
        ▼
SolutionState::Solved { solution, elapsed_secs }
        │
        ▼
Step-by-step operation list rendered with
intermediate board states (result tile highlighted)
```

The solve runs synchronously on the main thread. A 0ms `setTimeout` (via `gloo-timers`) is used before solving to yield to the browser event loop, allowing the "Solving..." UI state to render before the potentially blocking computation.

## Getting Started

### Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
npm install
```

### Development

```bash
# Local dev server with hot reload
trunk serve

# Build for production
trunk build --release
```

Tailwind CSS is compiled automatically via a Trunk pre-build hook (`npm run build` runs before each build).

### Testing

```bash
# Unit tests
cargo test

# Format check
cargo fmt --check

# Benchmarks
cargo bench
```

### E2E Tests

```bash
# Install dependencies (first time)
cd tests/e2e && uv sync
uv run playwright install chromium --with-deps

# Build and serve
trunk build --release
python3 -m http.server 8080 --directory ./dist &

# Run tests
cd tests/e2e && uv run pytest tests/
```

The E2E suite uses Playwright with a Page Object Model pattern. A `OPTS_DEV_FAST_COMPETE` localStorage flag is injected by the test fixtures to reduce the compete timer from 30 seconds to 2 seconds.

## CI/CD Pipeline

The GitHub Actions workflow (`.github/workflows/main.yml`) runs on push to `main`:

1. `cargo fmt --check` -- formatting gate
2. `cargo test` -- unit tests
3. `trunk build --release` -- WASM build
4. Playwright E2E tests against the built `dist/`
5. Deploy to Azure Static Web Apps via Bicep deployment stack + SWA CLI

## Project Structure

```
src/
├── main.rs                          # WASM entry point, tracing setup
├── app.rs                           # Root component, screen routing
├── components/
│   ├── game_provider.rs             # Game setup (random, custom split, manual)
│   ├── game_board.rs                # Game display, solve/compete, solution rendering
│   └── manual_entry.rs              # Number picker with validation
├── game/
│   ├── board.rs                     # Board type, builder, adjuster, validation
│   └── model.rs                     # Game type, Problem impl
└── solver/
    ├── traits.rs                    # Problem, Solver traits; Operation, Solution types
    └── iterative_deepening.rs       # IDDFS solver with arena allocation

tests/e2e/                           # Playwright/pytest E2E tests
├── pages/page_objects.py            # Page Object Model
├── tests/                           # Test files (game flow, solver, compete, etc.)
└── conftest.py                      # Fixtures

benches/solver_benchmark.rs          # Criterion benchmarks with allocation tracking
deploy/main.bicep                    # Azure Static Web App infrastructure
