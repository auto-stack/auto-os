const { chromium } = require('playwright');
const path = require('path');

const artifactsDir = 'C:/Users/zhaop/.gemini/antigravity/brain/cedfde18-c9ba-4c86-a8f3-c18fa94bf607';

async function run() {
    console.log('--- Testing Interactive Column Moves, Auto-Flip, and Foundation ---');
    const browser = await chromium.launch({ headless: true });
    const page = await browser.newPage({ viewport: { width: 1400, height: 900 } });

    await page.goto('http://localhost:17600/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(500);

    // 1. Reset to fresh game with New Game button to match final state
    const newGameBtn = page.locator('button:has-text("新对局")').first();
    await newGameBtn.click();
    await page.waitForTimeout(500);

    // Look for Ace of diamonds (in col 5, text "A" and red)
    console.log('Looking for Ace of diamonds on board...');
    const diamondAce = page.locator('button:has-text("A")').filter({ has: page.locator('svg.lucide-diamond, [class*="text-red"]') }).first();
    const aceVisible = await diamondAce.isVisible();
    console.log('Diamond Ace visible:', aceVisible);

    if (aceVisible) {
        // Double click diamond Ace to trigger AutoSend to foundation
        console.log('Double clicking Ace of diamonds to auto-send to Foundation...');
        await diamondAce.dblclick();
        await page.waitForTimeout(600);

        // Check if foundation has Ace
        const foundationWithCard = page.locator('button:has-text("A")').first();
        console.log('Foundation has Ace:', await foundationWithCard.isVisible());

        // Screenshot after auto-send & auto-flip
        const autoFlipScreenshot = path.join(artifactsDir, 'klondike_after_autoflip.png');
        await page.screenshot({ path: autoFlipScreenshot, fullPage: true });
        console.log('Saved auto-flip screenshot to:', autoFlipScreenshot);

        // Check moves count
        const moves = await page.locator('text=步数:').locator('..').innerText();
        console.log('Moves after auto-send:', moves);

        // Test Undo of the auto-send & auto-flip
        console.log('Clicking Undo...');
        const undoBtn = page.locator('button:has-text("撤销")').first();
        await undoBtn.click();
        await page.waitForTimeout(600);

        const movesReverted = await page.locator('text=步数:').locator('..').innerText();
        console.log('Moves after undo:', movesReverted);

        const undoScreenshot = path.join(artifactsDir, 'klondike_after_undo_autoflip.png');
        await page.screenshot({ path: undoScreenshot, fullPage: true });
        console.log('Saved undo auto-flip screenshot to:', undoScreenshot);
    }

    await browser.close();
    console.log('--- Interactive column moves test finished! ---');
}

run().catch(err => {
    console.error('Interactive test failed:', err);
    process.exit(1);
});
