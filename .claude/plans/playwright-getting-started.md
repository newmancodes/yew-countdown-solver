# E2E Testing Implementation Plan

## Context

The Yew countdown solver application currently has unit tests and formatting checks in CI, but lacks end-to-end testing to verify the complete user journey through the browser. The user has started a Python project at `./tests/e2e` with uv for dependency management and wants Playwright tests that validate:

1. Users can generate a random game
2. The game board displays correctly with target and 6 numbers
3. Users can request a solution (and see success/failure message)
4. Users can reset back to the initial screen

The CI pipeline already installs Python 3.13, uv, and Playwright browsers, but doesn't execute the tests. This plan adds comprehensive e2e tests without modifying any Rust code, integrates them into the CI/CD workflow, and provides local testing guidance.

## Implementation Overview

**Approach:** Use Playwright with pytest (Python), following the Page Object Model pattern for maintainability. Tests will use aria-label selectors for robustness and accessibility validation. CI will serve the built WASM app via Python's HTTP server, run tests, and upload failure screenshots.

## Critical Files

- `/home/steve/Projects/yew-countdown-solver/tests/e2e/pyproject.toml` - Dependencies already configured
- `/home/steve/Projects/yew-countdown-solver/.github/workflows/main.yml` - CI pipeline to modify
- `/home/steve/Projects/yew-countdown-solver/src/components/game_provider.rs` - UI reference for selectors
- `/home/steve/Projects/yew-countdown-solver/src/components/game_board.rs` - UI reference for selectors

## File Structure to Create

```
tests/e2e/
├── pytest.ini                  # pytest configuration
├── conftest.py                 # pytest fixtures and helpers
├── tests/                      # test directory
│   ├── __init__.py
│   ├── test_game_flow.py       # Core game flow tests
│   └── test_solver.py          # Solver functionality tests
├── fixtures/                   # Page Object Model
│   ├── __init__.py
│   └── page_objects.py         # AppPage class with helper methods
└── screenshots/                # Auto-created on test failures
```

## Detailed Implementation Steps

### 1. Create pytest.ini

**File:** `tests/e2e/pytest.ini`

Configure pytest with test discovery and output settings. Enable async mode for Playwright-pytest integration.

**Key configuration:**
- Test discovery: `tests/` directory, `test_*.py` files
- Log output to console for debugging
- Verbose output with short tracebacks

### 2. Create conftest.py with Fixtures

**File:** `tests/e2e/conftest.py`

Implement pytest fixtures for browser management and automatic screenshot capture on failure:

- `playwright` fixture (session scope): Playwright instance
- `browser` fixture (session scope): Chromium browser with configurable headless mode
- `context` fixture (function scope): New browser context per test
- `page` fixture (function scope): New page per test with auto-screenshot on failure
- `app_page` fixture: Returns AppPage helper object

