"""Core game flow tests for the Countdown Solver application."""

import pytest
from pages.page_objects import AppPage


def test_app_loads_successfully(app_page: AppPage):
    """Verify the app loads and shows the provider screen."""
    assert app_page.is_on_provider_screen(), "App should load on provider screen"
    assert app_page.page.locator('text=Choose Numbers Round Setup').is_visible(), \
        "Provider heading should be visible"


def test_random_game_generation(app_page: AppPage):
    """Test generating a random game and validating the game board."""
    # Click Random Game button
    app_page.click_random_game()

    # Verify navigation to game board
    assert app_page.is_on_game_board(), "Should navigate to game board"

    # Validate all game board elements are visible
    app_page.validate_game_board()

    # Validate target number is in valid range (1-1000)
    target = app_page.get_target_number()
    assert 1 <= target <= 1000, f"Target {target} should be between 1 and 1000"

    # Validate exactly 6 numbers are displayed
    numbers = app_page.get_available_numbers()
    assert len(numbers) == 6, f"Should have exactly 6 numbers, got {len(numbers)}"

    # Validate all numbers are positive
    assert all(n > 0 for n in numbers), f"All numbers should be positive: {numbers}"


def test_reset_button_returns_to_provider(app_page: AppPage):
    """Test that clicking Reset returns to the provider screen."""
    # Generate a game
    app_page.click_random_game()
    assert app_page.is_on_game_board(), "Should be on game board"

    # Click Reset button
    app_page.click_reset()

    # Verify return to provider screen
    assert app_page.is_on_provider_screen(), "Should return to provider screen after reset"
    assert not app_page.is_on_game_board(), "Should not be on game board after reset"


def test_multiple_random_games(app_page: AppPage):
    """Test generating multiple random games sequentially."""
    targets = []
    number_sets = []

    for i in range(3):
        # Generate a new game
        app_page.click_random_game()
        assert app_page.is_on_game_board(), f"Game {i+1} should load game board"

        # Capture game data
        target = app_page.get_target_number()
        numbers = app_page.get_available_numbers()

        targets.append(target)
        number_sets.append(numbers)

        # Validate game
        assert 1 <= target <= 1000, f"Game {i+1} target should be valid"
        assert len(numbers) == 6, f"Game {i+1} should have 6 numbers"

        # Reset for next iteration
        app_page.click_reset()
        assert app_page.is_on_provider_screen(), f"Should reset after game {i+1}"

    # Verify we got different games (at least one different target or number set)
    # Note: There's a tiny chance all 3 games are identical, but extremely unlikely
    assert len(set(targets)) > 1 or len(set(map(tuple, number_sets))) > 1, \
        "Should generate varied random games"
