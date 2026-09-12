# -*- coding: utf-8 -*-
"""PLAN-014 mock 图生成：深色/浅色双主题 ×（桌面全景 + 通知中心/切换器）共四张。
版式、元素位置、尺寸、标注编号与引线关系在两主题间完全一致，仅做配色映射。"""
import math
import os
import random
from PIL import Image, ImageDraw, ImageFont, ImageFilter

W, H = 1920, 1080
FONT_PATH = "C:/Windows/Fonts/msyh.ttc"
OUT_DIR = "D:/autostack/auto-os/docs/plans/evidence/014"

ACCENT = (167, 139, 250)          # 主题紫 #a78bfa（双主题共用）
ACCENT_DEEP = (139, 92, 246)
WHITE = (255, 255, 255)
RED = (239, 68, 68)
CHIP = {
    "Calculator": (178, 118, 66),
    "Todo": (139, 92, 246),
    "Notes": (199, 166, 62),
    "Files": (59, 130, 246),
}

# ---------- 双主题调色板（light 沿用 shadcn 浅色惯例） ----------
THEMES = {
    "dark": {
        "wall_c1": (96, 42, 148), "wall_c2": (16, 20, 54),
        "blob_a": (240, 150, 210, 12), "blob_b": (150, 110, 250, 14),
        "dot_pink": (244, 190, 224), "dot_white": (255, 255, 255),
        "glass": (22, 22, 36, 205), "glass_border": (255, 255, 255, 48),
        "shadow_alpha": 95,
        "titlebar_strip": (255, 255, 255, 14),
        "sep": (255, 255, 255, 30),
        "win_dot": (255, 255, 255, 70), "win_dot_close": (255, 120, 120, 170),
        "display_bg": (10, 10, 22, 190), "display_fg": WHITE,
        "key": (47, 47, 63), "key_fg": WHITE, "key_orange": (228, 124, 50), "key_orange_fg": WHITE,
        "taskbar": (21, 21, 34, 238), "taskbar_line": (255, 255, 255, 40),
        "btn_fill": (255, 255, 255, 18), "btn_fg": WHITE,
        "entry_fill": (255, 255, 255, 14), "bar_blur": (150, 150, 165),
        "clock_fg": WHITE, "clock_sub": (168, 168, 184),
        "menu_bg": (24, 24, 38, 242), "hover": (255, 255, 255, 20),
        "sel_block": (255, 255, 255, 30), "sel_block_border": (255, 255, 255, 70),
        "launch_dot": (190, 190, 200), "launch_bar": (255, 255, 255, 90),
        "note_line": (255, 255, 255, 55),
        "status_bg": (255, 255, 255, 16), "status_border": (255, 255, 255, 40),
        "ok": (74, 222, 128), "err": (248, 113, 113), "info": (175, 175, 190),
        "close": (255, 255, 255, 130),
        "hover_row": (255, 255, 255, 20),
        "preselect_fill": ACCENT_DEEP + (150,), "preselect_border": ACCENT + (220,), "preselect_text": WHITE,
        "thumb_fill": (255, 255, 255, 16), "thumb_border": (255, 255, 255, 45), "thumb_hatch": (255, 255, 255, 16),
        "fg": WHITE, "sub": (168, 168, 184),
        "accent": ACCENT,
        "accent_text": ACCENT,
        "display_border": None,
        "leader": (255, 255, 255, 110), "leader_dot": (255, 255, 255, 170),
        "caption": (255, 255, 255, 105),
    },
    "light": {
        # stella light tokens（事实源 auto-lang design_tokens/registry.rs）
        "wall_c1": (245, 241, 232), "wall_c2": (239, 233, 221),  # background #F5F1E8 → accent #EFE9DD
        "blob_a": (226, 205, 165, 25), "blob_b": (210, 190, 160, 20),
        "dot_pink": (222, 184, 135), "dot_white": (125, 119, 109),
        "glass": (251, 248, 242, 230), "glass_border": (227, 221, 209, 235),  # card #FBF8F2 / border #E3DDD1
        "shadow_alpha": 60,
        "titlebar_strip": (42, 39, 35, 10),
        "sep": (42, 39, 35, 50),
        "win_dot": (42, 39, 35, 70), "win_dot_close": (239, 68, 68, 200),  # destructive #EF4444
        "display_bg": (251, 248, 242, 215), "display_fg": (42, 39, 35), "display_border": (227, 221, 209, 235),
        "key": (227, 221, 209), "key_fg": (42, 39, 35), "key_orange": (228, 124, 50), "key_orange_fg": WHITE,
        "taskbar": (251, 248, 242, 235), "taskbar_line": (42, 39, 35, 35),
        "btn_fill": (42, 39, 35, 9), "btn_fg": (42, 39, 35),
        "entry_fill": (42, 39, 35, 7), "bar_blur": (125, 119, 109),
        "clock_fg": (42, 39, 35), "clock_sub": (125, 119, 109),  # foreground / muted-foreground #7D776D
        "menu_bg": (251, 248, 242, 248), "hover": (0, 0, 0, 9),
        "sel_block": (227, 221, 209, 120), "sel_block_border": (125, 119, 109, 100),
        "launch_dot": (125, 119, 109), "launch_bar": (42, 39, 35, 70),
        "note_line": (42, 39, 35, 65),
        "status_bg": (42, 39, 35, 8), "status_border": (42, 39, 35, 60),
        "ok": (34, 197, 94), "err": (239, 68, 68), "info": (59, 130, 246),  # #22C55E / #EF4444 / #3B82F6
        "close": (42, 39, 35, 110),
        "hover_row": (0, 0, 0, 8),
        "preselect_fill": (100, 102, 241, 225), "preselect_border": (100, 102, 241, 255),
        "preselect_text": (248, 250, 252),  # primary #6466F1 / primary-foreground #F8FAFC
        "thumb_fill": (42, 39, 35, 10), "thumb_border": (42, 39, 35, 70), "thumb_hatch": (42, 39, 35, 20),
        "fg": (42, 39, 35), "sub": (125, 119, 109),  # #2A2723 / #7D776D
        "accent": (100, 102, 241),  # primary indigo
        "accent_text": (100, 102, 241),
        "leader": (60, 55, 45, 160), "leader_dot": (60, 55, 45, 220),
        "caption": (125, 119, 109),  # #7D776D
    },
}
T = THEMES["dark"]


