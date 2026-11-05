# 宿主主题策略与正式包交付（host-theme-policy-and-delivery）

> 来源：auto-os/PLAN-053（2026-11-03 交付）。supersedes：无（新增域页，旧
> 暗亮/内置名链保留为 legacy 支持面）；关联：[showdesk-ux-polish](showdesk-ux-polish.md)
> P041 双路径契约、[../apps/calculator-theme-adoption.md](../apps/calculator-theme-adoption.md)、
> [../apps/clock-theme-adoption.md](../apps/clock-theme-adoption.md)。

## 产品批次消费面（PLAN-055 增补）

本页策略模型/提交序/拒绝保旧/交付通道不变；PLAN-055 为第五实际消费者
（独立产品 **auto-clock**：首个 outproc 真载体 + 独立产品仓消费，登记见
[../apps/clock-theme-adoption.md](../apps/clock-theme-adoption.md)）：

- clock 同 056–059 消费形态（固定 `vendor/themes` 快照 + `dep themes` +
  `use themes.themes: Auto`；pac `theme` 键缺省 = follow——clock as-built
  **follow-only**：C-03 VM 轨边界「启动播种只写已声明 dark_mode；未声明即
  无主题面」+ scope App override 档运行时无生产者（`HostThemeInputs` 唯一
  生产者 = 策略快照，`app_override` 恒 Unset）——CLI `--theme` 对未声明
  采纳者只达窗口 chrome，031 内容面照随宿主策略（PLAN-055 修复轮实测
  clock4 + 代码核验，SD-02 边界记录）。
- **反选同场腿（角色等价，C-03 合规）**：显式受控启动钉死档由有声明面的
  calc 承担（CLI `--theme dark` 无 follow env → 钉死 dark），clock follow
  light——宿主策略翻转×2 双向跟随者全跟、钉死方恒定、宿主 config 零被
  钉死方写入；重开无 flag 落最新 revision。像素 oracle（PrintWindow 全窗
  luma）+ MCP 状态双面。
- **真载体 outproc 四腿 + 真实 OS 事件窗口腿全绿**（G-1/G-2 闭合，见下
  两节）；正式组合 pin = gitlink `3495671`（候选=被测=origin/main 同源）。
- **054 边界读回**：本批不触八产品 pin/装配与 037/038/918 边界；健康门
  check-app-submodules 组树通过（`012-clock` 注册，无批量删除签名）。

## 产品批次消费面（PLAN-060 增补）

本页策略模型/提交序/拒绝保旧/交付通道不变；PLAN-060 为第五实际消费者批次
（桌面游戏三产品：037-klondike、036-tetris、038-minesweeper，登记见
[../apps/product-theme-batch-060.md](../apps/product-theme-batch-060.md)）：

- 三产品同 056–059 消费形态（固定 `vendor/themes` 快照 + `dep themes` +
  `use themes.themes: Auto`；solitaire/tetris 原显式 `theme:"dark"` 键退役
  = follow，minesweeper 历来无键）。
- **双产品相反策略隔离腿（standalone follow 臂）**：solitaire@Solarized/dark
  与 tetris@Auto/light 同时运行（各自独立 `AUTOOS_DESKTOP_CONFIG`），角点/
  占比双 oracle 过（#002b36 精确 / #ffffff 12.9%），策略互不串扰；错
  digest 候选 boot 具名拒绝保旧实录（"内容 digest 不符…候选拒绝（保
  legacy 链）"）；owned 退出（taskkill 树+会话自收割）后 app 进程归零。
- **包源消费腿**（solitaire 实证、机制同源）：缺 manifest/字节篡改/同
  mtime 内容变更三坏包均具名拒绝（"主题包目录不可用（包缺席或校验失败）
  ——候选拒绝（保 legacy 链…）"）、boot 存续；恢复后 family 重应用精确。
  内容锁（per-file sha256）不受同 mtime 伪装影响。
- **OS056-REAPPLY-NOTE 在本批复现（solitaire 一臂）**：活体外写翻转像素
  滞留；tetris/minesweeper 活体翻转实测生效。合同命名恢复路径"重开"全过；
  053 owner 注记维持，非本批回归。
