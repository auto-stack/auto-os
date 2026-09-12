import { test, expect } from '@playwright/test'

async function openGame(page) {
  await page.route('**/api/tetris/record', route =>
    route.fulfill({ status: 200, contentType: 'application/json', body: '0' }),
  )
  await page.route('**/api/tetris/score', route =>
    route.fulfill({ status: 200, contentType: 'application/json', body: 'true' }),
  )
  await page.goto('/')
  await expect(page.getByText('俄罗斯方块').first()).toBeVisible()
}

test('首屏显示棋盘、下一个方块和纪录面板', async ({ page }) => {
  await openGame(page)
  const grids = page.locator('.grid')
  await expect(grids).toHaveCount(2)
  const boardColumns = await grids.nth(0).evaluate(el =>
    getComputedStyle(el).gridTemplateColumns.trim().split(/\s+/).length,
  )
  const nextColumns = await grids.nth(1).evaluate(el =>
    getComputedStyle(el).gridTemplateColumns.trim().split(/\s+/).length,
  )
  expect(boardColumns).toBe(10)
  expect(nextColumns).toBe(4)
  await expect(grids.nth(0).locator('button')).toHaveCount(200)
  const body = await page.locator('body').innerText()
  expect(body).toContain('下一个')
  expect(body).toContain('最高纪录')
  expect(body).toContain('等级')
  expect(body).toContain('消行')
  expect(body).toContain('准备好了吗')
})

test('开始、暂停和恢复共用 Store 状态机', async ({ page }) => {
  await openGame(page)
  await page.getByRole('button', { name: '开始游戏' }).click()
  await expect(page.getByText('进行中').first()).toBeVisible()
  await page.getByRole('button', { name: '暂停 P' }).click()
  await expect(page.getByText('已暂停').first()).toBeVisible()
  await page.getByRole('button', { name: '继续游戏' }).click()
  await expect(page.getByText('进行中').first()).toBeVisible()
})

test('键盘硬降和玩法说明可用', async ({ page }) => {
  await openGame(page)
  await page.getByRole('button', { name: '开始游戏' }).click()
  await page.keyboard.press('Space')
  await page.getByRole('button', { name: '玩法说明' }).click()
  await expect(page.getByText('← → 移动，↑ 顺时针旋转')).toBeVisible()
  await page.getByRole('button', { name: '返回游戏' }).click()
  await expect(page.getByText('已暂停').first()).toBeVisible()
})
