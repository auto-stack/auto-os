#!/usr/bin/env python3
"""Generate experimental/autoime/windows/ime/autoime.ico (PLAN-069 T-02).

16x16 32bpp icon: AutoIME brand color background with a white "A"-like mark.
Pure stdlib: writes the ICO container with an embedded uncompressed BMP.
"""
import struct

SIZE = 16
BG = (0x2B, 0x6C, 0xF0, 0xFF)        # blue, BGRA
FG = (0xFF, 0xFF, 0xFF, 0xFF)        # white

# 16x16 blocky "A" glyph grid ('1' = foreground pixel)
A = [["0"] * 16 for _ in range(16)]
for c in range(3, 13):
    A[2][c] = "1"            # top bar
    A[3][c] = "1"
for r in range(4, 13):       # legs
    A[r][3] = "1"; A[r][4] = "1"
    A[r][11] = "1"; A[r][12] = "1"
for r in range(7, 9):        # crossbar
    for c in range(5, 11):
        A[r][c] = "1"

rows = []
for y in range(SIZE - 1, -1, -1):        # BMP rows bottom-up
    for x in range(SIZE):
        b, g, r_, a = FG if A[y][x] == "1" else BG
        rows.append(struct.pack("<BBBB", b, g, r_, a))
xor = b"".join(rows)
and_mask = b"\x00" * (SIZE * ((SIZE + 31) // 32) * 4)   # fully opaque

bmp_header = struct.pack("<IiiHHIIiiII", 40, SIZE, SIZE * 2, 1, 32, 0,
                         len(xor) + len(and_mask), 0, 0, 0, 0)
image = bmp_header + xor + and_mask
ico_header = struct.pack("<HHH", 0, 1, 1)
entry = struct.pack("<BBBBHHII", SIZE, SIZE, 0, 0, 1, 32,
                    len(image), 6 + 16)
with open("ime/autoime.ico", "wb") as f:
    f.write(ico_header + entry + image)
print("ime/autoime.ico written,", 6 + 16 + len(image), "bytes")
