# Desktop bootstrap（PLAN-051）

## Owner与入口

scripts/desktop.ps1与desktop.sh是AutoOS开发桌面入口。默认Vue，iced为VM桌面；保留fullscreen、dry-run及DESKTOP_OS_ROOT。语言SDK/宿主载体来自auto-lang，UI实现/tool来自auto-ui，CLI来自auto-man。

## 源码组合

LangRoot/UiRoot/ManRoot参数优先于各AUTO_*_ROOT，再OS兄弟和主检出；显式错误根拒绝。现有Cargo path依赖要求三仓同级且保留auto-lang/auto-ui/auto-man目录名；启用特性所需auto-down等依赖也须准备。

各仓自己的CWD运行cargo build --locked，按Man auto、UI auto-ui-tool、iced时语言ui_desktop顺序构建，统一CARGO_TARGET_DIR=<BuildRoot>/target；默认BuildRoot=<OS>/.auto/desktop-runtime，可由参数或AUTO_DESKTOP_BUILD_ROOT覆盖。iced沿既有mpv-widget媒体面。产物在debug下，宿主在debug/examples，CLI可沿父目录探测。

## 候选组合

CandidateDir与BuildRoot互斥，不构建、不回退PATH或旧语言target。目录平铺auto/auto-ui-tool；iced另需ui_desktop（Windows带.exe），Vue仍须语言examples/desktop-host工程。缺少实际文件拒绝；dry-run只打印，不要求候选文件已存在。

AUTO_UI_TOOL_BIN指向本轮明确工具；CLI、工具和宿主的统一邻接布局满足既有子进程发现。调用者负责匹配源码版本、资源/运行库和profile；文件存在不构成版本或应用兼容性认证。

## 生命周期和失败

OS/apps根、rqhost、显式mpv和Bash storage位置保持；脚本结束恢复调用者环境与CWD，不改系统PATH。构建与启动非零退出码传播；构建失败立即停止，成功后缺件也拒绝，不启动陈旧exe。dry-run无目录创建、Cargo或GUI副作用。

## 验证边界

scripts/tests/test_desktop_launcher.py覆盖PS/Bash的13项定向调度/路径/退出码/环境回归；PowerShell Parser和bash -n核语法。受控cargo/launch只证明脚本，不证明真实三仓构建、桌面首帧或app交互；真实运行结果另记带版本和截图的报告。

来源：[PLAN-051](../../plans/archive/051-desktop-three-repository-launcher.md)。