def use_theme(name):
    global T
    T = THEMES[name]


def f(size, bold=False):
    idx = 1 if bold else 0
    try:
        return ImageFont.truetype(FONT_PATH, size, index=idx)
    except Exception:
        return ImageFont.truetype(FONT_PATH, size)


F10 = f(10)
F10B = f(10, True)
F12 = f(12)
F12B = f(12, True)
F13 = f(13)
F13B = f(13, True)
F14 = f(14)
F15 = f(15)
F22 = f(22)


# ---------- 基础绘制 ----------
def wallpaper():
    gw, gh = 320, 180
    grad = Image.new("RGB", (gw, gh))
    px = grad.load()
    for y in range(gh):
        for x in range(gw):
            t = (x / (gw - 1) + y / (gh - 1)) / 2
            px[x, y] = tuple(int(a + (b - a) * t) for a, b in zip(T["wall_c1"], T["wall_c2"]))
    base = grad.resize((W, H), Image.BICUBIC).convert("RGBA")

    ov = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    rnd = random.Random(14)
    # 大面积柔光斑，低对比
    for _ in range(7):
        cx, cy = rnd.randint(0, W), rnd.randint(0, H)
        r = rnd.randint(70, 220)
        col = T["blob_a"] if rnd.random() < 0.5 else T["blob_b"]
        d.ellipse([cx - r, cy - r, cx + r, cy + r], fill=col)
    ov = ov.filter(ImageFilter.GaussianBlur(50))
    # 小圆点 / 花瓣装饰
    d = ImageDraw.Draw(ov)
    for _ in range(42):
        cx, cy = rnd.randint(0, W), rnd.randint(0, H)
        r = rnd.uniform(1.2, 3.6)
        a = rnd.randint(18, 46)
        col = T["dot_pink"] + (a,) if rnd.random() < 0.45 else T["dot_white"] + (max(a - 6, 12),)
        d.ellipse([cx - r, cy - r, cx + r, cy + r], fill=col)
    base.alpha_composite(ov)
    return base


