# App 子模块日常工作流程

生效：2026-10-04，用户最新约定。所有已纳入子模块的app，以`auto-os/apps/<目录>`为日常使用、计划和修改入口；不再把临时独立clone或外部app worktree作为默认工作目录。

## 1. 两种状态

**日常状态**：子模块detached HEAD，指向已经提交/推送、完成相应验证并由AutoOS gitlink记录的最新提交。切到分支本身不会fetch；本地origin引用也可能过时。

**工作状态**：在同一个`apps/<目录>`切到本仓`v0.6-dev`，同步远端后编写计划、修改、验证、提交。结束时先推送app，再更新并提交AutoOS对应gitlink，最后显式切回detached。

这条用户约定覆盖此前app文档中的外部worktree示例及通用app工作位置要求。Plan设计/确认、作用域验证和独立复审仍保留；AutoLang/AutoUI/AutoOS核心修改仍按相应仓规约使用专用worktree，不在本次改动其流程。

## 2. 开始工作

先检查app状态；有未提交内容就保留当前状态并协调，不能reset、stash或切走他人工作。不同app可并行，同一个app检出只有一个写入者。

在app根目录执行：

```powershell
git status --short --branch
git fetch origin v0.6-dev
git switch v0.6-dev
git merge --ff-only origin/v0.6-dev
```

没有本地分支时，在确认没有同名分支后使用`git switch --track -c v0.6-dev origin/v0.6-dev`。快进失败先分析分歧，不强推、不hard reset。

计划放本仓`docs/plans/`；各app独立取号，不占语言/OS全局.next-id。代码和计划都在本app中维护，框架依赖按AUTO_LANG_ROOT等解析规则接入，禁止junction/symlink。

## 3. 完成并记录版本

只暂存本次文件，核对暂存列表后提交；完成对应验证与复审。先`git push origin v0.6-dev`并确认远端可获取该commit，再更新AutoOS gitlink。

以Notes为例，app已完成并推送后：

```powershell
git -C D:/autostack/auto-os/apps/015-notes rev-parse HEAD
git -C D:/autostack/auto-os diff --submodule=short -- apps/015-notes
git -C D:/autostack/auto-os commit --only apps/015-notes -m "chore(apps): advance notes submodule"
git -C D:/autostack/auto-os push origin v0.6-dev
git -C D:/autostack/auto-os/apps/015-notes switch --detach HEAD
```

父仓只提交本次app路径，保留其他app和其他文件的WIP；多个app同批更新可明确列出路径。按实际代码变化验证宿主；纯文档提交不运行cargo测试。

最后核对子模块HEAD等于父仓`git ls-tree HEAD -- apps/<目录>`所记录SHA，且`git symbolic-ref --quiet HEAD`无分支引用。单纯`submodule update`在HEAD恰好等于gitlink时未必改变当前分支状态，结束工作须显式detach。

## 4. 日常同步与在途工作

父仓及目标app干净时，同步AutoOS已发布提交后按固定gitlink执行`git submodule update --init --recursive -- apps/<目录>`。不要以`update --remote`替代父仓组合版本管理；需要更新app时走工作状态流程。

尚未提交的app保持在工作分支，不因其他app完成而统一detach所有子模块。分支切换、父仓指针、远端状态分别检查；父仓显示`M apps/...`可能是app提交领先固定指针，也可能是内部WIP，不能仅凭一个M决定如何处理。

本约定适用于apps中已收编的仓库；外部四大app的物理迁入仍是单独集成工作，不在本次移动目录或改manifest。既有临时clone只作历史保留，后续操作/文件链接使用实际apps检出。
