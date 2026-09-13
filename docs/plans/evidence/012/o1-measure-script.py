#!/usr/bin/env python3
"""PLAN-012 O1：任务栏图标字形居中像素测量。

输入：整窗截图（2x 缩放，客户区 1280x800 逻辑 → 2560x1600 px）。
方法：dock h-14(56) 底部锚定；按钮 h-10 w-10(40) 水平序列 launcher 起
px-2(8) gap-2(8)。逐槽裁剪 → 亮度离群 = 字形 ink → 包围盒中心 vs 槽位
按钮盒中心（x 精确；y 按有无底条两档估算）。报告偏差（逻辑 px）。
"""
import sys

from PIL import Image


def analyze(path):
    im = Image.open(path).convert("L")
    W, H = im.size
    scale = W / 1280.0
    dock_top = int((H / scale - 56) * scale)  # dock 条顶
    strip = im.crop((0, dock_top, W, H))
    px = strip.load()
    sw, sh = strip.size

    # 槽位序列（逻辑 x）：launcher 16..56，之后每槽 48 步进（40+8）。
    slots = []
    n = int((sw / scale - 16 + 8) // 48)
    for i in range(n):
        x0 = 16 + i * 48
        slots.append((x0, x0 + 40))
    results = []
    for i, (lx0, lx1) in enumerate(slots):
        cx0, cx1 = int(lx0 * scale), int(lx1 * scale)
        if cx1 + 8 > sw:
            break
        # 槽内 ink：与槽内中位亮度差 > 48
        vals = []
        for y in range(sh):
            for x in range(cx0, cx1):
                vals.append(px[x, y])
        vals.sort()
        med = vals[len(vals) // 2]
        xs, ys = [], []
        for y in range(sh):
            for x in range(cx0, cx1):
                if abs(px[x, y] - med) > 48:
                    xs.append(x)
                    ys.append(y)
        if not xs:
            results.append((i, None))
            continue
        gx0, gx1, gy0, gy1 = min(xs), max(xs), min(ys), max(ys)
        gcx, gcy = (gx0 + gx1) / 2, (gy0 + gy1) / 2
        # 按钮盒（px，槽内）：无底条 y = (112-80)/2..+80；有底条按 col 居中
        # 上移 3 逻辑 px（(46-40)/2）。
        btn_cx = (cx0 + cx1) / 2
        res = {
            "slot": i,
            "glyph_w": (gx1 - gx0) / scale,
            "glyph_h": (gy1 - gy0) / scale,
            "dx_nobar": (gcx - btn_cx) / scale,
            "dy_nobar": (gcy - (dock_top_off + 16 + 40)) / scale if False else None,
        }
        # y 基准：dock 顶(0) 起，无底条按钮盒 16..56（逻辑），中心 36。
        res["dy_nobar"] = (gcy) / scale - 36
        # 有底条档：col(46) 居中 → col 顶 5 → 按钮 5..45，中心 25。
        res["dy_bar"] = (gcy) / scale - 25
        results.append((i, res, med))
    return results, sh / scale


if __name__ == "__main__":
    path = sys.argv[1]
    res, dock_h = analyze(path)
    print(f"dock 逻辑高: {dock_h:.1f}")
    for r in res:
        if r[1] is None:
            print(f"slot {r[0]}: 空（无 ink）")
            continue
        _, d, med = r
        print(
            f"slot {d['slot']}: glyph {d['glyph_w']:.1f}x{d['glyph_h']:.1f} 逻辑px, "
            f"中心x偏差 {d['dx_nobar']:+.1f}, 中心y偏差(无底条档) {d['dy_nobar']:+.1f} / (有底条档) {d['dy_bar']:+.1f}"
        )
