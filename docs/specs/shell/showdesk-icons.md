# Spec: 桌面图标网格 / 任务栏图标消费 / 切换器预览（showdesk icons）

> Source of truth for desktop icon grid, taskbar icon consumption, and
> workspace-switcher preview semantics introduced by PLAN-022
> (icon-taskbar-polish，R1–R17 用户裁定全轮打磨).
> 布局记忆键控（拖拽落格写路/每壁纸 positions）归
> `docs/specs/shell/showdesk-wallpaper.md` SD-04；图标资产管线与
> mapping.json schema 归 README §图标资产（PLAN-018）；iconfile 协议族
> 归 auto-lang `docs/specs/auto-lang/ui/overview.md` §icon 字符串协议族。
> 实现：auto-os `shell/desktop.at` + `shell/shell.at`；auto-lang
> `ui/iced/renderer.rs`（desktop_icon_cells / icon_drag 门 /
> __desktop_label_dark / publish_workspace_previews）。

## SD-01 桌面图标网格

- **满幅 tile**：图标 48px（`w-12 h-12`）满幅渲染——位图自带圆角板，
  不套 chip 外框/描边（PLAN-018-FU2 用户裁定推广到桌面格）；格高固定
  `h-[72px]`（标题回归 + 杀 grid 行拉伸）。018-FU3 边框高亮退役。
- **标题三级配色**：图片壁纸 = 白字 + `bg-black/30` 底片（暗壁纸可读）；
  纯色壁纸按亮度自适应（宿主 `__desktop_label_dark`：`#hex` 直算 luma、
  图片 32×32 均值解码按路径缓存；亮度累加走 u16 防溢出）。
- **hover**：`bg-white/10` 整组底（图标 + 标题一体）。
- **排布（列主序）**：未定位图标按列主序填充——左列自上而下占满再排
  下一列（`desktop_icon_cells` 收 `rows` 参数；rows = 视口高 / 80px 行距
  扣任务栏预留，容量下限保证）。已定位图标按 SD-04（壁纸 spec）键控表
  last-wins 解析 (c,r)。
- **拖拽/点击判别（6px 阈值）**：拾起记录起点光标（`session.icon_drag`
  附 origin 元组）；位移 < 6px 视为点击——两条落格动词（全局
  `__mouse_released` 兜底臂 + BlankDrop `desktop_icon_drop_at`）统一受
  `icon_drag_moved` 门：拒落、只清视觉态。保证单击/双击不跳格、第一击
  不闪拖拽副本。超阈拖拽 = 幽灵跟随（`drag_icon`/`drop_c`/`drop_r`
  数据面坐标锚 popover）+ 落点格高亮；落格写路 = 壁纸 spec SD-04 键。

## SD-02 任务栏图标消费

- **通道**：按钮 `icon (name: "iconfile:<stem>")` 双主题位图臂；资产根
  = `AUTO_OS_ROOT/assets/icons/{light,dark}/<stem>.png`，主题跟随
  `dark_mode()`。pinned/窗口条目图标经 app registry mapping 链解析
  （`mapping.json` 别名 → stem）。
- **变体**：`variant: "ghost"`（去 PLAN-571 预设发丝描边——位图外圆角框
  来源）；满幅 tile 语义同 SD-01，lucide 字标才保留品牌色 chip。
- **几何**：`text-4xl`（36px 图标）+ `h-11 w-11`（hover/激活底 44px）；
  激活条带出现不抬升图标。
- **激活条带（R17）**：`h-1 w-6 rounded-full bg-primary`，`mt-[3px]`
  **五臂同值**（pinned 焦点/运行/透明占位三态 + 窗口条目二态）——
  2px 时条带视觉贴住 44px 高亮框，3px 为用户 3x 放大截图实测缝隙裁定。
  改条带偏移必须五臂一起改。

## SD-03 切换器预览发布

- **独立化**：`publish_workspace_previews` 独立函数（boot/refresh 与
  tick 两调用点），预览数据发布不再搭其他重建便车。
- **SWR 补抓**：切换面板开着时由 400ms ServiceTick 逐可见窗
  `request_capture` → 快照入缓存 → 重新发布预览数据 + shell 重建；面板
  收起即停止（零常态开销）。
- **新建桌面卡**：预览网格尾部常驻"+"卡（`WorkspaceAdd` →
  `__desktop_cmd = "workspace_add"`）。
- **已知债（非阻塞）**：深色主题下深色窗缩略贴深色底对比度低——后续
  预览底换浅色 surface 或瓦片加 1px 描边。

## 验证

- 代码标记（HEAD 复核）：shell.at 7 枚 iconfile 按钮全 ghost、`mt-[3px]`
  五臂；desktop.at `w-12 h-12`/`h-[72px]`/`bg-black/30`；renderer
  `icon_drag_moved`（renderer×5/session×2）、`desktop_icon_cells(rows)`、
  `publish_workspace_previews` 双调用点。
- lang 回归：023 r1 全量 tv 3743/3743（树含 022 双提交
  94bc69c22+9bd26d884）；a2vue 金样对拍重生成（auto-lang 0407a9f9b，
  desktop.at 022 网格终版双端同源）。
- 实机：执行期用户逐条截图验收 R1–R17（条带缝隙 3x 放大实测在案；
  拖拽手感反馈通道开放）。
