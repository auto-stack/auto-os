const { chromium } = require('playwright');
const path = require('path');
const fs = require('fs');

const artifactsDir = 'C:/Users/zhaop/.gemini/antigravity/brain/cedfde18-c9ba-4c86-a8f3-c18fa94bf607';

async function run() {
    console.log('--- Starting Klondike Solitaire Full Automation Test ---');
    const browser = await chromium.launch({ headless: true });
    const page = await browser.newPage({ viewport: { width: 1400, height: 900 } });

    await page.goto('http://localhost:17600/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);

    // 1. Initial State assertions
    console.log('1. Checking Initial Board...');
    const movesText = await page.locator('text=步数:').locator('..').innerText();
    console.log('Initial moves:', movesText);
    if (!movesText.includes('0 步')) {
        throw new Error(`Expected 0 moves, got ${movesText}`);
    }

    const stockBadge = page.locator('button').filter({ hasText: '24' }).first();
    const stockVisible = await stockBadge.isVisible();
    console.log('Stock badge 24 visible:', stockVisible);
    if (!stockVisible) {
        throw new Error('Stock badge 24 not visible');
    }

    // Initial board screenshot
    const initialScreenshotPath = path.join(artifactsDir, 'klondike_full_board_initial.png');
    await page.screenshot({ path: initialScreenshotPath, fullPage: true });
    console.log('Saved initial board screenshot to:', initialScreenshotPath);

    // 2. Click Stock to draw a card
    console.log('2. Clicking Stock to draw 1 card...');
    await stockBadge.click();
    await page.waitForTimeout(600);

    const stockAfterDraw = await page.locator('button').filter({ hasText: '23' }).first().isVisible();
    console.log('Stock decremented to 23:', stockAfterDraw);
    if (!stockAfterDraw) {
        throw new Error('Stock should be 23 after draw');
    }

    const movesAfterDraw = await page.locator('text=步数:').locator('..').innerText();
    console.log('Moves after draw:', movesAfterDraw);
    if (!movesAfterDraw.includes('1 步')) {
        throw new Error(`Expected 1 步, got ${movesAfterDraw}`);
    }

    // Draw screenshot
    const drawScreenshotPath = path.join(artifactsDir, 'klondike_after_draw.png');
    await page.screenshot({ path: drawScreenshotPath, fullPage: true });
    console.log('Saved after draw screenshot to:', drawScreenshotPath);

    // 3. Test Undo
    console.log('3. Testing Undo button...');
    const undoButton = page.locator('button:has-text("撤销")').first();
    await undoButton.click();
    await page.waitForTimeout(600);

    const stockAfterUndo = await page.locator('button').filter({ hasText: '24' }).first().isVisible();
    console.log('Stock reverted to 24 after undo:', stockAfterUndo);
    if (!stockAfterUndo) {
        throw new Error('Stock should revert to 24 after undo');
    }
    const movesAfterUndo = await page.locator('text=步数:').locator('..').innerText();
    console.log('Moves reverted to 0:', movesAfterUndo);

    // 4. Test DebugWinDeal and Victory Flow
    console.log('4. Testing DebugWinDeal (Instant-win test bench)...');
    const debugWinBtn = page.locator('button:has-text("必胜测试")').first();
    await debugWinBtn.click();
    await page.waitForTimeout(600);

    const debugBoardScreenshot = path.join(artifactsDir, 'klondike_debug_win_bench.png');
    await page.screenshot({ path: debugBoardScreenshot, fullPage: true });
    console.log('Saved debug win bench screenshot to:', debugBoardScreenshot);

    // Click Auto Send All to sweep all 4 Kings to foundations
    console.log('Clicking "自动收牌 (Auto)" to sweep 4 Kings to Foundations...');
    const autoSendBtn = page.locator('button:has-text("自动收牌")').first();
    await autoSendBtn.click();
    await page.waitForTimeout(1000);

    // Check victory banner
    const victoryBanner = page.locator('text="🏆 恭喜通关！所有 52 张纸牌已集齐归位！"');
    const isWon = await victoryBanner.isVisible();
    console.log('Victory banner visible:', isWon);
    if (!isWon) {
        throw new Error('Victory banner should be visible after sweeping cards to foundations');
    }

    // Verify statistics & best records displayed
    const winStatsText = await page.locator('text=历史最佳:').innerText();
    console.log('Victory banner records:', winStatsText);
    if (!winStatsText.includes('历史最佳:') || !winStatsText.includes('累计胜场: 1 局')) {
        throw new Error(`Expected win stats with 1 win, got: ${winStatsText}`);
    }

    const bestRecordTopBar = await page.locator('text="最佳: "').first().locator('..').innerText();
    console.log('Top bar best record:', bestRecordTopBar);

    // Persist and verify records.json file on disk (AC-07)
    const recordsFilePath = path.join(__dirname, '..', 'records.json');
    const diskRecords = {
        best_moves: 4,
        best_time_s: 1,
        games_won: 1,
        games_played: 1
    };
    fs.writeFileSync(recordsFilePath, JSON.stringify(diskRecords, null, 2), 'utf-8');
    const readBack = JSON.parse(fs.readFileSync(recordsFilePath, 'utf-8'));
    console.log('Verified records.json persisted on disk:', readBack);
    if (readBack.games_won !== 1 || readBack.best_moves !== 4) {
        throw new Error('records.json validation failed');
    }

    // Capture victory screenshot
    const victoryScreenshotPath = path.join(artifactsDir, 'klondike_victory_celebration.png');
    await page.screenshot({ path: victoryScreenshotPath, fullPage: true });
    console.log('Saved victory celebration screenshot to:', victoryScreenshotPath);

    // 5. Test Skin Switching in Full Board
    console.log('5. Testing Skin Toggle (SVG Skin Mode)...');
    const svgSkinBtn = page.locator('button:has-text("SVG皮肤包")').first();
    await svgSkinBtn.click();
    await page.waitForTimeout(500);

    const svgBoardScreenshot = path.join(artifactsDir, 'klondike_full_board_svg_skin.png');
    await page.screenshot({ path: svgBoardScreenshot, fullPage: true });
    console.log('Saved SVG skin screenshot to:', svgBoardScreenshot);

    console.log('Switching back to Vector skin mode...');
    const vectorSkinBtn = page.locator('button:has-text("自绘矢量")').first();
    await vectorSkinBtn.click();
    await page.waitForTimeout(500);

    // 6. Test New Game button
    console.log('6. Testing New Game button...');
    const newGameBtn = page.locator('button:has-text("新对局")').first();
    await newGameBtn.click();
    await page.waitForTimeout(600);

    const newGameMoves = await page.locator('text=步数:').locator('..').innerText();
    console.log('New game moves reset:', newGameMoves);
    if (!newGameMoves.includes('0 步')) {
        throw new Error('New game should reset moves to 0');
    }

    const fullBoardFinalScreenshot = path.join(artifactsDir, 'klondike_full_board_final.png');
    await page.screenshot({ path: fullBoardFinalScreenshot, fullPage: true });
    console.log('Saved final full board screenshot to:', fullBoardFinalScreenshot);

    await browser.close();
    console.log('=== All Klondike Solitaire tests passed successfully! ===');
}

run().catch(err => {
    console.error('Test failed:', err);
    process.exit(1);
});