def shadow(base, box, radius=14, blur=14, offset=(0, 7)):
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    x1, y1, x2, y2 = box
    d.rounded_rectangle([x1 + offset[0], y1 + offset[1], x2 + offset[0], y2 + offset[1]],
                        radius=radius, fill=(0, 0, 0, T["shadow_alpha"]))
    ov = ov.filter(ImageFilter.GaussianBlur(blur))
    base.alpha_composite(ov)


def glass_card(base, box, radius=14, with_shadow=True):
    if with_shadow:
        shadow(base, box, radius=radius)
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    d.rounded_rectangle(box, radius=radius, fill=T["glass"], outline=T["glass_border"], width=1)
    base.alpha_composite(ov)
    return ImageDraw.Draw(base)


def text(d, xy, s, font, fill, anchor="la"):
    d.text(xy, s, font=font, fill=fill, anchor=anchor)


def annotate(base, num, cx, cy, tx, ty):
    """主题紫圆数字标注 + 1px 引出线 + 末端 2px 小圆点。"""
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    dx, dy = tx - cx, ty - cy
    dist = math.hypot(dx, dy) or 1
    sx, sy = cx + dx / dist * 12, cy + dy / dist * 12
    d.line([sx, sy, tx, ty], fill=T["leader"], width=1)
    d.ellipse([tx - 2, ty - 2, tx + 2, ty + 2], fill=T["leader_dot"])
    d.ellipse([cx - 11, cy - 11, cx + 11, cy + 11], fill=T["accent"], outline=(255, 255, 255, 220), width=1)
    d.text((cx, cy - 0.5), str(num), font=F12B, fill=WHITE, anchor="mm")
    base.alpha_composite(ov)


def caption(base, s):
    d = ImageDraw.Draw(base)
    text(d, (W / 2, 1052), s, F10, T["caption"], anchor="mm")


def taskbar_strip(base):
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    d.rectangle([0, H - 56, W, H], fill=T["taskbar"])
    d.line([0, H - 56, W, H - 56], fill=T["taskbar_line"], width=1)
    base.alpha_composite(ov)


# ---------- 字形（简化图标） ----------
def _pts(pts):
    return [(int(round(a)), int(round(b))) for a, b in pts]


def glyph_calc(d, cx, cy, col=WHITE):
    s, gap = 5, 3
    x0, y0 = cx - (3 * s + 2 * gap) / 2, cy - (3 * s + 2 * gap) / 2
    for r in range(3):
        for c in range(3):
            x = x0 + c * (s + gap)
            y = y0 + r * (s + gap)
            d.rounded_rectangle([x, y, x + s, y + s], radius=1.5, fill=col)


def glyph_check(d, cx, cy, col=WHITE, w=3):
    d.line(_pts([(cx - 8, cy + 1), (cx - 2, cy + 7), (cx + 9, cy - 7)]), fill=col, width=int(round(w)), joint="curve")


def glyph_notes(d, cx, cy, col=WHITE):
    d.rounded_rectangle([cx - 8, cy - 9, cx + 8, cy + 9], radius=2, outline=col, width=2)
    d.line(_pts([(cx - 4, cy - 3), (cx + 4, cy - 3)]), fill=col, width=2)
    d.line(_pts([(cx - 4, cy + 3), (cx + 4, cy + 3)]), fill=col, width=2)


def glyph_folder(d, cx, cy, col=WHITE):
    d.polygon([(cx - 9, cy - 3), (cx - 9, cy - 8), (cx - 2, cy - 8), (cx + 1, cy - 5),
               (cx + 9, cy - 5), (cx + 9, cy + 7), (cx - 9, cy + 7)], fill=col)


def glyph_search(d, cx, cy, col):
    d.ellipse([cx - 8, cy - 8, cx + 3, cy + 3], outline=col, width=3)
    d.line(_pts([(cx + 2, cy + 2), (cx + 9, cy + 9)]), fill=col, width=3)


def glyph_dashboard(d, cx, cy, col):
    s, gap = 8, 3
    x0, y0 = cx - s - gap / 2, cy - s - gap / 2
    for r in range(2):
        for c in range(2):
            x = x0 + c * (s + gap)
            y = y0 + r * (s + gap)
            d.rounded_rectangle([x, y, x + s, y + s], radius=2.5, fill=col)


