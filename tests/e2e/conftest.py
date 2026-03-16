"""Pytest configuration and fixtures for e2e tests."""

import pytest
from playwright.sync_api import BrowserContext, Page


@pytest.fixture
def context(new_context) -> BrowserContext:
    """Create a browser context with OPTS_DEV_FAST_COMPETE in localStorage.

    Overrides the default pytest-playwright context fixture to inject a
    localStorage flag that reduces the compete timer from 30 s to 2 s,
    keeping E2E tests fast.
    """
    context = new_context()
    context.add_init_script(
        "window.localStorage.setItem('OPTS_DEV_FAST_COMPETE', 'true');"
    )
    return context


@pytest.fixture
def app_page(page: Page, base_url: str):
    """Provide an AppPage helper positioned on the app's start screen."""
    from pages.page_objects import AppPage

    page.goto(base_url)
    return AppPage(page)


def pytest_configure(config):
    """Display environment configuration at test startup."""
    base_url = (
        config.getoption("base_url", default=None)
        or config.getini("base_url")
        or "(not set)"
    )
    screenshot = config.getoption("--screenshot", default="off")
    output_dir = config.getoption("--output", default="test-results")
    print(f"\n{'=' * 60}")
    print("E2E Test Configuration")
    print(f"{'=' * 60}")
    print(f"BASE_URL:    {base_url}")
    print(f"SCREENSHOT:  {screenshot}")
    print(f"OUTPUT_DIR:  {output_dir}")
    print(f"{'=' * 60}\n")
