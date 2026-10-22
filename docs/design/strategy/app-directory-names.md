# 应用产品目录命名（PLAN-050）

生效：2026-10-22。产品目录按远端产品仓名命名，使用 `apps/auto-*`；稳定桌面 ID、pac 工程名/展示名与物理目录分开管理。

| 迁移前目录 | 当前目录 |
|---|---|
| `apps/011-calculator` | `apps/auto-calc` |
| `apps/012-clock` | `apps/auto-clock` |
| `apps/013-todo` | `apps/auto-todo` |
| `apps/014-weather` | `apps/auto-weather` |
| `apps/015-notes` | `apps/auto-notes` |
| `apps/016-calendar` | `apps/auto-calendar` |
| `apps/017-chat` | `apps/auto-chat` |
| `apps/018-book-reader` | `apps/auto-reader` |
| `apps/023-realworld` | `apps/auto-blog` |
| `apps/030-video-player` | `apps/auto-video` |
| `apps/020-music-player` | `apps/auto-music` |
| `apps/029-photo-gallery` | `apps/auto-photos` |
| `apps/031-paint` | `apps/auto-paint` |
| `apps/025-sys-monitor` | `apps/auto-monitor` |
| `apps/026-database` | `apps/auto-database` |
| `apps/027-file-manager` | `apps/auto-explorer` |
| `apps/028-launcher` | `apps/auto-launcher` |
| `apps/036-tetris` | `apps/auto-tetris` |
| `apps/037-klondike` | `apps/auto-solitaire` |
| `apps/038-minesweeper` | `apps/auto-minesweeper` |
| `apps/024-charts` | `apps/auto-charts` |
| `apps/kanban` | `apps/auto-kanban` |
| `apps/os-config` | `apps/auto-os-config` |
| `apps/039-syslog` | `apps/auto-syslog` |

`apps/auto-term` 原名已经符合规则；`apps/common` 和 `apps/autostack` 是辅助目录，不作为产品 app 重命名。24 个子模块的 URL、固定 SHA 以及所有端口均保持原值；syslog 仍是父仓内部组件。旧 Demo 编号继续用于稳定 manifest ID，不再用于产品目录。

## 注册与检出

AutoUI 用相邻 `apps.manifest` 的 active repo/local 路径匹配稳定 ID，容器和主根使用相同规则，Vue 共用容器展开实现。需要更新带此规则的语言工具/宿主，旧二进制应重建后再启动。没有有效清单/匹配时继续按目录名发现普通 app。

`.gitmodules` 的逻辑节名保留旧名，只有 `path` 改为产品路径；初始化模块的 `.git` 文件和 Git 内部 `core.worktree` 通过 Git 路径移动处理，不搬动 `.git/modules` 数据，也不重新 clone。不能靠修改 `.gitmodules` 文本代替实际 gitlink/检出移动。

历史 `docs/reports/v0.6-app-import.json`、已归档 Plan、来源证据保留原始路径。当前工具从 manifest ID 和产品仓名解析现位置；研究来源记录可同时保留 `path` 与 `current_path`。CLI/API 中 `/apps/<稳定ID>/...` 是路由身份，不是物理路径，不随目录改名替换。

## 验证

未初始化 app 时运行 `python scripts/verify-app-links.py --metadata-only`，只表示组合元数据通过；完整校验仍运行不带参数的命令并要求初始化检出。发布/功能验收继续按各 app 的实际范围完成。改名不更新 app 提交、不强制 detach 他人的在途分支，也不修改源码来源快照。

新 app 的 .gitmodules path 应为 `apps/<远端仓名>`；manifest repo 可指向其中嵌套应用根（例如 `apps/auto-os-config/auto`）。
