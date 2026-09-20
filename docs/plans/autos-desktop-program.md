# AutoOS 虚拟桌面程序总览（Program Tracker）

> **性质**：活账（living tracker）。每次计划状态头变更时同步本文件（已写入各计划
> finish-plan 步骤）。详细状态只住在各计划自己的状态头里，这里只放指针级一行。
> **架构依据**：`docs/design/autoui/virtual-desktop.md`（下称 Design 23）。
> **本文拥有的东西**：依赖图、入口条件仪表盘、裁定登记簿。计划内部进度不在此抄。
>
> **台账接棒（Stage B P-5，2026-09-07，auto-lang PLAN-590）**：本台账自
> auto-lang `docs/plans/autos-desktop-program.md` 迁入（git 历史留档源仓）。
> 自此桌面域计划状态变更**只记本侧**（Design 01 §5 迁移机制 4，单一事实源）；
> auto-lang 侧 `docs/plans/INDEX.md` 留去向指针行。随批同迁：apps/
> {028-launcher, 025-sys-monitor, 038-minesweeper, common/settings} +
> 顶层 {ui-gallery, widgets-gallery}（经 P-3 容器探测/解析序定位注册与消费）。

## 目标

一套窗口管理代码（WM 是 AutoUI App），在 Win/Mac 上是单 OS 窗口的虚拟桌面，
在 Linux 上是原生合成器宿主，在 Web/移动端是同构嵌入——设计级一致，
App 一次编写处处原生。里程碑 M0-M6 详见 Design 23 §6。

## 计划一览

452 已立项（2026-08-26）；453+ 编号以立项时实际分配为准。

> **当前状态**：452/453/459 已完成归档（2026-08-28，M1 收口：多 App 会话 + daemon
> 多窗口 + panic 隔离）。2026-08-28 立项 M2–M4 计划族（Design 24：
> `docs/design/autoui/desktop-shell-and-launcher.md`）——462/463/464/465；
> **462 已完成**（单 OS 窗口多虚拟窗口 + WM 最小集，实机验收 + 3222 测试全绿），
> ~~463/464/465 未开工~~ → 463/464/465 均已交付并归档（2026-09-01 Plan 513 回写）。

**编号解析规则**：本表是"里程碑 ↔ 实际计划号"的**唯一事实源**。Design 23 与
各计划正文中的 453–457 字样均为**提案编号**，一律经本表解析为实际编号。
立项时若实际编号顺延（如 453 被其他工作占用、里程碑 M1 实际立项为 458），
只需三步：① 本表"计划"列改实际编号并在单元格内注记原提案号；② 本节追加
一行映射说明；③ 之后的新计划引用实际编号。**不逐处改正文里的提案编号**。

| 计划 | 里程碑 | 范围一行 | 状态 | 依赖 |
|---|---|---|---|---|
| 452 设计+裁定翻转+IME spike | M0 | 正式收编 Design 23；执行 §5 同步清单；跑 §9 spike | ✅ 完成 2026-08-26（报告 reports/452-ime-spike.md，已归档） | 无 |
| 453 多 App 会话运行时 | M1 | AppSession/DesktopSession；(AppId,·) 扇出；panic 边界；多 OS 窗口验证 | ✅ 完成 2026-08-28（随 459 验收一并归档） | 452 ✅ |
| 459 DesktopSession 多窗口化 | M1 收口 | iced daemon 迁移；AppId 递增分配与打标；双 AppSession 双窗口 demo；panic 隔离验证 | ✅ 完成 2026-08-28（C0–C4 + 实机验收，已归档） | 453 ✅ |
| 462（原提案 454）VirtualWindow + WM | M2 | 路线 A：`virtual_window` 注册 widget、chrome、事件/焦点分区、桌面宿主入口、R4 接缝 v1（`462-virtual-window-wm.md`） | ✅ 完成 2026-08-28（实机双窗 demo 全交互 + `cargo t` 3222 绿 + I2 五套 desktop_mcp 全绿；IME 两项残留转 463） | 459 ✅ |
| 463（原提案 455）桌面 shell | M3 | 全屏 borderless、shell pack+任务栏、layout 三模式+snap、`desktop.*` 命令接缝、workspace 驱动模型、桌面热键、pac.at 注册表（`463-desktop-shell-auto-arrange.md`；Design 25 §3/§4.1/§6 对齐） | ✅ 完成 2026-08-28（复审通过 + 合入归档 + specs 沉淀 P463-1..4） | 462 ✅ |
| 464 Launcher App（吸收 441） | M3 组成 | `examples/ui/028-launcher`：palette+网格双形态、模糊搜索、键盘流、真注册表+`LaunchApp`（`464-launcher-app.md`） | 📦 已归档——✅ 已交付（2026-08-29 收口并 specs 沉淀；吸收 441；原格"未开工"为立项时残留，2026-09-01 Plan 513 回写） | 462+463 |
| 465（原提案 456）Vue 虚拟桌面 | M4 | DOM 嵌入 + 多挂载宿主（`createApp`/虚拟窗）+ registry 构建期生成 + tauri 全屏壳 + E1/E2 接缝语义（`465-vue-virtual-desktop.md`） | 📦 已归档——✅ 已交付（2026-08-29 收口并 specs 沉淀；原格"未开工"为立项时残留，2026-09-01 Plan 513 回写） | 462 契约（与 463/464 并行） |
| 472 shell-track M1 | M3+ | **AutoShell 地基**：状态投影协议 v1（`__wm_*` 族 schema 化）+ workspace 驱动模型补课（463 §3.6 未实施）+ dock 升级（图标/pinned/配置）（`472-shell-track-m1-projection-dock.md`） | 📦 已归档——✅ execution_done 2026-08-29（T1–T6 全落；cargo t 3234/3234 绿；协议合同 schema/projection-protocol-v1.md；复审以 /auto-plan:review 承载） | 463 ✅ + 464 ✅ |
| shell-track M2（478） | M3+ | **switcher overlay + workspace pager**：Ctrl+Tab 召唤 MRU 面板（464 overlay 同型第二槽）+ dock 升格 pager（1 基标签/当前高亮/增删分区）+ send_to 跨区发送 + 协议 v1.1（`478-shell-track-m2-switcher-pager.md`） | 📦 已归档——🔄 execution_done 2026-08-29（T1–T7 全落；cargo t 3236/3236 + ui-iced 档 3879/3879 绿；I2 五套 desktop_mcp 14/11/11/19/26 与 472 基线同数；T6 键流实机项注入通道受阻按 472 先例 headless 指针成文；复审以 /auto-plan:review 承载；缩略挂 386、shell IME 挂 457 不变） | 472 ✅ + 464 ✅ |
| shell-track M3（479） | M3+ | **通知中心（S6）+ desktop.notify 动词**：463 瞬时 toast 升格「浮现+历史聚合」双面——dock 铃铛+未读 badge + 第三枚 overlay 槽面板（逐条 ×/全部清除/Esc）+ notify 出向动词 + storage 定长槽持久化 + 协议 v1.2（`479-shell-track-m3-notification-center.md`） | 📦 已归档——🔄 execution_done 2026-08-29（T1–T7 全落；cargo t 3253/3253 + ui-iced 档 3912/3913 绿——唯一失败 `vm_code_editor_natives_end_to_end` 为 master 既有（Plan 474 并行会话同款，本仓 master 检出复现，与本计划零交集）；I2 五套 14/11/11/19/26 与 472 基线同数；T6 铃铛渲染/落盘/boot 恢复实机 PASS（机会性实机流量铁证），面板交互项按 472/478 先例 headless 指针成文；复审以 /auto-plan:review 承载；勿扰/设置挂 M4、OS 桥挂 457/386 不变） | 478 ✅ + 472 ✅ |
| **509**（原提案 457）Smithay 宿主 Stage 1 | M5 | Linux 原生合成器宿主，复用桌面 shell——路线评估定案（auto-cosmic 复活/libcosmic/Smithay+桌面协议宿主三路线）+ 最小骨架 + shell 首帧 + I1 零分叉核对（`509-smithay-host-stage1.md`） | 🗄️ archived 2026-09-01（T1 裁定 **B**：Smithay+桌面协议宿主，smithay 0.7.0 入 manifest——报告 `509-smithay-route-verdict.md`；T5 环境 = WSL2+WSLg，基线成文；T2 双平台编译绿（host-smithay 新 crate）；T4 合成循环实跑 + 像素证据；T3 shell 首帧上屏（生产链渲染→宿主纹理）+ I1 diff 干净（auto-lang 仅 3 文件 cfg 差异）；live attach 登记 Stage 2；复审全量门 cargo tf 3350/3350 绿，I1 复验零行变更；merge 前置 = 报告 §7 大依赖确认位勾销） | 462+463 ✅ |
| 386（**已复活**） | M6 | 路线 B：桌面协议五通道 + 双模 exe + 形态迁移，Stage 1-3 重构见 386 文件 §0 | 🔄 复活 2026-08-28（Stage 1 即刻可开工，前置仅 462✅+蓝图；Stage 2 待 463/464，Stage 3 含内存实测验收） | Stage1: 462✅；Stage2: +463/464 |

