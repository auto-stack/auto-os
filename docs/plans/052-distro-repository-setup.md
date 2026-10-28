---
plan_id: PLAN-052
status: execution_done
feature_name: auto-os-distro 建仓与 Autonix 路线图
author: [codex]
created_at: 2026-10-29
updated_at: 2026-10-29
plan_revision: 2
current_step: 5
total_steps: 7
supersedes_spec_components: []
new_spec_components: []
touched_goals: []
---

# PLAN-052：auto-os-distro 建仓与 Autonix 路线图

## 0. 变更摘要

将既有 E:/AutoOS/src/autoos-nixos Git 仓库改名为 auto-os-distro，保留全部提交、分支、未提交的 ISO 验证记录及既有 worktree，建立 auto-stack/auto-os-distro 私有 GitHub 远端。登记主分支、里程碑和标签治理；按用户 2026-10-29 确认正式记录 M1～M4 与暂定产品名 Autonix，列清 M1 剩余验证，不执行后续阶段或本轮 VM 安装。

## 1. 目标

- 仓库、工作树及 Git 管理指针改名后有效，M1 镜像及源提交 79e1435598b0367481919e1a7486d83e599ee358 不变。
- 远端保存既有 main、plan-001-dev，以及明确标为 preview 的 M1 基线标签。
- 正式记录已批准的主线、阶段标签、短期功能分支、四个里程碑与 Autonix 暂定名，提供 M1 验证操作和通过条件。
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
- 对照 docs/specs/apps/independent-repositories.md 与 shell/desktop-bootstrap.md：发行构建是独立基础设施，不是应用；现有 Specs 未定义发行仓治理。revision 1 建立行政归属和推荐方案；revision 2 用户正式批准路线及暂定名。产品运行契约未变。
- GitHub 当前登录 zhaopuming，现有产品组织 auto-stack。本次默认 private，正式公开发布另行决定。

## 5. 详细设计

1. 保存可恢复 Git bundle 与未提交进度副本，记录 refs 与进度文件哈希。
2. 先 git worktree move 改原实施树名称，再移动主仓目录，git worktree repair 修复反向指针；改名前后校对 refs、状态及进度文件哈希。
3. 将既有 Plan 验证记录单独提交为进度备份，不修改/合并 ISO 实现。
4. 新建本次说明文档工作树，README 标明实际主线/开发线、M1 preview 身份与验收缺口。路线文档记录用户已批准的治理和阶段；README 与新的 docs/verification/M1-CHECKLIST.md 保持验收状态一致。
5. GitHub 若不存在则创建私有仓；若已存在，先检查归属与状态，禁止覆盖远端历史。推送指定分支/标签，禁止 force/mirror/--all 意外扩散。

### 规范增量

| delta_id | add/modify/retire | target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| 无 | 无 | 无 | 不改变既有产品运行契约；已批准产品路线写入 ROADMAP，不冒充运行验收 | 改名、备份与路线记录 | AC-01～AC-04、AC-07 |

## 6. 测试设计

- 改名前后 ref SHA、工作树 branch/HEAD、未提交 Plan 哈希一致；git worktree list 与 git fsck --connectivity-only 通过。
- 不对代码执行重复构建；源码内容身份与既有 M1 ISO 校验值保持。
- 文档对照锁定实现的 target-flake、目标权限和原 M1 V-02～V-07，校验更新/回退命令、基线指纹、链接和未完成状态；核对四个 GitHub 里程碑 open，不创建未经验收的完成标签。
- git ls-remote 精确匹配推送分支与标签；GitHub 返回 private=true。
- git ls-tree 检查全部发布 refs，无 ISO/VMDK/凭据或非 Markdown 附件证据。
- 本次 work 自检不替代独立 review；未复审的 ISO 主线合并继续待办。

## 7. 验收标准

| ID | owner_stage | 可观察结果 |
|---|---|---|
| AC-01 | work | 新目录存在、旧路径已迁移、refs/HEAD/进度不丢，所有工作树可解析，无链接替代 |
| AC-02 | work | auto-stack/auto-os-distro 私有远端存在，指定分支与 preview 标签 SHA 一致 |
| AC-03 | work | 可审阅说明准确区分可启动预览和完整安装验收，已批准四阶段、代码所有权与 Autonix 暂定名明确 |
| AC-04 | work | 主仓其他 WIP 保留，大型产物及凭据未进入 Git，恢复 bundle 可验证 |
| AC-07 | work | 路线图/M1 检查清单有步骤、通过条件、证据与原用例映射；四个 GitHub milestone 开放，M1 剩余项保持待验证 |
| AC-05 | review | 独立核验本次改名、远端身份和说明文档，不复用实现者自检为最终审查 |
| AC-06 | merge | 审查通过后处理说明文档合入及本计划归档，不连带合并旧 M1 实现 |

## 8. 执行步骤

