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
| **Stage C** | v0.6 独立应用源码组合（manifest + 固定 submodule） | 21 个新产品仓 + 3 个已有应用接线；运行制品安装体系另行开发 |

**v0.6 应用组织（2026-10-03 用户确认）**：采用 `apps.manifest` + git submodule，
分别承担运行注册与源码版本固定。21 个新产品仓已从可见 v0.5 基线导入；
教学 Demo 保留在 auto-lang，源码组合不要求与 28 个展示 Demo 一一对应。
此规则覆盖早期 Stage A 暂不使用 submodule 的历史决策。

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

独立产品组合（具体 Git SHA、来源快照和旧基线范围见
[`docs/reports/v0.6-app-import.json`](docs/reports/v0.6-app-import.json)）：

| 应用 | 产品仓 | AutoOS 路径 | 端口 |
|---|---|---|---|
| 计算器 | [auto-calc](https://github.com/auto-stack/auto-calc) | `apps/011-calculator` | 17810 |
| 时钟 | [auto-clock](https://github.com/auto-stack/auto-clock) | `apps/012-clock` | 17812 |
| 待办 | [auto-todo](https://github.com/auto-stack/auto-todo) | `apps/013-todo` | 17814 / 17815 |
| 天气 | [auto-weather](https://github.com/auto-stack/auto-weather) | `apps/014-weather` | 17816 |
| 便笺 | [auto-notes](https://github.com/auto-stack/auto-notes) | `apps/015-notes` | 17818 / 17819 |
| 日历 | [auto-calendar](https://github.com/auto-stack/auto-calendar) | `apps/016-calendar` | 17820 |
| 聊天 | [auto-chat](https://github.com/auto-stack/auto-chat) | `apps/017-chat` | 17822 / 17823 |
| 图书阅读器 | [auto-reader](https://github.com/auto-stack/auto-reader) | `apps/018-book-reader` | 17824 / 17825 |
| 博客与文章社区 | [auto-blog](https://github.com/auto-stack/auto-blog) | `apps/023-realworld` | 17826 / 17827 |
| 视频播放器 | [auto-video](https://github.com/auto-stack/auto-video) | `apps/030-video-player` | 17828 / 17829 |
| 音乐播放器 | [auto-music](https://github.com/auto-stack/auto-music) | `apps/020-music-player` | 17830 / 17831 |
| 图库与图片查看器 | [auto-photos](https://github.com/auto-stack/auto-photos) | `apps/029-photo-gallery` | 17832 / 17833 |
| 画板 | [auto-paint](https://github.com/auto-stack/auto-paint) | `apps/031-paint` | 17834 |
| 系统监视器 | [auto-monitor](https://github.com/auto-stack/auto-monitor) | `apps/025-sys-monitor` | 17836 / 17837 |
| 数据库管理工具 | [auto-database](https://github.com/auto-stack/auto-database) | `apps/026-database` | 17838 |
| 文件与资源浏览器 | [auto-explorer](https://github.com/auto-stack/auto-explorer) | `apps/027-file-manager` | 17840 / 17841 |
| 通用启动器 | [auto-launcher](https://github.com/auto-stack/auto-launcher) | `apps/028-launcher` | 17842 |
| 俄罗斯方块 | [auto-tetris](https://github.com/auto-stack/auto-tetris) | `apps/036-tetris` | 17844 / 17845 |
| 纸牌接龙 | [auto-solitaire](https://github.com/auto-stack/auto-solitaire) | `apps/037-klondike` | 17846 / 17847 |
| 扫雷 | [auto-minesweeper](https://github.com/auto-stack/auto-minesweeper) | `apps/038-minesweeper` | 17848 |
| 图表工坊 | [auto-charts](https://github.com/auto-stack/auto-charts) | `apps/024-charts` | 17850 |

已有看板、终端、配置分别固定在 `apps/kanban`、`apps/auto-term`、`apps/os-config`。
终端的实际应用根为 `apps/auto-term/app`，配置为 `apps/os-config/auto`，通过 manifest 保留启动 ID。
系统日志 `apps/039-syslog` 保留为 OS 内部组件。四大主力 app 沿既有外部接入配置，
不在此次旧基线导入中迁移或改名；其中旧 Jade 配置仍待完整 v0.5 基线恢复后对齐。

### 检出与验证

```sh
git clone --branch v0.6-dev --recurse-submodules https://github.com/auto-stack/auto-os.git
cd auto-os
python scripts/verify-app-links.py
```

已有检出在 v0.6-dev 下运行 `git submodule update --init --recursive`。
不要以 `--remote` 更新代替固定版本初始化；组合更新应先发布产品提交再提交 gitlink。
新产品仓具有 `main`、`v0.6-dev` 和 `source-sync` 分支。子模块默认 detached HEAD 是 Git 的正常固定版本检出，
开发前请进入对应 app 切到其开发分支，不在 detached HEAD 上留未记录工作。

### 桌面入口

Windows：设置 `AUTO_LANG_ROOT` 为实际 AutoLang 检出，运行 `scripts/desktop.ps1 -Track vue`
或 `scripts/desktop.ps1 -Track iced`。Git Bash/Linux 使用 `scripts/desktop.sh vue|iced`。
两轨主根固定为本仓 `apps/`，manifest 提供嵌套应用根及外部应用，画廊保持原入口。
使用 `-DryRun` / `--dry-run` 可核对路径。不会以 examples/ui 的教学副本优先覆盖产品。

本轮交付仓库/源码组合接线，不表示新产品能力、合并后的全部业务功能或 21 项双端 UI 已验收。
应用所需后端、媒体库、AutoDown 引擎及原生 exe 的准备方式见各仓 README。

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
