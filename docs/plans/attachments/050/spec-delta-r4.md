# PLAN-050 r4 冻结规范增量


| delta_id | action | target | before/after | rationale | AC |
|---|---|---|---|---|---|
| SD-01 | modify | docs/specs/apps/independent-repositories.md | Demo 路径→产品路径；ID 与目录解耦 | 正式产品布局 | AC-01/02 |
| SD-02 | modify | docs/specs/shell/desktop-app-launch.md | 保留 ID、明确新路径解析 | 启动兼容 | AC-03 |
| SD-03 | modify | auto-lang docs/specs/auto-ui-compiler/project.md | 加发现映射与回退/复用边界 | 单源实现 | AC-03/04 |
| SD-04 | modify | docs/specs/apps/{syslog-app,kanban,tetris,minesweeper}.md | 当前物理路径更新 | 引用可达 | AC-01/05 |

