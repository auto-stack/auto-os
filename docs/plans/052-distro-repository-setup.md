---
plan_id: PLAN-052
status: execution_done
feature_name: auto-os-distro 仓库改名与远端备份
author: [codex]
created_at: 2026-10-29
updated_at: 2026-10-29
plan_revision: 1
current_step: 4
total_steps: 6
supersedes_spec_components: []
new_spec_components: []
touched_goals: []
---

# PLAN-052：auto-os-distro 仓库改名与远端备份

## 0. 变更摘要

将既有 E:/AutoOS/src/autoos-nixos Git 仓库改名为 auto-os-distro，保留全部提交、分支、未提交的 ISO 验证记录及既有 worktree，建立 auto-stack/auto-os-distro 私有 GitHub 远端。登记推荐的阶段管理方式，不开始下一阶段应用或配置生成器实现。

## 1. 目标

- 仓库、工作树及 Git 管理指针改名后有效，M1 镜像及源提交 79e1435598b0367481919e1a7486d83e599ee358 不变。
- 远端保存既有 main、plan-001-dev，以及明确标为 preview 的 M1 基线标签。
- 以可审阅文档说明主线、阶段标签、短期功能分支及四个里程碑建议。
- 非目标：合入未经复审的 ISO 实现、宣称完整安装验收完成、构建新 ISO、创建四条阶段长分支、确定最终品牌、公开发布产品。

## 2. 架构方案

- 产品根：D:/autostack/auto-os，计划登记于本仓。
- 发行版实现主检出：E:/AutoOS/src/auto-os-distro。
- 原 M1 实施树：E:/AutoOS/worktrees/plan-001/auto-os-distro，继续 plan-001-dev。
- 本次说明文档实施树（新增）：D:/autostack/.wt/os-052/auto-os-distro，branch plan-052-repo-setup。
- E:/AutoOS/artifacts、logs、vms、WSL 路径及现有 ISO manifest 的历史源身份不改。
- 不建文件系统链接，不将发行基础设施登记为 apps.manifest 应用。

## 3. 技术栈

Git / Git worktree / GitHub REST API / 本机 Git Credential Manager；凭据仅在进程内使用，不输出或写入文件。

## 4. 需求分析与背景调查

- 用户明确授权按 auto-os-distro 改名，并在 GitHub 建远端；询问四阶段长期分支是否合适。
- 既有 main=5356aae，plan-001-dev=79e1435；无 remote；main 只有 docs/plans/001-nixos-cosmic-iso.md 未提交验证进度。
- main 尚未包含全部构建实现；旧 M1 计划仍 executing，硬盘安装/更新/回退未记录通过。备份不等同于合并验收。
- AutoOS main 有其他计划的文档 WIP，保留原样；本次仅在其 docs/plans/ 分配 052 并记进度。
- 对照 docs/specs/apps/independent-repositories.md 与 shell/desktop-bootstrap.md：发行构建是独立基础设施，不是应用；现有 Specs 未定义发行仓治理。本次仅建立行政归属和推荐方案，无已生效的产品运行契约变更。
- GitHub 当前登录 zhaopuming，现有产品组织 auto-stack。本次默认 private，正式公开发布另行决定。

## 5. 详细设计

1. 保存可恢复 Git bundle 与未提交进度副本，记录 refs 与进度文件哈希。
2. 先 git worktree move 改原实施树名称，再移动主仓目录，git worktree repair 修复反向指针；改名前后校对 refs、状态及进度文件哈希。
3. 将既有 Plan 验证记录单独提交为进度备份，不修改/合并 ISO 实现。
4. 新建本次说明文档工作树，README 标明实际主线/开发线、M1 preview 身份与验收缺口。路线文档标为建议，等待产品路线确认。
5. GitHub 若不存在则创建私有仓；若已存在，先检查归属与状态，禁止覆盖远端历史。推送指定分支/标签，禁止 force/mirror/--all 意外扩散。

### 规范增量

| delta_id | add/modify/retire | target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| 无 | 无 | 无 | 不改变既有产品运行契约；推荐路线不冒充已批准规范 | 仅改名与备份 | AC-01～AC-04 |

## 6. 测试设计

- 改名前后 ref SHA、工作树 branch/HEAD、未提交 Plan 哈希一致；git worktree list 与 git fsck --connectivity-only 通过。
- 不对代码执行重复构建；源码内容身份与既有 M1 ISO 校验值保持。
- git ls-remote 精确匹配推送分支与标签；GitHub 返回 private=true。
- git ls-tree 检查全部发布 refs，无 ISO/VMDK/凭据或非 Markdown 附件证据。
- 本次 work 自检不替代独立 review；未复审的 ISO 主线合并继续待办。

## 7. 验收标准

