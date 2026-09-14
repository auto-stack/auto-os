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

**Submodule 纪律（PLAN-013 起，混合形态）**：本仓 submodule（现
apps/kanban）在 worktree 中默认不检出（gitlink 空目录）——桌面注册表
apps/ 容器臂按 pac.at 门静默跳过，不炸启动。组内开发需 kanban 时二选一：
组内补依赖 worktree（`.wt/os-<NNN>/auto-kanban`，manifest 臂兄弟解析命中）
或 `git submodule update --init`（容器臂命中）。标准桌面入口
`scripts/desktop.{sh,ps1}` 注入 `AUTO_OS_ROOT`=本仓根（主检出），注册表
与 submodule 检出状态无关恒完整。`git worktree remove` 会删 worktree 内
submodule 检出内容（git 数据在 `.git/modules/` 保留，可恢复）——wt-guard
扫 reparse point 不拦 gitlink，移除组前 `git submodule deinit` 更干净。

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
  的 30NN/80NN 带区分；auto-os-config=17700/17701，auto-kanban=17100/17101，
  auto-musk=17200/17201，jade-garden=17300/17301，auto-term=17400/17401）。
- **daemon 键（PLAN-013 定案）**：pac `daemon: <manifest-id>` 声明桌面
  launch 前置后端依赖——桌面 ensure 链按 apps.manifest 条目的 `daemon`
  对象（`port`/`bin`/`env_port`）ping :port（`GET /api/health` 契约固定）
  → 发现序 spawn → 注入 `<NAME大写蛇形>_DAEMON=<url>` env 到 App 会话。
  `daemon: autoos` 为历史特例（os-config 旧链原样，零回归门）；无独立
  后端的 app（如 auto-term，引擎进程内）不声明。
- **双端纪律**：Vue 轨（`auto run`）与 VM 轨（`auto run -r vm`）双端一致
  （验证走 auto-lang 的 autoui-verifier 技能）。
- **验证门档**：不改 auto-lang `crates/` 的工作，严禁在 auto-lang 跑
  `cargo t`/`docs_gen`（auto-lang AGENTS.md Category A）。

## 4. 伞形清单（apps.manifest）

`apps.manifest` 是伞形的唯一事实源。物理承载自 PLAN-013 起为**混合形态**：
repo 条目默认虚拟伞形（兄弟检出解析），产品 app 可叠加 git submodule
物理收编（首个样板 kanban = `apps/kanban`；examples demo 升格为独立仓后
沿此统一管理）——两臂按 id 去重，容器臂（apps/ 检出）先于 manifest 臂
（兄弟检出），同一 app 双形态并存时容器臂胜、内容同源零行为差。每 app 一行：

```json
{ "id": "...", "repo": "../<repo>", "kind": "repo",
  "ports": [17100, 17101], "status": "active", "added": "YYYY-MM-DD",
  "daemon": { "port": 17101, "bin": "<repo 相对二进制>",
              "env_port": "<DAEMON 端口覆盖 env 键>" } }
```

`kind` 字段为 Stage C 预留（`repo` | `subtree` | `submodule`——submodule
物理形态落 `apps/<id>/` 时 kind 仍记 `repo`，容器臂按目录名展开）。
`daemon` 对象可选（schema 见 §3 daemon 键；`bin` 缺席 = 只探不孵；健康
探针固定 `GET /api/health`）。**manifest 不承载展示名**（PLAN-015：`name`
键已退役）——App 展示名（英/中）以各 app pac.at 的 `title`/`title_zh` 为
唯一事实源。增删 app 时同步本仓 README 的 Apps 表；
submodule 收编/解除用 `git submodule add/deinit` 双写纪律（manifest 行 +
gitlink 同一提交）。

## 5. 桌面（Stage B 后）

桌面 shell 将自 auto-lang 搬迁入本仓（L2：先设计文档后拆 plan；架构依据
auto-lang `docs/design/autoui/virtual-desktop.md` Design 23——WM-as-app，
桌面 shell 是特权 AutoUI app）。Stage B 启动前，桌面相关计划与台账仍在
auto-lang（`docs/plans/autos-desktop-program.md`）维护。