def glyph_grid9(d, cx, cy, col):
    s, gap = 4, 2.5
    x0, y0 = cx - (3 * s + 2 * gap) / 2, cy - (3 * s + 2 * gap) / 2
    for r in range(3):
        for c in range(3):
            x = x0 + c * (s + gap)
            y = y0 + r * (s + gap)
            d.rounded_rectangle([x, y, x + s, y + s], radius=1.2, fill=col)


def glyph_bell(d, cx, cy, col):
    d.pieslice([cx - 8, cy - 9, cx + 8, cy + 7], start=180, end=360, fill=col)
    d.rectangle([cx - 8, cy - 1, cx + 8, cy + 6], fill=col)
    d.ellipse([cx - 2, cy + 7, cx + 2, cy + 11], fill=col)


def glyph_gear(d, cx, cy, col):
    d.ellipse([cx - 7, cy - 7, cx + 7, cy + 7], outline=col, width=2)
    d.ellipse([cx - 2.5, cy - 2.5, cx + 2.5, cy + 2.5], outline=col, width=2)
    for k in range(8):
        a = math.radians(k * 45)
        px, py = cx + 9.5 * math.cos(a), cy + 9.5 * math.sin(a)
        d.rectangle([px - 2, py - 2, px + 2, py + 2], fill=col)


def glyph_power(d, cx, cy, col):
    d.arc([cx - 8, cy - 8, cx + 8, cy + 8], start=300, end=240, fill=col, width=3)
    d.line(_pts([(cx, cy - 1), (cx, cy - 10)]), fill=col, width=3)


def glyph_status(d, cx, cy, kind):
    d.ellipse([cx - 11, cy - 11, cx + 11, cy + 11], fill=T["status_bg"], outline=T["status_border"], width=1)
    if kind == "ok":
        glyph_check(d, cx, cy, col=T["ok"], w=2.5)
    elif kind == "err":
        d.line(_pts([(cx - 5, cy - 5), (cx + 5, cy + 5)]), fill=T["err"], width=3)
        d.line(_pts([(cx + 5, cy - 5), (cx - 5, cy + 5)]), fill=T["err"], width=3)
    else:
        d.line(_pts([(cx, cy - 2), (cx, cy + 6)]), fill=T["info"], width=3)
        d.ellipse([cx - 1.5, cy - 7, cx + 1.5, cy - 4], fill=T["info"])


def glyph_close(d, cx, cy, col=None):
    if col is None:
        col = T["close"]
    d.line(_pts([(cx - 4, cy - 4), (cx + 4, cy + 4)]), fill=col, width=2)
    d.line(_pts([(cx + 4, cy - 4), (cx - 4, cy + 4)]), fill=col, width=2)


# ---------- 图一：桌面全景 ----------
def desktop_icon(base, name, x, y, selected=False, launching=False):
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    if selected:
        d.rounded_rectangle([x - 12, y - 8, x + 60, y + 76], radius=12,
                            fill=T["sel_block"], outline=T["sel_block_border"], width=1)
    col = CHIP[name]
    chip_fill = col + (170,) if launching else col + (255,)
    d.rounded_rectangle([x, y, x + 48, y + 48], radius=12, fill=chip_fill)
    cx, cy = x + 24, y + 24
    if name == "Calculator":
        glyph_calc(d, cx, cy)
    elif name == "Todo":
        glyph_check(d, cx, cy)
    elif name == "Notes":
        glyph_notes(d, cx, cy)
    else:
        glyph_folder(d, cx, cy)
    if launching:
        d.ellipse([x + 40, y - 4, x + 52, y + 8], fill=T["launch_dot"])
        d.rounded_rectangle([x, y + 62, x + 48, y + 65], radius=1.5, fill=T["launch_bar"])
    text(d, (x + 24, y + 52), name, F12, T["fg"], anchor="ma")
    base.alpha_composite(ov)


