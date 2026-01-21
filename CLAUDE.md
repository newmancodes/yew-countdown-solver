# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust/Yew WebAssembly application that solves the numbers round from the British game show "Countdown". Users input 6 numbers and a target (1-1000), and the solver finds solutions using basic arithmetic operations (+, -, *, /).

## Build & Development Commands

```bash
# Run tests
cargo test

# Check formatting (CI enforces this)
cargo fmt --check

# Fix formatting
cargo fmt

# Build WASM (requires trunk and wasm32 target)
trunk build --release

# Local development server
trunk serve

# Compile Tailwind CSS
npm run build

# Watch Tailwind CSS for changes
npm run watch
```

**Note:** Tailwind CSS is automatically compiled via Trunk pre-build hook. The `npm run build` command runs automatically before each `trunk build` or `trunk serve` operation.

### Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
npm install
```

## Architecture

### Module Structure

```
src/
├── main.rs          # Entry point, tracing setup
├── app.rs           # Root Yew component
├── game.rs          # Re-exports game module
│   ├── game/game.rs   # Game struct (board + target), implements Solvable
│   └── game/board.rs  # Board struct (6 numbers), BoardBuilder, validation
└── solver.rs        # Re-exports solver module
    ├── solver/solver.rs              # Solvable trait, Solver trait, Solution struct
    └── solver/iterative_deepening.rs # IterativeDeepeningSolver (WIP)
```

### Key Concepts

**Game Rules (Board)**
- Board contains exactly 6 numbers
- Small numbers (1-10): each can appear up to twice
- Large numbers (25, 50, 75, 100): each can appear at most once
- Numbers are stored sorted in ascending order

**Solver Architecture**
- `Solvable` trait: defines `is_solved()` method
- `Solver<S>` trait: generic over any `Solvable` state, returns `Option<Solution<S>>`
- `Solution`: holds initial state reference and step count
- `IterativeDeepeningSolver`: current solver implementation (incomplete - only handles trivial cases where target is already on the board)

**Stack**
- Yew 0.22 with CSR (client-side rendering)
- TailwindCSS 4.x for styling
- tracing/tracing-web for browser console logging
- Deployed to Azure Static Web Apps

## CI Pipeline

The GitHub Actions workflow runs on push/PR to main:
1. `cargo fmt --check`
2. `cargo test`
3. `trunk build --release`
