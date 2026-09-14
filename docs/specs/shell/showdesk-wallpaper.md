# Spec: 显示桌面（负一屏）与壁纸选择 carousel

> Source of truth for the show-desktop workspace and wallpaper picker
> semantics introduced by PLAN-019. Protocol face: auto-lang
> `schema/projection-protocol-v1.md` v1.7 (§2 fields, §4 verbs).
> Host implementation: auto-lang `ui/session.rs` (`WmState` bookkeeping)
> + `ui/iced/renderer.rs` (execute arms, projection).
> Shell consumption: auto-os `shell/shell.at` (sliver) + `shell/desktop.at`
> (picker layer).

## SD-01 负一屏（显示桌面）

- 语义对齐 Win11「显示桌面」，实现 = **保留空工作区**（非最小化全部——
  用户裁定否定 win_min 同族，PLAN-014 W-05 语义由本 spec 接管）。
- `show_desktop` 动词：懒建保留分区（追加尾部），记录 `showdesk_origin`
  = 进入前 current，切入。**幂等**：已在负一屏不覆盖 origin。
- `showdesk_return` 动词：picker 若开着先关 → current = origin → origin
  清零；无簿记 = no-op。
- 排除规则（负一屏躲开常规分区导航）：
  1. `__wm_workspaces` 投影过滤保留分区（pager 切换条不可见）；
  2. `workspace_next`/`prev` 环切跳过；
  3. `workspace_close` 保留分区静默拒绝（先于非空/保底 toast 门），
     删除其他分区时簿记下标压实跟随（删 origin → 迁至并入前驱）；
  4. `send_to` 目标为保留分区拒绝；
  5. 负一屏上 `activate` = 先 return 再启动/聚焦（新窗落 origin，
     防穿帮）。
- 入口：任务栏最右缘 sliver（`w-3` 细条 + 左缘分隔线 + hover 高亮），
  toggle 判据 = `__wm_showdesk` 投影等式（`"1"` → return，`""` →
  show_desktop）。
- `__wm_showdesk`（协议 §2）："1"/""，随分区指纹段差分同步（进出必翻
  current 位），无独立指纹段。

## SD-02 壁纸选择 carousel

- 载体：desktop.at 坐标锚 popover（880px 面板），`__wp_picker == "1"`
  门控——仅负一屏有意义（组合臂保证）；关态桌面零渲染。
- 双态渲染（判据 `__wp_preview`，载荷 = 预览图**路径**，"" = 栅格态）：
  - 栅格态：`grid cols-4` 缩略图（image cover 196×110）。**单击缩略图 =
    `set_wallpaper` 立即应用**（stella 交互；真桌面即合成预览——图标在
    新壁纸上实时可见，fill/crop 保真无模拟误差）。当前壁纸 primary 描边
    （path 等式判据）。「预览」钮进大图。
  - 预览态：大图 contain + ‹›（`wallpaper_nav` 环绕）+ 返回钮（空参
    `wallpaper_preview`）。
- 键盘（宿主订阅层 PICKER_KEYS_OPEN 原子门控——仅 picker 开时消费；
  负一屏无文本焦点窗，吞键无副作用）：
  - ←/→：预览态 = 大图游标环绕；栅格态 = flip 轮换**并立即应用**
    （游标对齐当前壁纸起步，点选同步游标）。
  - Esc：预览态 → 回栅格态；栅格态 → 关闭（execute_wallpaper_escape）。
- 注入面（协议 §2.1）：`__wp_picker`/`__wp_preview`/`__wp_dir`/
  `__wp_current`/`__wp_items`(+`wp_paths` 平行列表)。候选供源 =
  `scan_wallpapers_dir`（jpg/jpeg/png 文件名升序）。直写 + view_dirty
  （不走 shell 指纹门控——桌面面字段直写先例）。

## SD-03 更换壁纸组合与返回归属（用户裁定）

- **两个操作一个组合**：显示桌面（SD-01）与 carousel（SD-02）自足；
  「更换壁纸…」入口（desktop.at 图标/空白右键菜单）发 `wallpaper_pick`
  = show_desktop 幂等 + picker 开 + 归属裁决。
- **归属规则：谁切屏，谁负责切回**（宿主单点 `picker_return_on_close`）：
  - 组合调用（到达前不在负一屏）→ `return_on_close = true` → 关闭
    （选完/Esc/遮罩）自动 `showdesk_return` 回 origin；
  - 用户自入负一屏后开 picker → `return_on_close = false` → 关闭只关
    面板，不代管返回。
- 目录浏览：「浏览…」→ `wallpaper_browse_dir`（宿主 rfd pick_folder，
  无父窗绑定 v1）→ 选定走 `set_wallpapers_dir` 同一写臂（config 单源 +
  mtime 热轮询）+ 候选重扫 + 预览/游标簿记复位；取消 no-op。单目录语义
  不变（`wallpapers_dir` 解析链 config → env → 探测目录）。
- vue 端对拍：v1.7 协议文档承载（本版实现 = vm 端；vue 后续按文档实现，
  UI 形态自定）。

## SD-04 每壁纸图标布局记忆

- 键控：`shell.desktop.positions`（缺省底稿，兼容回退）+
  `shell.desktop.positions.wp/<fp>`。`fp` = 归一化壁纸路径 FNV-1a 64 位
  十六进制；归一化 = `\`→`/` + ASCII 小写折叠（非 ASCII 不折叠——NTFS
  大小写折叠仅 ASCII 等价）。`#hex`/`builtin:`/空壁纸无布局语境 → 缺省
  底稿。
- 读：壁纸键优先，缺席回退缺省底稿（首次切到该壁纸继承默认摆布）。
- 写：
  - 拖拽落格 → 当前壁纸键（键控壁纸）或缺省底稿（非键控壁纸）；
  - `set_wallpaper` 切换 → 切前把旧壁纸 effective 布局快照固化到旧键
    （幂等——缺席键固化当前继承值），切后按新键重注入格子表。
- 语义：壁纸 A/B 各自摆布互不覆盖、切换跟随、flip 对比所见即所得、
  重启保持（storage 持久）。

## 验证

- 宿主单测（auto-lang，`cargo t <filter>`，Category A 允许——本 spec 由
  改 crates 的计划引入）：showdesk 状态机/排除规则/投影过滤、pick-close
  归属两分支、nav 游标与 flip、Esc 链、布局键控迁移。
- 合同对拍：v1.7 §2/§4 字段行与 sync 实现一致；schema_drift 日常档绿。
- vue a2vue 金样：desktop.at 资产变更后 `AUTO_LANG_UPDATE_GOLDEN=1` 重
  生成（已执行）。
- 实机冒烟（人工清单，PLAN-012 先例；shell 宿主无 VM/MCP 通道——
  per-app harness 不适用）：
  1. sliver hover 高亮 → 点击切负一屏（空桌+图标+壁纸）→ 再点返回，
     窗口/焦点原样（AC-01）；
  2. 右键「更换壁纸…」→ 负一屏 + picker → 点缩略图立即换装（图标在新
     壁纸上）→ Esc/外点/选完自动回 origin（AC-03）；
  3. sliver 自入 → 右键更换壁纸 → 关闭留在负一屏（AC-04）；
  4. 预览态 ‹›/←/→/返回/Esc 逐级退回（AC-05/06）；
  5. 「浏览…」原生对话框换目录、列表重扫（AC-07）；
  6. 壁纸 A/B 各摆布局来回切换跟随、重启保持（AC-08）。