| ID | owner_stage | 可观察结果 |
|---|---|---|
| AC-01 | work | 新目录存在、旧路径已迁移、refs/HEAD/进度不丢，所有工作树可解析，无链接替代 |
| AC-02 | work | auto-stack/auto-os-distro 私有远端存在，指定分支与 preview 标签 SHA 一致 |
| AC-03 | work | 可审阅说明准确区分可启动预览和完整安装验收，四阶段建议与代码所有权明确 |
| AC-04 | work | 主仓其他 WIP 保留，大型产物及凭据未进入 Git，恢复 bundle 可验证 |
| AC-05 | review | 独立核验本次改名、远端身份和说明文档，不复用实现者自检为最终审查 |
| AC-06 | merge | 审查通过后处理说明文档合入及本计划归档，不连带合并旧 M1 实现 |

## 8. 执行步骤

| 完成 | ID | owner_stage | 依赖 | 操作与验证 | AC |
|---|---|---|---|---|---|
| [x] | T-01 | work | 无 | 备份并改名主仓/原工作树，修复指针；比较 refs、HEAD、状态与进度哈希 | AC-01、AC-04 |
| [x] | T-02 | work | T-01 | 独立工作树编写仓库说明与阶段管理建议，提交文档，不合并 M1 实现 | AC-03 |
| [x] | T-03 | work | T-02 | 建私有 GitHub 远端，推送指定 refs；通过 API 与 ls-remote 核验 | AC-02、AC-04 |
| [x] | T-04 | work | T-03 | 更新本计划证据与恢复入口，核对其他 WIP 不变并交接 review | AC-01～AC-04 |
| [ ] | T-05 | review | T-04 | 独立复核行政改名/发布边界与文档 | AC-05 |
| [ ] | T-06 | merge | T-05 pass | 合入本次说明文档并归档计划；旧 M1 安装计划继续独立跟踪 | AC-06 |

## 9. 复审记录

- stage: new；plan_revision: 1；outcome: pass；next: work。
- 改名与建远端范围已由用户授权；不为命名/备份重新索取同一授权。
- 推荐阶段治理尚为建议，不将用户的架构提问视为已批准最终路线或品牌。

## 10. 待澄清事项

- 最终品牌 Aunix/Autonix 暂不裁定，不影响仓库名。
- 四阶段边界与 Auto 配置能力并行节奏待产品决策，不阻止本次改名和远端备份。

### work 完成记录

- 用户授权范围已执行：本地仓库与 M1 工作树改名为 auto-os-distro，远端 https://github.com/auto-stack/auto-os-distro 已创建（private=true）。
- 目录改名前后 main/plan-001-dev refs 与未提交 ISO 进度文件哈希一致。修复 .git 反向指针后，两个原工作树可解析；git fsck --connectivity-only 返回 0。
- 改名前恢复 bundle：E:/AutoOS/logs/REPO-SETUP-052/before-rename.bundle，verify 返回 0；SHA-256 BDBB05DA9CAC06A85D05405567232E774329BB2D00FA5C26A237746503AA5FE7。未提交进度另存同目录 001-progress-before-rename.md，SHA-256 801F841CA367C264FE2BBFD0D315E26D004E94586EFC2497AAB4F236B6DF667B。恢复/改名明细位于 rename-receipt.json（本地证据，不入 Git）。
- main 单独提交原 ISO Plan 的验证进度为 6e7d07516489f883c656fc328b8edb9c8f52220a；构建实现 plan-001-dev 保持 79e1435598b0367481919e1a7486d83e599ee358。
- 本次实施树：D:/autostack/.wt/os-052/auto-os-distro；branch plan-052-repo-setup；文档提交 ceb502213c1613445f4cc6af5db029f5c1cad5cb。README/ROADMAP 明确主线当前仍是旧基线，构建应检出 preview 标签，安装验收待完成；阶段治理与品牌均标 proposed。
- git push 显式推送上述 3 个分支与 m1-nix-cosmic-preview；ls-remote 返回相同 SHA，annotated tag 对象 b46f74e97da27c285bbf36b5cb947196f429dbe1，peeled commit=79e1435。
- 首次 SSH 推送被本机已有 ProxyCommand 的 shell 兼容问题阻止；改用本机已缓存 HTTPS 凭据成功推送，未改全局 SSH 设置、未输出或落盘凭据。
- 已核查发布 trees 无 ISO/VMDK/VMEM/log/raw attachment，常见私钥/GitHub token 模式扫描无命中；大型资产保持 E:/AutoOS/artifacts/logs/vms 原目录。
- 文档草稿 PR：https://github.com/auto-stack/auto-os-distro/pull/1（draft，3 files，89 additions，base 6e7d075，head ceb5022）。尚未独立 review/merge，本次备份不连带合并 M1 实现。
- AutoOS main 原有 3 个修改文档、未跟踪 049 计划/设计/附件保持原状，本次仅新增/更新 052 计划与 .next-id。
- next: review（T-05）；T-06 待独立复审通过后处理。不把仓库整理当作 M2/M3/M4 已实施或最终产品发布。
