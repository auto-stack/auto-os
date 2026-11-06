# 独立应用仓与 AutoOS 组合

Status: v0.6 development baseline; product paths updated 2026-10-22 (PLAN-050).

## 仓库归属

| 应用 | 产品仓 | AutoOS 路径 | 端口 |
|---|---|---|---|
| 计算器 | [auto-calc](https://github.com/auto-stack/auto-calc) | `apps/auto-calc` | 17810 |
| 时钟 | [auto-clock](https://github.com/auto-stack/auto-clock) | `apps/auto-clock` | 17812 |
| 待办 | [auto-todo](https://github.com/auto-stack/auto-todo) | `apps/auto-todo` | 17814 / 17815 |
| 天气 | [auto-weather](https://github.com/auto-stack/auto-weather) | `apps/auto-weather` | 17816 |
| 便笺 | [auto-notes](https://github.com/auto-stack/auto-notes) | `apps/auto-notes` | 17818 / 17819 |
| 日历 | [auto-calendar](https://github.com/auto-stack/auto-calendar) | `apps/auto-calendar` | 17820 |
| 聊天 | [auto-chat](https://github.com/auto-stack/auto-chat) | `apps/auto-chat` | 17822 / 17823 |
| 图书阅读器 | [auto-reader](https://github.com/auto-stack/auto-reader) | `apps/auto-reader` | 17824 / 17825 |
| 博客与文章社区 | [auto-blog](https://github.com/auto-stack/auto-blog) | `apps/auto-blog` | 17826 / 17827 |
| 视频播放器 | [auto-video](https://github.com/auto-stack/auto-video) | `apps/auto-video` | 17828 / 17829 |
| 音乐播放器 | [auto-music](https://github.com/auto-stack/auto-music) | `apps/auto-music` | 17830 / 17831 |
| 图库与图片查看器 | [auto-photos](https://github.com/auto-stack/auto-photos) | `apps/auto-photos` | 17832 / 17833 |
| 画板 | [auto-paint](https://github.com/auto-stack/auto-paint) | `apps/auto-paint` | 17834 |
| 系统监视器 | [auto-monitor](https://github.com/auto-stack/auto-monitor) | `apps/auto-monitor` | 17836 / 17837 |
| 数据库管理工具 | [auto-database](https://github.com/auto-stack/auto-database) | `apps/auto-database` | 17838 |
| 文件与资源浏览器 | [auto-explorer](https://github.com/auto-stack/auto-explorer) | `apps/auto-explorer` | 17840 / 17841 |
| 通用启动器 | [auto-launcher](https://github.com/auto-stack/auto-launcher) | `apps/auto-launcher` | 17842 |
| 俄罗斯方块 | [auto-tetris](https://github.com/auto-stack/auto-tetris) | `apps/auto-tetris` | 17844 / 17845 |
| 纸牌接龙 | [auto-solitaire](https://github.com/auto-stack/auto-solitaire) | `apps/auto-solitaire` | 17846 / 17847 |
| 扫雷 | [auto-minesweeper](https://github.com/auto-stack/auto-minesweeper) | `apps/auto-minesweeper` | 17848 |
| 图表工坊 | [auto-charts](https://github.com/auto-stack/auto-charts) | `apps/auto-charts` | 17850 |

已有看板、终端、配置分别固定在 `apps/auto-kanban`、`apps/auto-term`、`apps/auto-os-config`。
终端的实际应用根为 `apps/auto-term/app`，配置为 `apps/auto-os-config/auto`，通过 manifest 保留启动 ID。
系统日志 `apps/auto-syslog` 保留为 OS 内部组件。四大主力 app 沿既有外部接入配置，
不在此次旧基线导入中迁移或改名；其中旧 Jade 配置仍待完整 v0.5 基线恢复后对齐。

博客以 RealWorld 为当前入口，旧博客阅读器保存在同仓 references；视频以播放器为入口，
社区来源保存在 references，近期不部署社区；图库为 Photos 默认入口，图片查看器来源保存在 references。
这些归属合并不表示已完成全部功能整合。教学 Demo 不回写产品改动。

## 配套契约

- **基础应用组合（PLAN-054）**：八个固定产品 pin、前后端支持形态和组合任务见
  [八应用桌面组合](basic-app-assembly.md)。产品交付/推送、组合 pin、实际桌面任务
  与公共 SDK 身份分别核验；不以更新 gitlink 或单个 app 独立通过替代组合验收。
- **主题批次（PLAN-056）**：notes/todo/calendar/chat/paint/database/charts 七产品 + blog（待其写窗）
  已按固定 `vendor/themes` 快照接入官方主题包（follow 语义 + design 默认腿），
  候选冻结与逐产品合同见 [product-theme-batch-056](product-theme-batch-056.md)；
  OS 组合 gitlink 更新等 PLAN-054 审定业务版本（§5.3 冻结裁定），阶段计数仅按实际验收。
- **主题批次（PLAN-057）**：weather/launcher/monitor/explorer/os-config 五产品独立仓候选
  （8c13952/134d047/6ff5646/8505ef2/0018066）+ syslog（OS local，`apps/auto-syslog` 树内），
  同 056 消费形态接入官方主题包；os-config 另含 053 policy 编辑消费面
  （详见 [product-theme-batch-057](product-theme-batch-057.md)）。本批 OS 组合
  gitlink 按本批自身门更新（fetch/detach 同 SHA + 健康门 + 实际启动后提交），
  无 054 全局前置；阶段计数仅按实际验收。
- **主题批次（PLAN-058）**：music/video/photos 三媒体产品独立仓候选
  （4b9e446/0922b44/7ce8bd2），同 056/057 消费形态接入官方主题包
  （详见 [product-theme-batch-058](product-theme-batch-058.md)；photos 另含
  本仓首个永久回归门补齐）。本批 OS 组合 gitlink 按本批自身门更新
  （fetch/detach 同 SHA + 健康门 + 实际启动后提交），无 054 全局前置；
  video 两项具名预存红（media 能力臂后端生成、冻结线子模块导入解析）
  owner=auto-lang，关闭条件与 A/B 证据见批次页；阶段计数仅按实际验收。
- **主题批次（PLAN-059）**：reader（018-book-reader）阅读产品独立仓候选
  （SHA 于 T-05 回填），同 056–058 消费形态接入官方主题包；阅读表面经
  READER-007 正式引擎端口按"显式书级选择 > 应用模式派生"接入（详见
  [product-theme-batch-059](product-theme-batch-059.md)）。本批 OS 组合
  gitlink 按本批自身门更新（fetch/detach 同 SHA + 健康门 + 实际启动后
  提交），无 054 全局前置；vue 轨 design 投影门控缺口具名修复
  owner=auto-ui（证据见批次页）；阶段计数仅按实际验收。
- **主题批次（PLAN-061）**：看板/终端/外部工作区五产品独立演进候选——
  kanban（auto-kanban，a19c4ba0，main）、auto-musk（d93d9198，musk-061
  worktree plan-061-dev）、jade-garden（auto-down 嵌套 front/auto，
  2180f5a8，os-061-dev owner 树）、auto-term（c9e5e561+0677020，main，
  app/ 根）、jade-edit（8d2dbf86，main）。接入形态：四产品同 056–060
  vendor 快照消费形态；jade-garden 为构建链静态投影消费（生成四块→部署
  CSS 变量，品牌 overlay 保留；单族边界具名）。musk 为 musk-061 专属
  worktree（其 AGENTS 规约）、auto-down 为 os-061 组 owner 树——外部/
  嵌套根无 OS gitlink，组合身份以候选 SHA+daemon/工具身份固定，不伪造
  submodule。逐产品表面/固定值见各仓 docs/specs/theme-adoption.md；
  详见 [product-theme-batch-061](product-theme-batch-061.md)。预存红
  （kanban manual 写流/jade-edit vm_matrix 深检查/term CLI 轨——base
  对拍同域）owner 见批次页；阶段计数仅按实际验收，OS pin 按 merge 自身
  门更新。
- **主题批次（PLAN-060）**：桌面游戏三产品独立仓候选——solitaire
  （037-klondike，54c0051）、tetris（036-tetris，d252f36）、minesweeper
  （038-minesweeper，6873674），同 056–059 消费形态接入官方主题包
  （逐产品表面/固定值见各仓 docs/specs/theme-adoption.md；详见
  [product-theme-batch-060](product-theme-batch-060.md)）。本批 OS 组合
  gitlink 按本批自身门更新（fetch/detach 同 SHA + 健康门 + 实际启动后
  提交），无 054 全局前置；tetris/minesweeper 的 a2r 生成预存红（base
  同败）owner=auto-lang，主题轨走 `--server vm` 既有支持模式，关闭条件见
  批次页；阶段计数仅按实际验收。
- apps.manifest 使用 kind=repo，repo 指实际子模块应用根；.gitmodules 与 160000 gitlink 同一提交维护。
- 21 个新产品的物理目录使用正式产品仓名（如 `apps/auto-notes`）；原主来源 ID 继续保留桌面身份、图标与引用。新产品端口在 17810–17851，
  各 app 实际占用以 pac.at 与 manifest 为准，无后端的不声明后端端口。
- 原 AutoOS 产品目录 025/028/036/037/038 迁为同路径 gitlink；源码先保存在各产品仓，不覆盖隐藏 v0.5 更新。
- 默认桌面脚本在 Vue 轨设置 AUTO_DESKTOP_APPS=<os>/apps，VM 轨透传 --apps-dir <os>/apps。
  不使用 AUTO_DESKTOP_APPS_EXTRA 全替换，以保留 manifest 的嵌套根启动 ID 和画廊聚合。
- StyleKit 为应用仓内固定 vendor 源码；便笺的 AutoDown 使用固定子模块与当前 engine 包。
  开发机媒体目录不是默认配置，用户通过对应运行时 env 设置。

## 来源恢复

每仓 SOURCE-IMPORT.json 记录原 source_repo/commit/path 与逐文件 SHA256。
首个 source-sync 提交是未包装的源码快照；main/v0.6-dev 包含后续包装。主机恢复后，
从 source-sync 基线导入完整 v0.5 差异，再与产品开发线三方合并，禁止整目录覆盖产品改动。
AutoOS 更新 gitlink 前，目标提交必须已推送且可取回。系统日志无需产品仓。

## 验证

python scripts/verify-app-links.py 核对固定 SHA、manifest、端口、依赖与来源 hash。
注册表发现还需按实际 app_registry.rs 验证 entry 来自子模块；该验证与 GUI 功能验收区分。
全新 clone 初始化子模块是组合可复现性的验收。运行生成器、媒体解码、后端行为及双端交互另按 app 验证。

## 目录名与稳定 ID（PLAN-050）

物理目录按产品仓名命名；manifest `id` 不随目录移动改变。注册表按相邻 manifest 的 `repo` 路径匹配稳定 ID，主扫描、容器扫描与 Vue 采用同一规则；无匹配或清单不可读时回退目录名。`.gitmodules` 逻辑节名和 `.git/modules` 内部存储名保留，以避免重新初始化或改写子模块历史。历史导入报告保留原路径；校验器按当前 manifest/产品仓解析现路径。`--metadata-only` 仅核对组合元数据，完整默认校验仍要求初始化检出。

迁移映射与日常操作见[产品目录命名](../../design/strategy/app-directory-names.md)。
