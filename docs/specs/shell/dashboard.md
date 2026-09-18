# Spec: 桌面常驻小组件层 / 面板 / 孵化会话（dashboard）

> Source of truth for the resident desktop widget layer introduced by
> PLAN-024 (dashboard-widgets，S10；用户裁定 R1–R21 走查收敛——召唤式
> v1 形态已升级为**常驻层**). 布局与图标网格键控归
> `docs/specs/shell/showdesk-icons.md` / `showdesk-wallpaper.md`；
> 语言扩展（`view mini` 多命名视图）与孵化/升格会话归 auto-lang
> `docs/specs/auto-lang/ui/architecture.md` ADR-21；协议词表归 auto-lang
> `schema/projection-protocol-v1.md` §v1.8。
> 实现：auto-os `shell/dashboard.at`（面板 chrome）+ `shell/shell.at`
> （dock ▦ 钮）；auto-lang `ui/iced/renderer.rs`（dashboard_layout /
> toggle_dashboard / refresh_dashboard_panel / dynamic_view_impl face 分支 /
> open 拦截臂）+ `ui/session.rs`（split_ref_dashboard/face / hatch_mini_app /
> open_window_for_session）+ `ui/dynamic.rs`（view_named）。

## SD-01 常驻小组件层（z 序与显隐）

- **z 序**：面板与 face 卡的层插在壁纸（+scrim）之上、桌面图标层与全部
  app 虚拟窗之下（"仅高于壁纸"的用户语义与 R12 可交互性折衷：图标层
  全屏 BlankPress mouse-area 会吞面板 click，故面板必须在其**上层**；
  视觉上面板右上与图标网格默认不重叠）。app 窗照常遮挡面板（R3）。
- **常驻语义（取代 v1 召唤式）**：boot 即挂载并显示（`toggle_dashboard`
  首调=挂载+显示，幂等）；**× = 隐藏**（visible="0"）、**dock ▦ = 切换**；
  无 scrim、无外点关闭、无 Esc 关闭。可见性单一事实 = 面板 .at
  `visible` state（switcher/通知先例），投影 `__wm_dashboard`（"1"/""）
  供 dock 钮两态高亮。
- **几何吸附**：面板外框吸附桌面图标网格——列距 88（w-20 80+gap 8）、
  行距 80（h-[72px] 72+gap 8）、原点 12；宽 10 列（872）、高 3 行（232）
  起，右上 12px 对齐（图标列主序占左，右上无碰撞）；内部格位按面板宽
  等比缩放（`dashboard_layout` 面单相对坐标 + 调用方单一 panel_x 注入，
  chrome wrapper 与 face 卡永远同源）。resize 重叠问题挂平板网格 v2
  （PLAN-024 §9.3）。
- **tab 化**：双固定页签「小组件」(main) /「系统」(system)（stella
  widget-tabs 语言：居中 pill，激活 `bg-primary/15 text-primary`）。
  face → tab 映射 = 注册表 category（`system` → 系统页，其余 → 小组件
  页）。**降耗**：非活动页/面板隐藏时孵化会话 `.Tick` 停订（订阅随消息
  周期重评估）——重轮询组件"平时不看不影响 CPU"。
- **× hover 显隐**：面板级 mouse-area enter/leave → show_close；× 条件
  渲染，槽位固定不跳版。

## SD-02 face（活渲染面）

- **face = app `view mini` 命名视图的宿主拆借渲染**（`view_named`，与
  主窗同 component/VM 桥——状态一致即视觉一致，非截图/缩放）；事件带
  face app 标签直达该会话（卡内交互零中转）。
- **格位**：`dashboard_layout` 行主序 next-fit（等宽 3 列 + span 1|2，
  span 存 `shell.dashboard.span.<app>`）；face 卡 px spacer 链定位
  （真实 Stack 子层 padding/align 不可依赖——notification O1 家法）。
- **卡体双击 → 三态打开**：已有窗 → activate 聚焦（跨分区）；孵化会话
  → **升格开窗**（`open_window_for_session`——为既有会话建虚拟窗，
  face/窗零分家）；无会话 → launch。合成消息 `__dashboard_open:<id>`
  + update 拦截臂；卡内交互控件优先命中（N6d 内外层机制）。
- **占位卡**：无会话且不可孵化（daemon/back_root/exe 门）→ 宿主合成面
  （标题 + "未运行 — 点击启动"），点击 = `__dashboard_launch:<id>`。

## SD-03 faces 推导与孵化

- **两级推导**：①注册表全量扫 `view mini` 文本探测（grep 级，含未
  运行）；②会话化后 `named_views()` 精确确认（文本误报兜底）。
- **三态**：running（有窗）/ hatched（静默孵化 windowless 会话）/
  placeholder（不可孵化）。**孵化门**：非 outproc/exe 且
  `daemon`/`back_root` 均无（inproc 合并 VM 内 back_port 不构成外部
  依赖——025-sys-monitor 以孵化形态直显真数据）。孵化会话常驻不回收；
  `shell.dashboard.enabled`（csv，缺席=全纳入）过滤 + span 读回。
- **注入面**：`face_ids/titles/icons/statuses/spans/tabs` 平行字符串
  列表 + `__dashboard_faces` 合同面 Obj 数组 + `__panel_w/h/top` px
  几何（宿主单一事实，.at 镜像）。faces 读回必须走物化读
  （`read_state_as_vec`——write_state_vec 落 VM 堆为 VmRef）。

## SD-04 配置与持久化

- `shell.dashboard.enabled`（csv 纳入清单；缺席 = 未配置 = 首次召唤
  自动纳入全部候选）、`span.<app>`（"1"|"2"）。宿主
  `storage_host_read`/`storage_host_publish` 直读写（非几何无动词，
  boot 生效——既定判定）；编辑 popover UI 入口延后（F-01，命令/存储链
  已通并有词表测试）。
- 协议 v1.8（auto-lang `schema/projection-protocol-v1.md` §6）：入向
  `__dashboard_faces`/`__wm_dashboard`/几何注入；出向
  `__dashboard_cmd`（toggle/close/pin/unpin/span/launch 六动词，面板
  上行）+ shell `__desktop_cmd` 的 `dashboard_toggle`（dock 钮）。
