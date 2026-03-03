# Plan: Surface Operation Details in Solution Steps

## Context

The solver correctly finds solutions and records each intermediate board state as a sequence of `Instruction<Board>` values. However, `Instruction<S>` currently only holds the resulting board state — it has no knowledge of *which two numbers were combined* or *which operation was applied* to produce that state. The UI therefore cannot display anything more meaningful than a raw board snapshot. The placeholder text `"left operand operation right operand = {:?}"` in `game_board.rs:106` makes this gap explicit.

The goal is to attach per-step operation metadata (`6 × 7 = 42`) to each `Instruction` so the UI can display a proper step-by-step solution.

---

## Approach

Add an `Operation` value (left operand, operator, right operand, result) to each non-initial `Instruction`, thread it through the solver, and render it in the UI.

### Three files change

| File | Change |
|---|---|
| `src/solver/solver.rs` | Add `Operator` enum + `Operation` struct; extend `Instruction<S>` with `Option<Operation>` |
| `src/solver/iterative_deepening.rs` | `generate_children` returns `Vec<(Board, Operation)>`; `StateTraversal` carries the operation that produced it; build `Instruction::with_operation` |
| `src/components/game_board.rs` | Skip the initial instruction and render each operation string |

---

## Step-by-step Implementation

### 1. `src/solver/solver.rs` — New types + extend `Instruction`

Add **before** the existing `Solution` struct:

```rust
#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Operation {
    pub left: u32,
    pub operator: Operator,
    pub right: u32,
    pub result: u32,
}
```

Extend `Instruction<S>`:

```rust
pub struct Instruction<S> {
    state: S,
    operation: Option<Operation>,  // None for the initial board state
}

impl<S> Instruction<S> {
    pub fn new(state: S) -> Self {
        Self { state, operation: None }
    }

    pub fn with_operation(state: S, operation: Operation) -> Self {
        Self { state, operation: Some(operation) }
    }

    pub fn state(&self) -> &S { &self.state }

    pub fn operation(&self) -> Option<&Operation> { self.operation.as_ref() }
}
```

### 2. `src/solver/iterative_deepening.rs` — Thread operations through

**`generate_children` signature change:**

```rust
fn generate_children(board: &Board) -> Vec<(Board, Operation)>
```

Each arm already knows both operands, the operator, and the result — record them in an `Operation` and return `(board, operation)` pairs. For subtraction and division, record the operands in the order that gives a positive/integer result (larger ÷/− smaller).

**`StateTraversal<S>` gains an `operation` field:**

```rust
struct StateTraversal<S> {
    previous_state: Option<Box<StateTraversal<S>>>,
    state: S,
    operation: Option<Operation>,   // operation that produced `state`; None for root
}
```

Constructors:
- `initial_state`: `operation: None`
- `intermediate_state` / `final_state`: accept `Operation` and store it

**Building instructions in `solve()`:**

When reconstructing the path from the linked list, pass the stored `operation` to `Instruction::with_operation`. The initial-state instruction is created with `Instruction::new` (no operation) exactly as today. The already-solved trivial branch needs no change.

### 3. `src/components/game_board.rs` — Render operation strings

Replace the `for solution.instructions().iter().map(...)` loop body:

```rust
{ for solution.instructions().iter().filter_map(|instruction| {
    instruction.operation().map(|op| {
        let symbol = match op.operator {
            Operator::Add      => "+",
            Operator::Subtract => "−",
            Operator::Multiply => "×",
            Operator::Divide   => "÷",
        };
        html! {
            <li class="p-2 bg-white rounded border border-green-300" role="listitem">
                <code class="text-gray-800">
                    { format!("{} {} {} = {}", op.left, symbol, op.right, op.result) }
                </code>
            </li>
        }
    })
})}
```

`filter_map` naturally skips the initial instruction (whose `operation` is `None`) without needing an explicit `skip(1)`.

Add the necessary import in `game_board.rs`:
```rust
use crate::solver::solver::{Operator, Solver};
```

---

## Verification

```bash
# Unit tests (all existing solver tests must still pass)
cargo test

# Check formatting
cargo fmt --check

# Visual check: trunk serve, click "Solve" and expand "View solution"
# Each step should now read e.g. "6 × 7 = 42" instead of the placeholder
trunk serve
```

The existing test `solvable_games_are_solved_in_the_expected_number_of_operations` validates correct solution depth; no new tests are strictly required, but a test asserting that `instruction.operation()` is `Some` for all but the first instruction would be a nice addition.
