"""E2E tests for the Manual Entry game mode."""

import pytest
from pages.page_objects import AppPage


def test_manual_entry_shows_entry_screen(app_page: AppPage):
    """Clicking Manual Entry should navigate to the manual entry setup screen."""
    app_page.click_manual_entry()
    assert app_page.is_on_manual_entry_screen(), "Should be on manual entry setup screen"


def test_back_button_returns_to_provider(app_page: AppPage):
    """Clicking Back from manual entry should return to the provider screen."""
    app_page.click_manual_entry()
    assert app_page.is_on_manual_entry_screen(), "Should be on manual entry setup screen"

    app_page.click_back_to_options()
    assert app_page.is_on_provider_screen(), "Should return to provider screen after clicking Back"


def test_confirm_disabled_initially(app_page: AppPage):
    """Confirm button should be disabled when no numbers or target are selected."""
    app_page.click_manual_entry()
    assert app_page.is_confirm_disabled(), "Confirm should be disabled initially"


def test_can_select_numbers(app_page: AppPage):
    """Clicking number buttons should make them appear as remove-buttons in the selected area."""
    app_page.click_manual_entry()
    app_page.click_number(5)
    assert app_page.page.locator('button[aria-label="remove 5"]').is_visible(), \
        "Selected number 5 should appear as a remove button"


def test_can_remove_selected_number(app_page: AppPage):
    """Selecting a number then clicking remove should remove it from the selected area."""
    app_page.click_manual_entry()
    app_page.click_number(7)
    assert app_page.page.locator('button[aria-label="remove 7"]').is_visible(), \
        "Number 7 should be in selected area"

    app_page.remove_selected_number(7)
    assert not app_page.page.locator('button[aria-label="remove 7"]').is_visible(), \
        "Number 7 should no longer be in selected area after removal"


def test_large_number_disabled_after_selection(app_page: AppPage):
    """Clicking a large number (e.g. 25) should disable its button."""
    app_page.click_manual_entry()
    app_page.click_number(25)
    assert app_page.page.locator('button[aria-label="number 25"]').is_disabled(), \
        "25 button should be disabled after being selected once"


def test_small_number_disabled_after_two_uses(app_page: AppPage):
    """Clicking a small number twice should disable its button."""
    app_page.click_manual_entry()
    app_page.click_number(5)
    app_page.click_number(5)
    assert app_page.page.locator('button[aria-label="number 5"]').is_disabled(), \
        "5 button should be disabled after being selected twice"


def test_confirm_enabled_after_valid_selection(app_page: AppPage):
    """Selecting 6 numbers and a valid target should enable the Confirm button."""
    app_page.click_manual_entry()
    app_page.select_numbers([1, 2, 3, 4, 5, 6])
    app_page.set_target(42)
    assert app_page.is_confirm_enabled(), "Confirm should be enabled after valid selection"


def test_valid_manual_entry_creates_game(app_page: AppPage):
    """Entering a complete valid game and confirming should create a game board with those values."""
    app_page.click_manual_entry()
    app_page.select_numbers([1, 2, 3, 4, 5, 6])
    app_page.set_target(21)
    app_page.click_confirm_game()

    assert app_page.is_on_game_board(), "Should navigate to game board after confirming"

    target = app_page.get_target_number()
    assert target == 21, f"Target should be 21, got {target}"

    numbers = app_page.get_available_numbers()
    assert sorted(numbers) == [1, 2, 3, 4, 5, 6], \
        f"Board numbers should be [1,2,3,4,5,6], got {sorted(numbers)}"


def test_reset_after_manual_entry_returns_to_provider(app_page: AppPage):
    """Full flow: manual entry → confirm → reset → should return to provider screen."""
    app_page.click_manual_entry()
    app_page.select_numbers([1, 2, 3, 4, 5, 6])
    app_page.set_target(21)
    app_page.click_confirm_game()

    assert app_page.is_on_game_board(), "Should be on game board after confirming"

    app_page.click_reset()
    assert app_page.is_on_provider_screen(), "Should return to provider screen after reset"