| 完成 | ID | owner_stage | 依赖 | 操作与验证 | AC |
|---|---|---|---|---|---|
| [x] | T-01 | work | 无 | 备份并改名主仓/原工作树，修复指针；比较 refs、HEAD、状态与进度哈希 | AC-01、AC-04 |
| [x] | T-02 | work | T-01 | 独立工作树编写仓库说明与阶段管理建议，提交文档，不合并 M1 实现 | AC-03 |
| [x] | T-03 | work | T-02 | 建私有 GitHub 远端，推送指定 refs；通过 API 与 ls-remote 核验 | AC-02、AC-04 |
| [x] | T-04 | work | T-03 | 更新本计划证据与恢复入口，核对其他 WIP 不变并交接 review | AC-01～AC-04 |
| [x] | T-07 | work | T-04 | 正式记录用户批准路线、暂定名与 M1 验证清单，建立四个 GitHub milestones；核对文档与实际基线 | AC-03、AC-07 |
| [ ] | T-05 | review | T-04、T-07 | 独立复核行政改名/发布边界与文档 | AC-05 |
| [ ] | T-06 | merge | T-05 pass | 合入本次说明文档并归档计划；旧 M1 安装计划继续独立跟踪 | AC-06 |

## 9. 复审记录

- stage: new；plan_revision: 1；outcome: pass；next: work。
- 改名与建远端范围已由用户授权；不为命名/备份重新索取同一授权。
- revision 1 时阶段治理为建议；revision 2 已按后续用户明确指令批准路线与 Autonix 暂定名。

## 10. 待澄清事项

- 用户已确认 Autonix 为暂定产品名；最终定名和正式发行版本号后续决定，不影响当前路线图。
- 四阶段与主线/里程碑/标签方式已获批准；Auto→Nix 可与 M2 并行。各阶段详细实现仍按后续 Plan 取号与验收。

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

### revision 2：用户确认路线与暂定产品名

- stage: new；outcome: pass；next: work/T-07。用户明确批准主分支加里程碑和标签、四阶段路线及 Autonix 暂定名，并要求列清 M1 安装/更新/回退验证。
- 已有工作 T-01～T-04 和历史证据保留；新增 T-07/AC-07，更新 AC-03 与 T-05 复核范围。review 尚未执行，不存在可沿用的最终 pass。
- 本轮为文档和 GitHub 里程碑整理，不实际安装 VM、不生成新的发行版完成标签。旧 M1 Plan 的 V-02～V-07 仍是执行验收来源。
- 本轮候选交付增加 docs/verification/M1-CHECKLIST.md；当前构建源码/ISO/lock 指纹不变。无产品运行规范改动，既有规范增量仍为空。

### revision 2 work 完成：正式路线与 M1 清单

- code_commit: 5100ce1ac323f51d40ea26819376f17a04c40f3a；branch/worktree 保持 plan-052-repo-setup / D:/autostack/.wt/os-052/auto-os-distro。git push 成功，ls-remote 与本地 HEAD 相同，工作树干净。
- ROADMAP.md 状态 accepted，记录 Autonix 暂定名、main/短期分支/里程碑/标签规则，四阶段交付与完成门槛，M2/M3 可并行边界。README 同步指向正式路线与验收入口。
- 新增 docs/verification/M1-CHECKLIST.md：固定 79e1435/source lock/ISO 指纹；12 个唯一待验收 ID（11 项剩余运行验证 + 1 项交付收口）；仅 BASE-01/BASE-02 有已通过证据，其余保持待验证。
- 检查清单明确安装、脱离 ISO 启动、目标配置/账户隔离、实际桌面功能、配置应用、运行时回退、启动菜单回退、配置源恢复、安装下载失败及旧产物保护；逐项写操作、通过条件与原 V/AC 映射。
- 文档验证返回 0：全部相对链接存在；12 个 ID 唯一、未执行 checkbox 无误标；源码/ISO/lock 与现有 manifest 一致；目标 flake autoos 与实际模板一致；原 V-02～V-07 均被覆盖；git diff --check 通过。
- GitHub 里程碑已建立并 GET 复核 4 项 state=open：/milestone/1（M1）、/2（M2）、/3（M3）、/4（M4）。不设置虚构截止日期，不关闭 M1，不创建尚未验收的完成标签。
- m1-nix-cosmic-preview peeled commit 仍为 79e1435598b0367481919e1a7486d83e599ee358；未改源码、输入锁、ISO 或 VM 状态。
- PR #1 标题/描述已重写为最终交付：Record approved Autonix roadmap and M1 verification checklist；head=5100ce1，仍 draft，未 merge。本次 GitHub 元数据修改不等于独立复审通过。
- T-01～T-04、T-07 work 已完成，current_step=5/7；T-05 独立复审与 T-06 合并归档保持待办。next: review。AutoOS main 其他研究/设计 WIP 保持原样。
