---
plan_id: PLAN-009
status: executing              # drafting → executing → execution_done → reviewed → archived
feature_name: p6-desktop-bootstrap
author: [zhaopuming, ZCode]
created_at: 2026-09-07
updated_at: 2026-09-07

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: []
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [auto-os/scripts, auto-os/ci, auto-lang/ci]  # 受影响的 specs 路径
current_step: 2
total_steps: 8
---

# [PLAN-009] Stage B P-6——桌面启动引导批（§3-a 包装脚本 + V1/V2/V3 实机验收 + CI 保活 + 画廊部署触发）

## 变更摘要

P-5 本体批（auto-lang PLAN-590，已归档）收官后的 P-6 周边件承载计划。四件：
① §3-a 包装脚本（本仓 `scripts/desktop.ps1`/`desktop.sh`——解析序定位
auto-lang，注入 env 调既有两条入口，框架零改动）；② P-5 遗交的 **V1/V2/V3
实机验收**（590 复审 R1，用户裁定维持移交至本批）；③ **R4 CI 保活**——
auto-lang CI runner checkout auto-os，画廊语料围栏（gallery_pages_compile/
schema_drift/docs_gen 等 solo-skip 面）在 CI 不再静默跳过；④ **画廊变更触发
website 部署**——本仓 workflow 触发 auto-lang deploy-website 的 `gallery`
输入（590 已改好消费端，缺触发端）。

**V8（随迁计划可执行性）已随本计划立项先行冒烟完成**：os-003（554 clock）
drafting→executing + T1 探针 ✅（路径经解析序 `../auto-lang` 换算全可达，
提交 `a627aae`）。os-003 后续执行属其自身计划，不属本件。

## 目标

1. `scripts/desktop.ps1`/`desktop.sh`：按解析序（`AUTO_LANG_ROOT` env →
   `../auto-lang` → `D:/autostack/auto-lang`）定位框架仓，从本仓任一位置
   一命令起桌面（vue 轨 `auto run --desktop` / iced 轨 `cargo run
   --example ui_desktop`），注册表经 P-3 三源聚合呈现本仓 `apps/`。
2. **V5 重证（经包装脚本）**：boot 注册表三类条目（框架 demo + 本仓 apps/
   + kanban），与 590 直接 boot 实证（38 entries/22 desktop-visible）同口径。
3. **V2 desktop_mcp 零漂移**：随迁三 app（025-sys-monitor/028-launcher/
   038-minesweeper）+ 框架侧在装 app 的 `tests/desktop_mcp.py` 从**迁移后
   位置**跑，锚数与 I2 台账（14/11/11/19/26 五套）同数零漂移。
4. **V3 双端一致性抽查**：launcher + minesweeper，vue/vm 双轨（autoui-verifier）。
5. **V1 五面交互实机**：wrapper 起全屏桌面——dock/任务栏/switcher/pager/
   通知中心 + launcher 启动迁移 app，证据笔记成文。
6. **R4**：auto-lang `vm-files-ci.yml`（及涉画廊围栏的 workflow）checkout
   `auto-stack/auto-os` + `AUTO_OS_ROOT` env——围栏在 CI 激活（非 SKIP）。
7. **画廊部署触发端**：本仓 `.github/workflows/`（新建）在 `ui-gallery|
   widgets-gallery|apps/**` 变更时触发 auto-lang deploy-website
   （`gallery=true`）——跨仓触发需 PAT，token 配置为待澄清项。

## 架构方案

- **包装脚本（§3-a 定案形态）**：薄包装，零框架改动。步骤：解析序定位
  auto-lang → 依参数选轨（`-Track vue|vm|iced`，缺省 vue）→ vue/vm 轨调
  auto-lang 仓 `auto run --desktop`（vm 轨 `-r vm`；CWD=本仓根时 P-3 的
  `../auto-os/apps` 兄弟探测**自然命中**，无需 env 注入；跨仓根运行再注
  `AUTO_DESKTOP_APPS`）→ iced 轨 `cargo run -p auto-lang --features ui-iced
  --example ui_desktop -- --fullscreen`（CWD=auto-lang 根）。
- **V2/V3 执行面**：`tests/desktop_mcp.py` 随 app 目录已物理在本仓
  `apps/*/tests/`；驱动器=auto-lang `.agents/skills/autoui-verifier/scripts/
  test_vm_mcp.py`（跨仓调用，路径经解析序）。零漂移口径=590 迁移前后同数。
- **R4**：vm-files-ci 加一步 checkout auto-stack/auto-os（path: auto-os）+
  `AUTO_OS_ROOT: ${{ github.workspace }}/auto-os`——`os_paths::
  resolve_os_top_dir` env 臂命中，围栏激活。
- **触发端**：v1 用 repository_dispatch → auto-lang workflow_dispatch
  `gallery=true`（需 `AUTO_LANG_DISPATCH_TOKEN` secret，用户配置）；
  无 token 时的降级=手动 dispatch，文档注记。

## 需求分析与背景调查
（从 auto-os Design 01 与 590 复审记录取材）

- Design 01 §3（入口委托 ADR：Stage B=选项 a 薄包装，b 一等子命令列
  Stage C）；§4-P3 落地注记（「§3-a 包装脚本随 P-6 落地，用户裁定；P-3
  以 env 注入等价形态验证」）；§6 V1/V2/V3/V5（P-5 适用项已随 R1 移交
  本批）；§7 P-6 行（590 增补：V1/V2/V3 + 包装脚本 + 触发端）。
- 590 复审 R1（用户裁定维持移交）/R4（solo-skip CI 保活）。
- V2 基线：I2 五套锚数 14/11/11/19/26（472/478/479 台账，590 迁移前形态）。
- V8 已证：os-003 T1 探针（解析序路径可达 + vue 轨 `running` 门控差发现）。

