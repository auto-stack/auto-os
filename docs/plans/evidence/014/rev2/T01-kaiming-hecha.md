# T-01 开工核对证据（PLAN-014 rev2，2026-09-17）

worktree：`D:/autostack/.wt/os-014/auto-os`（plan-014-dev @ 325095b）+
`D:/autostack/.wt/os-014/auto-lang`（auto-os-dev @ eefb5d84d）。

## 1. W-01 五死消息宿主投递路径核查 —— 结论：全死，可删

| 消息 | pack 内出现点 | 宿主侧同名物 | 定性 |
|---|---|---|---|
| `LayoutFree` | shell.at:58 声明 / :571 handler | `HotkeyAction::SetLayoutFree`（session.rs:4057）→ 热键路由直发 `DM::Wm(WmCommand::SetLayout(Free))`（renderer.rs:8510） | 宿主热键走 WM 命令直发，不经 pack 消息；pack 内无发送者（526 T35 已退役 Free 钮） |
| `WorkspaceNext` | shell.at:60 / :591 | `HotkeyAction::WorkspaceNext`（session.rs:4058）→ `DM::Wm(WmCommand::NextWorkspace)`（renderer.rs:8513） | 同上，全死 |
| `WorkspaceClose(str)` | shell.at:61 / :594 | `DesktopCommand::WorkspaceClose(usize)`（session.rs:1340/1533/1800） | 宿主侧是**命令总线**变体（`workspace_close\t<n>` 解析产物），非 pack 消息投递；pack msg 无发送者 |
| `HoverWs(str)`/`HoverWsEnd` | shell.at:81 / :623-624 | renderer.rs 仅注释命中（:24940/:24942/:25066——526 T18 历史测试注记） | 无宿主投递路径；pack 内无发送者 |

grep 口径：auto-os `shell/*.at` 四文件仅声明+handler 行命中（无 view 发送点）；
auto-lang `crates/**.rs` 消息名命中均为 HotkeyAction/DesktopCommand 枚举变体
或注释。**附带发现**：`ws_hover`（shell.at:134）唯一写点即 :623-624 死
handler——随 W-01 一并删除（属同一死组，不扩scope）。

## 2. 协议现状 —— v1.7，v1.8 落点确认

- 文件：auto-lang `schema/projection-protocol-v1.md`（331 行），头部版本块
  :1-16，§2 字段表 :29-47，§2.0.1 通知面板缝 :49-54，§2.1 桌面本体面
  :56-76，§3 指纹 :78-94，§4 动词词表 :96-137，§5 对拍 :139-168，§6 变更
  记录 :170-331（v1.7 节居首）。
- v1.8 落点：头部版本块补行 + §2 字段表行内增注（`__wm_running` 注入面 /
  `__wm_notes.app` / `__wm_clock` 邻接新 `__wm_date` 行 / `__desktop_cmd`
  追加语义）+ §6 新增 v1.8 节。零新动词（`activate` 两臂 v1:113 在案，W-08 复用）。

## 3. 022 网格常量（W-04 mock 参数，抄自 showdesk-icons.md SD-01 + desktop.at 实码）

| 常量 | 值 | 实码锚点 |
|---|---|---|
| 列数 | 8 | desktop.at:134 `grid (cols: 8, gap: 8)` |
| 格宽/格高 | 80px（w-20）/ 72px（h-[72px]） | desktop.at:140/156-161 |
| 栅格定宽 | 696px（8×80 + 7×8） | desktop.at:135 `w-[696px]` |
| 图标 | 48px 满幅位图（w-12 h-12，`full=="1"` 臂） | desktop.at:180-184 |
| 行距（纵 pitch） | 80px（72px 格 + 8 gap）——宿主 rows = 视口高/80 扣任务栏，clamp 4..24 | renderer.rs inject_desktop_surface（rows 计算） |
| 排布 | 列主序（desktop_icon_cells） | renderer.rs:11816 |
| 拖拽阈值 | 6px（icon_drag_moved 门） | showdesk-icons.md SD-01 |
| hover | `bg-white/10` 整组 | desktop.at:161 |
| 标题 | text-xs 三级配色（图片壁纸白字+bg-black/30 底片/亮度自适应） | desktop.at:209-222 |

## 4. W-02 spike 前置定案材料（Q1 倾向：追加语义可行）

- `DesktopCommand::parse_records`（session.rs:1645）按 `[REC_SEP, '\n']`
  切分——多记录排空**宿主已支持**。
- 排空点：renderer.rs:9652 每 update 周期 `drain_desktop_commands()`（shell +
  desktop 双表面，session.rs:2505-2518），读后即清（session.rs:2529）。
- 结论倾向：同周期双命令在下一排空点全部入 `Vec<DesktopCommand>` 顺序执行，
  追加语义无指纹冲突。T-03 以同周期双命令日志用例实证后定案（退化路径
  预计不动用）。

## 5. 字段审计（W-12' 输入，v1.4 漏记实证）

- `__desktop_icons` 条目实注入 `{id, icon, label, src, color}`（renderer.rs
  inject_desktop_surface：`badge_color_for(id)`）——schema §2.1 行
  （:65）与 desktop.at 头注（:15-17）均缺 `color`。
- `__desktop_cells` 条目实注入含 `full`（renderer.rs:11901）——schema §2.1
  行（:67）字段集缺 `full`。
- `__wm_running` 注入点：renderer.rs:12840，shell 层投影 fp 门控组内
  （:12831 fp 等式早退）——W-04 扩注 desktop 层即在此函数/邻位加写点。

## 6. 开工环境

- 主检出 main @ 325095b，`shell/`、`docs/specs/`、`scripts/` 零 WIP
  （025-sys-monitor / ui-gallery 等在飞 WIP 为他会话所有，不在本计划冲突面）。
- auto-os worktree 基线 = main HEAD；auto-lang worktree 基线 = master HEAD
  eefb5d84d（其主检出有他会话 WIP，不触碰）。
- 内嵌快照：auto-lang `crates/auto-lang/assets/shell.at` 存在于 eefb5d84d
  （同含五死消息 :58-61/:81，T-10 由 `scripts/shell-pack-sync.py` 对齐）。
