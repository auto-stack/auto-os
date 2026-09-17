# PLAN-014 rev2 desktop mock redraw (T-05) — 022 grid semantics + W-04/W-05'/W-06'/W-07
from PIL import Image, ImageDraw, ImageFont

W, H = 1920, 1080
FONT = "C:/Windows/Fonts/msyh.ttc"

def font(sz):
    return ImageFont.truetype(FONT, sz)

def rounded(draw, xy, r, fill):
    draw.rounded_rectangle(xy, radius=r, fill=fill)

def draw_mock(path, dark):
    if dark:
        c_bg1, c_bg2 = (45, 36, 64), (23, 18, 38)
        c_bar = (42, 37, 54, 242)
        c_text = (235, 232, 240)
        c_muted = (160, 155, 170)
        c_sel = (255, 255, 255, 26)
        c_hover = (255, 255, 255, 18)
        c_menu = (36, 32, 46, 250)
        c_win = (30, 27, 38, 250)
        c_accent = (124, 108, 240)
        c_labelplate = (0, 0, 0, 77)
        c_icon_fill = (124, 108, 240)
        c_badge = (239, 68, 68)
        c_badge_text = (255, 255, 255)
        c_dot = (140, 140, 150)
    else:
        c_bg1, c_bg2 = (232, 225, 211), (208, 199, 182)
        c_bar = (247, 244, 239, 242)
        c_text = (61, 55, 41)
        c_muted = (120, 112, 96)
        c_sel = (227, 221, 209, 170)
        c_hover = (0, 0, 0, 12)
        c_menu = (252, 250, 246, 250)
        c_win = (250, 248, 244, 250)
        c_accent = (100, 102, 241)
        c_labelplate = (0, 0, 0, 0)
        c_icon_fill = (100, 102, 241)
        c_badge = (239, 68, 68)
        c_badge_text = (255, 255, 255)
        c_dot = (120, 120, 128)
    img = Image.new("RGBA", (W, H), c_bg1)
    d = ImageDraw.Draw(img)
    # vertical gradient wallpaper
    for y in range(H):
        t = y / H
        col = tuple(int(c_bg1[i] + (c_bg2[i] - c_bg1[i]) * t) for i in range(3)) + (255,)
        d.line([(0, y), (W, y)], fill=col)
    f_label = font(13)
    f_menu = font(15)
    f_clock = font(15)
    f_date = font(11)
    f_badge = font(11)
    f_note = font(13)

    # ---- desktop icon grid (022: 8 cols, cell 80x72, gap 8, icon 48 full-bleed,
    # column-major; W-04 selected white/10 rounded block; launching = 50% + dot) ----
    x0, y0 = 12, 12
    cell_w, cell_h, gap = 80, 72, 8
    icons = ["Calc", "Todo", "Notes", "Files"]
    for i, name in enumerate(icons):
        col, row = divmod(i, 4)  # column-major: fill column top->bottom
        cx = x0 + col * (cell_w + gap)
        cy = y0 + row * (cell_h + gap)
        selected = name == "Todo"
        launching = name == "Notes"
        if selected or launching:
            rounded(d, (cx, cy, cx + cell_w, cy + cell_h), 8, c_sel)
        icon_cx, icon_cy = cx + cell_w // 2, cy + 24
        if launching:
            tile = Image.new("RGBA", (48, 48), (0, 0, 0, 0))
            td = ImageDraw.Draw(tile)
            rounded(td, (0, 0, 48, 48), 10, c_icon_fill + (128,))
            td.text((24, 24), name[0], font=font(22), fill=(255, 255, 255, 200), anchor="mm")
            img.alpha_composite(tile, (int(icon_cx - 24), int(icon_cy - 24)))
            d.ellipse((cx + cell_w - 9, cy + 2, cx + cell_w - 3, cy + 8), fill=c_dot)
        else:
            rounded(d, (icon_cx - 24, icon_cy - 24, icon_cx + 24, icon_cy + 24), 10, c_icon_fill)
            d.text((icon_cx, icon_cy), name[0], font=font(22), fill=(255, 255, 255), anchor="mm")
        label_y = cy + 52
        if dark:
            d.rounded_rectangle((cx + 4, label_y, cx + cell_w - 4, label_y + 18), 4, fill=c_labelplate)
            d.text((cx + cell_w // 2, label_y + 9), name, font=f_label, fill=(255, 255, 255), anchor="mm")
        else:
            d.text((cx + cell_w // 2, label_y + 9), name, font=f_label, fill=c_text, anchor="mm")

    # ---- a normal app window (context) ----
    wx, wy, ww, wh = 620, 180, 480, 320
    rounded(d, (wx, wy, wx + ww, wy + wh), 10, c_win)
    d.line([(wx, wy + 36), (wx + ww, wy + 36)], fill=c_muted + (60,), width=1)
    d.text((wx + ww // 2, wy + 18), "计算器", font=font(14), fill=c_text, anchor="mm")

    # ---- blank context menu (W-05': 更换壁纸…/显示设置/恢复默认图标; no 显示桌面) ----
    mx, my, mw, mh = 640, 560, 176, 116
    rounded(d, (mx, my, mx + mw, my + mh), 6, c_menu)
    items = ["更换壁纸…", "显示设置", "恢复默认图标"]
    for i, it in enumerate(items):
        iy = my + 10 + i * 32
        if it == "恢复默认图标":
            rounded(d, (mx + 4, iy - 4, mx + mw - 4, iy + 26), 4, c_hover)
        d.text((mx + 14, iy + 7), it, font=f_menu, fill=c_text)

    # ---- taskbar ----
    bar_y = H - 56
    d.rectangle((0, bar_y, W, H), fill=c_bar)
    d.line([(0, bar_y), (W, bar_y)], fill=c_muted + (40,), width=1)
    # launcher + pinned/running apps
    bx = 10
    for name, running in [("S", False), ("计算器", True), ("便签", True)]:
        if len(name) == 1:
            rounded(d, (bx, bar_y + 10, bx + 44, bar_y + 54), 12, c_hover)
            d.text((bx + 22, bar_y + 32), name, font=font(22), fill=c_text, anchor="mm")
        else:
            rounded(d, (bx, bar_y + 10, bx + 44, bar_y + 54), 12, c_icon_fill)
            d.text((bx + 22, bar_y + 32), name[0], font=font(22), fill=(255, 255, 255), anchor="mm")
            d.rounded_rectangle((bx + 14, bar_y + 50, bx + 30, bar_y + 53), 2,
                                fill=c_accent if name == "便签" else c_muted)
        bx += 48
    # clock two lines (W-06': HH:MM + M月D日 周X)
    d.text((1650, bar_y + 20), "14:32", font=f_clock, fill=c_muted, anchor="mm")
    d.text((1650, bar_y + 40), "9月17日 周四", font=f_date, fill=c_muted, anchor="mm")
    # layout buttons
    for k in range(2):
        lx = 1716 + k * 48
        rounded(d, (lx, bar_y + 10, lx + 44, bar_y + 54), 12, c_hover)
    # notification bell with circular badge (W-07: 16px circle, -top-1 -right-1)
    nb_x, nb_y = 1812, bar_y + 10
    rounded(d, (nb_x, nb_y, nb_x + 44, nb_y + 44), 12, c_hover)
    d.text((nb_x + 22, nb_y + 22), "N", font=font(18), fill=c_text, anchor="mm")
    d.ellipse((nb_x + 36, nb_y - 4, nb_x + 52, nb_y + 12), fill=c_badge)
    d.text((nb_x + 44, nb_y + 4), "3", font=f_badge, fill=c_badge_text, anchor="mm")
    # config + shutdown (schematic glyphs)
    for k in range(2):
        sx = 1866 - k * 48
        rounded(d, (sx, bar_y + 10, sx + 44, bar_y + 54), 12, c_hover)
        d.text((sx + 22, bar_y + 32), "C" if k == 0 else "P", font=font(16), fill=c_muted, anchor="mm")
    # showdesk sliver
    d.rectangle((W - 12, bar_y + 10, W, bar_y + 54), fill=c_muted + (40,))

    # ---- annotations ----
    def note(x, y, num, text):
        d.ellipse((x - 12, y - 12, x + 12, y + 12), fill=c_accent)
        d.text((x, y), str(num), font=f_note, fill=(255, 255, 255), anchor="mm")
        d.text((x + 18, y), text, font=f_note, fill=c_text, anchor="lm")
    note(240, 40, 1, "单击选中：白/10 圆角高亮块（W-04，022 网格 80px 格/48px 满幅位图）")
    note(240, 84, 2, "启动中：chip 半透明 + 右上角灰点（W-04，__wm_running 扩注 ack 收敛）")
    note(840, 610, 3, "空白菜单 +恢复默认图标（W-05'，hidden 去重 + refresh 即时回）")
    note(1450, 40, 4, "时钟两行 HH:MM / M月D日 周X（W-06'，独立脏帧）")
    note(1450, 84, 5, "未读圆形角标压位图右上 + 9+ 截断（W-07）")
    d.text((W // 2, bar_y - 16), "PLAN-014 rev2 mock · 桌面（022 网格语境）", font=f_note, fill=c_muted, anchor="mm")

    img.convert("RGB").save(path)
    print("saved", path)

draw_mock("014-rev2-mock-desktop.png", dark=True)
draw_mock("014-rev2-mock-desktop-light.png", dark=False)
