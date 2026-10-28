# App 本地开发 clone 清单

扫描与补建：2026-10-29。依据用户当日决定，在 `D:/autostack/` 使用独立 clone 进行 app 编辑、计划和开发，AutoOS 子模块用于固定组合版本；流程见[工作约定](app-submodule-workflow.md)。

## 扫描结果

`.gitmodules` 登记24个应用仓：原有5个独立clone，缺少19个，本次全部补建。新clone均来自对应产品远端，拥有独立 `.git/`，使用main并跟踪origin/main，包含当时AutoOS固定提交；没有创建目录链接或共享对象库。已有clone保留原HEAD、分支和WIP，不自动追新。

| 应用仓 | 开发路径 | 本次处理 |
|---|---|---|
| auto-kanban | `D:/autostack/auto-kanban` | 已有，保留原状 |
| auto-calc | `D:/autostack/auto-calc` | 新建独立clone，main → origin/main |
| auto-clock | `D:/autostack/auto-clock` | 新建独立clone，main → origin/main |
| auto-todo | `D:/autostack/auto-todo` | 新建独立clone，main → origin/main |
| auto-weather | `D:/autostack/auto-weather` | 新建独立clone，main → origin/main |
| auto-notes | `D:/autostack/auto-notes` | 已有，保留原状 |
| auto-calendar | `D:/autostack/auto-calendar` | 新建独立clone，main → origin/main |
| auto-chat | `D:/autostack/auto-chat` | 新建独立clone，main → origin/main |
| auto-reader | `D:/autostack/auto-reader` | 已有，保留原状 |
| auto-blog | `D:/autostack/auto-blog` | 新建独立clone，main → origin/main |
| auto-video | `D:/autostack/auto-video` | 新建独立clone，main → origin/main |
| auto-music | `D:/autostack/auto-music` | 新建独立clone，main → origin/main |
| auto-photos | `D:/autostack/auto-photos` | 新建独立clone，main → origin/main |
| auto-paint | `D:/autostack/auto-paint` | 新建独立clone，main → origin/main |
| auto-monitor | `D:/autostack/auto-monitor` | 新建独立clone，main → origin/main |
| auto-database | `D:/autostack/auto-database` | 新建独立clone，main → origin/main |
| auto-explorer | `D:/autostack/auto-explorer` | 新建独立clone，main → origin/main |
| auto-launcher | `D:/autostack/auto-launcher` | 新建独立clone，main → origin/main |
| auto-tetris | `D:/autostack/auto-tetris` | 新建独立clone，main → origin/main |
| auto-solitaire | `D:/autostack/auto-solitaire` | 新建独立clone，main → origin/main |
| auto-minesweeper | `D:/autostack/auto-minesweeper` | 新建独立clone，main → origin/main |
| auto-charts | `D:/autostack/auto-charts` | 新建独立clone，main → origin/main |
| auto-term | `D:/autostack/auto-term` | 已有，保留原状 |
| auto-os-config | `D:/autostack/auto-os-config` | 已有，保留原状 |

## 其他应用与例外

| 应用 | 已有开发位置 | 处理 |
|---|---|---|
| AutoMusk | `D:/autostack/auto-musk` | 已有独立仓，保留 |
| AutoJade | `D:/autostack/jade-edit` | 已有独立仓，沿现有仓名保留；Jade Garden是旧称，不重复建应用仓 |
| Jade Garden历史源码 | `D:/autostack/auto-down/jade-garden`，属于 `D:/autostack/auto-down` 仓 | 旧manifest接入留存，不改历史源码或另建重复应用仓 |
| AutoShell | `D:/autostack/auto-shell` | 生态补表，已有独立仓，保留 |
| AutoEdit | `D:/autostack/auto-edit` | 生态补表，已有独立仓，保留 |
| AutoSyslog | `D:/autostack/auto-os/apps/auto-syslog` | 按用户原决定保留内部组件，不拆仓或克隆伪仓 |
| common / autostack | AutoOS内部辅助目录 | 不是产品app，无独立clone要求 |

## 验证与衔接

- 24个子模块仓均能找到直接位于 `D:/autostack/` 的对应独立clone，origin身份与产品远端匹配；原有5个不修改、不强制同步。
- 新建19个clone拥有独立Git数据并跟踪main；其中六个应用随后仅修订既有基础计划的开发位置/跨仓链接及交付收尾描述，未修改业务源码、未执行基础能力开发。
- Todo/Calendar/Paint/Database/Charts计划更新至r2，Chat至r3；功能目标、work验收、任务进度及Spec增量保持，旧版本在Git中保留。计划开发位置为独立clone，组合子模块取其已推送的文档提交。
- Notes、Reader独立clone有在途内容，保持原状；其计划仍沿用已有合同，owner继续维护时适用新的开发位置约定。Blog本次补建clone，但不改已有001计划或源码。
- `.gitmodules` 与 `apps.manifest` 未更改；仅clone补建不升级任何pin。本轮六个gitlink变化只纳入上述计划路径修订，其他子模块的提交、分支和WIP保持扫描时状态。

具体远端、提交与验证记录见[机器可读回执](../../plans/attachments/b-apps/development-clones-20261029.json)。本文件是本机2026-10-29快照，不是自动同步承诺；后续使用前应核对仓库实际状态。
