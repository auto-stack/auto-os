# 八个基础应用的桌面组合

PLAN-054 的规范候选；最终公共工具链、独立复审和主分支交付门尚未关闭。
本页区分已验运行形态与最终交付状态，不把构建成功等同于任务验收。

## 支持矩阵

前端承载与后端实现分别选择。稳定 ID 来自 apps.manifest；物理目录和产品
显示名不替代 ID。所有产品编辑在独立 clone，AutoOS 只消费已交付的固定 pin。

| 应用 / ID | 固定产品提交 | Vue 桌面后端 | iced 桌面形态 | 数据作用域 |
|---|---|---|---|---|
| Notes / 015-notes | bb064ca5ecdde8e39dfd3b0a9bac4e7f93902ae4 | Rust HTTP | Rust outproc，内嵌后端 | NOTES_DATA_DIR |
| Todo / 013-todo | e915894d57d739719e06f016488f89989f68e843 | Rust HTTP | Rust outproc，内嵌后端 | AUTO_TODO_DATA |
| Calendar / 016-calendar | 7dab8f2f2e14c75bba3993b07d107853c373c31f | Rust HTTP | Rust outproc，内嵌后端 | AUTO_CALENDAR_DATA |
| Paint / 031-paint | c49ef417631dd9a703bafabcb708b7ea13d00d88 | VM HTTP | inproc VM / VM API | Vue 同源 storage；iced 独立 VM storage |
| Database / 026-database | dee81e00f7ba28e045c274cbae6c9f2140635933 | Rust HTTP | Rust outproc，内嵌后端 | 本次显式打开的只读 SQLite 夹具 |
| Chat / 017-chat | 5cdca0f9e33883c7a72c3c9afbeb88234300c918 | Rust HTTP | Rust outproc，共享 Rust HTTP 服务 | CHAT_DATA_DIR |
| Blog / 023-realworld | 33df0fd9db3a18f3d1a760adaa75b495827c1633 | Rust HTTP | Rust outproc，内嵌后端 | AUTO_BLOG_DATA |
| Charts / 024-charts | 64c9db54e75c016673733adfcfa8efd0cbbffc2e | VM HTTP | inproc VM / VM API | AUTO_CHARTS_DATA |

iced 的六个 Rust 应用与两个 VM 应用不能统称八个原生 exe。949 的临时 Todo
VM 前端＋VM API 反例验证是框架消费控制，不替代上表的产品 Rust profile。
Reader 与其他非目标应用的 pin 不属于本包升级范围。

## 组合入口与身份

使用 `scripts/desktop.ps1 -Track vue|iced`（或同语义 Bash 入口），候选组合
显式指定 DESKTOP_OS_ROOT。源码模式的 Lang/UI/Man 必须是同组真实兄弟检出；
预构建模式用 CandidateDir 选择同目录的 auto、auto-ui-tool、ui-desktop。
入口将 AUTO_BIN/AUTO_UI_TOOL_BIN 传给子进程并恢复调用者环境。

共同 SDK 的各仓锁文件必须匹配所选跨仓源码，正式构建保持 `--locked`。
必要的解析器锁更新属于具名组合改面：记录基线与最小依赖差异，并与相同
Lang/UI/Man 根共同构建验证，不以解除锁定或复用旧程序代替成功产物。

组合保留完整注册表，不能生成只含八个目标的替代桌面来宣称正式入口通过。
运行身份须记录应用 pin、候选 manifest、SDK 程序哈希、生成输入与发布/实际
编译产物的一致性。实际数据目录按 app 和本次 run 绑定；Vue/iced 的测试
目录独立，不承诺不同 profile 之间自动迁移或同步存储。

## 用户任务与独立结果

两桌面分别完成：Notes 中文正文保存/重开/再保存；Todo 创建、完成、编辑及
重开再编辑；Calendar 跨日事件编辑/删除/重开/继续创建；Paint Save/Load、
Undo/Redo 与完整 256 格 SVG 一致；Database 真树选择、只读 SQL、拒写和切库；
Chat 合法 A/B 身份收发、恢复登录与重开后的下一次发送；Blog 作者草稿的
多行编辑、预览、重开再保存；Charts 中文 CSV、小数柱图、保存重开和离线
导出。不得以注册图标、健康响应、旧种子内容或 API 预写替代实际 UI 操作。

SQLite/文件/恢复身份、SVG 与实际界面分别核对。Calendar 删除后比较稳定
generation、递增水位与删除事实，不要求包含 rev_high 的整个身份文件不变。
Charts 保存需要源 CSV；保存后重开工程可在 CSV 暂不可读时导出已有数据，
不能把缺源保存拒绝改报成功。数据库夹具的内容和 schema 必须保持不变。

## 永久汇总检查

`python scripts/tests/verify_basic_app_assembly.py --profile vue|iced --os-root <候选根> --run <外部运行包.json>`

运行包采用 basic-app-assembly-run/1，包含恰好八个 cases、完整 registry_ids/
gitlinks、固定 candidate_lock、三个 SDK 程序与真实任务产物的路径/哈希。
每个 case 绑定 task_observation、reopen_observation、下一次操作的实际 PNG、
预期输入与独立磁盘 oracle；原始运行包、截图、OCR/观察记录留在 plan-artifacts，
不进入 Git。读取数据库使用只读连接。Pillow 用于从 iced Charts 的真实 PNG
测量三根蓝柱；SVG 浮点比例另算，像素边缘允许各一物理像素的取整误差。

该入口重新核对当前候选声明/checkout/全体子模块健康、冻结产物字节和真实
任务的观察记录、当前持久结果。它不重放原 UI 手势；复审仍须检查帧内容、
操作轨迹与源/依赖变更影响，不能据一个 outcome 字段或独立 app 的 pass
冒充桌面任务。没有可绑定的实际 run 时入口必须失败。

必需 case 缺失、pending/skip、零场景、错误候选端口/源、真实界面结果缺失
或被修改、持久内容或几何不符，整体非零。混合同场、故障/恢复、进程归属
以及候选回退/最新 main 重放另由对应实际场景证明；汇总入口不会据此自动
授予独立最终复审或合入资格。

## 生命周期与边界

原生共享 Chat 服务由桌面实例持有，最后应用窗口关闭后仍可复用；桌面正常
退出时走身份核验后的优雅关闭。私有 rqhost 的无窗口持有和取消回收见
[桌面 app 启动](../shell/desktop-app-launch.md)。Vue 开发服务有独立持有者；
本次受控取消的真实非零退出不能记作自然退出 0。

Vue Chat 的既有 store 在关窗重开时保留当前登录身份；已验恢复流程包含
显式退出与合法恢复登录，不承诺每窗独立认证态。任意 Win32 HWND 拖入不在
本八应用 adopted 渲染 profile 中；历史一次未定位触发源的 native-dock toast
保留为复审局限，不能宣称已修复。已验当前控制是拒绝不存在的 PID 后，
原应用仍可正常保存、关闭、重开。

