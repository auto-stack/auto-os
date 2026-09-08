---
plan_id: PLAN-009
status: reviewed                # drafting → executing → execution_done → reviewed → archived
feature_name: p6-desktop-bootstrap
author: [zhaopuming, ZCode]
created_at: 2026-09-07
updated_at: 2026-09-08（execution_done：T1-T7 全 ✅，T8 交接 review/merge）

# /auto-plan:review 结束时填写：
supersedes_spec_components:
  - "docs/design/01-stage-b-desktop-migration.md: 修改——§7 P-6 行承载批落地回填"
new_spec_components:
  - "scripts/desktop.ps1+desktop.sh: 新增——§3-a 桌面薄包装（解析序定位 auto-lang；vue 轨三 env 注入含容器展开；iced 轨 lang 侧 build+本仓 CWD 直 exec；DESKTOP_OS_ROOT 覆盖臂；cargo CWD 栈旗标与 cygpath 路径两教训注记）"
  - ".github/workflows/deploy-gallery-trigger.yml: 新增——画廊/桌面域资产变更→repository_dispatch 触发 auto-lang website 部署（token 未配降级在案）"
  - "auto-lang .github/workflows/vm-files-ci.yml: 修改——R4 画廊围栏 CI 保活（auto-os sparse checkout+AUTO_OS_ROOT+围栏真跑步）"
touched_goals:
  - "GOAL-010: 应用轨道产品化——auto-os 一命令起桌面（双轨）+V1/V2/V3 实机验收收拢+CI 围栏保活"

affects: [auto-os/scripts, auto-os/ci, auto-lang/ci]  # 受影响的 specs 路径
current_step: 7
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
- [✅ 已完成] **T3 V2 零漂移**：三迁移 app + 框架在装 app desktop_mcp 全跑，锚数
  对账 14/11/11/19/26
  验证：对账表写入本计划测试设计节
  —— worktree `847a769`。**对账表（2026-09-07 实测）**：
  | 套件 | 位置 | 基线 | 实测 | 裁定 |
  |---|---|---|---|---|
  | 025-sys-monitor | 本仓 apps/ | 13（541） | **13P/0F** | ✓✓ 零漂移 |
  | 028-launcher | 本仓 apps/ | 24 断言（464） | **24P/0F** | ✓✓（nres 4→5 组成漂移断言更新随批；fresh .am 落点注记） |
  | 038-minesweeper | 本仓 apps/ | 无文档数 | **22P/0F** | ✓ 绿（基线=现状 22 在案） |
  | 013-todo / 015-notes | lang examples/ui | 11/11 | 22P/0F、13P/0F/1S | ✓ 绿（套件已增长） |
  | 011-calculator / 024-charts | lang examples/ui | 14/19（472 时代） | 12P/5F、12P/7F | **master 预存红**（未迁 app，590 零代码路径交集；债候选转 R 清单） |
  迁址注记：套件 auto 二进制相对定位（`..\..\..\..\target`）随迁失效，
  需 `AUTO_BIN=<lang>/target/debug/auto.exe` env（在案，套件头已支持）。
- [✅ 已完成] **T4 V3 双端抽查**：launcher+minesweeper vue/vm（autoui-verifier）
  验证：双轨证据（截图/断言输出）
  —— worktree `dfea1a1`。vue 轨（`auto run` dev server + playwright
  headless）三截图 `docs/reports/p6-v3/`（1280×800：launcher 待召态 9K /
  开 palette 30K / minesweeper 开局 18K，内容量递增=渲染非空白）；vm 轨
  证据=T3 套件（028=24/0、038=22/0，palette/过滤/启动/雷区交互深覆盖）。
  环境注记：vue 首装遇 huawei 镜象未同步 `@tanstack/virtual-core@3.17.9`
  404——`pnpm install --registry=https://registry.npmjs.org` 过（55.7s）。
- [✅ 已完成] **T5 V1 五面实机**：wrapper 全屏桌面五面交互 + launcher 启动 025/038
  验证：证据笔记（plan 内嵌或 docs/reports/p6-v1/）
  —— worktree `d0a031f`。`docs/reports/p6-v1/`：七截图+证据笔记
  （dock/任务栏 ✓、launcher Ctrl+Space 召唤+键盘级过滤+Enter **启动迁移
  app 038 端到端 ✓✓**、switcher MRU ✓、pager ✓、通知面 ✓）。注记：shell
  面输入须 `autoui_keyboard` 逐键（find/type 只及聚焦 app）。
- [✅ 已完成] **T6 R4 CI 保活**：auto-lang `vm-files-ci.yml` checkout auto-os +
  AUTO_OS_ROOT（lang 侧 worktree 提交）
  验证：CI 日志无 `SKIPPED — auto-os`（本地等价：AUTO_OS_ROOT=… 跑
  gallery_pages_compile 非 SKIP）
  —— lang `os-009-dev` `018ec4325`：sparse checkout（widgets-gallery）+
  env 权威臂 + 语料在位断言步 + **gallery_pages_compile 真跑步**（此前 CI
  filter 不含该围栏=静默 solo-skip）。本地等价双向实证：env 命中 11.3s
  真编译 / env 指空 0.009s SKIP（显式关断语义 ✓）。
