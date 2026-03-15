"""Pytest configuration and fixtures for e2e tests."""

import os
from pathlib import Path
from typing import Generator

import pytest
from playwright.sync_api import (
    Browser,
    BrowserContext,
    Page,
    Playwright,
    sync_playwright,
)


# Environment variables with defaults
BASE_URL = os.getenv("BASE_URL", "http://localhost:8080")
HEADLESS = os.getenv("HEADLESS", "true").lower() in ("true", "1", "yes")
SOLVER_TIMEOUT = int(os.getenv("SOLVER_TIMEOUT", "30000"))

# Screenshot directory
SCREENSHOT_DIR = Path(__file__).parent / "screenshots"


@pytest.fixture(scope="session")
def playwright() -> Generator[Playwright, None, None]:
    """Provide a Playwright instance for the entire test session."""
    with sync_playwright() as p:
        yield p


@pytest.fixture(scope="session")
def browser(playwright: Playwright) -> Generator[Browser, None, None]:
    """Provide a Chromium browser instance for the entire test session."""
    browser = playwright.chromium.launch(headless=HEADLESS)
    yield browser
    browser.close()


@pytest.fixture(scope="function")
def context(browser: Browser) -> Generator[BrowserContext, None, None]:
    """Provide a new browser context for each test.

    Injects the OPTS_DEV_FAST_COMPETE localStorage flag so the compete
    timer runs for 2 seconds instead of 30, keeping E2E tests fast.
    """
    context = browser.new_context()
    context.add_init_script(
        "window.localStorage.setItem('OPTS_DEV_FAST_COMPETE', 'true');"
    )
    yield context
    context.close()


@pytest.fixture(scope="function")
def page(
    context: BrowserContext, request: pytest.FixtureRequest
) -> Generator[Page, None, None]:
    """Provide a new page for each test with automatic screenshot on failure."""
    page = context.new_page()
    page.goto(BASE_URL)

    yield page

    # Capture screenshot on test failure
    if request.node.rep_call.failed:
        SCREENSHOT_DIR.mkdir(parents=True, exist_ok=True)
        screenshot_name = f"{request.node.name}.png"
        screenshot_path = SCREENSHOT_DIR / screenshot_name
        page.screenshot(path=str(screenshot_path))
        print(f"\nScreenshot saved: {screenshot_path}")

    page.close()


@pytest.fixture(scope="function")
def app_page(page: Page):
    """Provide an AppPage helper object for interacting with the app."""
    from pages.page_objects import AppPage

    return AppPage(page)


@pytest.hookimpl(tryfirst=True, hookwrapper=True)
def pytest_runtest_makereport(item, call):
    """Hook to make test results available to fixtures.

    This allows the page fixture to check if the test failed and capture a screenshot.
    """
    outcome = yield
    rep = outcome.get_result()
    setattr(item, f"rep_{rep.when}", rep)


def pytest_configure(config):
    """Display environment configuration at test startup."""
    print(f"\n{'=' * 60}")
    print(f"E2E Test Configuration")
    print(f"{'=' * 60}")
    print(f"BASE_URL: {BASE_URL}")
    print(f"HEADLESS: {HEADLESS}")
    print(f"SOLVER_TIMEOUT: {SOLVER_TIMEOUT}ms")
    print(f"SCREENSHOT_DIR: {SCREENSHOT_DIR}")
    print(f"{'=' * 60}\n")
