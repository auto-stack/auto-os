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
- **几何吸附（PLAN-035 SD-01 修订：8×3 定框 + 网格算术卡位）**：面板
  外框 = 屏幕右上 **8 列 × 3 行** 图标网格块（696×232 @ 12px 边距；列距
  88 = w-20 80+gap 8、行距 80 = h-[72px] 72+gap 8）——用户 2026-09-20
  裁定（原 10 列框 + 内部三等分 sx/sy 缩放退役）。**无头行**
  （PLAN-035 rev4 T-18：72px tab 头行占满一整行网格且空旷，退役——
  分页触发改外框右上角紧凑 pill，main↔system 互换，SelectTab 既有
  消息面；lazy 语义不变：非活动页 face 不渲染 + 孵化 Tick 停订）；
  face 卡 = 网格单元整数倍：宽 **2 格（168）缺省 / 3 格（256）声明或
  存储**，高 = **满高 3 行格（232）**，卡框线落 88/80 节距网格线；
  `dashboard_layout` 直出视口绝对矩形（外框 + 格位同一算式，chrome
  wrapper 与 face 卡永远同源）。8 列单卡行装不下者裁剪 + dev 日志
  （滚动/增高挂平板网格 v2，PLAN-024 §9.3）。
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
- **格位（PLAN-035 SD-02 修订）**：`dashboard_layout` 行主序单卡行
  next-fit（**span 2|3 缺省 2**，卡高恒 2 行格 = 152px；余量不足裁剪）；
  span 三级消费序 = 存储覆写 `shell.dashboard.span.<app>` → **app 源
  声明标记**（`view mini` 邻域 240 字符内 `span: N` 文本探测，注释形态
  `// dashboard span: 3`——parser view-tag 无 props 通道，文本契约与
  `view mini` 探测同族）→ 缺省 2；旧存储 "1" 迁移读作 2。face 卡 px
  spacer 链定位（真实 Stack 子层 padding/align 不可依赖——notification
  O1 家法）。
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
- **孵化会话上行（PLAN-035 SD-03）**：联合排空段扩容——windowless
  孵化会话不在 `wm.wins`，其 `__desktop_cmd`（notify 词面）自
  PLAN-035 起并入注册表窗分段排空（registry_id = 孵化映射键，归因
  沿用；词表白名单不变）。孵化发件方自持去重旗标（如 music 空曲库
  一次性通知），宿主 push_notification 另有尾条 kind+msg 去重。

## SD-04 配置与持久化

- `shell.dashboard.enabled`（csv 纳入清单；缺席 = 未配置 = 首次召唤
  自动纳入全部候选）、`span.<app>`（PLAN-035："2"|"3"，缺省 2；旧
  "1" 迁移读作 2）。宿主
  `storage_host_read`/`storage_host_publish` 直读写（非几何无动词，
  boot 生效——既定判定）；编辑 popover UI 入口延后（F-01，命令/存储链
  已通并有词表测试）。
- 协议 v1.8（auto-lang `schema/projection-protocol-v1.md` §6）：入向
  `__dashboard_faces`/`__wm_dashboard`/几何注入；出向
  `__dashboard_cmd`（toggle/close/pin/unpin/span/launch 六动词，面板
  上行）+ shell `__desktop_cmd` 的 `dashboard_toggle`（dock 钮）。