- **standalone 跟随臂 dark_mode 位不同步注记（053 owner 观察扩展）**：
  跟随臂渲染层应用已提交策略派生 mode（像素 oracle 过），state 内
  dark_mode 位不同步；显式 AUTO_UI_THEME 轨位同步正常。若日后需命名
  "跟随臂位回写"为必需语义，属本页新合同项（053 owner），非 060 消费面。
- tetris/minesweeper 的 a2r 生成预存红（`??` E0599 / 数值比较 E0308，
  base 同败）与本批主题门轨选择（`--server vm` 既有支持模式）详见批次页；
  owner=auto-lang 后端生成。

## 产品批次消费面（PLAN-059 增补）

本页策略模型/提交序/拒绝保旧/交付通道不变；PLAN-059 为第四实际消费者批次
（Reader 阅读产品：018-book-reader，登记见
[../apps/product-theme-batch-059.md](../apps/product-theme-batch-059.md)）：

- reader 同 056–058 消费形态（固定 `vendor/themes` 快照 + `dep themes` +
  `use themes.themes: Auto`；pac `theme` 键缺省 = follow，产品历来无
  theme 键）。App 级 `design { theme: "Auto" }` + `dark_mode` 位派生
  mode（独立默认 light = 产品原纯浅色语义）。
- **阅读表面经产品正式引擎端口消费（本批新增消费形态）**：宿主/模式
  不直染出版物内容——阅读引擎配色（sepia/dark）经 READER-007 命令端口
  （`/api/reader/command` `op:"settings"`）按"显式书级选择 > 应用模式
  派生"接入；`window.__reader.appliedSettings.theme` 为端口读回 oracle。
  内容域（正文/图片/封面/批注语义色）归产品/引擎 owner 豁免。
- **vue 轨 design 投影门控缺口（本批具名修复，owner=auto-ui）**：
  `design { theme }` 运行时注入此前被 `has_accent_color ||
  has_theme_name` 静默截断——无 accent 产品（本批 reader、056 todo 同形）
  vue 轨官方包不上身；056–058 像素 oracle 均为 VM 轨，该缺陷未被此前
  批次门覆盖。修复（auto-ui 组树 `ui_gen/vue/script.rs` 门控扩三臂
  design.theme 臂）后 reader vue 轨像素/读回七腿全绿；056 todo 的 vue
  轨像素门从未执行的事实由本页注记，不追溯其验收结论。
- **OS056-REAPPLY-NOTE 边界确认**：本批恢复路径按合同命名"重开"实测
  通过（腿 FR）；live 外写翻转臂滞留维持 053 owner 注记，非本批回归。

## 产品批次消费面（PLAN-058 增补）

本页策略模型/提交序/拒绝保旧/交付通道不变；PLAN-058 为第三实际消费者批次
（媒体三应用：020-music-player、030-video-player、029-photo-gallery，登记见
[../apps/product-theme-batch-058.md](../apps/product-theme-batch-058.md)）：

- 三产品同 056/057 消费形态（固定 `vendor/themes` 快照 + `dep themes` +
  `use themes.themes: Auto`；pac `theme` 键缺省 = follow——music/video 原
  显式 `theme:"dark"` 键退役，photos 历来无键）。
- **双产品相反策略隔离腿（standalone follow 臂）**：music@Solarized/dark 与
  video@Auto/light 各持独立 `AUTOOS_DESKTOP_CONFIG` 同时运行，策略互不串扰
  （逐腿像素 oracle：Solarized dark `#002b36` / Auto light card `#ffffff`）；
  坏 digest 候选 boot 具名拒绝保旧（"内容 digest 不符——候选拒绝（保
  legacy 链）"实录）；策略文件改写后**重开追最新**双腿像素过
  （reopen@Auto/dark `#09090b` 实测）。**OS056-REAPPLY-NOTE 在本批复现**：
  运行中 live 外写翻转臂停留旧主题（#051a21 混合帧滞留；跟随探针 settle 8s 与翻转探针 60s 轮询两臂均不落定）——
  053 owner 注记维持，非本批回归；合同命名恢复路径"重开"实测通过。