## 详细设计

- `scripts/desktop.ps1`：param(`-Track`/`-Fullscreen`)；`Resolve-LangRoot`
  （env → 兄弟 → 主检出，三候选 join 探测 `crates/auto-lang` 存在性）；
  轨道分发 + CWD/env 注入（见架构方案）；启动前打印解析结果一行（可审计）。
- `scripts/desktop.sh`：同语义 bash 版（WSL/git-bash 可用）。
- CI：`vm-files-ci.yml` job 步骤插 checkout（auto-os）+ env；跑后核
  gallery_pages_compile 非 SKIP（日志 grep `SKIPPED — auto-os` 零命中）。
- 触发 workflow：`deploy-gallery-trigger.yml`——push main 路径过滤
  （`ui-gallery/**`,`widgets-gallery/**`,`apps/**`）→ repository_dispatch
  （auto-stack/auto-lang，event `deploy-website`，client_payload gallery=true）
  + auto-lang 侧 deploy-website.yml 增 `on: repository_dispatch` 臂（该文件
  属 auto-lang，改动走 lang 侧 worktree——本计划跨仓件）。

## 测试设计

- V5：wrapper iced 轨 boot 日志 grep `app registry: 38 entries`（同 590 口径）。
- V2：三 app + 框架在装五套 desktop_mcp 锚数对账表（前后同数）。
- V3：launcher/minesweeper vue+vm 双轨截图/结构断言（autoui-verifier 惯例）。
- V1：五面交互证据笔记（docs/reports/ 或 plan 内嵌，截图目录）。
- R4：CI 日志核围栏激活（无 SKIPPED 行）。
- 触发端：dry-run（workflow_dispatch 手动触发一次 deploy，gallery 输入
  生效=build 段进入）。

## 验收标准

1. wrapper 双轨可起桌面；V5 经 wrapper 重证（三源聚合 boot 日志）。
2. V2 零漂移（五套锚数对账表在案）；V3 双端一致抽查过。
3. V1 五面交互证据笔记成文（含 launcher 启动迁移 app 实录）。
4. R4：CI 围栏激活实证（vm-files-ci 日志无画廊 SKIPPED）。
5. 触发端 workflow 在案（或 token 未配时降级路径文档化+待澄清登记）。
6. Design 01 §7 P-6 行回填 + 台账更新；两仓提交在案。

## 执行步骤
（原子任务：精确文件路径 + 确切操作 + 验证命令；每步完成后追加 [✅ 已完成] 一行证据）

- [✅ 已完成] **T1 包装脚本**：`scripts/desktop.ps1`+`desktop.sh`（解析序+轨道分发）
  验证：`./scripts/desktop.ps1 -Track iced` boot 日志含 `app registry: 38 entries`
  —— worktree 提交 `a860a17`。bash/ps1 双 dry-run 过；worktree iced 轨真 boot
  **36 entries (20 desktop-visible)**=33 主检出 demo+3 本仓 apps 容器自命中
  （组内无 os-config/kanban 兄弟=solo 语义）。**实证教训（脚本内注记）**：
  cargo 按调用方 CWD 发现 `.cargo/config.toml`——从本仓 cargo run 丢
  `/STACK:32MB` 致起动即栈溢出；修法=lang 侧 build + 本仓 CWD 直接 exec exe。
- [✅ 已完成] **T2 V5 重证**：wrapper 起 iced 桌面，boot 日志三源对账（38/22 口径）
  验证：日志 grep + 条目分类清单
  —— `DESKTOP_OS_ROOT=/d/autostack/auto-os bash scripts/desktop.sh iced` →
  **`38 entries (22 desktop-visible)`** = 33 框架 demo + 3 本仓 apps + os-config
  + kanban，与 590 直接 boot 实证全三源同口径 ✓（经 wrapper 路径成立）。
- [ ] **T3 V2 零漂移**：三迁移 app + 框架在装 app desktop_mcp 全跑，锚数
  对账 14/11/11/19/26
  验证：对账表写入本计划测试设计节
- [ ] **T4 V3 双端抽查**：launcher+minesweeper vue/vm（autoui-verifier）
  验证：双轨证据（截图/断言输出）
- [ ] **T5 V1 五面实机**：wrapper 全屏桌面五面交互 + launcher 启动 025/038
  验证：证据笔记（plan 内嵌或 docs/reports/p6-v1/）
- [ ] **T6 R4 CI 保活**：auto-lang `vm-files-ci.yml` checkout auto-os +
  AUTO_OS_ROOT（lang 侧 worktree 提交）
  验证：CI 日志无 `SKIPPED — auto-os`（本地等价：AUTO_OS_ROOT=… 跑
  gallery_pages_compile 非 SKIP）
- [ ] **T7 触发端**：本仓 `deploy-gallery-trigger.yml` + auto-lang
  deploy-website 增 repository_dispatch 臂（lang 侧 worktree）
  验证：手动 dispatch 一次 gallery=true 全链生效（或降级文档+待澄清）
- [ ] **T8 收口**：Design 01 §7 P-6 行 ✅ + 台账 + specs 沉淀 + 归档

## 复审记录

## 待澄清事项

1. **跨仓触发 token**：repository_dispatch 需 `AUTO_LANG_DISPATCH_TOKEN`
   （PAT）配置于本仓 secrets——用户配置后 T7 全链生效；未配则降级手动
   dispatch + 文档注记（不阻断本计划）。
2. V1 五面交互为本机实机项（Windows 首选）；若执行环境受限按 472/478
   headless 指针先例成文（用户知情后裁定）。
