"""Compete mode tests for the Countdown Solver application.

These tests exercise the "Compete" button which starts a countdown timer
before automatically solving the game. The OPTS_DEV_FAST_COMPETE localStorage
flag (injected in conftest.py) reduces the timer from 30s to 2s for fast tests.
"""

import pytest
from pages.page_objects import AppPage


def test_compete_button_visible_on_game_board(app_page: AppPage):
    """Test that the Compete button is visible and enabled on the game board."""
    app_page.click_random_game()
    assert app_page.is_on_game_board(), "Should be on game board"

    assert app_page.is_compete_button_enabled(), (
        "Compete button should be enabled initially"
    )


def test_compete_button_disabled_after_clicking_solve(app_page: AppPage):
    """Test that the Compete button is disabled after clicking Solve."""
    app_page.click_random_game()

    # Click Solve (not Compete)
    app_page.click_solve(timeout=30000)

    assert app_page.is_compete_button_disabled(), (
        "Compete button should be disabled after solving"
    )


def test_compete_starts_countdown_timer(app_page: AppPage):
    """Test that clicking Compete starts the countdown timer display."""
    app_page.click_random_game()
    app_page.click_compete()

    timer_value = app_page.get_time_remaining()
    assert 0 < timer_value <= 2, (
        f"Timer should be between 1 and 2 (fast mode), got {timer_value}"
    )


def test_compete_disables_all_action_buttons(app_page: AppPage):
    """Test that all action buttons are disabled during competition."""
    app_page.click_random_game()
    app_page.click_compete()

    assert app_page.is_solve_button_disabled(), (
        "Solve button should be disabled during competition"
    )
    assert app_page.is_reset_button_disabled(), (
        "Reset button should be disabled during competition"
    )
    assert app_page.is_compete_button_disabled(), (
        "Compete button should be disabled during competition"
    )


def test_compete_timer_counts_down(app_page: AppPage):
    """Test that the timer value decreases over time."""
    app_page.click_random_game()
    app_page.click_compete()

    initial_value = app_page.get_time_remaining()

    # Wait just over 1 second for a tick
    app_page.page.wait_for_timeout(1100)

    # Timer should have decreased (or competition ended)
    timer_visible = app_page.page.locator(AppPage.TIMER_DISPLAY).is_visible()
    if timer_visible:
        current_value = app_page.get_time_remaining()
        assert current_value < initial_value, (
            f"Timer should have decreased from {initial_value}, got {current_value}"
        )


def test_compete_triggers_solve_after_countdown(app_page: AppPage):
    """Test that the solver runs automatically after the countdown ends."""
    app_page.click_random_game()
    app_page.click_compete()

    # With fast mode (2s timer), wait up to 10s for the full flow
    app_page.wait_for_competition_end(timeout=10000)

    # Verify a solution message appeared
    assert app_page.has_success_message() or app_page.has_failure_message(), (
        "Should show a solution result after competition ends"
    )

    # Timer display should be gone
    assert not app_page.page.locator(AppPage.TIMER_DISPLAY).is_visible(), (
        "Timer display should be hidden after competition ends"
    )


def test_compete_button_disabled_after_competition(app_page: AppPage):
    """Test that Compete is disabled and Reset is enabled after competition."""
    app_page.click_random_game()
    app_page.click_compete()
    app_page.wait_for_competition_end(timeout=10000)

    assert app_page.is_compete_button_disabled(), (
        "Compete button should be disabled after competition"
    )
    assert app_page.is_reset_button_enabled(), (
        "Reset button should be enabled after competition"
    )


def test_reset_after_compete_reenables_all_buttons(app_page: AppPage):
    """Test that resetting after competition re-enables all buttons."""
    app_page.click_random_game()
    app_page.click_compete()
    app_page.wait_for_competition_end(timeout=10000)

    # Reset to provider screen
    app_page.click_reset()
    assert app_page.is_on_provider_screen(), "Should be on provider screen"

    # Generate new game
    app_page.click_random_game()

    # All buttons should be enabled again
    assert app_page.is_solve_button_enabled(), (
        "Solve button should be enabled for new game"
    )
    assert app_page.is_compete_button_enabled(), (
        "Compete button should be enabled for new game"
    )
    assert app_page.is_reset_button_enabled(), (
        "Reset button should be enabled for new game"
    )
