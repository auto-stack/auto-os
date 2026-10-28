# App 开发 clone 与组合子模块工作流程

生效：2026-10-29，按用户要求采用“独立 clone 开发，子模块固定组合版本”。替代 2026-10-13 在 `auto-os/apps/` 内开发的约定，也覆盖旧 Plan 固化的开发路径。功能范围、验收门和计划归属不因位置改变。

## 1. 两种检出

| 用途 | 默认位置 | Git 状态 |
|---|---|---|
| app 的计划、编辑、实现与顺序复审 | `D:/autostack/<产品仓名>` | 独立 clone，开发分支 main |
| AutoOS 启动、集成验证和版本装配 | `D:/autostack/auto-os/apps/<产品仓名>` | submodule，detached 在父仓 gitlink |

独立 clone 有自己的 `.git/`，不用 junction/symlink、共享 Git 数据或 submodule 工作目录替代。已有 clone 直接复用，不覆盖；同一 app 一个写入者，实现/检查/复审顺序使用同一个独立 clone，不在两处同时开发。`v0.6-dev` 保留为历史分支。AutoLang/AutoUI/AutoOS 核心的 worktree 纪律保持。

## 2. 开始工作

先核对仓库根、origin、HEAD、分支和 WIP；有未提交或未推送工作时先辨明归属，不 reset、stash、强推或切走他人工作。缺少 clone 时从 `.gitmodules` 对应的真实远端建立，例如：

```powershell
git clone --branch main https://github.com/auto-stack/auto-todo.git D:/autostack/auto-todo
```

在无其他写入者且工作区适合切换时同步：

```powershell
git -C D:/autostack/auto-todo status --short --branch
git -C D:/autostack/auto-todo fetch origin main
git -C D:/autostack/auto-todo switch main
git -C D:/autostack/auto-todo merge --ff-only origin/main
```

缺少 main 时核实无同名分支后用 `git switch --track -c main origin/main` 建立。快进失败先分析，不覆盖。已有 clone 不保证等于组合 pin 或最新远端，执行前须核实实际版本。

Plan 仍在各 app 仓的 `docs/plans/`，各仓独立取号。旧 Plan 的 `auto-os/apps/<产品仓名>` 开发位置按本页改为独立 clone；保留功能合同、任务与历史证据。当前六份新 B 类计划已修正路径，其余在途计划由 owner 继续维护时更新，不批量改写他人 WIP。

跨仓源码遵守环境变量、兄弟检出、主检出的解析序。指向 AutoOS 的文档链接宜用兼容两处检出的远端文档链接，或明确本地 AutoOS 根，避免沿用只适合旧目录深度的相对链接。

## 3. 先交付 app，再更新组合 pin

1. 在独立 clone 只提交本次文件，完成相应验证与最终复审，push app main，并确认交付提交可从远端获取；文档/计划提交同样先交付 app。
2. 核对子模块状态与 owner；有 WIP 或其他在途工作就保留，不切换、不覆盖，报告集成阻塞。
3. 子模块干净且无人写入时 fetch，并显式 detached 到本次已经验证的确切交付 SHA，不以远端最新替代验收版本。
4. AutoOS 暂存对应 gitlink，跑子模块健康门与必要组合验证；只提交本次路径并 push 父仓。
5. 核对远端 app 提交、父仓已提交 pin 与子模块 HEAD 一致；子模块 clean/detached，独立开发 clone 留在 main。

以 Todo 为例，下列更新步骤只在上述状态条件成立时执行；变量必须指向已验证并推送的交付提交：

```powershell
$todoDelivery = git -C D:/autostack/auto-todo rev-parse HEAD
git -C D:/autostack/auto-os/apps/auto-todo fetch origin main
git -C D:/autostack/auto-os/apps/auto-todo switch --detach $todoDelivery
git -C D:/autostack/auto-os add -- apps/auto-todo
pwsh -File D:/autostack/auto-os/scripts/check-app-submodules.ps1
git -C D:/autostack/auto-os diff --cached --submodule=short -- apps/auto-todo
git -C D:/autostack/auto-os commit --only apps/auto-todo -m "chore(apps): advance todo submodule"
git -C D:/autostack/auto-os push origin main
```

每一步失败即停止分析，不串行忽略错误。父仓保留其他 app/文件的 WIP；多 app 同批交付须明确列路径。纯文档不运行语言仓 Cargo 测试。`branch = main` 不代表固定版本自动跟随 main，装配版本仍由 gitlink SHA 决定。

## 4. 同步与保留

消费已发布 AutoOS 时，仅在目标子模块干净且无人写入的条件下执行 `git submodule update --init --recursive -- apps/<产品仓名>`。不用 `update --remote` 批量追新；独立 clone 的修改不自动进入桌面，需交付 pin 后做集成验证。

`.gitmodules` 保留真实产品远端，不改成本地路径，不搬动 `.git/modules` 或更改历史节名。仅补建 clone 不升级 pin、不改 manifest；本次六个 pin 的后续变化只记录已有计划的工作位置/链接修订。此前已有的其他子模块工作分支不因本次补建而统一切换。

[本地开发 clone 清单](app-development-clones.md)记录本次扫描/补建。AutoMusk、AutoJade、AutoShell、AutoEdit 已有独立仓；Jade Garden 历史源在 auto-down 仓内，按同一 AutoJade 产品计数，不为旧名重复建仓。内部 auto-syslog 继续在 AutoOS 内维护。

## 5. 正式目录与已有工作

开发目录与产品仓名一致，例如 `D:/autostack/auto-notes`；AutoOS 内保持 `apps/auto-notes`，不恢复 Demo 编号目录。目录名与 manifest 稳定 ID 分离，映射见[产品目录命名](app-directory-names.md)。

已有 Notes/Reader 等独立仓含 WIP 时，不因子模块有后续提交就覆盖同步；由该 app 的 owner 合并并验证后再交付组合 pin。本次补建保留这些仓库的 HEAD、分支与 WIP。