def calc_window(base, box):
    x1, y1, x2, y2 = box
    glass_card(base, box, radius=14)
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    # 标题栏
    d.rounded_rectangle([x1, y1, x2, y1 + 36], radius=14, fill=T["titlebar_strip"])
    d.rectangle([x1, y1 + 22, x2, y1 + 36], fill=T["titlebar_strip"])
    d.line([x1 + 1, y1 + 36, x2 - 1, y1 + 36], fill=T["sep"], width=1)
    text(d, ((x1 + x2) / 2, y1 + 18), "计算器", F13, T["fg"], anchor="mm")
    for i in range(3):
        cxx = x2 - 20 - i * 24
        cc = T["win_dot_close"] if i == 2 else T["win_dot"]
        d.ellipse([cxx - 4.5, y1 + 18 - 4.5, cxx + 4.5, y1 + 18 + 4.5], fill=cc)
    # 显示屏
    d.rounded_rectangle([x1 + 16, y1 + 48, x2 - 16, y1 + 104], radius=10, fill=T["display_bg"],
                        outline=T["display_border"], width=1)
    text(d, (x2 - 30, y1 + 76), "1,024", F22, T["display_fg"], anchor="rm")
    base.alpha_composite(ov)
    # 键盘
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    keys = [["C", "±", "%", "÷"], ["7", "8", "9", "×"], ["4", "5", "6", "−"],
            ["1", "2", "3", "+"], ["0", ".", "⌫", "="]]
    gx, gy, gap = x1 + 16, y1 + 116, 8
    kw = (x2 - x1 - 32 - 3 * gap) / 4
    kh = (y2 - 16 - gy - 4 * gap) / 5
    for r, row in enumerate(keys):
        for c, k in enumerate(row):
            kx = gx + c * (kw + gap)
            ky = gy + r * (kh + gap)
            if c == 3:
                d.rounded_rectangle([kx, ky, kx + kw, ky + kh], radius=9, fill=T["key_orange"])
                kc = T["key_orange_fg"]
            else:
                d.rounded_rectangle([kx, ky, kx + kw, ky + kh], radius=9, fill=T["key"])
                kc = T["key_fg"]
            text(d, (kx + kw / 2, ky + kh / 2 - 1), k, F14, kc, anchor="mm")
    base.alpha_composite(ov)


def notes_window(base, box):
    x1, y1, x2, y2 = box
    glass_card(base, box, radius=14)
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    d.rounded_rectangle([x1, y1, x2, y1 + 36], radius=14, fill=T["titlebar_strip"])
    d.rectangle([x1, y1 + 22, x2, y1 + 36], fill=T["titlebar_strip"])
    d.line([x1 + 1, y1 + 36, x2 - 1, y1 + 36], fill=T["sep"], width=1)
    text(d, ((x1 + x2) / 2, y1 + 18), "便签 — Notes", F13, T["fg"], anchor="mm")
    for i in range(3):
        cxx = x2 - 20 - i * 24
        d.ellipse([cxx - 4.5, y1 + 18 - 4.5, cxx + 4.5, y1 + 18 + 4.5], fill=T["win_dot"])
    widths = [330, 285, 320, 250, 300, 170]
    ly = y1 + 60
    for wln in widths:
        d.rounded_rectangle([x1 + 20, ly, x1 + 20 + wln, ly + 6], radius=3, fill=T["note_line"])
        ly += 24
    d.rectangle([x1 + 20, y1 + 56, x1 + 23, y1 + 68], fill=T["accent"])
    base.alpha_composite(ov)


def context_menu(base, box):
    items = ["更换壁纸…", "显示设置", "整理图标", "恢复默认图标", "显示桌面"]
    glass_card(base, box, radius=12)
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    x1, y1, x2, y2 = box
    d.rounded_rectangle([x1 + 4, y1 + 5, x2 - 4, y1 + 5 + 32], radius=8, fill=T["hover"])
    for i, it in enumerate(items):
        cy = y1 + 5 + i * 32 + 16
        text(d, (x1 + 16, cy), it, F13, T["fg"], anchor="lm")
    base.alpha_composite(ov)


