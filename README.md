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
| **Stage B** | 桌面域资产自 auto-lang 搬迁入本仓（[Design 01](docs/design/01-stage-b-desktop-migration.md)） | 🔄 P-1..P-4/P-7 ✅；**P-5 ✅ + P-6 承载批 ✅**（2026-09-08，PLAN-009：§3-a 包装脚本 `scripts/desktop.{ps1,sh}`+V1/V2/V3 实机验收+CI 围栏保活+画廊部署触发端）；随迁七计划本体执行在途（os-003 开工） |
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
├── apps.manifest        # 伞形 app 清单（JSON，唯一事实源；daemon 字段可选）
├── apps/                # in-repo 桌面 app（Stage B P-5 随迁；含 pac.at 的
│                        #   子目录 = local app root，P-3 容器探测注册）
│   ├── 025-sys-monitor/ #   系统监视器（541 终态；tests/desktop_mcp 随目录）
│   ├── 036-tetris/      #   俄罗斯方块（Plan 005；Vue/VM/Rust 双端）
│   ├── 028-launcher/    #   桌面启动器（464；注册表型特权 app）
│   ├── 038-minesweeper/ #   扫雷（games-wave1 基底）
│   ├── kanban/          #   gitlink → auto-kanban（PLAN-013 首例 submodule，
│   │                    #   容器臂/manifest 臂 id 去重，内容同源）
│   └── common/settings/ #   共享 SettingsPopover 组件（ui-gallery 消费）
├── ui-gallery/          # UI 示例画廊（顶层；收割 auto-lang examples/ui，
│                        #   解析序 AUTO_GALLERY_APPS → ../auto-lang）
├── widgets-gallery/     # 组件文档画廊（顶层；框架 docs/schema 管线语料，
│                        #   auto-lang 侧经 resolve_os_top_dir 解析序消费）
├── shell/               # shell 四件（P-7 权威真相源；hash-lock 同步契约）
├── docs/plans/          # auto-plan 范式计划目录（.next-id 自 001 起）
│   └── autos-desktop-program.md  # 桌面程序台账（P-5 接棒，单一事实源）
├── docs/design/         # 本仓设计文档（01 = Stage B 迁移定案）
├── scripts/             # new-plan.sh / shell-pack-sync.py
└── .autoos/specs.json   # spec ledger（六节，结构对齐 auto-lang 同名文件）
```

## Apps

伞形登记（详情见 `apps.manifest` + `apps/` 容器探测）：

| id | name | repo / 目录 | kind | ports | status |
|---|---|---|---|---|---|
| kanban | 通用看板（v1 计划板） | [../auto-kanban](../auto-kanban)（submodule `apps/kanban/`，PLAN-013 首例） | repo | 17100 / 17101 | active (Plan 579) |
| auto-musk | Auto Musk（Coding Agent） | [../auto-musk](../auto-musk) | repo | 17200 / 17201 | active (2026-09-11；daemon 链 PLAN-013) |
| jade-garden | Jade Garden（类 Obsidian 知识库） | [../auto-down](../auto-down)`/jade-garden/front/auto` | repo | 17300 / 17301 | active (2026-09-11；daemon 链 PLAN-013) |
| auto-term | AutoTerm（桌面终端） | [../auto-term](../auto-term)`/app` | repo | 17400 / 17401（端口占位：无 back，引擎进程内） | active (PLAN-013 T7) |
| 025-sys-monitor | 系统监视器 | `apps/025-sys-monitor/` | local | 4025 / 8025 | active (PLAN-590 随迁) |
| 028-launcher | 桌面启动器 | `apps/028-launcher/` | local | 4028 | active (PLAN-590 随迁) |
| 038-minesweeper | 扫雷 | `apps/038-minesweeper/` | local | 4038 | active (PLAN-590 随迁) |
| tetris | 俄罗斯方块 | `apps/036-tetris/` | local | 17500 / 17501 | active (Plan 005) |
| 037-klondike | 经典纸牌接龙 | `apps/037-klondike/` | local | 17600 / 17601 | active (PLAN-006) |

> 真实 app 独立仓存放（沿 [auto-os-config](../auto-os-config) 先例），
> examples/ui 归 demo。app 仓结构约定见 AGENTS.md（§3 含 daemon 键 schema；
> §4 混合形态/submodule 双写纪律）。in-repo `apps/` 为 Stage B 随迁的桌面
> 域 app（沿 examples 的 30NN/80NN 端口带；升格独立仓时改 17xxx 带）。
> demo 升格流水线：examples demo 独立为外部仓 → manifest repo 条目 →
> git submodule 收编 `apps/<id>/`（首个样板 kanban，PLAN-013）。
> AutoTerm 独立 app 已落地（PLAN-013 T7：`../auto-term/app`，引擎经
> auto.term.* catalog shim 进程内加载 autoterm_core.dll；部署件用
> auto-os-config `scripts/deploy-autoterm.sh`）；os-config 内嵌终端页
> 保留。

## 图标资产（PLAN-018）

桌面 app 图标的唯一设计源是 `assets/icons.png`（浅）/ `assets/icons_dark.png`
（深）两张精灵表（7×4=28，浅深同序不同位移）。运行时只吃切片：

```
assets/icons/{light,dark}/<stem>.png   # 切片产物（RGBA，≈152×148）
assets/icons/mapping.json              # registry id → stem（27 条；browser 预留位不入映射）
assets/icons/preview.png               # 蒙太奇预览（人眼复核用）
```

再生成与校验（改设计源后必跑）：

```
python scripts/slice_icons.py            # 切片 + 写 mapping + preview + 自校验
python scripts/slice_icons.py --verify   # 只校验（像素级往返 + mapping 恰等）
```

运行时链路：桌面 boot 读 mapping 把命中 id 的 `entry.icon` 改写为
`iconfile:<stem>`，渲染端（iced/vue）按当前主题选 `{light,dark}/<stem>.png`；
未映射 app 自动落回 lucide（回退链契约见 auto-lang `docs/specs/` icon
字符串协议族节）。

## 关联

- 框架根：[../auto-lang](../auto-lang)（语言/编译器/VM/AutoUI/examples）
- 桌面架构：auto-lang `docs/design/autoui/virtual-desktop.md`（Design 23）
- 桌面程序台账：[docs/plans/autos-desktop-program.md](docs/plans/autos-desktop-program.md)
  （Stage B P-5 随迁本仓，接棒单一事实源；auto-lang INDEX 留指针行）
