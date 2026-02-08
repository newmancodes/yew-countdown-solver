"""Solver functionality tests for the Countdown Solver application."""

import os

import pytest
from pages.page_objects import AppPage


# Get solver timeout from environment (default: 30 seconds)
SOLVER_TIMEOUT = int(os.getenv("SOLVER_TIMEOUT", "30000"))


def test_solve_button_triggers_solver(app_page: AppPage):
    """Test that clicking Solve triggers the solver and produces a result."""
    # Generate a game
    app_page.click_random_game()
    assert app_page.is_on_game_board(), "Should be on game board"

    # Click Solve button
    app_page.click_solve(timeout=SOLVER_TIMEOUT)

    # Verify a solution message appears (either success or failure)
    message = app_page.get_solution_message()
    assert "Solution found" in message or "No solution found" in message, \
        f"Should show a solution result, got: {message}"


def test_solve_button_disabled_after_solving(app_page: AppPage):
    """Test that the Solve button is disabled after solving."""
    # Generate a game
    app_page.click_random_game()

    # Verify Solve button is initially enabled
    assert app_page.is_solve_button_enabled(), "Solve button should be enabled initially"

    # Click Solve and wait for result
    app_page.click_solve(timeout=SOLVER_TIMEOUT)
    app_page.get_solution_message()  # Wait for message to appear

    # Verify Solve button is now disabled
    assert app_page.is_solve_button_disabled(), \
        "Solve button should be disabled after solving"


def test_solution_shows_instruction_count(app_page: AppPage):
    """Test that successful solutions show a valid instruction count."""
    # Try up to 10 random games to find a solvable one
    for attempt in range(10):
        app_page.click_random_game()

        # Solve the game
        app_page.click_solve(timeout=SOLVER_TIMEOUT)

        # Check if we got a success message
        if app_page.has_success_message():
            # Extract and validate instruction count
            instruction_count = app_page.get_instruction_count()
            assert instruction_count is not None, "Should have a instruction count"
            assert instruction_count >= 1, f"Step count should be at least 1, got {instruction_count}"

            # Success! Test passed
            return

        # Reset and try again
        app_page.click_reset()

    # If we get here, none of the 10 games were solvable
    pytest.skip("Could not find a solvable game in 10 attempts")


def test_solver_handles_multiple_games(app_page: AppPage):
    """Test that the solver can handle multiple solve operations."""
    results = []

    for i in range(5):
        # Generate a new game
        app_page.click_random_game()

        # Solve the game
        app_page.click_solve(timeout=SOLVER_TIMEOUT)

        # Capture result
        if app_page.has_success_message():
            instruction_count = app_page.get_instruction_count()
            results.append(("success", instruction_count))
        elif app_page.has_failure_message():
            results.append(("failure", None))
        else:
            pytest.fail(f"Game {i+1} did not produce a valid result")

        # Reset for next iteration
        app_page.click_reset()

    # Verify all 5 operations completed
    assert len(results) == 5, f"Should have 5 results, got {len(results)}"

    # Log results for debugging
    success_count = sum(1 for r in results if r[0] == "success")
    print(f"\nSolver results: {success_count}/5 successful")
    print(f"Step counts: {[r[1] for r in results if r[1] is not None]}")


@pytest.mark.slow
def test_reset_after_solving_reenables_solve(app_page: AppPage):
    """Test that resetting after solving re-enables the Solve button."""
    # Generate and solve a game
    app_page.click_random_game()
    app_page.click_solve(timeout=SOLVER_TIMEOUT)
    app_page.get_solution_message()

    # Verify Solve button is disabled
    assert app_page.is_solve_button_disabled(), \
        "Solve button should be disabled after solving"

    # Reset to provider screen
    app_page.click_reset()
    assert app_page.is_on_provider_screen(), "Should be on provider screen"

    # Generate a new game
    app_page.click_random_game()

    # Verify Solve button is enabled again
    assert app_page.is_solve_button_enabled(), \
        "Solve button should be enabled for new game"
