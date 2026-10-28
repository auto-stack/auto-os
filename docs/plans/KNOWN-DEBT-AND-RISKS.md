# Known debt and verification limits

## PLAN-876 桌面入口迁移（2026-10-28，本仓首次按新归属登记）

- **OS876-APP-EXE（open）**：`028-launcher` / `036-tetris` / `037-klondike` /
  `038-minesweeper` 在 `pac.at` 声明 `desktop_exe:`，但 clone 不携带产物（各仓 README
  明写 `auto build -r rust` 生成），故无法原生启动。现网按 PLAN-876 语义**降级解释态**
  并在 boot 自检打 `declared_exe_missing=` 与建议；清偿 = 逐 app `auto build -r rust`
  出产物，或在 `apps.manifest` 对它们显式 `"launch": "vm"` 定版（二选一需产品裁定）。
- **OS876-SUBMODULE（open）**：2026-10-28 走查发现 24 个 app 子模块工作区被清空
  （只剩 `.git` + 索引成批 `deleted`），导致注册表缩到 9 条、小组件 `faces=0`。已按
  "取证 → `git reset --hard HEAD` → 5 个残留目录移备份 → `git submodule update
  --init --recursive`" 恢复（证据 `auto-lang/tmp/autoos-apps-state-20261028-152823.txt`，
  备份 `D:\autostack\tmp\auto-os-apps-stale-20261028-152844`）。健康面 `apps_empty`
  只做检测、不自动修复；**清空根因未知**，闭合需定位是哪条流程/脚本造成。
- **OS876-TERM-TICK（open，跨计划）**：`auto-term` 重启后 `handler_App_Tick` 持续报
  `RuntimeError("future_all/race: empty or invalid future list")`（crash ip=0x22da）。
  auto-lang `docs/plans/KNOWN-DEBT-AND-RISKS.md` 的 P723-D1 是同一现象的旧记录；
  修复归终端 app + VM future 语义，本计划只登记指针。
- **OS876-USE-DIAG（open，转派）**：`widgets-gallery` 启动多条
  `[AUTO-USE-DIAG] P-15 use 模块 '{ package' 解析失败`（`use { package "official"
  ... }` 形态）。属语言侧 `use` 语法/plan 664 解析线，本计划不修；闭合需在 auto-lang
  侧定位后回填本行。
- **OS876-DOC-SWEEP（open，app 仓）**：`apps/auto-syslog/README.md` 仍写旧入口
  旧 auto-lang example 入口命令。该文件在子模块内，
  auto-os 不能代提交；需在 app 仓改并推进 pin。

## Plan 051 合入后真实 VM 检查（2026-10-28）

- **OS051-RUN-VIDEO（open）**：迁后三仓构建并启动桌面成功，但 `030-video-player` 在 `apps/auto-video/src/front/playlist.at:124:8` 报 undefined variable，阻止该应用构建。尚未定位导入/样式解析原因，也未与旧组合对照；不可直接认定迁移回归。闭合需定位原因并验证实际视频入口与媒体交互。
- **OS051-RUN-MEDIA（open）**：Music `/api/media/scan`、Photos `/api/photos/scan` 的 back proxy 返回 404 no-route；空媒体视图不能充当扫描成功。闭合需核对后端路由/调用契约并完成真实有内容的扫描验证；尚未与旧组合对照。
- **OS051-RUN-STACK（closed）**：真实独立 `auto run -r vm` 暴露 UI 工具 Windows 主栈 1 MiB 导致 `0xc00000fd`；auto-ui `5a89bfd` 加入工具 owner 的 64 MiB 主栈链接策略，普通 Cargo 重建及 PE 检查通过，相同入口真实按钮运算 `2+3=5`。桌面内计算器也通过；不代表所有 app 通过。

输入提交、命令、产物/截图 hash、证据路径与范围见 [051 实测收据](attachments/051/runtime-check.md)。原始证据仅本地保留。


## v0.6 应用独立化（2026-10-04）

- **V06-APP-001（既有基线/范围外）**：当前开发组缺少旧 manifest 引用的 Musk 与 Jade Garden 外部应用根；注册表产生对应跳过警告。四大主力应用按用户决定暂不迁移；完整 v0.5 恢复后对齐实际仓与入口。新 21 项产品入口不受此项影响。
- **V06-APP-002（运行验证范围）**：未执行 21 项完整 GUI/后端/媒体交互；本机无可用 Auto CLI/桌面二进制。本轮验收为仓库组织与接线，不把路径探针称为双端运行验收。下一阶段逐 app 使用 autoui-verifier 验证。
- **V06-APP-003（知识/发布工程）**：Notes 暂以固定 AutoDown 子模块携带 engine 源码，StyleKit 为固定 vendor 快照。统一 package/install 成熟后可改为版本依赖；本轮不改包管理协议。终端历史证据日志很大， fresh-clone 验证以浅/过滤/稀疏模式获取同一 pin，正式配置不改。
- **V06-APP-004（恢复任务）**：五个 OS 目录转 gitlink，隐藏 v0.5 修改须先导入对应产品 source-sync，再合产品开发线并更新 OS pin；禁止用旧目录覆盖新产品，父仓文件/目录冲突按应用处理。临时计划编号待主机恢复后登记，避免占用不可见计划 ID。