- **video 双工具链例外（本批实录，非策略语义变更）**：video VM 轨在
  lang 2f558fc93 冻结线 fatal（子模块 dep-style 导入解析），主检出线正常；
  其门跑主检出链。owner=auto-lang 解析线（具名红，见批次页）。
- **media 能力臂后端生成红（video 真实 scan/播放腿阻塞）**：`backend
  Cargo.toml missing`（media_root 能力臂、无 api.at 形态；两链同败；
  photo 能力臂正常）——owner=auto-lang 后端生成，关闭条件见批次页。
  本批不削减该承诺。

## 产品批次消费面（PLAN-057 增补）

本页策略模型/提交序/拒绝保旧/交付通道不变；PLAN-057 为第二实际消费者批次
（桌面工具与设置六入口，登记见
[../apps/product-theme-batch-057.md](../apps/product-theme-batch-057.md)）：

- 六产品同 056 消费形态（固定 `vendor/themes` 快照 + `dep themes` +
  `use themes.themes: Auto`；pac `theme` 键缺省 = follow）。
- **os-config = 首个经 config 外写通道的策略编辑消费者**：其设置面
  「主题包策略」卡（family/mode）经既有 `PUT /api/config/desktop` 链写
  `theme_family`/`theme_mode` 两键（Plan 504 同一文件承诺），宿主 config
  外写轮询差分臂按 P041 送本页唯一提交入口（stage 校验→version/digest
  回填→publish revision；拒绝具名保旧）。`theme_rev` 外部写方不手改；
  family 未选时 mode 键为 no-op。宿主侧 boot 恢复消费**完整候选**（含
  version/digest）——仅 family+mode 的半候选被具名拒绝保旧（"候选 '(缺席)'
  ≠ 目录 0.2.0" 实录），family 候选需重新选择。
- **OS056-REAPPLY-NOTE 在本批复现**：拒绝后无间隔立即重投偶发不生效
  （weather standalone 轮询臂 1/2 次）；合同命名恢复路径"重开或重连"
  双产品像素级通过（reopen@dark luma 50/11）。维持 053 owner 注记不变。
- **独立默认腿 frame-0 观察（053/458 owner 注记，非本批缺陷）**：无已提交
  策略时 design mode 表达式（`if .dark_mode …`）在 frame-0 读到模型默认
  落位前值，启动呈现 family 默认浅色且位同步为 false；基线（未采纳）同现
  （A/B 双臂 light boot），一次用户翻转后策略链双向正常。宿主策略在场时
  boot 恢复即暗（luma 50/11），不受此影响。若需命名"无策略启动默认腿"
  为必需语义，属本页/458 新合同项（053 owner）。

## 产品批次消费面（PLAN-056 增补）

本页策略模型/提交序/拒绝保旧/交付通道不变；PLAN-056 为其**实际消费者扩展
批次**（七个生产力/内容产品 + blog 待其写窗，登记与逐产品合同见
[../apps/product-theme-batch-056.md](../apps/product-theme-batch-056.md)）：

- 产品经固定 `vendor/themes` 快照 + `dep themes` + `use themes.themes: Auto`
  （031 合同）消费；pac `theme` 键缺省 = follow 宿主，原显式键退役。
- **standalone 跟随臂实证约束**：会话子进程 = `auto-ui-tool.exe`
  （`--internal-vm-exec`），hosted 钩子路径的 arm 依赖**配套工具宿主二进制
  含 053 机制**——验证/部署前必须核验两二进制（旧工具宿主 = 跟随臂永不
  武装，全部 family 候选具名拒绝，仅 legacy 链生效）。
- 生成工作区一致性：产品 `auto run` 再生成 back workspace 时，组树工具链
  需 `AUTO_LANG_ROOT`/`AUTO_WEB_ROOT` 指主检出，否则混入组树路径触发
  `links` 冲突（消费侧运行事实，非策略语义变更）。
