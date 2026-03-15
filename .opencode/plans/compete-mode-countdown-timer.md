# Plan: "Compete" Mode with 30-Second Countdown Timer

## Overview

Add a "Compete" button to the `GameBoard` component that initiates a 30-second countdown timer, simulating the Countdown game show experience. When the timer reaches zero, the timer display hides, the normal "Solving..." flow runs, and the solver displays the best solution.

## User Decisions

- **Timer duration**: Fixed at 30 seconds. Slow E2E tests marked with `@pytest.mark.slow`.
- **Timer at zero**: Timer display hides, then shows `Solving...` state (same as clicking Solve directly).
- **Button placement**: Compete button sits alongside Solve and Reset in the same row.

---

## Rust Changes

### 1. Modify `SolutionState` in `src/components/game_board.rs`

Add a `Competing(u8)` variant to the existing enum:

```rust
enum SolutionState {
    NotAttempted,
    Competing(u8),  // seconds remaining: 30 → 0
    Solving,
    Solved(Solution<Game, Board>),
    NotFound,
}
```

### 2. Add timer logic using `gloo_timers::callback::Interval`

Inside the `GameBoard` component, use `use_effect_with` to watch for `Competing(n)` state changes:

- When `Competing(n > 0)`: create a `gloo_timers::callback::Interval` that fires every 1 second. Each tick decrements the counter via `solution_state.set(Competing(n - 1))`.
- When `Competing(0)`: the interval is dropped (cleaned up by the effect), and the solver runs immediately, transitioning to `Solving` → `Solved`/`NotFound`.
- The `Interval` is stored so it gets dropped when the effect re-runs or cleans up.

**Note on `use_effect_with` API**: In Yew 0.22, `use_effect_with` takes `deps` and a closure that returns a destructor. The destructor is called when deps change or on unmount. The above pattern stores the `Interval` in the effect closure and drops it on cleanup — this is the standard approach.

**Implementation detail**: The `Interval` callback needs a reference to the state handle. Since `gloo_timers::callback::Interval::new` takes a `Fn()` closure, we can clone the state handle into the closure. On each tick:

```rust
use gloo_timers::callback::Interval;

let is_competing = matches!(*solution_state, SolutionState::Competing(_));

use_effect_with(is_competing, {
    let solution_state = solution_state.clone();
    let game = game.clone();
    move |is_competing| {
        if !is_competing {
            return || {};  // no cleanup needed
        }
        let interval = Interval::new(1_000, move || {
            let current = (*solution_state).clone();
            if let SolutionState::Competing(n) = current {
                if n > 1 {
                    solution_state.set(SolutionState::Competing(n - 1));
                } else {
                    // Timer reached 0 — trigger solve
                    solution_state.set(SolutionState::Solving);
                    let solver = IterativeDeepeningSolver::new(&game);
                    if let Some(solution) = solver.solve() {
                        solution_state.set(SolutionState::Solved(solution));
                    } else {
                        solution_state.set(SolutionState::NotFound);
                    }
                }
            }
        });
        // Return cleanup closure that drops the interval
        move || drop(interval)
    }
});
```

**Key design consideration**: The effect should be keyed on whether we're in `Competing` state (a boolean), not the seconds value. If keyed on seconds, the interval would be destroyed and recreated every second, which is wasteful. Keying on the boolean means the interval is created once when entering `Competing` mode and dropped when leaving it.

### 3. Add "Compete" button to the UI

Add a third button in the existing action buttons `<div>`:

```rust
<button
    class="bg-orange-500 hover:bg-orange-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-bold py-3 px-8 rounded-lg shadow-md transition-colors duration-200 cursor-pointer flex items-center gap-2"
    onclick={on_compete_click}
    disabled={*solution_state != SolutionState::NotAttempted}
    aria-label="Compete"
>
    <span>{"Compete"}</span>
</button>
```

The `on_compete_click` callback sets `solution_state.set(SolutionState::Competing(30))`.

### 4. Add timer display

When in `Competing` state, show a countdown display between the game board section and the action buttons:

```rust
if let SolutionState::Competing(seconds) = *solution_state {
    html! {
        <div class="w-full max-w-md bg-yellow-100 border-2 border-yellow-500 rounded-lg p-4 text-center">
            <div class="text-yellow-800 font-semibold mb-1">{"Time remaining"}</div>
            <div class="text-5xl font-bold text-yellow-900" aria-label="Time remaining">
                { seconds }
            </div>
        </div>
    }
}
```

When the timer hits 0, the state transitions to `Solving` (no longer `Competing`), so the timer display disappears and the normal "Solving..." button text appears.

### 5. Button state management

| State | Solve | Reset | Compete |
|-------|-------|-------|---------|
| `NotAttempted` | enabled | enabled | enabled |
| `Competing(n)` | disabled | disabled | disabled |
| `Solving` | disabled | disabled | disabled |
| `Solved(_)` | disabled | enabled | disabled |
| `NotFound` | disabled | enabled | disabled |

Updated disable conditions:
- **Solve**: `disabled={*solution_state != SolutionState::NotAttempted}`
- **Reset**: `disabled={matches!(*solution_state, SolutionState::Solving | SolutionState::Competing(_))}`
- **Compete**: `disabled={*solution_state != SolutionState::NotAttempted}`

---

## Playwright Test Changes

### 6. Extend Page Object (`tests/e2e/pages/page_objects.py`)