### Stage B 随迁计划区（P-1 批自 auto-lang 迁入，2026-09-07；origin 溯源见各件 frontmatter）

| os 计划 | origin | 状态指针 |
|---|---|---|
| 002 desktop-ux-followups | PLAN-535 | 🔄 executing（2026-09-09：A1/A2 已交付实机验证——根因=convert_view_messages 缺 WindowThumbnail 臂（D-GAP 第四例）+desktop.at T36 括号回归+resolve 组目录解析，三单修；auto-lang `efc7e64b9`/auto-os `56cc364`。**B 已交付**（布局件交互原语：`wrap_layout_events` 三臂=onclick/oncontextmenu/hover 变体类，新增 `hover_area.rs` 零重建 hover；试点=launcher 网格格+桌面图标格；auto-lang `d5b345fb1`/auto-os `fa77bc9`，设计稿 `docs/design/autoui/layout-interaction.md`，实机 scratch/p002）。C/E 未完） |
| **003 clock-app** | PLAN-554 | **📦 已归档——✅ 已交付（2026-09-08 merge）**——Clock 四 tab（秒表真走表/计时器横幅/世界时钟 8 城/闹钟 storage 5 槽）desktop_mcp 12/12；框架交付 ts_adapter Time vue 桥；债候选 P003-R1..R3（VM str to_int 接收者/Tick 后重渲染面/Lap 劫持）移交框架侧；复审/合并以 /auto-plan:review/:merge 承载 |
| **004 games-wave1** | PLAN-556 | **🗄️ 已归档/搁置（2026-09-14）**——底层能力由 003-clock/005-tetris/006-klondike 充分验证，自带游戏生态由 minesweeper/klondike/tetris 经典三件套覆盖，032–035 小体量 demo 批次按裁定不再单独开发，计划归档封存 |
| 005 tetris | PLAN-557 | 🔄 executing（2026-09-14：revision 2，标准前后端分层 + M1–M7 多端模式矩阵 + 20ms Tick/Ghost/SRS 规则验证；worktree `os-005` 执行收尾中） |
| **006 klondike** | PLAN-558 | **📦 已归档——✅ 已交付（2026-09-14 merge）**——037-klondike 纸牌接龙入 `apps/`（ports 17600/17601）；前后端分层 + 140 格安全平铺 + 撤销栈 + 点击/拖放双通道；Playwright 烟测通过；登记 manifest |
| **007 p534-debt-batch-1** | PLAN-577 | **📦 已归档——✅ 已交付（2026-09-09 merge）**——avatar 家族补齐（三臂+props desugar+Fill 零高根因，真 hover 不借语料工程,P534-D4 结案）/breadcrumb 环守卫（自名折叠递归,P530-D1+P534-D5 结案,全站 68/68）/schema 滞留清偿（419 元素+P3 拆册,三围栏绿）；KNOWN-DEBT 三条 ✅；复审/合并以 /auto-plan:review/:merge 承载 |
| 008 desktop-gallery-apps | PLAN-578 | **📦 已归档——✅ 已交付（2026-09-11 merge）**——画廊两件上架（resolve_os_top_dir 解析序锚+scan_galleries 开关，双轨+三轨 parity）+022-kanban 退策展（C 档 16）；验收 1-6 全过（AC3 按 ④ amend）；auto-os `f3e4570`/auto-lang `c4481ca58`；tf 3509/3510 唯一红=在案 flake；specs 沉淀 P008-1/P008-2；KNOWN-DEBT 候选 ext-materialize 登记；复审/合并以 /auto-plan:review/:merge 承载 |
| **035 desktop-ux-rev3** | — | 🔄 **execution_done（2026-09-20 work 交付，next=review）**——五组走查修缮：任务栏右组右对齐（根因=适配层对无 width 类 justify-center 列给包装容器 Fill 宽，headless 探针守护）/壁纸 picker 游离块根治（坐标锚 popover 内容 open 态守卫；深层合成层泄漏债 DEBTS-035-01）/sliver w-2+满覆盖+动态提示/dashboard 8×3 网格（layout v2 直出绝对矩形，span 2|3 缺省 2 旧 1 迁移+源声明 span: N 标记探测，头行=网格行 0）/三 mini（clock 居中收紧、todo Recompute 归一+preview 前三+span3、music 紧凑 2×2+空曲库 notify 化+孵化 drain 扩容+通知尾条去重）；双仓 worktree `.wt/os-035/`（auto-os plan-035-dev 325f1ff / auto-lang auto-os-035-dev ce7a64014+08efc9cbe）；spec 沉淀 dashboard/showdesk-wallpaper/showdesk-icons 三文件 SD-01..05；实机+headless 双证据 docs/plans/evidence/p035/；门：cargo t 40 红全数基线/在册归因（a2vue 金样唯一真回归已重生成），定向 4/4+1/1 绿；**rev3/rev4 回环（同日）**：面板四围 padding + tab 头行退役改右上角紧凑分页 pill + 卡满高 3 行格 + 时钟 3×2 表盘放大（declared_span 探测改全文唯一标记）+ clock fit 重试加固（10→150 次防慢首帧放弃）+ MCP dashboard 槽；门 39 红同集零新增，定向 23/23；**rev2 回环（同日）**：用户复核四新发现收口——iconfile 资产根无 env 回退（裸 exec 位图全空根治）/sliver 高亮分隔线右侧全高/桌面右键菜单「桌面小组件」checkbox 开关（a2r 词汇门 checkbox 补臂+表扩容）/关窗 face 回正钩（投影 apply 挂 refresh，hatched→running→hatched 日志三拍）；门 39 红与 rev1 同集零新增，定向 7/7 绿 |