def taskbar_btn(base, x, glyph_fn, badge=None):
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    y = H - 56
    d.rounded_rectangle([x, y + 8, x + 40, y + 48], radius=10, fill=T["btn_fill"])
    glyph_fn(d, x + 20, y + 28, T["btn_fg"])
    if badge:
        bx, by = x + 40 - 12, y + 8 - 4
        d.ellipse([bx, by, bx + 16, by + 16], fill=RED, outline=(255, 255, 255, 220), width=1)
        text(d, (bx + 8, by + 8), badge, F10B, WHITE, anchor="mm")
    base.alpha_composite(ov)
    return x + 40


def taskbar_entry(base, x, name, focused, glyph_fn):
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    y = H - 56
    tw = d.textlength(name, font=F10)
    w = 16 + 24 + 6 + tw + 14
    d.rounded_rectangle([x, y + 8, x + w, y + 48], radius=10, fill=T["entry_fill"])
    d.rounded_rectangle([x + 12, y + 16, x + 36, y + 40], radius=7, fill=CHIP[name] if name in CHIP else ACCENT_DEEP)
    glyph_fn(d, x + 24, y + 28)
    text(d, (x + 42, y + 28), name, F10, T["fg"], anchor="lm")
    bar_col = T["accent"] if focused else T["bar_blur"]
    d.rounded_rectangle([x + 12, y + 52, x + w - 12, y + 55], radius=1.5, fill=bar_col)
    base.alpha_composite(ov)
    return x + w + 8


def fig_desktop():
    base = wallpaper()
    # 窗口
    calc_window(base, (150, 100, 530, 560))
    notes_window(base, (1220, 480, 1640, 780))
    # 桌面图标（左上紧凑单列）
    names = ["Calculator", "Todo", "Notes", "Files"]
    glyphs = {"Calculator": glyph_calc, "Todo": glyph_check, "Notes": glyph_notes, "Files": glyph_folder}
    for i, n in enumerate(names):
        desktop_icon(base, n, 32, 40 + i * 110, selected=(n == "Calculator"), launching=(n == "Todo"))
    # 空白菜单
    context_menu(base, (600, 484, 800, 484 + 10 + 5 * 32 + 7))
    # 任务栏
    taskbar_strip(base)
    x = taskbar_btn(base, 12, glyph_search)
    x = taskbar_entry(base, x + 8, "计算器", True, glyph_calc)
    x = taskbar_entry(base, x, "便签", False, glyph_notes)
    # 右侧按钮组：切换器 / 布局 / 铃铛(角标) / 设置 / 电源
    rx = W - 12 - 40
    taskbar_btn(base, rx, glyph_power)
    rx -= 48
    taskbar_btn(base, rx, glyph_gear)
    rx -= 48
    taskbar_btn(base, rx, glyph_bell, badge="3")
    bell_cx = rx + 20
    rx -= 48
    taskbar_btn(base, rx, glyph_grid9)
    rx -= 48
    taskbar_btn(base, rx, glyph_dashboard)
    # 时钟
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    ccx = rx - 14 - 40
    text(d, (ccx, 1038), "14:32", F15, T["clock_fg"], anchor="ma")
    text(d, (ccx, 1058), "9月14日 周一", F10, T["clock_sub"], anchor="ma")
    base.alpha_composite(ov)
    # 标注
    annotate(base, 1, 210, 68, 96, 74)
    annotate(base, 2, 128, 232, 84, 162)
    annotate(base, 3, 545, 468, 598, 520)
    annotate(base, 4, 140, 950, 50, 1031)
    annotate(base, 5, 320, 950, 236, 1032)
    annotate(base, 6, bell_cx + 8, 950, bell_cx + 8, 1029)
    annotate(base, 7, ccx, 950, ccx, 1030)
    label = "改进后桌面" if T is THEMES["dark"] else "改进后桌面 · 浅色"
    caption(base, f"PLAN-014 mock · {label}（非实现截图）")
    return base


# ---------- 图二：通知中心 + 切换器 ----------
def notify_row(base, box, kind, msg, tm, hover=False):
    x1, y1, x2, y2 = box
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    if hover:
        d.rounded_rectangle(box, radius=8, fill=T["hover_row"])
    cy = (y1 + y2) / 2
    glyph_status(d, x1 + 22, cy, kind)
    text(d, (x1 + 44, cy - 10), msg, F13, T["fg"], anchor="lm")
    text(d, (x1 + 44, cy + 12), tm, F10, T["sub"], anchor="lm")
    glyph_close(d, x2 - 18, cy)
    base.alpha_composite(ov)


