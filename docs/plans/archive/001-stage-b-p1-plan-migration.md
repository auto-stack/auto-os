---
plan_id: PLAN-001
status: archived                # drafting → executing → execution_done → reviewed → archived
feature_name: Stage B P-1 计划随迁批（七项桌面域 drafting 计划自 auto-lang 迁入）
author: [zhaopuming, ZCode]
created_at: 2026-09-07
updated_at: 2026-09-07

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: [reports/P001-1, reviews/P001-R1]
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: []                   # 纯文件迁移，零代码/规格面
current_step: 1
total_steps: 6
---

# [PLAN-001] Stage B P-1 计划随迁批——七项桌面域 drafting 计划自 auto-lang 迁入

## 变更摘要

[Design 01](../design/01-stage-b-desktop-migration.md)（Plan 584 定案）§7 实施
拆解的第一批：§1-A2 七项桌面域在途计划（535/554/556/557/558/577/578，均
drafting、无 worktree）整体随迁本仓。纯文件、零冲突面（§5 推论），无需等
541/582 收口窗口（两者已于 2026-09-07 先后合并，窗口已净）。

编号映射按 §5 迁移机制：本仓 `scripts/new-plan.sh` 中央取号重编 os-NNN，
frontmatter 增 `origin: PLAN-5xx` 溯源注记；auto-lang 侧 `docs/plans/INDEX.md`
（本批新建）留去向指针行；两仓正文互链（随迁横幅 ↔ INDEX）。随迁计划以
drafting 原状迁移，状态机续用不重置；正文 auto-lang 相对路径**不回改**，
推迟到 P-6 开工时按本仓 AGENTS §2 解析序换算。

**门禁分类：Category A**（纯计划跟踪/文件迁移）——不触 auto-lang `crates/`，
严禁 `cargo t` 与 `docs_gen`（auto-lang AGENTS 分级门禁）。**worktree 裁定：
免**——沿 Plan 584 Category A 先例（纯 docs 零冲突，主检出直做，
b173c9b51「无 worktree/分支可清(Category A 裁定)」）。

## 目标

1. 七项计划在本仓就位：`docs/plans/002..008-*.md`，frontmatter 带 origin
   溯源 + 随迁横幅（互链 Design 01 §5 与 auto-lang INDEX），正文与源逐字
   保真（frontmatter 重编维度与 H1 编号除外）。
2. auto-lang 侧收口：七文件 `git rm`（git 历史留档——577 即为补账提交
   42219a602 的存在理由），`docs/plans/INDEX.md` 新建承载去向指针行
   （含 577 编号消歧与 545/570 留守注记）。
3. Design 01 §5 处置表回填去向（os-NNN + 执行日期，§5 迁移机制明文要求）。
4. V7「搬迁零残留」P-1 部分：auto-lang 活动区 grep 七计划名引用清零
   （INDEX 指针行除外；archive/ 历史与 git 历史不动——归档不回改原则）。

## 架构方案

编号映射（源 → 目标，均 drafting 原状）：

| 源（auto-lang docs/plans/） | 目标（本仓 docs/plans/） | origin |
|---|---|---|
| 535-desktop-ux-followups.md | 002-desktop-ux-followups.md | PLAN-535 |
| 554-clock-app.md | 003-clock-app.md | PLAN-554 |
| 556-games-wave1.md | 004-games-wave1.md | PLAN-556 |
| 557-tetris.md | 005-tetris.md | PLAN-557 |
| 558-klondike.md | 006-klondike.md | PLAN-558 |
| 577-p534-debt-batch-1.md | 007-p534-debt-batch-1.md | PLAN-577（p534 债批一） |
| 578-desktop-gallery-apps.md | 008-desktop-gallery-apps.md | PLAN-578 |

三项机制要点（Design 01 §5 迁移机制原文执行）：

1. **重编**：new-plan.sh 中央取号（001 本批次计划 + 002..008 七随迁），
   `.next-id` 推进至 009。
2. **指针**：auto-lang INDEX.md 去向行 + Design 01 §5 处置表回填；
   两仓正文互链（随迁横幅反向指 auto-lang 源路径与 git 历史）。
3. **状态保持**：drafting 原状迁移；相对路径换算推迟到开工（P-6）。

## 需求分析与背景调查

- **用户裁定**（Design 01 §5 原文）：已动工的 541/582 本仓收口；未开工的
  桌面域计划整体随迁 auto-os 后再执行；545/570（语言域）留守。
- **编号冲突消歧**：auto-lang archive 既有 `577-emitter-gaps-batch.md`
  （trans 域，已归档合并），与随迁对象 `577-p534-debt-batch-1.md`（桌面
  债批，active drafting）同名异计划——active 577 系 p534 债批一漏提交后
  补账入库（auto-lang 42219a602）。重编 os-007 后 auto-lang 活动区编号
  冲突随 git rm 自然消解；INDEX 指针行与随迁横幅均带消歧注记。
- **取号基建修正**：本仓 new-plan.sh `.next-id` 算术自增丢 3 位补零
  （种子 001 → 自增产出 2），本批顺带修正（`10#$ID` 防八进制 +
  `printf %03d`；隔离副本冒烟 009→010 通过）。
- **零冲突面实证**：grep auto-lang 活动区（docs/ 非 archive），七计划名
  引用仅存在于七个计划文件自身；台账 autos-desktop-program.md 不追踪
  这批 drafting 计划（其计划一览止于 509/386 世代）。

## 详细设计

转换规则（python 一次成型，逐文件断言防呆）：