**Environment variables:**
- `BASE_URL`: App URL (default: http://localhost:8080)
- `HEADLESS`: Run headless (default: true)

**Screenshot on failure:** Hook into pytest's test result reporting to capture screenshots when tests fail, saving to `screenshots/` directory.

### 3. Create Page Object Model

**File:** `tests/e2e/fixtures/page_objects.py`

Create `AppPage` class with methods for interacting with the app:

**Selectors (using aria-labels):**
- `RANDOM_GAME_BUTTON = 'button[aria-label="Generate random game"]'`
- `TARGET_NUMBER = '[aria-label="Target number"]'`
- `AVAILABLE_NUMBERS = '[aria-label="Available numbers"]'`
- `SOLVE_BUTTON = 'button[aria-label="Solve game"]'`
- `RESET_BUTTON = 'button[aria-label="Reset game"]'`

**Key methods:**
- `is_on_provider_screen()`: Check if on game selection screen
- `is_on_game_board()`: Check if on game board screen
- `click_random_game()`: Click Random Game button and wait for board
- `click_solve()`: Click Solve button
- `click_reset()`: Click Reset and wait for provider screen
- `get_target_number()`: Extract target number as int
- `get_available_numbers()`: Extract list of 6 numbers
- `get_solution_message()`: Get success/failure message text
- `has_success_message()`: Check for "✓ Solution found in N steps!"
- `has_failure_message()`: Check for "✗ No solution found"
- `validate_game_board()`: Assert all required elements are visible

### 4. Create Test Files

#### test_game_flow.py

**Core user journey tests:**

1. `test_app_loads_successfully`
   - Verify app loads and shows provider screen
   - Check "Choose Numbers Round Setup" heading visible

2. `test_random_game_generation`
   - Click "Random Game" button
   - Verify navigation to game board
   - Validate target is 1-1000
   - Validate exactly 6 numbers are displayed

3. `test_reset_button_returns_to_provider`
   - Generate a game
   - Click Reset button
   - Verify return to provider screen

4. `test_multiple_random_games`
   - Generate 3 games sequentially
   - Verify each game is created successfully

#### test_solver.py

**Solver functionality tests:**

1. `test_solve_button_triggers_solver`
   - Generate game
   - Click Solve button
   - Wait for solution result (15s timeout)
   - Verify message contains "Solution found" or "No solution found"

2. `test_solve_button_disabled_after_solving`
   - Verify Solve button initially enabled
   - Click Solve and wait for result
   - Verify Solve button now disabled

3. `test_solution_shows_step_count`
   - Try up to 10 random games to find a solvable one
   - Extract step count from success message
   - Verify step count ≥ 2 (minimum valid steps)

4. `test_solver_handles_multiple_games`
   - Run 5 solve operations sequentially
   - Verify each produces a result

### 5. Create __init__.py Files

Empty `__init__.py` files in:
- `tests/e2e/tests/__init__.py`
- `tests/e2e/fixtures/__init__.py`

### 6. Update CI/CD Pipeline

**File:** `.github/workflows/main.yml`

**Insert after line 68** (after "Build Application" step):

```yaml
      - name: Start WASM Application Server
        run: |
          python3 -m http.server 8080 --directory ./dist &
          echo $! > .server.pid
          timeout 30 bash -c 'until curl -s http://localhost:8080 > /dev/null; do sleep 1; done'
          echo "Server started on http://localhost:8080"

      - name: Run E2E Tests
        working-directory: tests/e2e
        env:
          BASE_URL: http://localhost:8080
          HEADLESS: true
        run: |
          uv run pytest tests/ \
            --verbose \
            --tb=short

      - name: Upload E2E Screenshots
        if: failure()
        uses: actions/upload-artifact@v4
        with:
          name: e2e-screenshots
          path: tests/e2e/screenshots/
          retention-days: 7

      - name: Stop WASM Application Server
        if: always()
        run: |
          if [ -f .server.pid ]; then
            kill $(cat .server.pid) || true
            rm .server.pid
          fi
```

**Why Python HTTP server:** Fast startup, no build tools needed, matches production build environment. The built artifacts in `dist/` are already production-ready from `trunk build --release`.

**New CI flow:**
1. Setup (existing)
2. Quality checks (existing)
3. Build application (existing)
4. **Start HTTP server** (new)
5. **Run e2e tests** (new)
6. **Upload screenshots if tests fail** (new)
7. **Stop server** (new)
8. Deploy to Azure (existing)

### 7. Delete Placeholder File

Remove `tests/e2e/main.py` as it's no longer needed (just contains "Hello from e2e!").

## Local Testing Instructions

### Initial Setup

```bash
# Navigate to e2e directory
cd tests/e2e

# Install dependencies (if not done)
uv sync

# Install Playwright browsers
uv run playwright install chromium --with-deps
```

### Running Tests

**Option 1: Using trunk serve (recommended for development)**

```bash
# Terminal 1: Start development server
trunk serve --port 8080

# Terminal 2: Run tests
cd tests/e2e
uv run pytest tests/ --verbose
```

**Option 2: Using built dist/ (matches CI exactly)**

```bash
# Terminal 1: Build and serve
trunk build --release
python3 -m http.server 8080 --directory ./dist

# Terminal 2: Run tests
cd tests/e2e
BASE_URL=http://localhost:8080 uv run pytest tests/
```

### Useful Test Commands

```bash
# Run in headed mode (see browser)
HEADLESS=false uv run pytest tests/ --verbose

# Run specific test file
uv run pytest tests/test_game_flow.py

# Run specific test
uv run pytest tests/test_game_flow.py::test_random_game_generation

# Verbose output with no capture (see all logs)
uv run pytest tests/ --verbose --capture=no
```

### Debugging Failed Tests

- Screenshots saved to `tests/e2e/screenshots/` on failure
- Add `page.pause()` in any test to open Playwright Inspector
- Use `HEADLESS=false` to watch tests run in browser
- Check console logs with `page.on("console", lambda msg: print(msg.text()))`

## Verification Steps

After implementation, verify:

1. **Local testing works:**
   ```bash
   trunk serve --port 8080
   cd tests/e2e && uv run pytest tests/
   ```
   - All tests should pass
   - Should see "Random Game" test generate and validate a game
   - Should see Solve and Reset tests complete

2. **CI integration works:**
   - Commit and push changes to main branch
   - GitHub Actions should run e2e tests after build step
   - Check workflow logs for "Run E2E Tests" step
   - Verify tests pass or screenshots are uploaded on failure

3. **Test coverage is complete:**
   - Random game generation ✓
   - Game board displays correctly ✓
   - Solve button triggers solver ✓
   - Solution message appears ✓
   - Reset returns to provider ✓

## Key Selectors Reference

| Element | Selector | Location |
|---------|----------|----------|
| Random Game button | `button[aria-label="Generate random game"]` | game_provider.rs:55 |
| Target number | `[aria-label="Target number"]` | game_board.rs:67 |
| Available numbers | `[aria-label="Available numbers"]` | game_board.rs:72 |
| Solve button | `button[aria-label="Solve game"]` | game_board.rs:85 |
| Reset button | `button[aria-label="Reset game"]` | game_board.rs:97 |
| Success message | `text=/Solution found in \\d+ steps!/` | game_board.rs:110 |
| Failure message | `text=No solution found` | game_board.rs:117 |

## Notes

- **No Rust code changes required:** All selectors already exist with proper aria-labels
- **Playwright vs Selenium:** Playwright chosen for speed, modern API, better WASM support
- **pytest vs Playwright Test Runner:** Using pytest for Python ecosystem familiarity and better fixture support
- **Future enhancements:** Could add visual regression testing, cross-browser testing (Firefox/WebKit), or mobile viewport testing