- V-04 包源消费腿（tampered 全不可用具名拒绝+design 默认保底 / 第二有效根
  验证后照常驱动）在 calendar 实证，机制全体产品同源（per-file sha256）。
- **宿主页 owner 注记（053 交付遗留观察，非本批缺陷）**：拒绝后无间隔成功
  提交的立即活体重投偶发不生效（standalone 轮询臂）；合同命名恢复路径
  "重开或重连"双腿双产品像素级全过。若日后需命名立即重投为必需语义，
  属本页新合同项（053 owner），非 056 消费面。

## 策略模型（SD-01）

桌面单源配置（`~/.config/autoos/apps/desktop/config.at`，env
`AUTOOS_DESKTOP_CONFIG` 覆盖）在既有 legacy 键（`dark_theme`/`theme_source`/
`theme_name`）之上承载**宿主主题策略组**：

- `theme_family`：官方主题包 family 标识（`Auto`/`Solarized`）。缺省缺键 =
  未选，行为与升级前逐字节一致（legacy 链原样）。
- `theme_family_version` + `theme_family_digest`：固定包内容锁（version
  `0.2.0` + 全文件聚合 sha256），与宿主目录快照比对，任一不符整候选拒绝。
- `theme_mode`：`light`|`dark`|`system`（运行派生值不落盘）。
- `theme_rev`：宿主分配的单调 revision（有效提交一次 +1）。

**提交序（唯一入口）**：完整候选校验（family 在目录、version/digest 相符、
mode 被 family 声明）→ 候选字段改写（含 legacy 兼容映射：mode light/dark →
`dark_theme` + `theme_source=manual`；system → `theme_source=system`）→
`config.at` 原子落盘 → 发布内存 revision → 通知（in-proc 喂端口 + outproc
广播 + shell 状态面）。写失败零内存变更零成功 ack；全等候选 `Unchanged`
零落盘（杀外写/重复写回环）。**bus 动词与 config 外写轮询走同一入口**
（P041 双路径等价的策略层）。

**拒绝与保旧**：未知 family、版本/digest 不符、mode 未声明、包目录不可用、
落盘失败 → 具名原因回写 shell 状态面（`__theme_error`），内存 policy 与
盘面保旧。已拒绝候选的 legacy `dark_theme` 翻转不得改色采纳者。

## 目录（managed channel）

宿主/standalone 进程的包目录解析为 env 双源、有序尝试：
`AUTOOS_THEME_PACKAGE_DIR` → `AUTO_UI_ROOT/packages/themes`。每个候选先
verify（owner/package/version/schema + per-file sha256 + 家族模式可用）
再算聚合 digest；失败跳过下一个候选；**全失败 = 降级**（family 链具名
拒绝，legacy 链不受影响）。**无主检出 fallback**：managed channel 缺席 =
降级，绝不静默换源到编译期锚定的检出目录。

## 交付通道（SD-01）

- **in-proc（desktop 宿主内 vm 应用）**：提交/启动/挂载三路对有主题
  scope 的组件喂 031 宿主端口（`HostThemeInputs{follow_host:true,
  host_family, host_mode}`）；发布失败整帧保旧（031 publish 语义）。
  采纳者的 `dark_mode` 模型位由**已提交策略的派生 mode**驱动（视图类
  分支消费），legacy `cfg.dark_theme` 翻转不越权。
- **outproc（协议客户端）**：`Hello` 尾段声明能力 `theme-policy/1`；
  attach/重连/策略变更三路推送完整快照帧
  （`ThemePolicyPush{revision, record}`，record =
  `rev␟family␟version␟digest␟mode`）；客户端喂 031 端口、帧构建评估后
  回 `ThemePolicyResult{revision, applied|rejected|unsupported|stale,
  reason}`；低于本连接已推 revision 的结果记 `stale` 不覆盖新状态。
  **已声明能力才推送**（旧客户端零未知帧）。零装载采纳（`adopt` 外部
  rqhost 客户端，D1 客户端权威）后同路推送。
