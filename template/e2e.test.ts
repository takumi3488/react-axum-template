import { test, expect } from '@playwright/test';

const origin = process.env.TARGET_ORIGIN as string;

test('got health check', async ({ page }) => {
  await page.goto(origin);
  await expect(page.getByText('OK')).toBeVisible({
    timeout: 10000
  })
});
