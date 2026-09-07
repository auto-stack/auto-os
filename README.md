# auto-os

**AutoOS 产品根 / 伞形组织根（umbrella）**——AutoOS 操作系统及其桌面、
设置中心、真实应用的组织与集成仓。

> **两轴分离（Plan 579 Stage A，2026-09-07 立项）**：
> - [`auto-lang`](../auto-lang) = **语言/框架根**——Auto 语言编译器、VM、
>   AutoUI 框架栈（iced/Vue 双端）、examples demo 集。
> - `auto-os`（本仓）= **产品根**——AutoOS 桌面 shell、真实 app 的伞形
>   组织、集成清单与产品级文档。
>
> 本仓**不包含** auto-lang 的框架代码；app 仓经 `auto` CLI（来自
> auto-lang）拉起，跨仓引用一律走解析序（见 AGENTS.md），不建任何
> junction/symlink。

## 阶段路线（Stage A / B / C）

| 阶段 | 范围 | 状态 |
|---|---|---|
| **Stage A** | 伞形仓骨架（本仓）+ 首个真实 app [auto-kanban](../auto-kanban)（v1 计划板，只读） | 🔄 Plan 579 执行中（2026-09-07） |
| **Stage B** | 桌面 shell 自 auto-lang 搬迁入本仓（L2：设计文档 + 拆 plan；前置 = auto-lang 在途计划落地） | 未启动 |
| **Stage C** | 伞形组合机制升级评估（manifest vs submodule；触发条件 = 出现"CI 钉树构建 OS 镜像"类真实需求） | 未启动 |

**submodule 裁定（Stage A）**：不使用 git submodule——现有扁平兄弟仓 +
跨仓 worktree 组 + 解析序约定已覆盖需求；submodule 的 detached-HEAD /
更新仪式 / 与 wt-guard 的交互风险在真实需求出现前不引入。`apps.manifest`
是伞形的唯一事实源（虚拟伞形）。

## 目录结构

```
auto-os/
├── README.md            # 本文件
├── AGENTS.md            # agent 工作规约（app 仓约定/解析序/wt-guard 纪律）
├── apps.manifest        # 伞形 app 清单（JSON，虚拟伞形唯一事实源）
├── docs/plans/          # auto-plan 范式计划目录（.next-id 自 001 起）
├── scripts/new-plan.sh  # 取号建骨架脚本（自 auto-lang 拷贝适配）
└── .autoos/specs.json   # spec ledger（六节，结构对齐 auto-lang 同名文件）
```

## Apps

伞形登记的真实 app（详情见 `apps.manifest`）：

| id | name | repo | kind | ports | status |
|---|---|---|---|---|---|
| kanban | 通用看板（v1 计划板） | [../auto-kanban](../auto-kanban) | repo | 17100 / 17101 | active (Plan 579) |

> 真实 app 独立仓存放（沿 [auto-os-config](../auto-os-config) 先例），
> examples/ui 归 demo。app 仓结构约定见 AGENTS.md。

## 关联

- 框架根：[../auto-lang](../auto-lang)（语言/编译器/VM/AutoUI/examples）
- 桌面架构：auto-lang `docs/design/autoui/virtual-desktop.md`（Design 23）
- 桌面程序台账：auto-lang `docs/plans/autos-desktop-program.md`（Stage B 随迁本仓）