- frontmatter：`plan_id: PLAN-5xx` → `PLAN-00N`，紧随插 `origin: PLAN-5xx`；
  `updated_at` → 迁移日；author/created_at/affects/current_step/
  total_steps 原样保留。
- 正文：H1 `# [PLAN-5xx]` → `# [PLAN-00N]`（仅首处）；frontmatter 后插
  随迁横幅（含 origin/源路径/Design 01 §5 链接/INDEX 反链/解析序换算
  推迟声明；007 加编号消歧段）。正文其余逐字保真。
- auto-lang INDEX.md：七行去向指针 + 577 消歧 + 留守注记 + P-5 台账
  指针预留说明。

## 测试设计

Category A：无 cargo/docs_gen。验证三件：

1. **正文保真**：反向变换（dst 剥横幅/H1 还原编号）与源 diff 为空。
2. **V7 grep**：auto-lang 活动区七计划名引用清零（INDEX 指针行除外；
   archive/git 历史不动）。
3. **frontmatter 结构**：七文件 plan_id/origin/status: drafting 齐整。

## 验收标准

- [x] os-002..008 七文件就位，origin 注记齐，status 均 drafting 原状。
- [x] 正文保真：反向变换 diff 为空（仅 frontmatter 重编维度 + H1 编号 +
      随迁横幅差异）。
- [x] auto-lang 七文件 git rm；INDEX.md 指针行七行在案（含 577 消歧、
      545/570 留守注记）。
- [x] Design 01 §5 处置表七行回填去向 + 迁移机制节补执行注记。
- [x] V7 grep：auto-lang 活动区零残留（INDEX 指针行除外）。
- [x] 两仓提交在案；本计划复审通过后 specs.json 沉淀 + 归档终态。

## 执行步骤

（原子任务；每步完成后追加 [✅ 已完成] 一行证据。）

1. [✅ 已完成] **取号**：new-plan.sh ×8（001 批次 + 002..008 七随迁占位）
   → `.next-id` 009；顺带修正脚本补零缺陷（10# + printf %03d，隔离副本
   冒烟 009→010 过）；即交提交 `0c775a5`（即交纪律，577 漏提交教训）。
2. [✅ 已完成] **迁移转换**：python 转换七文件（frontmatter 重编 + origin
   + 横幅；007 编号消歧段；断言防呆 7/7 过）。
3. [✅ 已完成] **auto-lang 收口**：`docs/plans/INDEX.md` 新建（七行去向指针 +
   577 消歧/留守注记/P-5 台账指针预留）；七文件 `git rm`（staged）。
4. [✅ 已完成] **Design 01 §5 回填**：处置表七行补 os-002..008 去向 +
   迁移机制节补「P-1 执行注记」。
5. [✅ 已完成] **验证**：反向变换保真 diff 7/7 空（git show HEAD 源对照，
   仅横幅块+其分隔空行差异）；V7 grep 活动面命中=INDEX 指针行（允许）+
   archive 历史三件（575/583/584，归档不回改）；frontmatter plan_id/origin/
   status/updated_at 7/7 齐。
6. [✅ 已完成] **复审 + merge**：复审记录填写；specs.json 沉淀（reports/reviews）；
   `git mv archive/` + `status: archived` 终态。
   [✅ 已完成] 复审 C1–C6 全过（见复审记录）；specs.json 沉淀 P001-1/P001-R1；
   git mv archive/ + status: archived（本 merge 提交）。

## 复审记录

（2026-09-07 复审，verify-don't-trust 口径。）

**验收对账（C1–C6）**：

| # | 验收项 | 结果 | 证据 |
|---|---|---|---|
| C1 | 七文件就位 + origin + drafting | PASS | frontmatter 结构核对 7/7（plan_id/origin/status/updated_at） |
| C2 | 正文保真 | PASS | 反向变换 diff 7/7 空（git show HEAD 源对照；差异仅横幅块+分隔空行） |
| C3 | auto-lang 收口 | PASS | `8a57f3f3b`：INDEX.md 七行指针 + 七文件 git rm |
| C4 | Design 01 §5 回填 | PASS | `e0ae86d`：处置表七行去向 + 迁移机制执行注记 |
| C5 | V7 搬迁零残留 | PASS | 活动面命中 = INDEX 指针行（允许项）+ archive 历史三件（575/583/584，归档不回改原则内） |
| C6 | 两仓提交 + 归档 | PASS | 本复审后随 merge 提交闭合 |

**遗漏/延后/workaround 扫描**：

- V8（随迁计划可执行性冒烟）Design 01 §6 标注适用批次「P-1 后」——属
  P-6 开工批职责，非本批遗漏。
- 迁移机制 4「台账接棒」前置条件 = A1 台账迁移（P-5 本体批），本批
  INDEX 预留注记已埋。
- new-plan.sh 补零修正是顺手清偿的取号基建缺陷（`0c775a5`，隔离副本
  冒烟 009→010），非绕行 workaround；无其他未批准延后项。

**健康检查**：无代码面（Category A）；校验脚本为一次性 heredoc 未落盘，
无 debug 残留；两仓 `git status` docs/ 面干净。

**裁定合规**：Category A 免 worktree 沿 Plan 584 先例（b173c9b51）；
编号映射沿 Design 01 §5 待澄清 #1 默认裁定，无新决策点。

**spec-impact**：`new_spec_components: [reports/P001-1, reviews/P001-R1]`
（本仓 specs.json 首两笔入账）；`supersedes/touched_goals` 空（本仓尚无
goals.md 与被替代组件）。

## 待澄清事项

（无——编号映射沿用 Design 01 §5 待澄清 #1 默认裁定，无新决策点。）
