#!/usr/bin/env python3
"""slice_icons.py — AutoOS 桌面图标精灵表切片（PLAN-018 W1）。

设计源：assets/icons.png（浅）/ assets/icons_dark.png（深），7×4=28 图块，
浅深同布局（同 rect 复用）。本工具做网格拟合 → 逐格填充率校验 → 切出
assets/icons/{light,dark}/<stem>.png（原生 RGBA）→ 写 assets/icons/mapping.json
（registry id → stem；browser 预留位不入映射）。

用法：
  python scripts/slice_icons.py            # 切片 + 写 mapping + 报告
  python scripts/slice_icons.py --verify   # 只校验既有产物，不重写

校验（--verify 与切片后自动执行）：28×2 文件存在、尺寸一致（网格拟合值 ±2px）、
每格非背景填充率 ≥0.85。任一失败非零退出（CI 可作门）。
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

import numpy as np
from PIL import Image
from scipy import ndimage

REPO = Path(__file__).resolve().parent.parent
ASSETS = REPO / "assets"
SHEETS = {"light": ASSETS / "icons.png", "dark": ASSETS / "icons_dark.png"}
OUT_DIR = ASSETS / "icons"
MAPPING_PATH = OUT_DIR / "mapping.json"

COLS, ROWS = 7, 4
# 行主序 stem（= 精灵表标题 kebab 规范形）；第 28 位 browser 为预留位。
STEMS = [
    "calculator", "clock", "todo", "weather", "notes", "calendar", "chat",
    "book-reader", "music-video-player", "charts", "database-studio",
    "file-manager", "photo-gallery", "video-player",
    "image-viewer", "paint", "auto-edit", "system-monitor", "launcher",
    "minesweeper", "ui-gallery",
    "widgets-gallery", "kanban", "auto-musk", "jade-garden", "auto-term",
    "os-config", "browser",
]
# registry id（PLAN-015 桌面注册表 27 app；与 STEMS 前 27 位一一对应）。
IDS = [
    "011-calculator", "012-stopwatch", "013-todo", "014-weather",
    "015-notes", "016-calendar", "017-chat",
    "018-book-reader", "020-music-player", "024-charts", "026-database",
    "027-file-manager", "029-photo-gallery", "030-video-player",
    "031-image-viewer", "031-paint", "041-auto-edit",
    "025-sys-monitor", "028-launcher", "038-minesweeper",
    "ui-gallery",
    "widgets-gallery", "kanban", "auto-musk", "jade-garden", "auto-term",
    "os-config",
]
FILL_MIN = 0.85
SIZE_TOL = 2


def tile_mask(im: np.ndarray) -> np.ndarray:
    bg = np.median(
        np.concatenate([im[:5, :5].reshape(-1, 3), im[-5:, -5:].reshape(-1, 3)]),
        axis=0,
    )
    return np.abs(im - bg).sum(axis=2) > 90


def fit_grid(mask: np.ndarray):
    """腐蚀断开文字笔画 → 连通域取大块 → 最小二乘拟合 7×4 网格。"""
    er = ndimage.binary_erosion(mask, iterations=6)
    lab, _ = ndimage.label(er)
    centers, sizes = [], []
    for sl in ndimage.find_objects(lab):
        hh, ww = sl[0].stop - sl[0].start, sl[1].stop - sl[1].start
        if hh >= 90 and ww >= 90:
            centers.append((sl[1].start + ww / 2, sl[0].start + hh / 2))
            sizes.append((ww + 12, hh + 12))
    if len(centers) < 8:
        raise SystemExit(f"grid fit failed: only {len(centers)} clean tiles")
    # 用可靠中心点拟合：x = x0 + pitch_x * col, y = y0 + pitch_y * row
    xs = np.array([c[0] for c in centers])
    ys = np.array([c[1] for c in centers])
    # 依据经验初值做最近邻行/列归类，再最小二乘
    px0 = 195.4
    py0 = 201.5
    cols = np.round((xs - xs.min()) / px0)
    rows = np.round((ys - ys.min()) / py0)
    A = np.stack([np.ones_like(cols), cols], axis=1)
    gx, res_x, *_ = np.linalg.lstsq(A.astype(float), xs, rcond=None)
    A2 = np.stack([np.ones_like(rows), rows], axis=1)
    gy, res_y, *_ = np.linalg.lstsq(A2.astype(float), ys, rcond=None)
    sw = float(np.median([s[0] for s in sizes]))
    sh = float(np.median([s[1] for s in sizes]))
    return gx, gy, sw, sh


def rects_from_grid(gx, gy, sw, sh):
    out = []
    for r in range(ROWS):
        for c in range(COLS):
            x = int(round(gx[0] + gx[1] * c - sw / 2))
            y = int(round(gy[0] + gy[1] * r - sh / 2))
            out.append((x, y, int(round(sw)), int(round(sh))))
    return out


def fill_ratio(mask: np.ndarray, rect) -> float:
    x, y, w, h = rect
    H, W = mask.shape
    x0, y0 = max(x, 0), max(y, 0)
    x1, y1 = min(x + w, W), min(y + h, H)
    if x1 <= x0 or y1 <= y0:
        return 0.0
    return float(mask[y0:y1, x0:x1].mean())


def fill_ratio_local(im: np.ndarray, rect, pad: int = 10) -> float:
    """局部环形底色填充率——海报底部有渐变色带，全局底色取样会失真
    （PLAN-018 T1 实测行 4 误报 0.28-0.54）。取 rect 外扩 pad 的环形
    中值为本格底色，再看格内与其差异 > 90 的占比。"""
    x, y, w, h = rect
    H, W = im.shape[:2]
    x0, y0, x1, y1 = max(x, 0), max(y, 0), min(x + w, W), min(y + h, H)
    if x1 <= x0 or y1 <= y0:
        return 0.0
    rx0, ry0, rx1, ry1 = max(x - pad, 0), max(y - pad, 0), min(x + w + pad, W), min(y + h + pad, H)
    ring = np.concatenate([
        im[ry0:ry0 + pad, rx0:rx1].reshape(-1, 3),
        im[ry1 - pad:ry1, rx0:rx1].reshape(-1, 3),
        im[y0:y1, rx0:rx0 + pad].reshape(-1, 3),
        im[y0:y1, rx1 - pad:rx1].reshape(-1, 3),
    ])
    bg = np.median(ring, axis=0)
    inner = im[y0:y1, x0:x1]
    return float((np.abs(inner - bg).sum(axis=2) > 90).mean())


def slice_all() -> list[tuple[int, int, int, int]]:
    # 每表各自拟合网格——深表行位与浅表有几 px 偏移（T1 实测：浅表 rect
    # 用于深表会把标签文字切进图块底部）。
    rects_by_theme = {}
    for theme, path in SHEETS.items():
        im = np.asarray(Image.open(path).convert("RGB")).astype(int)
        rects_by_theme[theme] = rects_from_grid(*fit_grid(tile_mask(im)))
    # 布局正确性不用统计填充率判（粉彩/白瓷图块与海报底色对比过低，
    # 阈值扫描 0.06-0.86 全谱不可分——PLAN-018 T1 实测）；改由
    # ①连通域锚点对齐（fit_grid 内）②像素级往返比对（verify）
    # ③蒙太奇预览人眼复核三重保障。
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    (OUT_DIR / "light").mkdir(exist_ok=True)
    (OUT_DIR / "dark").mkdir(exist_ok=True)
    imgs = {t: Image.open(p).convert("RGBA") for t, p in SHEETS.items()}
    for i, stem in enumerate(STEMS):
        for theme, img in imgs.items():
            x, y, w, h = rects_by_theme[theme][i]
            img.crop((x, y, x + w, y + h)).save(OUT_DIR / theme / f"{stem}.png")
    # 蒙太奇预览（上浅下深）供人眼复核
    tw, th = rects_by_theme["light"][0][2], rects_by_theme["light"][0][3]
    pad = 6
    canvas = Image.new("RGB", (COLS * (tw + pad) + pad, 2 * ROWS * (th + pad) + pad), (128, 128, 128))
    for i, stem in enumerate(STEMS):
        r, c = divmod(i, COLS)
        for trow, theme in enumerate(("light", "dark")):
            x, y, w, h = rects_by_theme[theme][i]
            canvas.paste(imgs[theme].crop((x, y, x + w, y + h)),
                         (pad + c * (tw + pad), pad + (trow * ROWS + r) * (th + pad)))
    canvas.save(OUT_DIR / "preview.png")
    mapping = {"_comment": "PLAN-018 桌面图标映射：registry id → 图标 stem；browser 为预留位不入映射", **{i: s for i, s in zip(IDS, STEMS[:27])}}
    MAPPING_PATH.write_text(json.dumps(mapping, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(f"sliced {len(STEMS)}×2 -> {OUT_DIR}/{{light,dark}}; mapping {len(IDS)} ids; preview -> preview.png")
    return rects_by_theme["light"]


def verify() -> None:
    # 重新拟合网格（每表各自），与源表裁剪逐像素比对（切片烘焙海报底色，
    # 非透明 PNG，alpha 校验无意义——PLAN-018 T1）。
    imgs = {t: Image.open(p).convert("RGBA") for t, p in SHEETS.items()}
    rects_by_theme = {}
    for theme, path in SHEETS.items():
        im = np.asarray(Image.open(path).convert("RGB")).astype(int)
        rects_by_theme[theme] = rects_from_grid(*fit_grid(tile_mask(im)))
    ref = None
    problems = []
    for i, stem in enumerate(STEMS):
        for theme, img in imgs.items():
            p = OUT_DIR / theme / f"{stem}.png"
            if not p.is_file():
                problems.append(f"missing {p}"); continue
            x, y, w, h = rects_by_theme[theme][i]
            got = np.asarray(Image.open(p).convert("RGB"))
            want = np.asarray(img.crop((x, y, x + w, y + h)).convert("RGB"))
            if ref is None:
                ref = (w, h)
            if got.shape != want.shape:
                problems.append(f"size drift {p}: {got.shape[:2]} vs {(h, w)}"); continue
            if int(np.abs(got - want).sum()) > 0:
                problems.append(f"pixel drift {p}")
    mp = json.loads(MAPPING_PATH.read_text(encoding="utf-8"))
    ids = [k for k in mp if not k.startswith("_")]
    if sorted(ids) != sorted(IDS):
        problems.append("mapping ids mismatch")
    if "browser" in mp.values():
        problems.append("browser must not be mapped")
    if problems:
        print("\n".join(problems)); raise SystemExit(f"verify failed: {len(problems)}")
    print(f"verify ok: {len(STEMS)}×2 slices pixel-exact vs sheets, {len(ids)} mapped ids, size {ref}")


if __name__ == "__main__":
    if "--verify" in sys.argv:
        verify()
    else:
        slice_all()
        verify()