Add new selectors:

```python
COMPETE_BUTTON = 'button[aria-label="Compete"]'
TIMER_DISPLAY = '[aria-label="Time remaining"]'
```

Add new methods:

```python
def click_compete(self) -> None:
    """Click the Compete button and wait for timer to appear."""
    self.page.click(self.COMPETE_BUTTON)
    self.page.wait_for_selector(self.TIMER_DISPLAY, state="visible")

def get_time_remaining(self) -> int:
    """Extract the time remaining from the timer display."""
    text = self.page.locator(self.TIMER_DISPLAY).inner_text()
    return int(text.strip())

def is_compete_button_disabled(self) -> bool:
    return self.page.locator(self.COMPETE_BUTTON).is_disabled()

def is_compete_button_enabled(self) -> bool:
    return self.page.locator(self.COMPETE_BUTTON).is_enabled()

def is_reset_button_disabled(self) -> bool:
    return self.page.locator(self.RESET_BUTTON).is_disabled()

def is_reset_button_enabled(self) -> bool:
    return self.page.locator(self.RESET_BUTTON).is_enabled()

def wait_for_competition_end(self, timeout: int = 45000) -> None:
    """Wait for the competition to end and solution to appear."""
    self.page.wait_for_selector(
        'text=/Solution found|No solution found/',
        timeout=timeout,
        state="visible"
    )
```

### 7. Create `tests/e2e/tests/test_compete.py`

All tests marked `@pytest.mark.slow` since they involve 30-second waits (except test 1):

#### Test 1: `test_compete_button_visible_on_game_board`
- Generate random game
- Assert compete button is visible and enabled
- Assert compete button becomes disabled after clicking Solve (verify it follows the same disable pattern)

#### Test 2: `test_compete_starts_countdown_timer`
- Generate random game
- Click Compete
- Assert timer display is visible
- Assert timer value is <= 30 and > 0

#### Test 3: `test_compete_disables_solve_and_reset_buttons`
- Generate random game
- Click Compete
- Assert Solve button is disabled
- Assert Reset button is disabled
- Assert Compete button is disabled

#### Test 4: `test_compete_timer_counts_down`
- Generate random game
- Click Compete
- Record initial timer value
- Wait ~3 seconds (`page.wait_for_timeout(3000)`)
- Assert timer value has decreased from initial value

#### Test 5: `test_compete_triggers_solve_after_countdown` (slowest test, ~35s)
- Generate random game
- Click Compete
- Wait for competition end (45s timeout)
- Assert solution message is visible (either success or failure)
- Assert timer display is no longer visible

#### Test 6: `test_compete_button_disabled_after_competition`
- Generate random game
- Click Compete
- Wait for competition end
- Assert Compete button is disabled
- Assert Reset button is enabled

#### Test 7: `test_reset_after_compete_reenables_all_buttons`
- Generate random game
- Click Compete
- Wait for competition end
- Click Reset
- Generate new random game
- Assert Solve button is enabled
- Assert Compete button is enabled
- Assert Reset button is enabled

---

## File Changes Summary

| File | Action | Description |
|------|--------|-------------|
| `src/components/game_board.rs` | **Modify** | Add `Competing(u8)` state variant, timer effect with `gloo_timers::callback::Interval`, Compete button, timer display, updated disable logic |
| `tests/e2e/pages/page_objects.py` | **Modify** | Add compete/timer selectors and helper methods |
| `tests/e2e/tests/test_compete.py` | **Create** | New test file with 7 compete-mode E2E tests |
| `styles/output.css` | **Auto-regenerated** | Tailwind picks up new orange/yellow utility classes on build |

---

## Verification Steps

1. `cargo fmt` — fix formatting
2. `cargo fmt --check` — verify formatting passes
3. `cargo test` — ensure all Rust unit tests still pass
4. `trunk build --release` — ensure WASM build succeeds (also regenerates Tailwind CSS)
5. `trunk serve` — manual smoke test of compete flow
6. Run E2E tests: `cd tests/e2e && uv run pytest tests/test_compete.py -v` against running app
7. Run full E2E suite: `cd tests/e2e && uv run pytest -v` to ensure no regressions

---

## Risks and Considerations

1. **Solver blocks the main thread**: The `IterativeDeepeningSolver::solve()` call is synchronous. During the solve phase (after timer hits 0), the UI will freeze briefly. This is pre-existing behavior from the Solve button — no regression.

2. **`use_effect_with` dependency precision**: The effect is keyed on whether we're in `Competing` state (a boolean), not the seconds value. This means the interval is created once when entering `Competing` mode and dropped when leaving it.

3. **`Interval` vs `Timeout` chain**: Using a single `Interval` is simpler than chaining 30 `Timeout`s. The interval fires every 1000ms and we decrement the counter. The interval is dropped when the effect cleans up (state leaves `Competing`).

4. **TailwindCSS new classes**: The orange (`bg-orange-500`, `hover:bg-orange-700`) and yellow (`bg-yellow-100`, `border-yellow-500`, `text-yellow-800`, `text-yellow-900`) classes will be automatically included when Tailwind scans the Rust source files during the pre-build hook.

5. **E2E test timing**: Tests involving the full 30-second countdown will take ~35 seconds each. These are marked `@pytest.mark.slow` so they can be skipped in quick test runs with `pytest -m "not slow"`.
