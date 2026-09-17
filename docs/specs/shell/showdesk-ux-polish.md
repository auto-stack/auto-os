# Spec: 桌面 shell UX 打磨第二批（rev2 九项行为合同）

> Source of truth for the desktop selection/launch feedback, hidden-dedup
> + icon restore, clock/date injection, unread badge, notification source
> jump, and switcher preselect/hover semantics introduced by PLAN-014
> (rev2 保留九项中六项行为面；W-01/W-02 清理与总线语义归投影协议 v1.8
> 登记，不在本 spec 重复)。Protocol face: auto-lang
> `schema/projection-protocol-v1.md` v1.8 (§2 fields, §4 verbs)。
> Host implementation: auto-lang `ui/iced/renderer.rs`（注入/派生泵、
> notify 归因）+ `ui/session.rs`（WmState 簿记）。
> Shell consumption: auto-os `shell/desktop.at`（SD-01/02）、
> `shell/shell.at`（SD-03/04）、`shell/notification_center.at`（SD-05）、
> `shell/switcher.at`（SD-06）。

## SD-01 桌面图标选中态 + 启动中反馈

- **选中态**（`sel_id`，desktop.at 本地态）：图标格 mouse-area
  `onclick: .IconPress(e.id)` 置位（单击选中；与 022 拖拽阈值臂互洽——
  IconPress 既有拖拽拾起判定先行，选中只置本地字符串态，不动拖拽状态
  机，拖拽不误置选中、选中不误触拖拽）。空白点击 `BlankPress` 清空
  （连同菜单簿记）。视觉 = 选中格 `rounded-lg bg-white/10` 圆角高亮块
  （022 网格语境 rev2 形态：格 w-20 h-[72px]、图标 48px 满幅）。
- **启动中反馈**（`launching`，desktop.at 本地态）：双击
  `ActivateApp(app)` 置 `launching = app` 后发 activate 动词；视觉 =
  选中底 + `opacity-50` 半透明 + 右上角灰点（absolute
  `w-1.5 h-1.5 rounded-full bg-muted-foreground`，`top-0.5 right-1`）。
- **收敛两臂**（launching 无独立超时——以 running 集为 ack）：
  1. `RunningSync` 召唤臂：宿主 `__wm_running` 集合变化时
     call_handler 召唤（写状态不触发 handler 律），running 含 launching
     id（`,` + id + `,` 域串包含判据）即清——启动成功 ack；
  2. `Init` 臂求差自愈：装载/重注入时 running 不含 launching id 即清
     ——启动失败/过期残态不自愈则永驻灰点，此臂兜底。
- **注入面**（协议 v1.8 §2 登记）：`__wm_running`（v1 既有字段）注入
  面由 shell 层扩至 desktop 层——字段不新增，扩的是注入对象。

## SD-02 hidden 去重 + 恢复默认图标

- **去重**：`MenuRemove`（格右键「从桌面移除」）写
  `shell.desktop.hidden` 前做 `,` + id + `,` 域串包裹 contains 判重
  ——重复移除同 id 不产生重份（012 v1.6 handler 侧合法先例）。
- **恢复默认图标**：空白菜单（019 后形态）追加「恢复默认图标」
  ghost 项 = `ResetIconsBlank` → `storage.set("shell.desktop.hidden",
  "")` 单源清空 + 本地 `__desktop_hidden` 清空 +
  `refresh_desktop_icons` 即时重注入（协议 v1.6 既有动词；storage 已
  清故即时臂与重启全量回同效——验收两口径任一）。**不改 019 空白菜单
  簿记**（更换壁纸/显示设置项原样）。
- hidden 单源 = `shell.desktop.hidden` storage 键（012 定案不变）。

## SD-03 时钟两行 + 日期注入面

- **宿主注入**（`update_shell_clock` 泵）：分钟变化写 `__wm_clock`
  （"HH:MM"）；日期变化写 `__wm_date`（"M月D日 周X"，中文周几宿主
  chrono 格式化）。两字段**独立脏帧**——各自变化才写，另一字段不变
  时零写入零重建（稳态零重建口径维持）。
- **shell.at 消费**：托盘时钟区两行 col——上行 HH:MM（原样式）、下行
  日期（弱化色）。**无点击臂**（注册表无日历 app，PLAN-014 §10-Q3
  挂账；日后立项日历 app 时另行补线）。
- 协议 v1.8 §2 新字段：`__wm_date`。

## SD-04 未读角标形态

- 计数合同不变：`__wm_notes_unread`（数值）照旧；显示串由宿主派生
  `__wm_notes_badge`——unread > 9 → `"9+"`，0 → `""`，其余为十进制
  串（.at 视图无数值比较原语，I9 单点派生）。
- 视觉 = notification 位图钮包 relative col，badge 为 absolute 16px
  圆（`rounded-full bg-error text-primary-foreground`）压位图右上角
  （`-top-1 -right-1`）；数字 >9 显示 "9+"。双端（iced/Vue）同
  class 同形态（505 B1 数据驱动口径）；浅色主题角标 #EF4444 不变。

## SD-05 通知来源跳转

- **来源记录**：宿主 notify 落库记来源 app id——`notify_source` 分段
  归因（联合排空泵按注册表窗段分段执行，段前置置 source、命令顺序与
  扁平 concat 逐一相同，Shutdown 短路按段传递；特权段恒 None）。
  `NotificationEntry.app` 随 persist/restore 持久化（旧库缺键 → ""）。
- **注入面**（协议 v1.8 §2）：`__wm_notes` 增 `app` 字段（合同面）；
  B12 规避期平行列表同型加 `note_apps`——**召唤与活更新两个注入点都
  必须写**（单点漏写 → RebuildNotes 下标读 IndexError，域外防御读
  兜底）。W-14（B12 宿主修复）落地后 `note_apps` 消参（挂账
  PLAN-014 §10-Q5）。
- **跳转**：通知整行 mouse-area `onclick: .OpenSource(r.app)` →
  `activate\t<app>` 动词（复用 472 activate 两臂：未运行启动/运行聚
  焦，零新动词）。`app == ""`（无来源/特权通知/旧库条目）= no-op；
  跳转即收面板（配合 Esc/外点关闭语义回归绿）。

## SD-06 switcher 预选第 2 项 + hover 跟随

- **预选**：`RebuildMru` 重建快照后 `nres > 1 → sel = 1`（Alt-Tab 惯
  例——MRU 第 1 项 = 当前聚焦窗，Enter 即切最近其他窗）；单条目/空表
  保持 `sel = 0`。
- **hover 跟随**：行 mouse-area `onmouseenter: .HoverSel(r.i)` 置
  `sel = i`（`visible == "1"` 门控——隐藏态悬停不置位）；键盘
  Advance/Back 从当前 sel 继续（环走），无特判即与鼠标互洽。
- 快照语义不变：召唤时点定序，打开期间 Advance 不重注（MRU 切换器
  惯例，Plan 478 D1）。

## 验收锚（对应 PLAN-014 §7）

- AC-02 ← SD-01/SD-02：选中/启动反馈双轨双主题对拍 rev2 mock；启动
  失败残态自愈；hidden 去重无重份；恢复默认图标即时回（= 重启全量
  回）；022 拖拽族回归零红。
- AC-03 ← SD-03/SD-04：两行时钟分钟/日期独立刷新（跨天用例）+ 脏帧
  独立；圆形角标压位图右上、9+ 截断。
- AC-04 ← SD-05/SD-06：通知行点击跳来源两臂；外点关面板回归；
  switcher 预选第 2 项、Enter 切最近其他窗、hover 与键盘互洽。
