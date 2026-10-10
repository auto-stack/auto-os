---
plan_id: PLAN-048
status: archived
feature_name: AutoOS bilingual product README
author: [Codex]
created_at: 2026-10-10
updated_at: 2026-10-10
plan_revision: 1
current_step: 3
total_steps: 3
supersedes_spec_components: []
new_spec_components: []
touched_goals: []
---

# PLAN-048 — AutoOS 中英文产品介绍

## 0. 变更摘要

重写中文 README.md，新增 README.en.md，引用本仓持有的真实截图。

## 1. 目标

介绍架构、虚拟桌面、应用与下一阶段 Linux 发行版方向。两版同范围、互链。
不改运行代码、清单、规范或 auto-lang；不发布远端。

## 2. 架构方案

以现有规范和网站为依据，区分已实现能力与规划；保留开发入口及工程文档导航。
在 D:/autostack/.wt/os-048/auto-os 独立工作目录编辑，分支 codex/os-048-readme。

## 3. 技术栈

Markdown、Mermaid、原始 PNG 截图；无需运行应用或语言测试。

## 4. 需求分析与背景调查

用户已授权更新中英文 README，重点为架构、虚拟桌面、apps、下一版 Linux
发行版计划，并允许参考 auto-lang 网站内容与截图。授权范围为本地文档更新与
完成必要的仓库流程；无部署或远端发布要求。

依据：docs/design/00-intro.md、docs/design/01-stage-b-desktop-migration.md、
docs/specs/shell/desktop-app-launch.md（当前 manifest 10 项全为 VM）、
docs/specs/shell/dashboard.md、apps.manifest、scripts/desktop.{ps1,sh}。
本仓无 specs 总览；以相关模块规范为准。
auto-lang 来源基线 9dc21153d70c23e1d8a4ae90a987ddb2d8a435f7：
website/{zh/,}autoos/index.md、website/zh/os.md、
website/zh/articles/autoos-history.md、website/docs/releases/v0.5.md、
docs/design/autoui/virtual-desktop.md、website/public/desktop-showcase/。
网站真实桌面截图时间 2026-10-01。
主检出 ui-gallery 有他人 WIP；已向用户说明，隔离实施，不纳入提交。

## 5. 详细设计

中文默认 README.md 与英文 README.en.md 共用三个截图和相同章节顺序：
产品定位、桌面、架构、Apps、Linux 路线、启动、仓库与文档导航。
应用表按 manifest 列出全部 10 项，补充容器应用与相关设置/画廊时注明来源。
将 Linux 工作描述为下一阶段方向，不承诺底层发行版、发布日期或现成镜像。
本仓 docs/images/readme/ 保存网站原图，提供来源和日期说明。

### 规范增量

无规范影响：这是已有实现和既有路线的对外摘要，不改变任何行为、接口、
验收阈值或架构裁定；README 不作为新的 canonical Spec。无需 ledger 投影更新。

## 6. 测试设计

检查两版章节、全部清单 id、VM 当前形态与路线措辞；检查仓内链接及截图文件；
以 SHA256 核对复制截图，检查 git diff --check。外部 GitHub 导航按已知仓名
构造，不将在线服务可用性作为本地文档验收。不跑 cargo t/docs_gen。

## 7. 验收标准

- AC-01：中英文涵盖四个重点，结构与事实一致，语言互链可用；逐节复核。
- AC-02：现状与路线清楚分开，10 个 manifest id 全覆盖；对照规范/清单。
- AC-03：三个真实截图在本仓持有、两版引用可用、来源日期可追溯；链接与哈希检查。
- AC-04：启动命令和工程导航符合现有路径，无额外代码变更；路径检查及 diff 检查。

## 8. 执行步骤

- [x] T-01：复制三个网站截图到 docs/images/readme/，补来源说明；对应 AC-03。
- [x] T-02：重写 README.md 与新增 README.en.md；对应 AC-01/02/04。
- [x] T-03：执行内容/链接/哈希/格式检查并提交文档；对应全部 AC。

## 9. 复审记录

- stage: new | PLAN-048:r1 | outcome: pass | next: work | tasks: T-01..03 | acceptance: AC-01..04

- stage: work | PLAN-048:r1 | outcome: pass | code_commit: f58f70115dfe402ad839f6c9ec1110cb723cbe4e | base_commit: d304a34 | worktree: D:/autostack/.wt/os-048/auto-os | next: review
  - T-01：三个 PNG 原图复制，SHA256 与网站文件完全一致；来源文档列出固定版本。
  - T-02：两版六个主题章节、10 个 manifest id、三个相同截图，内容逐节对应。
  - T-03：本地 Markdown 链接全通过，git diff --cached --check 通过；PowerShell Vue/iced DryRun 均成功。检查脚本初始误设为七节，核对正文后修正为六节并重跑通过；未修改验收要求。
- stage: review | PLAN-048:r1 | outcome: pass | reviewed_commit: f58f70115dfe402ad839f6c9ec1110cb723cbe4e | base_commit: d304a34 | dependency: auto-lang 9dc21153d70c23e1d8a4ae90a987ddb2d8a435f7 | next: merge
  - 同会话复核，非独立代理；直接依据已提交文档、清单、规范与来源重建结论。
  - AC-01 pass：六章节逐节对照，产品/桌面/架构/应用/Linux/开发与导航事实相同、语言互链有效。
  - AC-02 pass：清单十项均覆盖、全为 VM；Linux Stage 1 与未来会话/镜像分开；不虚构底层发行版或发布日期。
  - AC-03 pass：三图肉眼检查、本地引用检查与 SHA256 一致；来源固定版本与时间齐备。
  - AC-04 pass：两版仓内链接及 git diff HEAD~1 HEAD --check 通过；PowerShell/Bash 的 Vue/iced DryRun 均通过。变更仅两版 README、三图、来源说明。
  - 规范依据 SHA256：desktop-app-launch.md E40D283F6F5C1DE1E6A78AF73D3613416F70D159F67214FE6531B190CDC9E54C；dashboard.md D805A09A8A315A66397491835D669B98A85B501685E9AFA92869F344E66B7416；apps.manifest 41C7ED73B53F27E9B179D3796BECCD6A605608909C3506F323B6125F555295A4。
  - frozen spec delta: 无（no Spec impact）；未将 README 建为规范，ledger 无受影响投影，复核确认无需写入。无未解决发现。
- stage: merge | PLAN-048:r1 | prepared: reviewed_commit = delivery_commit f58f70115dfe402ad839f6c9ec1110cb723cbe4e | canonical_delta: none | ledger: no affected projections
- stage: merge | PLAN-048:r1 | landed: main tip f58f70115dfe402ad839f6c9ec1110cb723cbe4e equals delivery_commit after ff-only merge
  - ledger_refreshed: no-op，经复核无规范增量或受影响投影；未改 ledger。
  - archived: docs/plans/archive/048-readme-product-introduction.md | completion_kind: delivered
  - artifact check: 文档与图片由 Git 直接消费，无需重建或重启生产进程；不适用 auto-lang 代码回归门。
  - cleanup: pending guarded removal of D:/autostack/.wt/os-048/auto-os and codex/os-048-readme.
## 10. 待澄清事项

无。采用中文默认入口 + README.en.md，保留已有中文入口兼容。
