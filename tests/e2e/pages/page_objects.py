"""Page Object Model for the Countdown Solver app."""

import re
from typing import List, Optional

from playwright.sync_api import Page, expect


class AppPage:
    """Helper class for interacting with the Countdown Solver app."""

    # Selectors based on aria-labels from the Rust code
    RANDOM_GAME_BUTTON = 'button[aria-label="Generate random game"]'
    TARGET_NUMBER = '[aria-label="Target number"]'
    AVAILABLE_NUMBERS = '[aria-label="Available numbers"]'
    SOLVE_BUTTON = 'button[aria-label="Solve game"]'
    RESET_BUTTON = 'button[aria-label="Reset game"]'
    GAME_BOARD = '[aria-label="Game board"]'
    PROVIDER_HEADING = 'text=Choose Numbers Round Setup'

    def __init__(self, page: Page):
        """Initialize the AppPage with a Playwright page instance.

        Args:
            page: Playwright page instance
        """
        self.page = page

    def is_on_provider_screen(self) -> bool:
        """Check if currently on the game provider screen.

        Returns:
            True if on provider screen, False otherwise
        """
        return self.page.locator(self.PROVIDER_HEADING).is_visible()

    def is_on_game_board(self) -> bool:
        """Check if currently on the game board screen.

        Returns:
            True if on game board, False otherwise
        """
        return self.page.locator(self.GAME_BOARD).is_visible()

    def click_random_game(self) -> None:
        """Click the Random Game button and wait for the game board to load."""
        self.page.click(self.RANDOM_GAME_BUTTON)
        # Wait for navigation to game board
        self.page.wait_for_selector(self.GAME_BOARD, state="visible")

    def click_solve(self, timeout: Optional[int] = None) -> None:
        """Click the Solve button.

        Args:
            timeout: Optional timeout in milliseconds for the solver to complete
        """
        self.page.click(self.SOLVE_BUTTON)
        # Wait for solution message to appear
        if timeout:
            self.page.wait_for_selector(
                'text=/Solution found|No solution found/',
                timeout=timeout,
                state="visible"
            )

    def click_reset(self) -> None:
        """Click the Reset button and wait for provider screen to load."""
        self.page.click(self.RESET_BUTTON)
        # Wait for navigation back to provider screen
        self.page.wait_for_selector(self.PROVIDER_HEADING, state="visible")

    def get_target_number(self) -> int:
        """Extract the target number from the game board.

        Returns:
            The target number as an integer

        Raises:
            ValueError: If target number cannot be extracted or parsed
        """
        target_text = self.page.locator(self.TARGET_NUMBER).inner_text()
        try:
            return int(target_text.strip())
        except ValueError:
            raise ValueError(f"Could not parse target number: {target_text}")

    def get_available_numbers(self) -> List[int]:
        """Extract the list of available numbers from the game board.

        Returns:
            List of 6 numbers

        Raises:
            ValueError: If numbers cannot be extracted or parsed
        """
        # Get the container with all numbers
        numbers_container = self.page.locator(self.AVAILABLE_NUMBERS)
        # Get individual number elements (they're in divs within the container)
        number_elements = numbers_container.locator("div[role='listitem']").all()

        numbers = []
        for element in number_elements:
            text = element.inner_text().strip()
            try:
                numbers.append(int(text))
            except ValueError:
                raise ValueError(f"Could not parse number: {text}")

        return numbers

    def get_solution_message(self) -> str:
        """Get the solution result message text.

        Returns:
            The message text (e.g., "Solution found in 3 steps!" or "No solution found. Try a new game!")

        Raises:
            TimeoutError: If no solution message appears
        """
        # Wait for either success or failure message
        message_locator = self.page.locator('text=/Solution found|No solution found/')
        message_locator.wait_for(state="visible", timeout=5000)
        return message_locator.inner_text()

    def has_success_message(self) -> bool:
        """Check if a success message is displayed.

        Returns:
            True if "Solution found in" message is visible
        """
        return self.page.locator('text=Solution found in').is_visible()

    def has_failure_message(self) -> bool:
        """Check if a failure message is displayed.

        Returns:
            True if "No solution found" message is visible
        """
        return self.page.locator('text=No solution found').is_visible()

    def get_step_count(self) -> Optional[int]:
        """Extract the step count from a success message.

        Returns:
            The number of steps if a solution was found, None otherwise

        Raises:
            ValueError: If success message is present but step count cannot be parsed
        """
        if not self.has_success_message():
            return None

        message = self.get_solution_message()
        # Extract number from "Solution found in N steps!"
        match = re.search(r'Solution found in (\d+) steps?', message)
        if match:
            return int(match.group(1))
        else:
            raise ValueError(f"Could not extract step count from: {message}")

    def validate_game_board(self) -> None:
        """Assert that all required game board elements are visible.

        Raises:
            AssertionError: If any required element is not visible
        """
        expect(self.page.locator(self.GAME_BOARD)).to_be_visible()
        expect(self.page.locator(self.TARGET_NUMBER)).to_be_visible()
        expect(self.page.locator(self.AVAILABLE_NUMBERS)).to_be_visible()
        expect(self.page.locator(self.SOLVE_BUTTON)).to_be_visible()
        expect(self.page.locator(self.RESET_BUTTON)).to_be_visible()

    def is_solve_button_disabled(self) -> bool:
        """Check if the Solve button is disabled.

        Returns:
            True if the Solve button is disabled
        """
        return self.page.locator(self.SOLVE_BUTTON).is_disabled()

    def is_solve_button_enabled(self) -> bool:
        """Check if the Solve button is enabled.

        Returns:
            True if the Solve button is enabled
        """
        return self.page.locator(self.SOLVE_BUTTON).is_enabled()
