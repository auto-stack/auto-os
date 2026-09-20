# PLAN-035 T-00 归因工件（环境口径与实机载决）

- 构建：auto-lang worktree `.wt/os-035/auto-lang` @ `auto-os-035-dev`
  （master 干净树 4aadc1f57）+ auto-os shell pack `.wt/os-035/auto-os/shell`
  @ `plan-035-dev`（4ec4f88）。依赖组：auto-down @ `auto-os-035-dev`。
- 驱动：`tmp/p035/drive.py`（AUTOUI_ACCEPTANCE=1；隔离 storage）。
  headless 探针：auto-lang `ui/iced/layout_tests.rs::p035_taskbar_right_group_right_aligned`。
- 用户环境旁证：用户截图面板 ~1164 逻辑 px ≠ master 872（R10 十列）——
  用户二进制滞后 master；但**五题在 master 全部成立**，居中 bug 与二进制
  新旧无关。

## 载决表

| # | 问题 | master 表现 | 载决 | 去向 |
|---|---|---|---|---|
| 1 | 任务栏右组居中 | **成立**（s1/s3 实测） | **Q-A②**：`apply_column_style` justify-center 臂给无 width 类列的包装容器 `width=Fill`（renderer.rs `justify_center → col_w = Fill`）——clock 列（`items-center justify-center px-2`）成为任务栏行第二个 Fill 元素，与 spacer FillPortion 平分自由空间 → 图标组被夹中央。headless 探针红测钉死：spacer 与 clock 容器各分得 405.5px | T-01：shell.at clock 列去 justify-center（行 items-center 已承载垂直居中）；探针转绿 = 回归守护；SD-05 登记语义 |
| 2 | 任务栏壁纸 picker 游离块 | **成立且更深**：boot 态即有**关闭态 popover 幽灵渲染**——blank 菜单三文本（更换壁纸/显示设置/恢复默认图标）50% 透明度常驻桌面左下（b0，未做过任何交互）；任务栏区另有白色小卡幽灵（b0 任务栏内）。用户可点击的「浏览…」块 = 同族（面板命中/绘制在关闭态泄漏）。popover widget 关闭臂本身干净（open=false 不出 panel overlay），vtree 中亦无幽灵节点 → 根因在更深的面/合成层（rq 表面/canvas 族，PLAN-032..034 线） | T-02（本仓可落）：desktop.at 三处坐标锚 popover 内容包显式 open 态 if 守卫（内容不物化即无幽灵可画可点）+ renderer picker 坐标钳制；深层根因 → DEBTS 登记（证据指针齐） | T-02 + 债 |
| 3 | sliver 宽/高亮/提示/溢出 | 成立（截图右缘形态与用户一致） | 按计划修 | T-03 |
| 4 | 面板非 8×3 网格 | 成立（master = 10 列 872 外框 + 内部三等分 sx/sy 缩放） | 按计划修 | T-04 |
| 5 | 三 mini 细节 | 成立（s3：clock 内容右倾；music「本地曲库为空」把控件挤出卡外——卡底只露按钮残边；todo 空疏） | 按计划修 | T-05..07 |

## 证据索引

- `t00_s1_picker_open.png`：picker 开启态（正常渲染，居中 720 宽）+ 左下
  幽灵 + 图标组居中 + 任务栏白卡幽灵。
- `t00_s3_dashboard.png`：面板 872 三等分（T-04 前形态）+ clock 右倾 +
  music 控件被裁 + 左下幽灵（Esc 关 picker 后仍在）。
- `t00_b0_boot12s.png` / `_bl.png`：**boot 12s 无交互**——幽灵已常驻
  （blank 菜单三文本 + 任务栏白卡）。
- `t00_b1_blankmenu.png`：handler 强开 blank 菜单——面板本体不可见
  （仅同款幽灵），面板 bg 未绘而文本绘出 → 合成层 drop fill op 族。
- `t00_vtree_props.atom.txt`：boot 树无 blank 菜单节点（幽灵不在 VM 视图
  树内 → 渲染/合成层产物）；任务栏 spacer x60 w405.5（headless 探针同
  分布）。
- headless 红测输出：`L=x21 S=x486.5 … P=x798.5`（1280 行宽，右组被
  Fill 平分挤到中央）+ containers 表（spacer 405.5 / clock 容器 405.5）。

## 附带观察（不在本计划范围）

- 负一屏/初始桌面左上角蓝色「?」圆徽（s1/s3）——疑似某图标位图 miss 的
  fallback 臂，另行排查。
- ui_desktop 调试构建首帧布局 >5s（screenshot 臂 "window size is zero"
  守卫间歇拦截）——验收脚本已配 12s boot 等待。