- **standalone（`auto run -r vm`）**：env `AUTOOS_DESKTOP_CONFIG` 在场 =
  显式跟随宿主桌面配置——400ms 轮询差异臂与宿主同一提交入口；
  env 缺席零开销零行为差（应用 design 默认/legacy 链）。

## shell 状态面

`desktop.at` 空白菜单主题策略区（Auto/Solarized × 浅/深/跟随系统六直选）
+ 宿主注入 `__theme_family/__theme_mode/__theme_rev/__theme_status`（摘要
`family@mode revN` + 每应用 `applied|pending|rejected: 具名|unsupported|
stale|legacy` 行）与 `__theme_error`（具名拒绝/落盘失败诊断）。pending
自续拍：eval 落定在帧构建路径，待定行下拍再查，落定即停。

## 真实系统事件与平台边界（SD-05，PLAN-055 已闭）

- 受控腿（053 实证）：`theme_mode` 经 `system_prefers_dark()`
  （Windows 注册表只读查询）在 boot/load/帧评估消费。
- **真实腿已闭（PLAN-055 T-06，2026-11-05）**：获准短窗口内真实 OS 事件
  端到端实证——真实注册表翻转（`AppsUseLightTheme` 1→0，读回 0x0）→
  宿主 config 轮询差异臂（外写方按 poll 锚文档的「任意写方」面携带真实
  注册表值）→ stage/commit 新 revision → outproc 载体 push/帧后 applied +
  standalone 消费者 applied（时钟载体与 calc 双消费）；重启后
  `theme_source:"system"` 零提交穿越（policy 仍 system）；finally 原值
  1→1 精确恢复读回（`SystemUsesLightTheme` 全程未触碰）。控制腿：同内容
  mtime touch 零效果（值源 = 真实注册表，非 touch 本身）。
- **as-built system 语义（PLAN-055 实证钉定）**：family 校验只收包声明
  modes（Auto=light/dark）——`theme_mode=system` 不可经 family 链提交
  （`validate_fields` 具名拒绝）；「跟随系统」真源 = `theme_source:"system"`
  选择器（PLAN-615：每次 config load 从 OS 派生 `dark_theme`）；family
  提交臂按 053 语义把 `theme_source` 终结为 manual（显式 mode = 用户选择），
  恢复 system 跟随须显式写回选择器。
- 平台边界：provider 仅 Windows（`reg query`）；Linux/Mac 未测，不冒
  跨平台全绿。

## outproc 载体后继闭环（G-2，PLAN-055 已闭）

桌面组合式 outproc 端到端已由真实载体 **auto-clock**（a2r 生成 exe，嵌入
官方包身份/族表——与宿主 catalog 同一聚合 digest `9fa07742…`，三重链：
Spec 记载 = 宿主目录 = 载体嵌入）实证四腿全绿（PLAN-055 T-03/T-06，隔离
probe，唯一管道名+独立存储）：attach 推送（Hello `theme-policy/1` caps →
push → 帧后 applied 读回）、变更广播（config 外写 → 新 revision → 推送
applied）、重连取最新（杀进程重连 attach 即推最新快照非旧 rev）、stale
分列（受控注入：低 rev 结果记 stale 不覆盖，`theme_pushed_rev` 锚 session
级常驻单测）。具名拒绝保旧与恢复后真实再消费同腿实证（坏 family 外写零
提交零推送、载体保持旧值、合法重投新 rev applied）。协议修复随本批
（auto-ui `codex/plan-055-clock-theme-ui`）：Hello 双尾段「缺席=不写」
歧义（surfaces 空 + caps 非空时解码错位 TooShort）——任一尾段在场则
surfaces 段恒显式（计数可 0），双空旧端线字节不变（回归
`hello_trailing_segments_caps_without_surfaces_round_trip`）；a2r 主产线
HeadlessFrameSource 补主题转发三方法。多实例隔离：broker 管道 env
`AUTOOS_BROKER_PIPE` 覆盖（无 env 零行为差）。
