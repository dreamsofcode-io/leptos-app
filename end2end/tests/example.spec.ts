import { test, expect } from "@playwright/test";

test("homepage has title and counter", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle("Leptos App");

  await expect(page.locator("h2")).toHaveText("Counter");
});