**编号映射（2026-08-28 立项）**：M2→**462**、M3→**463**、M4→**465**（原提案
454/455/456，正文历史提案号不回改）；新增 **464**（launcher，吸收 Plan 441，
其 palette 原语化降为 464 可选任务、vm 焦点原语改由 462 承载）。
2026-08-28 增补：**shell-track** 立项（`docs/design/25 曾用名（现 autoui/desktop-shell.md）`
——shell 表面统一为 AutoUI DSL：投影协议 + dock/switcher/pager/通知/settings/
桌面本体，I7–I9），立项时分配实际编号；463 同步转正 workspace 驱动模型与
`desktop.*` builtin 命令接缝（Design 25 §3/§6）。

## 入口条件仪表盘

定期（建议每次状态审计时）更新：

**386 复活条件（2026-08-28 复活时重构为 Stage 依赖，见 386 文件 §0）**：
- [x] I1 接缝评审 —— ✅ `reports/462-i1-seam-review.md`（view 侧零删除替换；E1/E2 扩展点挂 465 消费）
- [ ] 常驻 App ≥ 3 → **转为 Stage 2 前置**（463/464 合入即满足）
- [ ] 内存实测 → **转为 Stage 3 验收度量**（对比 1-5MB/App 目标；建议 463 合入后先测一次留基线）

**未完成计划执行顺序（2026-08-28 排定）**：
```
463 收尾合入（执行中）──┬─▶ 464 launcher ──┬─▶ shell-track M1+（立项）
386 Stage 1（即刻可开工）┤                  └─▶ 386 Stage 2 ─▶ 386 Stage 3
465 vue 虚拟桌面（并行）─┘
```

**457（→509）启动条件**：
- [x] 454+455 完成 —— ✅ 早已满足（= 462/463 已归档）
- [x] auto-cosmic 宿主复活评估（libcosmic 依赖决策，Linux 环境）—— ✅ 509 T1 兑现（2026-09-01）：三路线矩阵裁定 **B**（Smithay+桌面协议宿主）；libcosmic 出局证据 = git-only 无版本锚 + fork iced 与主线 0.14 并存 + Windows 不构建；报告 `docs/plans/reports/509-smithay-route-verdict.md`

## 终态批次（M7）：全 a2r 桌面（2026-09-19 规划；用户确认 ①→②→③ 序）

> B 前置序列（3c1 图像通道 / 3c2 live 输入 + shell queue 面 / 3c3 shell
> outproc v1）与 031 rqhost 第四形态之后的收口批次。**不合成单一执行
> 计划**（合并规模 ≈ 仓史最大计划三倍、三波风险特征异质——数据门驱动 /
> 深水布局语义 / 逐 app 探索性，030 rev1 needs_replan 教训在案），按波
> 次计划推进；本节为波次依赖图与出口判据的唯一事实源（防腐规则 1/2）。