- [✅ 已完成] **T7 触发端**：本仓 `deploy-gallery-trigger.yml` + auto-lang
  deploy-website 增 repository_dispatch 臂（lang 侧 worktree）
  验证：手动 dispatch 一次 gallery=true 全链生效（或降级文档+待澄清）
  —— 本仓 `8be535f`（触发端：ui-gallery/widgets-gallery/apps 路径过滤 →
  repository_dispatch auto-lang；token 未配=跳过+降级指引不阻断）+ lang
  `f6b40278c`（deploy-website 增 dispatch 臂+五处画廊条件扩展）。全链
  live 验证待 token 配置（待澄清 #1）；降级路径文档化在 workflow 内。
- [→ 交接 /auto-plan:review] **T8 收口**：Design 01 §7 P-6 行回填（本批）+
  台账（os-003 行已在案）+ specs 沉淀/归档（review/merge 承载）。
  执行侧交付：auto-os worktree `plan-009-dev` 五提交（a860a17/847a769/
  dfea1a1/d0a031f/8be535f）+ lang `os-009-dev` 两提交（018ec4325/f6b40278c）。

## 复审记录

## 待澄清事项

1. **跨仓触发 token**：repository_dispatch 需 `AUTO_LANG_DISPATCH_TOKEN`
   （PAT）配置于本仓 secrets——用户配置后 T7 全链生效；未配则降级手动
   dispatch + 文档注记（不阻断本计划）。
2. V1 五面交互为本机实机项（Windows 首选）；若执行环境受限按 472/478
   headless 指针先例成文（用户知情后裁定）。

## 复审记录

**复审人**：ZCode（/auto-plan:review 独立复审），2026-09-08。
**方法**：计划 vs 两 worktree 实际 diff（auto-os `plan-009-dev` 六提交 +
lang `os-009-dev` 两提交）逐项重证；关键验证全部重跑。

### 逐项验收裁定（verify, don't trust）

| # | 验收标准 | 裁定 | 复审证据 |
|---|---|---|---|
| 1 | wrapper 双轨可起桌面；V5 经 wrapper 重证 | **过（复审修正后）** | **复审打回→当批修复→复证**：执行期 vue 轨仅 dry-run（未真起）——复审真跑暴露三处缺陷（①漏注 `AUTO_DESKTOP_APPS` 主注册表，vue.rs 缺省解析 `<project>/examples/ui` 必败；②`AUTO_DESKTOP_APPS_EXTRA` 为单 app 根**全替换**语义，容器须脚本侧展开为 `;` 路径表；③Git-Bash `/d/` 路径对 Windows 进程 `is_dir()` 必假，须 `cygpath -m` 转 `D:/`）——`9d960fe` 修复后：vue 轨 host 起+**三 extra root 命中**（23 apps，launcher/minesweeper 直挂）+iced 轨 38/22 确定性复证（AUTO_OS_ROOT 原生形）+ps1 dry-run 展开对 |
| 2 | V2 零漂移对账表；V3 抽查 | **过** | 对账表在计划（T3）；复审档重跑 025-sys-monitor=**13/0**（541 基线零漂移锚复证）；V3 证据文件在案 |
| 3 | V1 五面证据笔记 | **过** | `docs/reports/p6-v1/` 笔记+11 截图+驱动脚本；launcher→038 启动端到端 |
| 4 | R4 CI 围栏激活 | **过** | 复审档重跑双向：env 命中 11.3s 真编译/指空 0.02s SKIP；yml 增真跑步（此前 CI filter 不含该围栏） |
| 5 | 触发端 workflow 在案 | **过（token 待配=计划预留降级口径）** | 双侧 workflow 落位+yaml 解析过；全链 live 待 `AUTO_LANG_DISPATCH_TOKEN`（待澄清 #1，用户知情） |
| 6 | Design 01/台账/两仓提交 | **过** | §7 P-6 行承载批回填；台账 os-003 行；提交清单核齐 |

### 遗漏/延后/Workaround 猎查

- **复审抓回的遗漏**：vue 轨未真跑即报 ✅（验收①「双轨」字面未兑现）——已按
  打回-修复-复证闭环（`9d960fe`），教训入脚本注记。
- **R 候选（不阻断）**：P009-R1 通知中心面板开合深交互未注入（面存在性+
  托盘在案，479 实测口径留后续）；P009-R2 calculator/charts master 预存红
  （T3 发现，与 590-R3 同族另案）；P009-R3 vue 桌面宿主 v1 front-only 注册
  限制跳过 sys-monitor/kanban（框架既有 Plan 465 登记限制，非迁移缺陷）；
  P009-R4 vue 轨 EXTRA 全替换语义不并 manifest/kanban（缺省臂可并；包装
  场景受限注记，Stage C `auto desktop` 子命令候选动机 +1）。
- **Workaround 均在案非隐藏**：rust workspace 陈旧 member 顺修（590）；
  AUTO_BIN env 迁址必设（T3 注记）；pnpm 镜象 404 →官方源（T4 注记）。

### 结论

六项验收全过（含一处复审打回当批修复复证）；R1-R4 显式在案。
**路由：`reviewed`**，就绪 `/auto-plan:merge`（auto-os 主仓 fold + lang 侧
`os-009-dev` 两提交随批合入）。