def notify_center(base, box):
    x1, y1, x2, y2 = box
    glass_card(base, box, radius=14)
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    text(d, (x1 + 16, y1 + 22), "通知", F13B, T["fg"], anchor="lm")
    text(d, (x2 - 16, y1 + 22), "全部清除", F10, T["accent_text"], anchor="rm")
    d.line([x1 + 16, y1 + 42, x2 - 16, y1 + 42], fill=T["sep"], width=1)
    base.alpha_composite(ov)
    rows = [("ok", "计算器 · 计算完成", "14:30", False),
            ("err", "便签 · 同步失败，请稍后重试", "14:28", True),
            ("info", "系统 · 有新的更新可用", "13:55", False)]
    ry = y1 + 52
    for kind, msg, tm, hover in rows:
        notify_row(base, (x1 + 10, ry, x2 - 10, ry + 62), kind, msg, tm, hover)
        ry += 70


def thumb(base, x, y):
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    d.rounded_rectangle([x, y, x + 96, y + 54], radius=8, fill=T["thumb_fill"], outline=T["thumb_border"], width=1)
    for i in range(1, 8):
        d.line(_pts([(x + i * 14, y + 54), (x, y + 54 - i * 14)]), fill=T["thumb_hatch"], width=1)
    base.alpha_composite(ov)


def switcher_row(base, box, title, preselected=False):
    x1, y1, x2, y2 = box
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    if preselected:
        d.rounded_rectangle(box, radius=10, fill=T["preselect_fill"])
        d.rounded_rectangle(box, radius=10, outline=T["preselect_border"], width=1)
    base.alpha_composite(ov)
    cy = (y1 + y2) / 2
    thumb(base, x1 + 12, int(cy - 27))
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    text(d, (x1 + 124, cy - 1), title, F13, T["preselect_text"] if preselected else T["fg"], anchor="lm")
    base.alpha_composite(ov)


def switcher(base, box):
    x1, y1, x2, y2 = box
    glass_card(base, box, radius=14)
    ov = Image.new("RGBA", base.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    text(d, (x1 + 20, y1 + 26), "切换窗口 / MRU", F13B, T["fg"], anchor="lm")
    text(d, (x2 - 20, y1 + 26), "Alt+Tab", F10, T["sub"], anchor="rm")
    base.alpha_composite(ov)
    titles = ["计算器", "便签 — Notes", "文件管理", "终端"]
    ry = y1 + 48
    for i, t in enumerate(titles):
        switcher_row(base, (x1 + 12, ry, x2 - 12, ry + 62), t, preselected=(i == 1))
        ry += 70


def fig_panels():
    base = wallpaper()
    taskbar_strip(base)
    # 通知中心（右下锚定）
    nbox = (W - 12 - 320, H - 56 - 12 - 258, W - 12, H - 56 - 12)
    notify_center(base, nbox)
    # 切换器（中上部居中）
    sw, sh = 560, 12 + 36 + 8 + 4 * 62 + 3 * 8 + 12
    sbox = ((W - sw) / 2, 240, (W + sw) / 2, 240 + sh)
    switcher(base, sbox)
    # 标注
    annotate(base, 8, 1510, 822, nbox[0] + 2, 822)
    annotate(base, 9, 1510, 966, nbox[0] + 84, 977)
    annotate(base, 10, 1290, 389, sbox[2] - 10, 389)
    label = "通知中心 + 切换器改进" if T is THEMES["dark"] else "通知中心 + 切换器改进 · 浅色"
    caption(base, f"PLAN-014 mock · {label}")
    return base


def main():
    os.makedirs(OUT_DIR, exist_ok=True)
    for theme, suffix in (("dark", ""), ("light", "-light")):
        use_theme(theme)
        fig_desktop().save(os.path.join(OUT_DIR, f"014-mock-desktop{suffix}.png"))
        fig_panels().save(os.path.join(OUT_DIR, f"014-mock-panels{suffix}.png"))
    print("saved 4 mocks:", OUT_DIR)


if __name__ == "__main__":
    main()
