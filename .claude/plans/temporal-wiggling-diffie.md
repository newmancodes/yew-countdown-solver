# Plan: Manual Entry Game Mode

## Context

The Manual Entry mode allows users to specify a complete game configuration — all 6 board numbers and the target — before starting. This supports solving a puzzle while the show is still airing. The "Manual Entry" button in `game_provider.rs` already exists with styling but has no `onclick` handler. We need to wire it up and implement the full entry screen.

## Files to Create

1. `src/components/manual_entry.rs` — New `ManualEntry` component
2. `tests/e2e/tests/test_manual_entry.py` — E2E tests for the manual entry flow

## Files to Modify

1. `src/components.rs` — Add `pub mod manual_entry;` and `pub use manual_entry::ManualEntry;`
2. `src/components/game_provider.rs` — Add `show_manual_entry` state, wire up the button, render `<ManualEntry>` in new `else if` branch
3. `tests/e2e/pages/page_objects.py` — Add Manual Entry selectors and helper methods

---

## Implementation Details

### `src/components/manual_entry.rs`

**Imports:** `yew::prelude::*`, `web_sys::HtmlInputElement`, `crate::game::board::{Board, BoardBuilder}`, `crate::game::game::Game`

**State struct** (derives `Clone`, `Default`):
```rust
struct ManualEntryState {
    selected: Vec<u8>,  // up to 6 numbers, in order added
    target_str: String, // raw text of target input
}
```

**Derived values per render:**
- `can_add(n: u8) -> bool`: `selected.len() < 6 && count(n) < (if n <= 10 { 2 } else { 1 })`
- `board_result: Result<Board, BoardError>`: iterate `selected`, call `BoardBuilder::add_number` for each, then `.build()`
- `target: Option<u16>`: parse `target_str`, filter to `1..=999`
- `can_confirm: bool`: `board_result.is_ok() && target.is_some()`

**UI structure** (all in a top-level `div[aria-label="Manual entry setup"]`):
1. Heading: `"Manual Entry"`
2. Target input: `input[type="number", min="1", max="999", aria-label="Target input"]` with `oninput` callback
3. **Large number buttons** (25, 50, 75, 100): `button[aria-label="number {n}"]`, disabled when `!can_add(n)`
4. **Small number buttons** (1–10): same aria-label pattern, disabled when `!can_add(n)`
5. **Selected slots** (6 positions, using index loop `0..6`):
   - Filled: `button[aria-label="remove {n}"]` — styled as a tile; click removes `selected[i]`
   - Empty: decorative placeholder `div`
6. Status text: `"Select N more number(s)"` when fewer than 6 chosen
7. `button[aria-label="Back to game options"]` (reuses existing page object selector)
8. `button[aria-label="Confirm game"]`, `disabled={!can_confirm}`

**Callback patterns** (standard Yew `use_state` clone-and-set):
- **Add number**: clone state, `push(n)`, set
- **Remove at index `i`**: clone state, `remove(i)`, set
- **Target input**: `e.target_unchecked_into::<HtmlInputElement>()`, set `target_str = input.value()`
- **Confirm**: re-build board + game from state values, call `on_game_specified.emit(game)`

Use `#[component]` macro (matching existing component style).

**Props:**
```rust
#[derive(Properties, PartialEq)]
pub struct ManualEntryProps {
    pub on_game_specified: Callback<Game>,
    pub on_back: Callback<MouseEvent>,
}
```

### `src/components.rs` changes

Add two lines alongside the existing entries:
```rust
pub mod manual_entry;
pub use manual_entry::ManualEntry;
```

### `src/components/game_provider.rs` changes

- Add `let show_manual_entry = use_state(|| false);`
- Add `open_manual_entry` / `close_manual_entry` callbacks (same pattern as `open_custom_split` / `close_custom_split`)
- Wire the Manual Entry button: `onclick={open_manual_entry}`
- Replace the `if *show_custom_split { ... } else { ... }` with:
  ```
  if *show_custom_split { ... }
  else if *show_manual_entry {
      <ManualEntry
          on_game_specified={props.on_game_specified.clone()}
          on_back={close_manual_entry}
      />
  }
  else { ... main menu buttons ... }
  ```

### `tests/e2e/pages/page_objects.py` additions

New class-level selectors:
```python
MANUAL_ENTRY_BUTTON = 'button[aria-label="Specify complete game setup"]'
MANUAL_ENTRY_SETUP  = '[aria-label="Manual entry setup"]'
TARGET_INPUT        = 'input[aria-label="Target input"]'
CONFIRM_GAME_BUTTON = 'button[aria-label="Confirm game"]'
```

New methods:
- `click_manual_entry()` — clicks button, waits for `MANUAL_ENTRY_SETUP` visible
- `is_on_manual_entry_screen() -> bool`
- `click_number(n: int)` — clicks `button[aria-label="number {n}"]`
- `remove_selected_number(n: int)` — clicks `button[aria-label="remove {n}"]`
- `set_target(value: int)` — fills target input with `str(value)`
- `click_confirm_game()` — clicks Confirm, waits for `GAME_BOARD` visible
- `is_confirm_disabled() -> bool`
- `is_confirm_enabled() -> bool`
- `select_numbers(numbers: List[int])` — helper: calls `click_number` for each

### `tests/e2e/tests/test_manual_entry.py`

Tests to implement:
1. `test_manual_entry_shows_entry_screen` — click Manual Entry → setup screen visible
2. `test_back_button_returns_to_provider` — navigate in, click Back → provider visible
3. `test_confirm_disabled_initially` — Confirm starts disabled
4. `test_can_select_numbers` — clicking numbers makes them appear as remove-buttons in selected area
5. `test_can_remove_selected_number` — select a number, click remove → no longer in selected area
6. `test_large_number_disabled_after_selection` — click 25 → 25 button is disabled
7. `test_small_number_disabled_after_two_uses` — click 5 twice → 5 button is disabled
8. `test_confirm_enabled_after_valid_selection` — 6 numbers + valid target → Confirm enabled
9. `test_valid_manual_entry_creates_game` — enter known numbers (e.g. 1,2,3,4,5,6) + target (e.g. 21) → Confirm → game board shows that target and those numbers
10. `test_reset_after_manual_entry_returns_to_provider` — full flow → confirm → reset → provider

---

## Verification

1. `cargo test` — no regressions in Rust unit tests
2. `trunk build --release` — WASM compiles cleanly (checks imports, types, Yew HTML macros)
3. Manual smoke test: launch app, navigate Manual Entry, select 6 numbers, type target, confirm → game board; solve → solution shown
4. E2E: `cd tests/e2e && BASE_URL=http://localhost:8080 uv run pytest tests/test_manual_entry.py --verbose`