| 波次 | 内容 | 承接债 | 出口判据 |
|---|---|---|---|
| **M7-a 覆盖 ramp v3 + 缺省翻转** ✅ **已交付（PLAN-032，2026-09-19）——裁定：翻转** | judged 六缺项补齐：012 映射兜底（native_style_token）/ tabs 整 kind（投影臂 + a2r tab 断裂）/ hidden（display:none 跳过）/ 样式 grid（GridCols→Grid walker）/ 定位族（absolute+offset 真渲 + fixed/sticky 口径定案）→ auto 缺省复评；实交付含 D5 族运行时面补臂（SelfCenter/Inset/LineClamp/FlexWrap——T-07 运行时口径发现） | P026-D3 ✅ 核销 | **复测 judged 22/22 = 100% ≥ 95% 过门 → 已翻转**：`resolve_native_frame_mode` Covered 臂 = Commands（auto 缺省 queue，观测行 flipped@ramp3）+ 仪器 judged 口径升级 + 防漏断言反转（跌破门即红）+ 六例 e2e（四进程腿 + 018/041 拒收留痕——truncate/codeeditor 真 not-yet 家族 P032-D3 登记）+ 台账裁定行（本行） |
| **M7-b shell a2r 编译面 + overlay outproc 化** ✅ **已交付（PLAN-036，2026-09-20）** | shell-lib 生成模式落地（generate_shell_pack_lib 五件→crates/shell-pack 入库 + mount_face 工厂 + freshness 字节门 + 真编译门清偿 a2r 七族/truncate 真渲⑫）；child 编译轨缺省（boot 面装载 25.7→0.8ms，显式 AUTO_SHELL_PACK=解释双轨）；overlay 四面全 outproc（switcher/通知/dashboard 并壳 exe 五面[OVERLAY=3/DASHBOARD=4 追加档] + launcher 一面一 exe[注册表源，D3-C 混合]）；D6 键盘事件位（ShellEvent tag6-9）+ D5 聚焦链 child 化（focus_first_input/伪窗可聚焦/bind 路由臂/热键宿主域） | P030-D1 ✅ / P030-D3 ✅（P036-D1..D6 新债随注：face:// 全下放/编译 launcher exe/launcher 看门兵/key_message 编译臂/跨进程色彩差/自隐镜像同步） | **p036_all_faces_outproc_arm 六腿绿**（五伪窗/五面帧/parity 结构全等/launcher exe/崩溃隔离）+ p030 四腿回归 + boot 时延行 + assets/036/ + §1.16 v1.16 + 度量报告 reports/p036-metrics.md |
| **M7-c app 组合批量 a2r** | 三梯队：①面内近邻 = klondike（icon/card）/ minesweeper（grid）/ kanban（input/textarea）/ launcher（icon/mouse-area/input/grid）/ tetris（dialog 折叠形态核对）；②tabs 户 = jade-garden（tab×27）/ auto-musk（tab×16 + alert-dialog×7 + table×1，17821 行拆批）——**依赖 M7-a tabs**；③table 族 = sys-monitor（table 全家 + svg + alert-dialog×16，table kind 捆绑立项）；特殊线 auto-term（terminal kind） | —（逐 app codegen 缺口即发现即修，缺口按 app 记账） | 各 app `auto build -r rust` + `desktop_exe:` 声明 + 三轨回归绿；超面 kind 缺口逐个入册或立项 |
| **M7-d 全 a2r 桌面收口** | 整桌面度量（对照 508 inproc 0.86 / 020 queue 2.42 MiB/App 基线）+ 全链 smoke + 终态入册（Design 23 里程碑收笔） | — | 度量报告 + 终态验收留痕 |

**顺序**：M7-a → {M7-b, M7-c①} 可并行（文件面错开：投影器/覆盖表 ‖ 生成器/宿主装配 ‖ app 仓）→ M7-c②③（②依赖 M7-a 的 tabs）→ M7-d 压轴。

**M7-a 裁定行（PLAN-032 收口，2026-09-19）**：native `Auto` 缺省 =
**queue**（ramp v3 数据门达标：judged 22/22 = 100%，仪器复现命令与
对差表见 lang `docs/plans/reports/p032-native-flip-row.md`；协议增量
§1.13 v1.13——五族口径 + 保真边界随注，零 wire 变体）。**M7-c②
解锁注记**：tabs kind 全链已交付（VM 轨投影臂 + a2r 断裂映射修复 +
046 真源编译过）——jade-garden（tab×27）/auto-musk（tab×16）可开工；
运行时口径缺口（P032-D3：truncate/codeeditor）M7-c 撞面时逐 app 记
账立项。

**M7 副线裁定交付行（PLAN-033 收口，2026-09-19）**：双投影器统一 + `-q` VM 轨改接 + pixels 臂退役门（澄清版裁定）**已交付**——`-q` VM 轨改接 RqProjector（物化 View 直喂，a2r/解释同律）；AppProjector 降级投影臂退役（**P020-D1 ✅ 销账**——统一兑现 + 更名 RqProjector 落定，销账注记 lang KNOWN-DEBT）；解释态 pixels 臂（run_independent_child + process_model=outproc 选项）即废；VM 轨三补（input_state_map 回写 / 多 timer 泵侧驱动 / `__desktop_cmd` DesktopBus 上行）+ L3 StateSnapshot native 注入（Component::apply_state_snapshot 钩子）随统一落定；remote 宿主孪生迁 native 重录。量化验收门：单 app `-q` 渲染正确 ✅（p033 e2e：003/001/027 开窗+首帧+覆盖门零拒；vm_typing 键入换算 212）+ **app ≤10MB ✅（003 -q 实测 8016KB；对照直挂 225088KB 省 212MB**——免每 app iced/wgpu 后端收益实证；027 富载体 20400KB 数据行 P033-D3 随 M7-c 批量化再裁）；rqhost ≤100MB ❌ 维持（debug 314MB——release 复测+归因优化 = **PLAN-034 前置**）。协议增量 §1.14 v1.14（零 wire 变体）。**M7-b/M7-c 波次注记**：RqProjector 替换点语义不变（更名后壳装配面同型）；M7-c app 批量化的 `-q` 内存门以 003 简单载体为参考线。

**M7 副线裁定交付行（PLAN-034，2026-09-20）**：RQHost+RqProjector 完整化第二件 **已交付**——①位图过线通道（P028-D1 核销：FrameMsg tag 10/11 + BufferAlloc.bm 尾追专用段 + `bitmap://` 词汇 + drain_bitmap_uploads 上传 API + TS decode 桩；canvas=位图快照过线 = 首个像素原生 kind 过线，043 样板 -q 真进程闭环）；②像素原生族五 kind 裁定入册（用户确认修正版：video=inproc/独立窗专属｜terminal=M7-c 撞面立项｜canvas=快照过线｜code_editor=not-yet 维持｜imagesurface=queue not-yet 维持+M7-c——"零消费者"前提证伪后修正）；③rqhost 内存量化门 release 复测 + 12 格归因矩阵（数据行 reports/assets/034/memory-matrix.txt——门判定见结论行）。协议增量 §1.15 v1.15（全追加式，PROTOCOL_VERSION 仍 1）。

