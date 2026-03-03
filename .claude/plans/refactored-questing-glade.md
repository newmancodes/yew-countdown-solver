# Plan: Custom Split Game Mode

## Context

The Custom Split button in `game_provider.rs` is currently a non-functional placeholder. This plan wires it up so users can choose how many large numbers (0–4) they want; the remaining slots are automatically filled with small numbers (totalling 6). The domain function `Board::random_with_number_mix_specified` already exists and handles this — we just need the UI flow and tests.

---

## Files to Modify

| File | Change |
|------|--------|
| `src/components/game_provider.rs` | Add custom split sub-view with state |
| `tests/e2e/pages/page_objects.py` | Add selectors and helper methods |
| `tests/e2e/tests/test_custom_split.py` | New E2E test file |

---

## Rust Implementation (`src/components/game_provider.rs`)

### State

Add two `use_state` hooks:
- `show_custom_split: bool` — toggles between the main buttons view and the custom split sub-view
- `selected_large_count: u8` — tracks which option (0–4) the user has picked; defaults to `0`

### Callbacks

- **Custom Split button `onclick`** — sets `show_custom_split` to `true`
- **Back button `onclick`** — sets `show_custom_split` to `false`
- **Each option button `onclick`** (one per large count 0–4) — sets `selected_large_count`
- **Generate button `onclick`** — computes `small_count = 6 - selected_large_count`, calls
  `Board::random_with_number_mix_specified(small_count, large_count)`, generates a random target
  using `rand::rng().random_range(1..=999)`, calls `Game::new(board, target).unwrap()`, emits via `on_game_specified`

### Template (conditional render)

```
if *show_custom_split {
    <custom-split-view>
        heading: "Choose Your Number Split"
        5 option buttons (0–4 large), each showing "N large / M small"
            - highlighted when selected_large_count matches
            - aria-label="Select N large number(s)"
        <Generate button> aria-label="Generate custom split game"
        <Back button>     aria-label="Back to game options"
    </custom-split-view>
} else {
    // existing 3-button layout (unchanged)
}
```

The option buttons that match `*selected_large_count` get a darker/distinct background (e.g. `bg-blue-700`) to show selection; unselected ones use `bg-blue-500 hover:bg-blue-700`.

### Imports to add

```rust
use crate::game::board::Board;
use rand::Rng;
```

---

## E2E Page Object (`tests/e2e/pages/page_objects.py`)

### New selectors

```python
CUSTOM_SPLIT_BUTTON     = 'button[aria-label="Create game with number constraints"]'
CUSTOM_SPLIT_SETUP      = '[aria-label="Custom split setup"]'
GENERATE_CUSTOM_BUTTON  = 'button[aria-label="Generate custom split game"]'
BACK_TO_OPTIONS_BUTTON  = 'button[aria-label="Back to game options"]'
```

`select_large_number_button(n)` builds `button[aria-label="Select {n} large number(s)"]` dynamically.

### New methods

- `click_custom_split()` — clicks `CUSTOM_SPLIT_BUTTON`, waits for `CUSTOM_SPLIT_SETUP` visible
- `select_large_count(n: int)` — clicks the option button for N large numbers
- `click_generate_custom_game()` — clicks generate button, waits for `GAME_BOARD` visible
- `click_back_to_options()` — clicks Back, waits for `PROVIDER_HEADING` visible
- `is_on_custom_split_screen()` — returns bool (locator visible check)
- Static helper: `large_numbers_in(numbers: List[int]) -> List[int]` — filters to {25, 50, 75, 100}

---

## E2E Tests (`tests/e2e/tests/test_custom_split.py`)

1. **`test_custom_split_shows_selector`**
   — Click Custom Split → assert `is_on_custom_split_screen()` — verifies navigation

2. **`test_back_button_returns_to_provider`**
   — Click Custom Split → click Back → assert `is_on_provider_screen()` — verifies Back works

3. **`test_generates_game_with_no_large_numbers`**
   — Select 0 large → Generate → get numbers → assert `large_numbers_in(numbers) == []` and 6 numbers all 1–10

4. **`test_generates_game_with_four_large_numbers`**
   — Select 4 large → Generate → get numbers → assert exactly 4 of {25,50,75,100} present

5. **`test_each_split_produces_correct_mix`** (parametrised 0–4)
   — For each `large_count`: Custom Split → select N → Generate → verify board → reset → repeat
   — Checks `len(large_numbers_in(numbers)) == large_count` for each case

---

## Verification

1. `cargo fmt` — no formatting issues
2. `cargo test` — all existing tests still pass (no Rust unit changes)
3. `trunk build --release` — WASM builds clean
4. Manual smoke test via `trunk serve`:
   - Click Custom Split → verify 5 options appear
   - Select each (0–4) in turn → generate → confirm board shows correct mix
   - Back button returns to provider
5. E2E: `cd tests/e2e && BASE_URL=http://localhost:8080 uv run pytest tests/test_custom_split.py --verbose`
