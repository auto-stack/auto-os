# auto-os Agent Guidelines

本仓是 **AutoOS 产品根 / 伞形组织根**。语言/框架在 [auto-lang](../auto-lang)
（两轴分离，Plan 579 Stage A 裁定）。AI 助手在本仓及其登记的 app 仓工作
时遵守以下规约。

## 1. 计划范式（auto-plan 四技能）

沿用 auto-lang 的 auto-plan 范式：`/auto-plan:new` → `/auto-plan:work` →
`/auto-plan:review` → `/auto-plan:merge`（范式设计见 auto-lang
`docs/design/autoplan-spec-ledger.md`）。

- 本仓计划：`docs/plans/<NNN>-<slug>.md`，取号用 `scripts/new-plan.sh <slug>`
  （在 main 主检出运行；`.next-id` 自 001 起）。
- Plan 状态机：`drafting → executing → execution_done → reviewed → archived`
  （终态；归档目录 `docs/plans/archive/`）。
- spec ledger：`.autoos/specs.json`（六节：reports/goals/architecture/
  designs/tests/reviews；结构对齐 auto-lang 同名文件）。
- app 仓内的工作（新功能/修复）也走 plan 流程：跨仓计划的 plan 文件放
  主导仓（通常是本仓或对应 app 仓），并在两仓 README/计划中互链。

## 2. 跨仓解析序（红线：永不建 junction/symlink）

任何跨仓引用（框架根路径、数据源等）按以下顺序解析，**禁止以任何形式的
文件系统链接（junction/symlink）代替**：

```
1. 环境变量覆盖：$AUTO_LANG_ROOT（以及各 app 自定义的 root env，如
   auto-kanban 各看板的 root_env）
2. 兄弟检出：../auto-lang（相对运行目录）
3. 主检出：D:/autostack/auto-lang
```

**Worktree 红线（沿 2026-09-03 三仓 .git 删除事故教训）**：worktree 内
禁止创建任何 junction/symlink——`git worktree remove` 的递归删除会穿透
链接删除目标仓内容。移除任何 worktree 前必须先跑
`bash D:/autostack/wt-guard.sh <worktree 路径>`（reparse point 扫描，
非空即拒）。本仓 worktree 组目录布局沿 Plan 529：`.wt/os-<NNN>/auto-os`。

## 3. app 仓结构约定

登记入 `apps.manifest` 的真实 app 仓（examples/ui 归 demo，真实 app
独立仓——沿 [auto-os-config](../auto-os-config) 先例）至少包含：

```
<app-repo>/
├── pac.at            # app 清单（name/scene/ports；仓根 examples 形态，
│                     #   或 auto/ 子目录形态——auto-os-config 先例，二者其一）
├── src/front/        # 前端（app.at + store + pages/）
├── src/back/         # 后端（api.at + #[api] 端点；纯 .at，无手写 Rust）
├── tests/            # playwright 套件（+ testdata/ 合成语料）
└── README.md         # 定位/运行/测试方式/伞形归属反链
```

- **端口带**：真实 app 统一 17xxx（front 17N00 / back 17N01，与 examples
  的 30NN/80NN 带区分；auto-os-config=17700/17701，auto-kanban=17100/17101）。
- **双端纪律**：Vue 轨（`auto run`）与 VM 轨（`auto run -r vm`）双端一致
  （验证走 auto-lang 的 autoui-verifier 技能）。
- **验证门档**：不改 auto-lang `crates/` 的工作，严禁在 auto-lang 跑
  `cargo t`/`docs_gen`（auto-lang AGENTS.md Category A）。

## 4. 伞形清单（apps.manifest）

`apps.manifest` 是伞形的唯一事实源（虚拟伞形，无 submodule——Stage C
出现"CI 钉树构建 OS 镜像"类真实需求再评估机制升级）。每 app 一行：

```json
{ "id": "...", "name": "...", "repo": "../<repo>", "kind": "repo",
  "ports": [17100, 17101], "status": "active", "added": "YYYY-MM-DD" }
```

`kind` 字段为 Stage C 预留（`repo` | `subtree` | `submodule`）。增删 app
时同步本仓 README 的 Apps 表。

## 5. 桌面（Stage B 后）

桌面 shell 将自 auto-lang 搬迁入本仓（L2：先设计文档后拆 plan；架构依据
auto-lang `docs/design/autoui/virtual-desktop.md` Design 23——WM-as-app，
桌面 shell 是特权 AutoUI app）。Stage B 启动前，桌面相关计划与台账仍在
auto-lang（`docs/plans/autos-desktop-program.md`）维护。
