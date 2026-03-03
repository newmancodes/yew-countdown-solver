"""E2E tests for the Custom Split game mode."""

import pytest
from pages.page_objects import AppPage


def test_custom_split_shows_selector(app_page: AppPage):
    """Clicking Custom Split should navigate to the custom split setup screen."""
    app_page.click_custom_split()
    assert app_page.is_on_custom_split_screen(), "Should be on custom split setup screen"


def test_back_button_returns_to_provider(app_page: AppPage):
    """Clicking Back from custom split should return to the provider screen."""
    app_page.click_custom_split()
    assert app_page.is_on_custom_split_screen(), "Should be on custom split setup screen"

    app_page.click_back_to_options()
    assert app_page.is_on_provider_screen(), "Should return to provider screen after clicking Back"


def test_generates_game_with_no_large_numbers(app_page: AppPage):
    """Selecting 0 large numbers should immediately produce a board with no large numbers."""
    app_page.click_custom_split()
    app_page.select_large_count(0)

    numbers = app_page.get_available_numbers()
    assert len(numbers) == 6, f"Should have exactly 6 numbers, got {len(numbers)}"

    large = AppPage.large_numbers_in(numbers)
    assert large == [], f"Expected no large numbers, got {large}"
    assert all(1 <= n <= 10 for n in numbers), f"All numbers should be 1-10: {numbers}"


def test_generates_game_with_four_large_numbers(app_page: AppPage):
    """Selecting 4 large numbers should immediately produce a board with exactly 4 large numbers."""
    app_page.click_custom_split()
    app_page.select_large_count(4)

    numbers = app_page.get_available_numbers()
    assert len(numbers) == 6, f"Should have exactly 6 numbers, got {len(numbers)}"

    large = AppPage.large_numbers_in(numbers)
    assert len(large) == 4, f"Expected 4 large numbers, got {large}"


@pytest.mark.parametrize("large_count", [0, 1, 2, 3, 4])
def test_each_split_produces_correct_mix(app_page: AppPage, large_count: int):
    """Each large count selection should immediately produce a board with the correct number mix."""
    app_page.click_custom_split()
    app_page.select_large_count(large_count)

    numbers = app_page.get_available_numbers()
    assert len(numbers) == 6, f"Should have exactly 6 numbers, got {len(numbers)}"

    actual_large = AppPage.large_numbers_in(numbers)
    assert len(actual_large) == large_count, (
        f"Expected {large_count} large number(s), got {actual_large}"
    )

    app_page.click_reset()
    assert app_page.is_on_provider_screen(), "Should return to provider after reset"
