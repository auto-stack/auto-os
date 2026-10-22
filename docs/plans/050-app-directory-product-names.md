---
plan_id: PLAN-050
status: drafting
feature_name: app-directory-product-names
author: [Codex]
created_at: 2026-10-22
updated_at: 2026-10-22
plan_revision: 1
current_step: 0
total_steps: 6
supersedes_spec_components: [docs/specs/apps/independent-repositories.md, docs/specs/shell/desktop-app-launch.md]
new_spec_components: []
touched_goals: []
---

# [PLAN-050] apps 目录使用正式产品名

## 0. 变更摘要

用户授权把 apps/ 下 Demo 编号目录改成正式产品名，例如 015-notes → auto-notes。
本计划完成目录/子模块路径/当前引用迁移，并补齐 AutoUI 发现链对稳定 ID 的支持。
桌面 ID、pac 展示名/工程名、端口、源码 pin 保持不变；不初始化全部空子模块、不动业务源码。

## 1. 目标

24 个目录改名（21 个导入产品、Kanban、配置、内部 syslog），auto-term 保持原名；
common/autostack 为辅助目录，不改。用户能在 apps/auto-notes 等路径继续工作，
已安装文件管理器的检出、detached 状态与内容保留。

## 2. 架构方案

本仓主导跨仓 Plan。auto-os 工作树 D:/autostack/.wt/os-050/auto-os，
分支 codex/app-directory-product-names；语言依赖树同组 auto-lang，
分支 codex/os050-app-registry-ids。每仓只有一个实现工作树。

AutoUI 的 scan_apps、容器展开目前用目录名作为 ID，改为优先解析容器父级
apps.manifest 的 active repo/local 物理路径→ID；无匹配回退目录名。
Vue 的容器展开调用同一实现，避免两条轨重复入口。映射只改变 ID，不改变 pac 展示。
不依赖 symlink/junction，不修改存储中的快捷方式、图标/会话 ID。

## 3. 技术栈

Git submodule 路径移动；Python 组合校验；Rust AutoUI 发现层及针对性回归。
不改变应用后端语言，不增加新业务服务。

## 4. 需求分析与背景调查

授权：本会话用户直接请求重命名，允许必要的注册表兼容、文档和验证修改；
不额外发布、推送或替换 app 内容。现有研究文档属于同会话产物；其他设计/PLAN-049 WIP 保留。

观察基线：auto-os 2b210066ef6855a5f584b118a020015b4146a3d7；
auto-lang 6b2f17c6f929372cc9016da8f21593cc981e4cc5。
24 gitlink；仅 Explorer 已初始化，HEAD 与父 pin 59a99b9f42896728aa32a7ff3e38fa08261c7d18 一致且 clean/detached。
21 产品来自 docs/reports/v0.6-app-import.json；此历史记录保持原路径。
apps/039-syslog 是内部跟踪文件，不是 submodule。
现有 docs/specs/apps/independent-repositories.md 尚将目录 ID 与路径绑定，需要更正。
发现源码：auto-lang crates/auto-ui-compiler/src/app_registry.rs；
Vue 镜像：crates/auto-ui-tools/src/vue.rs desktop_extra_app_roots。
两仓其余在途修改主要在 docs/，不纳入本计划。

## 5. 详细设计

产品目录根据 .gitmodules 远端仓名确定，旧目录→正式目录：
011-calculator→auto-calc、012-clock→auto-clock、013-todo→auto-todo、
014-weather→auto-weather、015-notes→auto-notes、016-calendar→auto-calendar、
017-chat→auto-chat、018-book-reader→auto-reader、020-music-player→auto-music、
023-realworld→auto-blog、024-charts→auto-charts、025-sys-monitor→auto-monitor、
026-database→auto-database、027-file-manager→auto-explorer、028-launcher→auto-launcher、
029-photo-gallery→auto-photos、030-video-player→auto-video、031-paint→auto-paint、
036-tetris→auto-tetris、037-klondike→auto-solitaire、038-minesweeper→auto-minesweeper、
039-syslog→auto-syslog、kanban→auto-kanban、os-config→auto-os-config。

保持 .gitmodules 的逻辑节名（及 .git/modules 存储名）不变，只改 path；
Git 负责重定位已初始化模块的 core.worktree。这样同一工作检出不克隆/切分支。
导入报告不改；verify-app-links 从 stable ID→manifest repo、旧 submodule 逻辑名→现 path
解析位置，metadata-only 能在默认未初始化工作树核对 pin/清单/命名，全验仍明确要求检出。
当前 docs/specs、README/操作文档与研究文档更新现路径，archive/reports 保持历史。

