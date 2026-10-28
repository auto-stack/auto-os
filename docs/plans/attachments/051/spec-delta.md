# PLAN-051 SD-01 proposal（未发布canonical）

目标：新增docs/specs/shell/desktop-bootstrap.md；owner_stage=merge，final review通过后沉淀。

- 桌面入口保持scripts/desktop.ps1和desktop.sh，默认Vue，iced为VM桌面宿主。公开Track/Fullscreen/DryRun用法保留。
- 源码模式参数根优先于AUTO_LANG_ROOT/AUTO_UI_ROOT/AUTO_MAN_ROOT，再OS兄弟和主检出；显式无效根立即报错，Cargo相对依赖要求同级且仓目录名固定。源码版本仍须调用者选择匹配组合。
- Man构建auto，UI构建auto-ui-tool，iced另在语言仓编ui_desktop；各自CWD读取Cargo配置，统一专属CARGO_TARGET_DIR。默认OS/.auto/desktop-runtime/target，BuildRoot或AUTO_DESKTOP_BUILD_ROOT可覆盖。无PATH/旧语言target回退。
- CandidateDir为预构建平铺目录；iced要求ui_desktop/auto/auto-ui-tool，Vue只要求后两者并仍需语言desktop-host工程。缺件拒绝，不构建；与BuildRoot互斥。非Windows无.exe后缀。
- AUTO_UI_TOOL_BIN明确指向本轮工具；统一邻接布局满足rqhost/进程外shell既有CLI发现。OS根/apps、rqhost、显式mpv和Bash storage保持；调用后恢复环境/CWD，不修改系统PATH。
- 所有构建失败和启动退出码传播；构建成功但缺件拒绝，失败不得启动旧exe。DryRun不创建目录、不构建/启动，不要求候选产物已经存在。
- 脚本只调度可执行文件，不构成版本认证、安装资源完整性或GUI验收。本轮仅脚本定向测试，不声称真实编译/首帧/app交互通过。

证据：实现d8dbb046fe0124f52f51a660c67c9a36b9ac94f6，scripts/tests/test_desktop_launcher.py（13测试，PS与Bash子案例）；真实三仓目录dry-run一致，未开GUI。命令和边界见Plan §9。