**副线债**（不阻塞终态，按需排期）：native pixels 半臂维持四条件（auto 翻转[032 ✅] / 覆盖收口[M7-c] / 位图过线[034 ✅] / 像素原生族裁定[034 ✅ 五 kind 入册 §1.15]——**剩余条件 = 覆盖收口[M7-c] + 退役执行件**；Linux 无强制场景：iced+wgpu 自渲 app 经 swapbuffers 呈递 OS compositor 属系统协议；P-RQ-PIX 在册）；rqhost 内存达标（release 复测+归因矩阵已交付——门判定见 assets/034/memory-matrix.txt 结论行，034）；位图过线通道 ✅（034 §1.15——web 真位图仍 P028-D3）；518 色彩上下文；GUI 级 OS 自动化（P020-D4 点击面）；Stage B 搬迁（桌面壳代码 auto-lang→auto-os）；imagesurface 交互族 / svgdoc 词汇；rqhost 生态（`--rq-host=desktop` attach 等）。

## 裁定登记簿

程序最大的风险不是丢任务，是旧裁定在新代码/新文档里悄悄复活。每次翻转裁定记一行，
核销所有受影响文档的同步状态。

| # | 裁定 | 旧内容（出处） | 新内容 | 变更载体 | 同步状态 |
|---|---|---|---|---|---|
| 1 | Win/Mac 宿主窗口拓扑 | 宿主非合成器，每 App 一 OS 窗口（Plan 365 archive） | 单 OS 窗口虚拟桌面 | Design 23 R2 / 452 | doc20 ✅ 386 ✅ 365注记 ✅（2026-08-26，T2–T4；提交号随本批提交后补） |
| 2 | chrome/窗口管理归属 | 宿主拥有窗口管理（doc 20 §6.1） | 特权桌面 App 拥有窗口语义，宿主只管合成 | Design 23 R1 / 452 | doc20 ✅（2026-08-26，T2） |
| 3 | RenderQueue 定位 | 独立的分离渲染内存优化（Plan 386） | R4 接缝的 RenderCommand 后端（路线 B） | Design 23 R7 / 452 | 386 ✅（2026-08-26，T4） |
| 3a | 编译 exe 客户端地位（PLAN-020 增行） | outproc 子进程 = `auto` re-exec 重释 .at（解释态隔离选项，508）；RenderQueue 客户端臂绑死解释态 `DynamicComponent` | **a2r 编译 exe 为 compositor 一等客户端**：协议客户端臂泛化到 `Component` seam（`RqProjector`，View 运行期投影）；宿主注册表 `desktop_exe:` 声明 > rust-workspace 约定路径分流孵化（exe App 天然 outproc，解释臂零变化）；native `auto` 裁决缺省 independent（queue 显式申明经 pac `desktop_render:` 透传）。PROTOCOL_VERSION 仍 1（零 wire 变体） | desktop-protocol-v1.md §1.6 / PLAN-020（auto-lang plan-020-dev） | 020 代码+e2e ✅（lang plan-020-dev 98a4cd502..b187ef7d0；真机截图 lang 仓 docs/plans/reports/assets/020/；冒烟 scripts/smoke-020-native-exe.sh）；canonical 行随 020 merge 发布 |
| 3b | native 覆盖爬坡 + 输入路由收口（PLAN-025 增行） | 3a 后 native queue 覆盖 = text/button + 线性布局（payload 族 not-yet、键/滚轮/右键不路由——P020-D2 半句） | **form/payload 族入册 + 输入路由两端收口**：native_queue_set 扩 input/textarea/checkbox/radio/slider/select + layouts scroll（View 无 Switch 变体不列——分表非缺口）+ flex-1/shadow 降级放行；聚焦=槽位序+帧后重定位，编辑=buffer→store_input_text 同线程代写→a2r on() 回写；select 开合=覆盖序选项列+命中互斥；右键/滚轮投影器消费 + 宿主 broker_key_event/broker_char/broker_scroll 生产路径（P020-D2 键/滚轮/右键半句核销，StateSnapshot 半句保留另立）；a2r codegen 补 slider/select 臂 + input value 绑定容差 + math.* 降级。PROTOCOL_VERSION 仍 1 | desktop-protocol-v1.md §1.7 / PLAN-025（auto-lang plan-025-dev） | 025 代码+e2e ✅（lang plan-025-dev 734f34ad5..d871f8e50；p025_native_input_arm 三腿真 exe PASS：003 键入 100→212 / slider 75% / select Medium；帧留痕 lang 仓 docs/plans/reports/assets/025/；冒烟 scripts/smoke-025-native-input.sh；新债 P025-D1 live 壳键盘订阅缺口、P025-D2 聚焦槽位边界）；canonical 行随 025 merge 发布 |
| 3b2 | native 覆盖爬坡第二批 + IME 闭环 + auto 裁决复测（PLAN-026 增行） | 3b 后 display 族/布局族/IME not-yet、auto 缺省 independent（翻转三闸 T-覆盖数据待复测） | **display/布局二批入册 + IME 闭环 + 不翻裁定**：native_queue_set 扩 image/progress（占位保真臂）+ layouts grid（cols 等宽 walker + center/legacy 尺寸/w-full 消费；center 不入册——View 层归一 Container，防漏钉钉②口径）；icon/badge/avatar/divider/separator/spacer/a 经 a2r codegen 降级归一（D1 定案对齐 VM 轨形态——icon→lucide Image + 尺寸契约，badge→样式 Row，card/divider/spacer/avatar→styled container，scroll→scrollable，link 子件组合；断裂映射 badge/card/icon/scroll/link 移除）；IME 闭环两端（投影器 Commit/Preedit 尾拼/Cancelled 消费 + 宿主 broker_ime_commit/preedit/cancelled 焦点窗路由）；样式降级放行批（overflow-/min-/leading-/flex/block/underline 族/cursor-/transition 等 + 字重族补全）；imagesurface 整 kind not-yet（D5）；**auto 裁决复测 = judged 76.2% < 95% 阈值 → 维持 independent 不翻**（翻转点已备：resolve_native_frame_mode 扫描制观测行 + Covered 臂 one-line 翻转；数据行 docs/plans/reports/p026-native-flip-data-row.md）。PROTOCOL_VERSION 仍 1 | desktop-protocol-v1.md §1.8 / PLAN-026（auto-lang plan-026-dev；os plan-026-dev） | 026 代码+e2e ✅（lang plan-026-dev 51e99ab49..T-08 提交；p026_native_display_arm 三腿真 exe PASS：004 display 占位+按钮 / 026-display display 族全件+grid+center / 003 broker_ime_commit(100)→212 联动；帧留痕 lang 仓 docs/plans/reports/assets/026/；冒烟 scripts/smoke-026-native-display.sh；新债 P026-D1 图像真渲、P026-D2 a2r 断裂映射残余（S1）、P026-D3 auto 缺省维持）；canonical 行随 026 merge 发布 |
| 3c | shell 编译化形态（PLAN-027 rev2 增行） | A 线工作假设 = shell 组件库链入宿主（inproc；设计 desktop-shell-a2r §5 推荐，PLAN-027 rev1 T-07/T-08 按此拆） | **shell 编译化 = B 形态（outproc 特权协议客户端）**：shell 与 app 同律独立进程、渲染经 RenderQueue 发桌面 compositor 统一渲染（§10-① 用户裁定 2026-09-18）；解释装载路径双轨常驻（开发态 AUTO_SHELL_PACK fallback + 编译态）。前置序列：图像 DrawOp 通道立项（硬阻断——壁纸/缩略图皆图像，DrawList 现仅 Quad/Text/TextStyled/Scissor；即 3b2 债 P026-D1 的协议化）→ 025 键盘真机实测 → 覆盖二批（display 族 + popover 开合）→ shell outproc client + 启动序/看门兵。**已落地承继资产**（PLAN-027 S1/S2，形态无关）：a2r 生成域补面（裸 popover/合成件槽位/mouse-area/显式拒绝门/五件词汇门）+ typed 接缝（ShellProjection 载体/ShellManifest 五件清单/DesktopBusHandle/HostStorage；52 动词 roundtrip 对拍捕获 SetThemeName 死词已修）——B 的 wire payload 词汇即此载体 | desktop-shell-a2r.md §5 裁定记录 / PLAN-027 rev2 §10-①（auto-os main b3739f1..rev2） | 027 S1/S2 代码+测试 ✅（lang plan-027-dev 97d0bb75d..T-09' 提交；全量失败集与改前基线 41 项逐一全等）；canonical 行随 027 merge 发布 |
| 3c1 | **图像 DrawOp 通道（PLAN-028 增行——B 形态前置序列第一件交付）** | 3c 前置序列首项：壁纸/窗口缩略图/壁纸预览全是图像，DrawList 无图像算子 = B 硬阻断（3b2 债 P026-D1 协议化） | **图像通道 v1 = src 引用 + 宿主侧解析（零位图过线、零新依赖）**：wire `DrawOp::Image{rect, src, fit}`（tag 6 追加式，`ImageFit` v1 = Stretch；op 字段定长不可尾部追加——filter/radius 不入 wire 未来新 tag；PROTOCOL_VERSION 仍 1）；src 词汇表 = 本地文件/`builtin:`/`data:`/`http(s)`（3s 超时）/`thumbnail://{wid}` 虚拟引用 + not-yet 词汇显式成文（`lucide:`/`svgdoc:` 字形线）；宿主栅格化 = `Frame::draw_image` 仓内首用（iced 0.14 feature 链）+ 进程级 Handle 缓存（key=src，含负缓存）+ http miss 占位先行/后台解码/下帧翻真（零新增触发器）+ 未解析占位 + `[drawlist-image]` 观测去重（I3）；`thumbnail://` 复用 snapshot 全套（SWR/冷却/TTL 零新机制——showdesk/workspace preview 前置能力本期直测）；两投影臂同刻度真图升级（解释态 layout_image src 绑定 read_state 代入 + native `View::Image` 臂，rect 推导零变化；icon lucide 降级形态随臂入线）；TS decode tag 6 必达 + 占位渲染 + web 真位图 not-yet 成文 + `IMAGE_FRAME_HEX` 双侧 golden；p026 占位 Quad 断言改写归因。P026-D1 图像半句核销（字形半句维持）；not-yet 三项入册（位图过线/字形真渲/web 真位图） | desktop-protocol-v1.md §1.9 + 顶表 v1.5–v1.9 / PLAN-028（lang plan-028-dev；os plan-028-dev） | 028 代码+e2e ✅（lang plan-028-dev：message.rs tag 6 round-trip/golden + 六锚点双侧对拍 + t028_* 宿主解析单测族 + p028_image_arm 真子进程 e2e PASS（004 http 80×80 op + p028 语料五形态一帧 + 宿主解析三路径实驱 + 度量行 frame_bytes=365/src_bytes=245）；帧留痕 lang 仓 docs/plans/reports/assets/028/；全量失败集与 master 基线逐一全等（40 项既有红，ui-iced 组合族））；canonical 行随 028 merge 发布 |
| 3c2 | **live 输入接线 + shell queue 面（PLAN-029 增行——B 形态前置序列第二件交付）** | 3c 前置序列第二项：P025-D1 在册（desktop_window_events 零键盘/滚轮/IME 臂——broker_* 六函数无事件源）+ shell pack 五件含四个 native queue 缺失 kind（popover×9/mouse-area×13/window_thumbnail×3/workspace_preview×1 + icon×7 走 lucide: 字形线）= B 覆盖门硬前置 | **live 接线 + 四 kind + lucide 真渲（零 wire 变体——全部宿主侧/词汇表演进，PROTOCOL_VERSION 仍 1）**：①live 接线两端（`desktop_window_events` 扩键盘/滚轮/IME 三族臂[Ignored 门] + 映射纯函数[Named→VK 表/Character→Chars/IME 三态/Lines×40px `WHEEL_LINE_PX`/wire 修饰位] + `DesktopEvent::LiveInput` 泵入[带发生 OS 窗——update 臂桌面窗过滤+picker 避让] + `route_live_input` → broker_* 六函数零改动直用[键盘/IME=焦点窗、滚轮=last_cursor 命中窗]）——**P025-D1 核销**；真机证据分层（e2e p029_live_input_arm 五腿[⑤协议级/Chars/VK_BACK/ImeCommit 中文/滚轮] + acceptance key verb[autoui_desktop action=key 六 kind，DesktopInject::Key] + sendinput.rs FFI 模块[user32 SendInput+KEYEVENTF_UNICODE 组装层]；真桌面 SendInput e2e 腿 not-yet 随注——依赖 B 程序启动序+child 观察）②四 kind 入册（popover 覆盖序渲染[PopoverOverlay 主块后追加+全 14 placement 几何纯函数：对向翻转/钳制/Modal scrim/Edge 贴边/Pointer=右键点/Point 锚恒 BottomStart]+命中[全屏 catcher 先 push、面板项后 push=rev 序胜；外点/Esc→on_dismiss+吞]+零开合状态机[open 随帧]；mousearea 透传+命中序[area 先 push、content 项 rev 序优先；click/contextmenu 族，hover 族 not-yet]；windowthumbnail/workspacepreview→Image 桥接）+ scan Popover 双子树递归（:562 缺口）+ 防漏钉矩阵四夹具 + 两降级放行（flex-col-reverse/opacity-——flex-1/shadow 同册先例）③词汇增量（`lucide:{name}#{rrggbb}` 真渲[resvg 0.45+tiny-skia 0.11 Contain 栅格化+tint 缺省 #FFFFFF+缓存键 {src}@{w}x{h}+未知名负缓存]——P026-D1 字形半句核销；`thumbnail://{wid}!{fallback}` miss→图标真渲；`workspace://{ws}!{fallback}` 宿主合成[workspace_preview 数据面+tile_rect+壁纸基色+近邻 blit/灰块]；svgdoc: 维持 not-yet）④**shell pack 五件装载期 Covered**（shell_pack_native_covered——B 覆盖门预演断言）⑤翻转复测 dual-exit：overall 16/36=44.4%/judged 16/22=72.7%（样本扩容稀释+046-tabs 入分母；009 opacity 翻绿/041 popover 半句清偿）→ 双口径均 <95% **维持不翻**（报告 p029-native-flip-retest-row.md；shell 五件单列不入 examples 分母） | desktop-protocol-v1.md §1.10 + 顶表 v1.10 / desktop-shell-a2r.md §10-①（前置序列两件 ✅ 剩 shell outproc client）/ PLAN-029（lang plan-029-dev；os plan-029-dev） | 029 代码+e2e ✅（lang plan-029-dev 2c038d889..ed182efc1：T-02 8f1aeb42f 接线/T-03 a0671a0dc 三件套/T-04-06 8ff59f2eb 三臂/T-07 6190fd4e5 词汇/T-08 b6d924f44 收口/T-09 ed182efc1 e2e+文档；live_input 单测 4+sendinput 4+popover 几何/渲染/命中 3+桥接/mousearea 2+t029_* 3+shell_pack 1；p029_live_input_arm 五腿 PASS + p029_shell_face_arm 五腿 PASS[五件套 ops 落 wire+popover 命中闭环+mousearea]；帧留痕 lang 仓 docs/plans/reports/assets/029/；desktop_protocol 168 过 2 红在册预存[plan624/P507-2]）；canonical 行随 029 merge 发布 |
| 3c3 | **B 程序主体：shell outproc client + 启动序/看门兵（PLAN-030 增行——B 前置序列第三件/终件交付）** | 3c 前置序列终项：DesktopBus broker 路径丢弃（endpoint 假透传实锤）+ 宿主→子进程零持续状态推送 + 无子进程自愈 = B 三缺口 | **协议增量 + 壳客户端 + 宿主孵化 + 看门兵 + 双轨开关（v1.11，PROTOCOL_VERSION 仍 1——ControlMsg tag 12-14/Hello/Welcome 尾段追加式，空尾段字节级不变）**：①投影下行推（ShellProjectionPush/ShellClockTick/ShellCursorMove——027 typed 载体 wire 激活[shell_projection::wire 全家族 + DesktopSurfaceSnapshot] + per-face 宿主侧指纹门 + respawn 强制失效全量重推）②DesktopBus 端点拆臂执行（HostAction::DesktopBus → parse_records 52 动词单点 + registry_id 归因[壳伪窗=面名] → desktop_bus_inbox 泵后同拍 execute；acceptance Bus verb 同臂改道）③表面 z 平面（Hello/Welcome 尾段多表面协商 + 端点 wid→surface 路由 + activate_multi；壳双表面 = 宿主 Stack 层槽[background 替 desktop 面槽/chrome 替任务栏槽] + 双伪窗 WM 命中[垫底 insert(0)/置顶带矩形——hit_test 无透明直通实锤]）④壳客户端 --autodesk-shell（shell_client.rs 双表面协商 + RqProjector View 全展开[AppProjector 队列臂 ForLoop no-op 实测定轨] + __desktop_cmd 读走 child 化 + 内联帧免 shm；re-exec auto + AUTO_SHELL_GEOM/PACK env）⑤boot 孵化序（shell.apps.shell_model inproc\|outproc 缺省 inproc[I1 双轨零回归] + env AUTO_SHELL_MODEL 便捷注入 + broker 后 spawn[倒挂清偿] + 失败回退 in-proc）⑥看门兵（EOF 检出→退避 1s/2s/5s 预算 3 次/60s 窗分拍 respawn→全量重推；耗尽降级观测+一次性回退 in-proc）⑦pointer 生产接线（D3 缺口清偿：GlobalPress/release 命中 broker wid 路由 + owns_wid 去单值化）**v1 边界（D6）**：常驻双面 outproc，overlay 三面+launcher 维持 in-proc；a2r 编译面轨（shell-lib 生成模式）另立（P030-D1）——v1 = 解释面 outproc child | desktop-protocol-v1.md §1.11 + 顶表 v1.11 / desktop-shell-a2r.md §10-① 终件 ✅ / PLAN-030（lang plan-030-dev；os plan-030-dev） | 030 代码+e2e ✅（lang plan-030-dev 104de2655[T-02]→2a3a83369[T-03]→0a3618807[T-04/05]→c150c3c61[T-06]→dc4ef0677[T-07/08 e2e+回归]→3b403d0cf[T-09 文档]；wire round-trip/golden + 端点多表面路由 + inbox 全链 + 指纹门三态 + 双轨断言；p030_shell_outproc_arm 四腿 PASS[双表面首帧/真按钮点击→DesktopBus→归因执行/投影推送帧变/kill→看门兵→respawn→全量重推恢复]，帧留痕 lang 仓 assets/030/；回归 261/262[唯一红=基线预存 imagesurface]+inproc gate 108/108；os smoke scripts/smoke-030-shell-outproc.sh；029 SendInput 挂账 dual 口径核销[P030-D1..D5 随注]）；canonical 行随 030 merge 发布 |
| 3d | **rqhost 共享合成器原生窗——第四运行形态（PLAN-031 增行）** | 三形态（直挂 inproc/桌面虚拟窗/独立窗）之外无宿主 OS 原生窗共享形态；`-r/--render` 撞名前科下无 -q 族旗标；broker serve_once 阻塞第二等连不适用于慢客户端（rust 轨 build 分钟级） | **`auto run -q`（vm/rust 两轨）= 宿主 OS 普通原生窗 × 共享 rqhost daemon（用户裁定砍 standalone——单 app shared 自动孵化等价，多 app 省 N-1 渲染进程）**：①rendezvous 采纳协议（well-known `autodesk-rqhost` + `adopt␟<name>` 管道串记录[DesktopBus 约定族，双动词兼容 incubate——旧生成物零改接驳] + per-app 管道先行 listen + **锁管道 FILE_FLAG_FIRST_PIPE_INSTANCE 单实例仲裁**[OS 级原子零竞态窗] + per-adoption wait_connect 线程化）；②客户端权威采纳（rqhost 无 resolver——Hello 凭据直接 activate，宿主零装载；桌面"宿主内容权威"零牵连）；③生命周期语义（用户关窗→Close→app 码 0 退出；app EOF→窗回收；**宿主死→app exit-on-EOF 策略档**[`ClientTarget::Rqhost` reconnect=None+观测行；桌面档 30s 重连不变]；末窗关→daemon 自退[iced 空窗不退的反面]；15ms 帧泵）；④输入按窗路由（事件自带 window_id 免 WM hit_test；029 LiveInput 族直调 + 指针全事件；窗内坐标即表面坐标）；⑤CLI 两轨（`-q/--render-queue` + `auto rqhost [--pipe]` 子命令；vm 轨 env 门装载链内分岔[lib.rs run_file_dynamic_ui_inner]；rust 轨 `--autodesk-rqhost` 标记注入[cargo `--` 分隔符前置修复]）；⑥附带根修：大帧超 16KiB shm 槽回退管道内联 FrameReady（此前静默弃帧=冻结，桌面 broker 同益）。**零 wire 变体**（rendezvous = 传输层管道串约定，PROTOCOL_VERSION 仍 1）。保真口径：vm 轨解释态全保真 + not-yet 占位/观测行承袭；native 轨 ensure_covered 既有门。实测：daemon ~314MB + 每 app 边际 ~7-9MB private（共享渲染宿主摊薄证据） | desktop-protocol-v1.md §1.11 + 顶表 v1.11 / virtual-desktop.md Design 23 §4 后端矩阵增行（Win/Mac·rqhost B 形态行）/ PLAN-031（lang plan-031-dev；os plan-031-dev） | 031 代码+e2e ✅（lang plan-031-dev 08526fda8..T-07 提交：T-02 c33cebe38 核心+T-03 801ed51fb daemon+T-04 3c8363e68 输入+T-05 8b1602838 客户端+T-06 149d299c7 CLI+T-07 e2e/大帧根修；rqhost 单测/集成 11 绿；e2e p031_rqhost_arm 七腿 PASS[单/多 app、竞态二实例码 0、resize、kill 双向、关窗 X→app 码 0、末窗自退、真实自动孵化、rust 轨、降级显式]；截图/进程清单留痕 lang 仓 docs/plans/reports/assets/031/；desktop_protocol 181 绿+在册红×2[merge-base 基线同红]；session 71 绿/auto-man rust_ui 23 绿）；os 冒烟 scripts/smoke-031-rqhost.sh（生产 well-known 全链演示 PASS）；canonical 行随 031 merge 发布 |
| 4 | 独立窗口模式地位 | （隐含）默认且唯一 | 永久一等公民 + 退化桌面构造（R3/R6） | Design 23 / 452 | — |
| 5 | shell 组件归属 + 排布/注册表/启动/热键（**追加 R8–R12**，非翻转） | — | launcher/任务栏=特权 AutoUI App；排布=WM 纯函数策略（free/grid/master-stack+snap）；注册表=pac.at 清单；启动=会话挂载非进程孵化；桌面级热键路由优先于 App 分发 | Design 24 / 462-465 立项 | — |
| 6 | shell 层工程化（**追加 I7–I9 + DesktopBus 定案**，非翻转） | 463 T1 命令接缝两候选待定 | `desktop.*` builtin 命名空间定案（候选 A）；驱动=内核/shell=用户态（投影唯一事实 I9、shell 无几何操作 I7、表面双端同源 I8）；workspace 驱动模型转正入 463 | Design 25 / shell-track 立项 | — |

同步动作完成时把 ⬜ 改 ✅ 并附提交号。归档计划（365）不改正文，只加状态注记。

## 防腐规则

1. 单一事实源：计划详细状态只在各计划状态头；本文件只放一行指针。
2. 本文件只拥有任何单个计划都不拥有的东西：依赖图、仪表盘、裁定登记簿。
3. 452 收编 Design 23 后，架构问题先查 Design 23，本文件不重复架构论述。
4. 周期审计（plans-status-audit 体系）引用本文件，不重新推导程序状态。