### 规范增量

| delta_id | action | target | before/after | rationale | AC |
|---|---|---|---|---|---|
| SD-01 | modify | docs/specs/apps/independent-repositories.md | Demo 路径→产品路径；ID 与目录解耦 | 正式产品布局 | AC-01/02 |
| SD-02 | modify | docs/specs/shell/desktop-app-launch.md | 保留 ID、明确新路径解析 | 启动兼容 | AC-03 |
| SD-03 | modify | auto-lang docs/specs/auto-ui-compiler/project.md | 加发现映射与回退/复用边界 | 单源实现 | AC-03/04 |
| SD-04 | modify | syslog/game/Kanban 相关本仓 Spec | 当前物理路径更新 | 引用可达 | AC-01/05 |

## 6. 测试设计

同一会话执行/重建验收，无独立上下文；如做 review 会诚实标注此限制，不创建子 agent。
稳定 ID 不变量：扫描主根、VM extra 容器、Vue extra 容器和 manifest 聚合对同一路径产生同一 ID，
不重复；未知应用仍目录名，缺/坏 manifest 不炸，local syslog 可映射；空 submodule 被跳过。
源码回归采用独立 tempfile：有效改名、多入口去重、local/inactive/无匹配、坏 JSON 回退。
OS 校验增加 metadata-only 与失效/旧路径反例；验证历史导入报告不改、24 pin 不变。
定向 cargo test -p auto-ui-compiler --lib app_registry::tests 与 Vue 相关过滤；
最终审查后、语言合入前按该仓 Category B 跑 cargo t，其他触面档由实际 diff 判定。
不宣称已重建/双端 UI 实机验收；编译与发现层测试只证明相应边界。

## 7. 验收标准

| ID | owner_stage | observable/verification |
|---|---|---|
| AC-01 | work | 24 目录完成新名，旧目录无活动检出；manifest/.gitmodules/gitlink一致，24 pin保持 |
| AC-02 | work | 已初始化 Explorer 文件内容/HEAD/分支状态不变且新路径 git status clean；其余不强制初始化 |
| AC-03 | work | 两种发现轨/聚合用原 ID 指向新路径，无重复；无/坏清单有兼容回退，fixture真实执行 |
| AC-04 | work | scoped Rust/Python/启动 dry-run 检查通过；元数据模式不把缺检出当完整运行验收 |
| AC-05 | work | 活动源码/脚本/当前规范/研究入口路径同步；历史导入、archive原始引用保留 |
| AC-06 | review | 检查冻结提交、逐 AC 证据和 Spec delta；报告上下文独立性限制 |
| AC-07 | merge | 语言最终门通过后落主线；OS组合落主线、主检出实际目录迁移；保留其他WIP |

## 8. 执行步骤

- [ ] T-01（work，AC-01/02）：建立专属工作树，记录 rename 映射和 pin/检出状态快照；保持用户已有 WIP。
- [ ] T-02（work，AC-03）：在语言工作树实现 manifest 路径→稳定ID及 Vue 共用容器扫描；补反例与有效控制。
- [ ] T-03（work，AC-01/04）：OS工作树移动目录与gitlink；清单/校验器同步，历史报告不回改。
- [ ] T-04（work，AC-04/05）：更新 README/操作规范/现状Spec/研究资料；验证路径与元数据、定向测试及启动 dry-run。
- [ ] T-05（review，AC-06）：提交并复核冻结实现；以同会话重建检查明确限制，不伪称独立agent复审。
- [ ] T-06（merge，AC-07）：按实际验证结果落主线/主检出目录，记录证据；无法完成则留明确未完成项，不冒称交付。

## 9. 复审记录

- stage: new | revision: 1 | outcome: pass | authorization: 用户本轮直接请求 |
  next: work | changed: T-01..06 / AC-01..07 | scope: 上述目录重命名与必要发现兼容。
- 分号/路径命名、worktree空子模块、初始化检出迁移是本计划边界；
  不删除任何仓或worktree，清理需要 wt-guard。

## 10. 待澄清事项

无需要用户追加授权的设计决策。实际编译/可执行环境是否可用由T-02/T-04验证。
