---
plan_id: PLAN-062
status: drafting
feature_name: Linux 平台准入：Counter、Notes 原生 Rust、RQHost 与 Nix 安装
author: [codex]
created_at: 2026-11-05
updated_at: 2026-11-05
plan_revision: 1
current_step: 0
total_steps: 13
supersedes_spec_components:
  - auto-notes/docs/specs/notes/standalone-runtime.md
new_spec_components:
  - docs/specs/platforms/linux-app-admission.md
  - auto-ui/docs/specs/runtime/linux-rqhost-local-runtime.md
  - auto-ui/docs/specs/runtime/native-rust-app-validation.md
  - auto-man/docs/specs/cli/linux-native-delivery.md
  - auto-os-distro/docs/specs/app-packaging.md
touched_goals: []
affects: [linux-admission, rqhost-transport, shared-memory, native-input, rust-app-delivery, notes-persistence, nix-packaging]
---

# PLAN-062：Linux 平台准入门

## 0. 变更摘要

用真实 Counter 与 Notes，在指定 NixOS + COSMIC/Wayland 环境跑通 **Auto 源码 → Rust 工程 → Linux ELF → 独立原生窗 / RQHost → 真实输入与保存 → 关闭重开 → Nix 包安装**。四个必过单元为 Counter×independent/rq、Notes×independent/rq；源码开发态通过后，干净机器安装态重复四项。

本计划是 Autonix M1 与 M2 之间的应用平台准入门，不新增第五产品阶段。它不关闭尚未取证的 M1 安装/更新/回退项目，不代表全部应用已可移植，也不替代 M3 自有桌面/合成器迁移。

## 1. 目标

1. **G1 可重建**：在 Linux 从锁定源码生成并编译 Counter、Notes 为真实 ELF；固定 CLI、生成器、语言 SDK、主题/文档/数据库等依赖组合和全部 Cargo/Nix 锁。
2. **G2 四格真实运行**：同一应用的同一个已编译入口分别以 independent 和 rq 运行；模式、进程、宿主与窗口身份可证明，无静默回退 VM/Vue/独立窗冒充 RQHost。
3. **G3 真实任务**：Counter 交互值正确；Notes 的中文/ASCII/emoji 输入、保存、失败保稿、关闭重开和下一次保存正确，沿用稳定 ID/revision/SQLite 契约。
4. **G4 生命周期**：两个应用共用一个 RQHost 时互不串状态，正常关闭与宿主故障有界回收，旁观用户进程不被结束。
5. **G5 Nix 交付**：Counter、Notes 与所需 RQHost 运行闭包可在干净 NixOS 安装使用，无开发机源码目录、共享 target 或运行时 Cargo/npm 构建依赖。

非目标：完整 Linux 发行/设备矩阵；替换 COSMIC 会话；新编译器/ACE/自举全量完成；全应用迁移；重做 Notes 编辑器/存储模型；远程跨机 RQHost；Windows 行为改变；修改 apps.manifest 全局默认或批量升级无关产品 gitlink；全量图形/语言回归重跑。源码修复只能服务上述闭环，不能用删功能/降级绕过验收。

## 2. 架构方案

```text
UI Counter 源 / Notes 独立产品源
  → AutoMan CLI / tool 协议编排
  → AutoUI rust_ui + AutoLang A2R / SDK + Web 后端服务组合
  → Linux 编译工程、来源 receipt 与 ELF
       ├─ independent：应用自有原生窗口
       └─ rq：应用进程 → 同用户 UDS / OS 共享映射 → RQHost → COSMIC Wayland 窗口
  → Nix 固定源构建、runtime 闭包与启动器
  → 干净 NixOS 安装后重复两应用×两模式的真实任务
```

### 仓库与写入归属

| 仓 | 当前事实/缺口 | 本计划责任 | 执行位置（拟定，work 再核） |
|---|---|---|---|
| auto-os | 发行路线/产品准入 owner；已有 Windows 桌面组合计划 054 在执行，不能把它当 Linux 验收 | 唯一主计划、跨仓组合身份、Linux 驱动/证据汇总和最终准入结论 | D:/autostack/.wt/os-062/auto-os，plan-062-dev |
| auto-ui | transport 非 Windows listen/connect/claim 为错误占位；shm 为进程内 HashMap；win_ime 非 Windows no-op；生成器有 rq/independent 分支但 Notes 初始化/编辑面覆盖未证明 | Linux 本地 IPC/共享映射/仲裁与清理，Wayland 输入/IME与编辑面，Rust 生成和真实宿主；Counter 源属于本仓 | D:/autostack/.wt/os-062/auto-ui，plan-062-ui-dev |
| auto-man | build/run/rqhost 编排已存在；Linux 命名/工具发现/子进程/资源发布完整链路未验 | 固定工具与 backend 选择、Linux 可执行文件发现、退出码、打包入口；不接管 UI/语言实现 | D:/autostack/.wt/os-062/auto-man，plan-062-man-dev |
| auto-lang | 已有 A2R 和 SDK，Windows 生成成功不能推出 Linux 消费通过；本轮未证明语言缺陷 | 默认是固定 SDK 输入；仅对 T-04 复现并归属确认的转译/运行时缺陷做必要修复与对应固定组回归 | 需要修改才建 D:/autostack/.wt/os-062/auto-lang，plan-062-lang-dev |
| auto-notes | 产品持久化/VM 原生门已有；现有 verify_standalone 默认 .exe/Windows process-group/taskkill，未提供 Linux 完整 Rust×RQHost 证据 | 在真实产品上补 Linux Rust 测试/启动适配，复用编辑与仓储，不复制产品或降级为 Counter 式替身 | D:/autostack/auto-notes 独立 clone main；一 app 一写入者，不在 apps/ 子模块或另一个 app worktree 开发 |
| auto-os-distro | main 为路线/历史记录，完整 M1 构建在 plan-001-dev；flake 尚无应用包输出 | 新增纯 Nix 源构建与安装闭包，干净 NixOS 四格验收；不顺带合并未验收 M1 | D:/autostack/.wt/os-062/auto-os-distro，plan-062-distro-dev，基于确认的 M1 构建提交而非假定 main 已有 flake |
| auto-web / auto-down | 当前 CLI/Notes 组合可能需要 Web 服务和文档资产 | 锁定、清点实际依赖与来源；默认只读消费，不复制实现到语言/UI；如出现需改 owner 的阻塞先具名判定 | 必要时同组真实 detached 依赖 worktree；不建链接 |

跨仓改动各自提交，未批准的新需求进入修订或子计划，不把所有问题笼统归为语言。两边 README/计划需互链本主计划；不再在语言仓分配另一份同范围主计划。

## 3. 技术栈与首验范围

- 平台：x86_64-linux；锁定 M1 NixOS 26.05 + COSMIC；真实普通用户 Wayland 会话；VMware 4 vCPU/8 GiB/3D、约 60 GiB 独立测试盘为参考。软件渲染可作诊断，不能冒充承诺的 GPU 路径。
- 首验只承诺这一组合；WSLg/Xvfb/Ubuntu/纯 X11 可作开发探针，不替代 NixOS COSMIC/Wayland 的最终结果。记录实际窗口 backend，不能只凭 WAYLAND_DISPLAY 环境变量判断。
- Counter：auto-ui/examples/ui/002-counter/{pac.at,src/front/app.at}，真实主题依赖 packages/themes；不新增持久化需求。
- Notes：auto-notes/{pac.at,src/front,src/back}；api:rust、NOTES_DATA_DIR、inbox.db、NoteDocument/revision 沿现有契约；输入保存测试只使用隔离合成数据。
- 编译入口：auto build -r rust；运行入口为发布 ELF 的 --render-mode independent / --render-mode rq。`auto run -r vm`、Rust API server 或窗口存活均不能替代完整 Rust 前端验收。
- RQHost 仍是运行服务依赖；“独立 Rust 应用”不承诺单文件/零运行库。Nix 必须把所需 auto rqhost 或私有稳定宿主启动器和资源一起闭包交付。
- 网络：构建/安装允许联网取固定输入，运行 Counter/Notes 本地任务不要求外网；本计划不承诺首次无网构建。

## 4. 需求分析与背景调查

### 4.1 授权与约束

用户要求“先建立计划，用 Counter 和 Notes 跑通独立 Rust 应用、RQHost、输入保存与关闭重开，再验证 Nix 打包安装”。**本轮仅建计划**；不修改实现、不建立实施工作树、不构建、不启动/格式化虚拟机、不更新 gitlink。用户未指定 token/时间预算，也未授权多代理执行。

后续 work 的预检首先核实已有安装：测试 VMDK 约 10 GB 的写入和旧欢迎页记录均不能单独证明硬盘启动。已有系统优先只读核实并复用；不因缺记录而重新擦盘。若尚无可用硬盘系统，把环境前置交给历史 M1 安装项或单独批准的环境准备，不能用 Live rebuild 代替目标安装验收。

主检出存在其他工作：AutoOS 设计/研究文档和 Charts 子模块 WIP；UI 有其他设计/计划与未跟踪 test/；Notes 有未跟踪 src/front/tests、tmp。只读调查不接管、提交或清理这些内容。app 主 clone 业务触面若与另一写入者重叠，先协调所有权；不 stash/reset、不复制另一份可编辑 app。

### 4.2 观察基线（不是执行冻结锁）

| 仓/来源 | 本轮观察身份 | 使用边界 |
|---|---|---|
| auto-os | 35026a17aa6397df6a93792258f93aa61ab8d5eb | 产品计划与支持矩阵 |
| auto-ui | bd3536e7961dc2701e57576d653e880e46a4e12e | 现有 transport/shm/IME/生成器与 Counter |
| auto-man | 03bb65ef7fb6181caa6a08d9752ac7d57c75516c | CLI 编排 |
| auto-lang | 7327e7cc987e876c06cad56b475a60d31d5d854e | 仓在其他工作中推进，T-01 需冻结兼容提交 |
| auto-notes | 38c7c8703a2f9fd4558684d5b80ee2765fedc040 | 当前产品来源；不混入未提交测试片段 |
| auto-os-distro main | 5100ce1ac323f51d40ea26819376f17a04c40f3a | 已批准 Autonix 路线图；PLAN-052 已归档 |
| distro M1 源码 | 79e1435598b0367481919e1a7486d83e599ee358 | m1-nix-cosmic-preview / 现存构建 worktree；安装状态单独核 |

执行时读取并固定组合，不要求其他在途计划停止推进。自动更新到“最新”、SDK/CLI/生成器不匹配或依赖尚未落地主干，均不能默默沿用旧验收。

### 4.3 已读规范与代码证据

- AutoOS：docs/specs/shell/desktop-bootstrap.md、desktop-app-launch.md；发行仓 ROADMAP.md 与 docs/verification/M1-CHECKLIST.md。旧桌面 spec 中 auto-lang/ui 路径是历史归属，现源 owner 在 auto-ui；本计划不用旧路径修改代码。
- AutoUI：docs/specs/runtime/native-vm-app-validation.md、host-theme-transport.md、ui/structure-test-map.md；native-vm 规范的 `-r vm` 范围不自动改称 Rust 发布形态。
- 语言：docs/specs/auto-lang/trans/rust-semantic-admission.md、repository-boundaries/standalone-language-build.md；旧 A2R 仍是生产入口，不要求改走 ACE。
- CLI：auto-man/docs/specs/cli/structure-closure.md；crates/auto/src/{cmd_build,cmd_run,cmd_ui,tool_ui_ops}.rs。
- Notes：docs/specs/notes/{standalone-runtime,durable-inbox}.md、NOTES-004 归档与 tests/basic/verify_standalone.py。旧原生证据是 VM 前端 + Rust backend，不推导完整 Rust 或 Linux。
- 历史 auto-ui/PLAN-052 证明六 app Windows PE/exe 构建和 Notes 8s 冒烟；不评六 app 完整业务，本计划重新做指定 Linux 任务。AutoOS PLAN-054 是 Windows/Vue/iced 组合，明确排除 Linux；可消费其已交付工具修复，不等待其无关全量结案。

| 关键源文件（相对 auto-ui，另注者除外） | 本轮 SHA-256 |
|---|---|
| crates/auto-ui-runtime/src/ui/desktop_protocol/transport.rs | 3b3b1dd3faf06e9d8f398d8d298ef7205cea0edde2f0252ca09f86cdba2a65db |
| crates/auto-ui-runtime/src/ui/desktop_protocol/shm.rs | 294771ef9533c1739311f8ea6aa2e5ecb53ab93cfc5b6089003b458dafbbda9c |
| crates/auto-ui-runtime/src/ui/desktop_protocol/win_ime.rs | bb3019fcb7bd0cd64fcbdc9b87bf9dcd497dd6a6c90d3d83bd259f55b27c1e81 |
| crates/auto-ui-tools/src/rust_ui.rs | f464ff5179b41a6b60dec4092280120e7727d89c8ce2d79d9dad26362a3a6cf7 |
| examples/ui/002-counter/src/front/app.at | 04937c35325faa74ac790dee8ca97eef5cb34715badaeecc04eb1a3fe26d0d69 |
| auto-notes/pac.at | a59ab766400ec36a6172ef2cdfb9e572ed4d0ee401e5893a6f0c9ea7badcd3df |
| auto-notes/tests/basic/verify_standalone.py | 7bf3b68d7cd8d8c7d45dcf8834a4b73cf2c3f17f50132aa25a23104ec6fe6bdb |

### 4.4 确认缺口与待探问题

| ID | 证据/分类 | owner / 闭合任务 |
|---|---|---|
| GAP-01 | transport.rs:399 起 non-Windows pipe_stub 的 listen/connect/try_claim 均返回错误；确定缺实现 | UI / T-02、T-03 |
| GAP-02 | shm.rs:134 起 non-Windows inproc_map 只能同进程 create/open；真实双进程、容量/所有权/回收尚缺 | UI / T-02、T-03 |
| GAP-03 | win_ime 非 Windows 为 no-op；不能拿协议中存在 ImeReq 当中文输入已通 | UI / T-02、T-05 |
| GAP-04 | rust_ui 已有模式选择和 ensure_rqhost_ready；孵化/async-init 注释保留 default 态/覆盖降级边界。Notes 后端装载、编辑面、路由在真实 rq 模式尚未验 | UI 主、CLI 辅 / T-02、T-04、T-05、T-07 |
| GAP-05 | .exe/工具发现/工作目录、动态库与打包发布在 Linux 尚未完整验；verify_publishable_image 非 Windows 仅初字节 0x7f，不足以替代真实 ELF/架构/执行证明 | UI 工具、Man / T-04、T-09 |
| GAP-06 | Notes 原门默认 Windows 路径、CREATE_NEW_PROCESS_GROUP/taskkill，API/数据端口与故障恢复需 Linux 适配；不是已证明语言后端失效 | Notes / T-02、T-07 |
| GAP-07 | 当前 Nix flake 无 Counter/Notes/RQHost 包输出、runtime 闭包和干净环境安装证据 | distro / T-09、T-10 |
| GAP-08 | 真实 Wayland 窗口/焦点/DPI/中文、宿主死亡、多应用隔离以及 Windows 非回归未在新组合联验 | UI、OS、Notes / T-05～T-08、T-11 |

## 5. 详细设计

### 5.1 T-02 的有界可行性与设计冻结

在依赖实现前，最多比较两种本地方案：默认候选 **UDS + 同用户私有 runtime 目录中的 OS mmap 文件**，保留现有通道/BufferAlloc 名称语义；备选为 memfd + FD 传递，只有可在既有端点/能力协商内落地才选择。不得以 TCP/浏览器 remote、loopback、把两端塞同进程来绕过 Linux 本地通道要求。

产出 UI 仓新增 docs/design/autoui/linux-rqhost-local-runtime.md：明确实际 producer/consumer、地址映射、claim 锁、peer 身份、段所有者、重连/退出策略和所有成功/失败清理点。先做真实 A/B 子进程 create/open/write/read/ack/exit 最小探针，再冻结一个方案；不同时建设两套后端。

同阶段完成三项整链调查：Linux 编译/动态库清单；Notes 完整 Rust 编辑/后端初始化的 rq 覆盖（不能只看空白首帧）；实际 IME/输入通路。若发现必须重写整个编辑器、建立新显示协议或引入新平台路线，具名 needs_replan，先修订受影响依赖，不能静默删除 Notes-rq 格。

### 5.2 IPC、共享资源与生命周期契约

- 同一登录 UID 会话的本地应用受支持；runtime 根归该用户、目录 0700，端点/共享文件 0600；拒绝非预期 owner、路径穿越、符号链接或不可证明安全的 runtime 根，不回落公共可写临时目录。
- 单实例 claim 必须原子并覆盖并发启动；不能先探测再孵化造成两个 daemon。只有确认非活动且属于本实例/本用户的残留才回收；旧 socket/段不得冒充新 generation。
- 控制消息沿现有分帧/codec；ULong/size/slot 计算必须 checked，先验容量再分配/映射。首验边界：控制帧最大 8 MiB、单共享段最大 64 MiB；0 slot、越界、乘积溢出、宣称长度超过真实段均具名拒绝。现有合法默认值保留；若 T-02 发现当前必需成功输入超出边界，先修订本计划，不能裁剪合法帧来过门。
- 发布、消费与 ACK 后复用的所有权可观察；关闭只释放相应 app/session 的句柄、槽位、端点，其他客户端保留。不能用进程私有 Arc/Vec 模拟跨进程共享。
- rq 档沿 ClientTarget::Rqhost 的 exit-on-EOF，不改成无界等待/静默 independent 回退；既有 Direct/Broker 的重连语义与 Windows 命名管道行为保持。
- 运行预算：沿用 ensure 的 15s daemon 就绪与现有 IPC 等待语义；已编译应用首启最多 120s；正常退出与故障收口各最多 30s。冷构建/下载独立记录，不塞入 UI 就绪预算；timeout 是失败，不算已回收。不得据此给所有 API/任务追加新全局超时。

### 5.3 编译产物与运行模式

- 两应用从冻结 Auto 源生成 Linux Rust 工程，使用对应 Cargo.lock 和兼容 SDK/生成器；不能直接交付 Windows PE、开发机共享 target 的同名旧程序或改用解释器。
- 记录 ELF header/架构、动态依赖、发布 receipt、二进制哈希和源码关系；用 `file`/`readelf` 加真实执行佐证。生成/编译/发布失败不得替换旧有效产物，错误保留首次失败阶段。
- 同一应用同一 ELF 分别 --render-mode independent 与 --render-mode rq；观测宿主 adopt/connection/app/window 身份，证明 rq 真经宿主产帧及输入回传。已建 rq 代码分支、env、进程存活或无内容截图不能独立充当 PASS。
- rq 形态中 Notes 的初始化 API、编辑动作与保存读回走真实产品；缺少覆盖具名失败，不允许退回 Vue、VM 前端、独立窗或由宿主直接写 DB。

### 5.4 Counter 与 Notes 的任务语义

- Counter 两模式均从 0 开始；真实鼠标/键盘完成 `0 → 1 → 2 → 1 → 0` 并逐次核对显示/状态；退出重开重置为 0，符合原产品，不增添保存计数要求。
- Notes 两模式均通过真实 UI 创建笔记，输入带本轮唯一标记的 ASCII、中文、emoji 与换行；至少一次实际 Linux 输入法组合/提交，防止只测 API/MCP 赋值绕开 IME。鼠标定位、选择/删除、resize 后再输入必须作用于正确文档。
- 保存时以当前稳定 note_id、revision 和既有 NoteDocument 语义核对：UI 成功、API/收据与 SQLite 只读结果一致。不能通过 API 预写后拍窗当输入成功。
- 正常关闭重开保持已提交内容，再次编辑保存 revision 严格推进；先 independent 保存再 rq 重开、反向亦然，数据格式兼容且使用同一指定 NOTES_DATA_DIR。
- 保存失败必须通过既有产品故障入口或确实造成写入失败的隔离目录权限控制触发；原 DB/revision 保持、UI 显示失败且草稿不丢，恢复条件后重试成功。T-02 固定可恢复触发步骤，不把无错误发生的权限修改算失败测试。
- 另保留产品已有崩溃后已提交内容恢复与“下一次保存”用例，移植 Linux 自有进程组清理；不扩大成“所有未保存草稿均保证跨强杀恢复”的新承诺。
- 每次测试隔离 NOTES_DATA_DIR、HOME/XDG、端口、runtime session 名与进程组；默认包不启用 NOTES_TEST_MODE/NOTES_SEED。只回收本次 app 的 frontend/backend/辅助进程；共享 daemon 和旁观者有独立 owner。

完整 Rust 路径的观测入口由 T-02 冻结：若未暴露 MCP 树，采用该运行形态实际具备的原生输入/窗口观测、可见内容和产品持久层只读回证；不得另启 VM 会话提供状态冒充被测 ELF。Linux Wayland 不要求存在 Windows HWND；以真实进程、surface/window、连接和帧的对应关系定位窗口。自动化未覆盖的 IME/输入动作可人工执行并绑定同一 run 的结果，仍是必过项，不因工具缺席跳过。

### 5.5 Nix 构建与安装交付

拟新增输出（由 T-09 提供，当前不可直接调用）：
`packages.x86_64-linux.autonix-counter`、`autonix-notes`、`autonix-rqhost`。

- 固定各仓 commit、Cargo/vendor 输入、主题/文档资产、Nix 锁与源码 hash；多仓 path 依赖在构建组合中保持原名/归属。Nix derivation 不读取 D:/、/mnt/d、/mnt/e 或用户主检出，不夹带 token，也不在构建时绕过 sandbox 下载未锁输入。
- auto CLI/auto-ui-tool/生成器属于构建工具；rq 运行确实需要的 auto rqhost/共享库/字体/主题或文档资产进入运行闭包。清点真实 dependencies，不能因“Rust 编译过”漏 SQLite、显示或文档资源；不预先假定必须引入某个 WebView。
- installed launcher 指向具体 store ELF，并分别支持 --render-mode independent/rq；必要 AUTO_BIN/AUTO_CLI 固定到同闭包宿主。缺件清晰报错，不从开发机 PATH/源目录找回旧程序或临时 Cargo 构建。
- 提供菜单 `.desktop` 和命令行入口；用户可写数据落 XDG/Notes 数据根，Nix store 不可写。包移除不删除用户数据，重装后仍可继续编辑保存。
- 在真实 COSMIC/Wayland 的干净 NixOS 测试用户上，仅依赖包闭包启动四格；不要求整个源码树、Cargo/npm/node 工具或构建缓存。准备闭包可联网，安装完成后本地应用任务在断开外网条件下可运行。
- 若生成工程无法在纯 Nix 构建中重现（动态源码发现、隐式网络/路径、缺锁），T-09 判失败并回 owner 修复，不能仅复制任意开发机 ELF 包一层就称源码包可重建。

### 5.6 产物/证据与交付边界

执行开始登记 owner=auto-os/PLAN-062 的唯一暂存 scope `D:/autostack/plan-artifacts/plan-062/auto-os/`；跨仓按子目录区分，不能与 UI 等仓同号 scope 混写。证据、构建来源清单、探针、导出副本只放统一 scope，不进 Git；仓内受维护的正式 tests/ 与源代码不是一次性探针副本。

Linux/客体的 native 构建优先使用 Nix 受管理 sandbox/构建目录；如需计划专用工作/target/数据缓存，T-01 先登记具体客体路径、owner、其与统一 scope 的关系及关闭条件，禁止另建宿主散落 target/evidence。运行产品的私有 XDG socket 和测试 NOTES_DATA_DIR 按生命周期回收，不把用户正式数据当缓存。旧 M1 ISO/VM 是既有具名交付资产，保持原 E 盘位置，不在本计划清理。

每条证据绑定全部 source SHA/lock、生成器/CLI/ELF/包 store identity、内核/会话/GPU/backend、用户/运行 ID、输入动作、读回、退出码、PID/端点清理与恢复结果。Git 只保存 Plan 内摘要和已验规范；原始材料在 merge 时按现行政策折叠后回收，具名未解决问题另记 owner/关闭条件。

### 规范增量

均为待实现/复审后定稿的增量，本轮不创建 canonical Specs，不写 legacy specs.json。

| delta_id | add/modify/retire | target | before/after rule | rationale | AC |
|---|---|---|---|---|---|
| SD-01 | add | auto-ui/docs/specs/runtime/linux-rqhost-local-runtime.md | Linux 占位 → 真实本地传输/映射、容量、owner、claim、退出与输入合同 | 不把 Windows/loopback 当 Linux | AC-03、AC-04、AC-06 |
| SD-02 | add | auto-ui/docs/specs/runtime/native-rust-app-validation.md | 原 NVM 为 VM 前端 → 单列完整 Rust 双模式验收，不改旧 NVM 意义 | 防止“原生”形态混淆 | AC-02、AC-04、AC-05 |
| SD-03 | add | auto-man/docs/specs/cli/linux-native-delivery.md | 组合未实证 → 固定工具/产物/模式/失败退出与运行闭包发现 | CLI 编排 owner | AC-02、AC-07 |
| SD-04 | modify | auto-notes/docs/specs/notes/standalone-runtime.md | Windows VM/Vue 门 → 增列 Linux 完整 Rust independent/rq 与同库恢复，原产品语义不变 | Notes 用户任务 | AC-05、AC-06、AC-08 |
| SD-05 | add | docs/specs/platforms/linux-app-admission.md | 无两应用准入合同 → 指定平台、四格、兼容身份与结论边界 | OS 产品准入真值 | AC-01、AC-04、AC-08、AC-10 |
| SD-06 | add | auto-os-distro/docs/specs/app-packaging.md | 无 app 包 → 固定源构建、闭包/入口/数据目录与安装验收 | 发行装配 | AC-07、AC-08 |

语言若无源码/语义改动不制造规范条目；若真实修复改变已承诺语义或源/ABI 支持范围，先修订 Plan 和实际受影响 Spec，不借本计划扩大语言重构。

## 6. 测试设计

### 6.1 必过矩阵与证据分层

| App | 源码构建后的 independent | 源码构建后的 rq | 干净 Nix 安装后的 independent | 干净 Nix 安装后的 rq |
|---|---|---|---|---|
| Counter | 计数/输入/关闭重开 | 真宿主、同任务 | 同任务 | 同任务，证明包内宿主来源 |
| Notes | 编辑/保存/失败/恢复 | 真宿主、同任务 | 同任务与数据根 | 同任务与同库跨模式重开 |

这是四个运行形态在两个交付阶段各验证一次；不合并分母。Unit/headless/WSLg 只证明相应底层或开发路径；实窗输入、数据、生命周期、纯包和干净安装分别闭合。任何格失败整体准入不通过，不能用 3/4 或 Windows 绿代替。

### 6.2 关键不变量、区分性用例与关闭条件

所有用例 owner_stage=work，真实输入与进程负控不能用源码字符串守卫替代；新增驱动由相应任务使之可执行。

| case_id | AC / invariant | 输入/设置与真实入口 | 可观察 oracle / 关闭条件 | 责任任务 / 覆盖边界 |
|---|---|---|---|---|
| V-01 | AC-01 来源/环境 | 固定源码锁、实际 VM/会话与 ELF backend；错误/缺工具路径合法对照 | 来源清单完整，错源具名失败不回落别的主检出；已有安装只读证明根盘 | T-01 / 单一声明组合 |
| V-02 | AC-02 编译发布 | Counter、Notes auto build -r rust；失败/损坏候选与有效旧 artifact 对照 | 真实 ELF/架构/依赖/执行成功；发布失败不覆盖旧有效哈希，不接受 PE/旧同名程序 | T-04 / 两真实消费者 |
| V-03 | AC-03 跨进程本地传输 | A/B 不同 PID 经 UDS 分帧；拆包/残帧/EOF；占用、错误 UID、旧 socket 反例 | 两端校验同一消息序列，失败有界；单实例未被抢占，非法 peer 拒绝 | T-02/03 / 不用 loopback 过门 |
| V-04 | AC-03 槽/段所有权 | A 写 B 读并 ACK、复用；非法 slot/长度/容量/乘积、进程死亡与重建 | 真实跨进程字节 hash 一致，先验证后映射；超限具名失败，无陈旧 generation/句柄泄漏 | T-03 / 控制帧8MiB、段64MiB边界 |
| V-05 | AC-04 Counter四格 | 真实 +/+/-/Reset、键盘激活、resize/重开 | 逐次0/1/2/1/0，重开0；rq 由 daemon 呈现，独立窗由自身呈现 | T-05/06/10 / Counter不要求存档 |
| V-06 | AC-04/05 Notes输入保存 | 两模式真实编辑/中文IME组合提交/emoji/换行；保存后 API/SQLite 只读 | 可见字符与合法文档内容匹配、note_id一致、revision推进；真实 rq 输入与后端装载可证 | T-05/07/10 / 不直接预写数据 |
| V-07 | AC-05 保存拒绝/恢复 | 先有已提交v1，UI编辑v2→受控写失败→恢复并重试 | DB仍v1/原revision、错误可见、UI草稿v2保留；恢复后恰当成功，不伪报保存 | T-02/07/10 / 现有 Notes 故障合同 |
| V-08 | AC-05/06 数据跨重开 | 正常关闭及产品原有强杀用例，independent↔rq 同库重开后再保存 | 已提交文本/ID保持，下一次revision推进；新数据根不串旧库 | T-07/10 / 不扩大未提交草稿承诺 |
| V-09 | AC-06 共享宿主隔离 | Counter+Notes 同时 rq；同时孵化/关闭其一/关闭末窗/待定采纳 | 只有一个 daemon，关一个不影响另一个；末窗且无pending后退出，端点/段回收 | T-03/08 / 用真实pending发布非手工塞队列 |
| V-10 | AC-06 故障 owner | 单 app 取消、宿主中止、后端不可用；健康对照/旁观进程 | rq HostLost 有界收口不静默回落，旁观者存活；旧数据有效、重启可恢复 | T-08/10 / 仅本测试进程树 |
| V-11 | AC-07 Nix源构建/闭包 | 固定源 sandbox nix build 三包；移除隐式开发路径/旧ELF对照 | 源重建成功，closure/资源齐，store中无用户可写数据；失败保旧包/receipt | T-09 / 不以拷贝任意ELF代替构建 |
| V-12 | AC-08 干净安装 | 干净NixOS用户安装三包，清除源码/开发工具PATH依赖，四格及移除/重装 | 全任务可用，无运行时构建/外网获取；菜单/CLI均可启动，Notes数据保留 | T-10 / 约定COSMIC/Wayland，不推全发行版 |
| V-13 | AC-09 Windows/旧门 | 改动前后同源Windows IPC/SHM/模式选择与Counter；Notes既有门按受影响路径 | 旧端/合法输入不变，新Linux不污染Windows；无语言改动不滥跑语言套件 | T-11 / 按真实改动选择完整职责族 |
| V-14 | AC-10 可核验结论 | 对照每格源码/ELF/package/window/session/data/进程终态及Spec delta | 证据无缺项，未执行不算通过，产品根/各仓文档互链 | T-11 / 自检不是最终review |

### 6.3 验证入口（当前/新增分列）

现有入口（work 在对应权威 worktree/Notes clone 执行，**本轮未运行**）：

- CLI：Man 仓 `cargo build -p auto --locked`；UI 工具 `cargo build -p auto-ui-tools --bin auto-ui-tool --locked`；各自编译目录/来源按 T-01 固定，禁止从产品主检出顺手构建。
- app：在真实项目源根运行 `auto build -r rust`，通过发布 receipt 找 ELF；直接运行发现的 ELF `--render-mode independent|rq`，不硬编码 `.exe`、hash member 或开发机路径。
- UI：既有 `cargo tu`（ui_gen 单进程）及 `cargo ti` 对触面选择；IPC/SHM/模式族可走 `cargo test -p auto-ui-runtime --lib ui::desktop_protocol::<已存在/新增模块族> -- --test-threads=1`，T-03 明确新增Linux cases和完整族名；真实窗口另列，测试 skip 不充 PASS。
- Notes：`python tests/basic/verify_standalone.py --profile vue|vm --case ...` 是既有门，不能拿 vm 通过替代新 Rust profiles；保留其断言，Linux rust-independent/rust-rq 的准确参数由 T-07 增补并写入 Spec。
- 若且仅若语言 crates 有修改：按真实触面完整固定组 `cargo t a2r`、必要 `cargo t a2r-std`/`http`/`vm`；无语言修改不跑语言 cargo t/docs_gen，不编实验 ACE/全AG。

拟新增入口（当前不存在，由标明任务提供，不作为已执行证据）：

- OS `scripts/linux-admission/`：四格编排/来源记录/自有进程收口/结果汇总（T-06～T-11）；必须有自测负控，拿错ELF/绕开rq/未保存/进程未退均能判失败。
- UI Linux 双进程协议/共享映射/claim/IME 与必要 Notes 编辑面的测试（T-03/05）。
- Notes Linux Rust 真实输入保存恢复驱动（T-07），复用本产品数据断言，不创建平行假 Notes。
- distro 新增 `nix build .#autonix-counter .#autonix-notes .#autonix-rqhost --no-link` 与 profile安装命令（T-09/10）；不在现有 main 上假称这些输出已经存在。

### 角色

实施与自检为 work；用户未指定模型/人员。最终 review 在各仓同一冻结实施位置、独立上下文核验 AC/V 与 Spec delta；merge 在 pass 后做定向冻结门与落地。不自动开新聊天或子代理，不以实现者自检冒充最终独立审查。

## 7. 验收标准

| ID | owner_stage | 可观察行为与判据 |
|---|---|---|
| AC-01 | work | 锁定真实Linux支持组合、仓/SDK/CLI/包/数据身份及环境；所有权/WIP清楚；现有硬盘安装先核实，未证明不标通过 |
| AC-02 | work | 两应用真实Linux Auto→Rust→ELF可构建并执行，来源receipt正确；错误候选不覆盖有效旧产物，无解释前端替代 |
| AC-03 | work | 真UDS/OS共享映射双进程互通、单实例与身份/容量/ACK/generation/清理反例通过；不保Linux占位充通道 |
| AC-04 | work | Counter/Notes同ELF的independent与rq四格实窗、输入/resize与真实宿主身份均通过，不静默回落 |
| AC-05 | work | Notes中文/emoji/多行真实输入、保存及只读持久层核对、失败保稿与恢复、跨模式重开和后续revision通过 |
| AC-06 | work | 两app共宿主时隔离、关其一/末窗/pending/宿主故障/后端失败有界处理，自有PID/端点/段回收，旁观者不受损 |
| AC-07 | work | 三个Nix输出由固定源码sandbox构建，完整运行闭包和入口可追踪，无绝对开发路径/旧缓存/运行时编译依赖 |
| AC-08 | work | 干净NixOS安装态四格重复通过；包移除重装不删Notes数据，离线本地任务可用；不借开发环境或Live替代 |
| AC-09 | work | Windows与旧公开模式/产品范围按实际触面零回归；条件测试与未测分列，未降低原用例oracle |
| AC-10 | work | 全证据与真实版本绑定，SD-01～SD-06按实现核定为可审阅规范；各仓链接本计划，未关闭无关M1/054或宣称全应用支持 |
| AC-11 | review | 独立核验当前Plan revision、全部受影响仓commit、真实四格/安装态结果、负控与规范增量，给revision-bound结论 |
| AC-12 | merge | review pass后依赖提交按序落地主分支，canonical知识与来源匹配，准入状态回填路线图，恢复/清理记录完整 |

## 8. 执行步骤

只有本表为当前执行任务；未来测试/构建/安装全部未执行，current_step 是已完成任务数。

| 完成 | ID | owner_stage | 依赖 | owner / 文件与操作 | 关闭方式 | AC |
|---|---|---|---|---|---|---|
| [ ] | T-01 | work | 无 | OS主导：复查各仓状态/Spec/实际M1安装；冻结源码/工具/支持会话/数据/端口/输出scope/空间，建立必要实施树；Notes clone确认单写者 | 来源与环境清单、现有根盘只读证明或具名环境阻塞；不开始擦盘 | AC-01 |
| [ ] | T-02 | work | T-01 | UI主导，Notes/Man/distro配合：有界比较UDS+mmap与memfd两案，真实双进程探针；核Notes完整Rust-rq编辑/初始化、Linux IME、依赖与故障入口，新增设计决策文档 | V-03/04最小控制及完整可行性结论；一个选择冻结，超范围则needs_replan，不删失败格 | AC-01、AC-03、AC-04 |
| [ ] | T-03 | work | T-02 | UI：desktop_protocol/{transport,shm,endpoint,client_entry,rqhost}.rs及新增Linux模块/测试；真实通道、映射、claim、peer/容量/回收 | V-03/04/09协议部分，Windows同族合法对照；资源负控失败非零 | AC-03、AC-06、AC-09 |
| [ ] | T-04 | work | T-02 | UI tools/Man：rust_ui.rs、cmd_build/run/tool_ui_ops等实际发现/发布/子进程接缝；修复两应用Linux构建；Lang仅对已归属缺陷必要修复 | V-02，Counter与Notes真实ELF/后端/资产、失败保旧产物与来源记录 | AC-02、AC-07、AC-09 |
| [ ] | T-05 | work | T-03、T-04 | UI：rqhost/输入/窗口/renderer及必要通用Notes编辑面；正确Wayland焦点、键盘/IME/resize及rq初始化，不重做产品 | 两模式真实输入，覆盖诊断不降级；V-05/06输入边界对照 | AC-04、AC-05 |
| [ ] | T-06 | work | T-05 | OS/UI Counter：新增Linux准入驱动，真实例的两模式计数、身份、关闭重开 | V-05逐值与进程证据；故意选错模式/旧ELF时驱动失败 | AC-04、AC-06 |
| [ ] | T-07 | work | T-05 | Notes独立clone：tests/basic/verify_standalone.py或同源新增Linux Rust驱动；数据根/端口/POSIX进程/后端路径；真实保存/失败/重开，无业务重写 | V-06/07/08两模式与同库跨模式；产品既有相关门保持，不能复用VM-only结论 | AC-05、AC-06、AC-09 |
| [ ] | T-08 | work | T-06、T-07 | OS/UI/Notes：两app同宿主、并发孵化、关其一/末窗/pending、host与backend故障恢复 | V-09/10与自有PID/端点/段终态，旁观实例对照存活 | AC-06 |
| [ ] | T-09 | work | T-04；交付依赖T-08 | distro：基于已确认M1构建源新增packages/与flake输出、固定源码vendor/lock和launchers/desktop assets；原ISO输出不变 | V-11，真实sandbox源构建三包及闭包检查，错误候选不替旧有效产物 | AC-07 |
| [ ] | T-10 | work | T-08、T-09 | distro/OS/Notes：干净NixOS测试用户仅装包，重复四格、数据恢复、菜单/CLI、卸载重装及无开发源运行 | V-12加V-05～V-10安装态必需行为；无源码/Cargo/npm/外网运行依赖 | AC-08 |
| [ ] | T-11 | work | T-10 | 全owner：定向Windows/旧门与必要Lang固定组；规范增量草稿、版本证据、各仓README反链与准入摘要 | V-13/14；work AC-01～AC-10完整，提交冻结后交review | AC-09、AC-10 |
| [ ] | T-12 | review | T-11 | 独立上下文在相同冻结实施位置复核所有源/包/实窗/负控/Spec；缺口返回原树修复 | AC/V逐项判定、reviewed commits/dependency pins绑定；不得只读勾选 | AC-11 |
| [ ] | T-13 | merge | T-12 pass | 按实际依赖拓扑落地（必要Lang/Web→UI→Man/Notes→distro/OS）；按app工作流只更新本计划必要Notes已交付pin；回填准入范围；归档与guard后清理本计划临时材料 | frozen定向门、来源/恢复摘要、canonical核对；原M1资产和他人WIP不动 | AC-12 |

T-09 可以在T-04后准备打包，但源码开发态四格及生命周期未过时不能声称交付安装态就绪。
若本计划产生 Notes 文档/测试/代码交付，T-13 遵守 app 工作流：Notes main 验证提交并 push 后，仅在干净无人写入的 apps/auto-notes fetch、detached 到审定提交，验证/提交父仓 gitlink；与 054 等在途候选有交叉时先核源码包含关系与组合身份，不能倒退别案已交付功能。父仓落地前执行 scripts/check-app-submodules.ps1 或同名 .sh；不批量更新其他 app。
T-11完成后为execution_done；T-12/13保持真正待办，不在work阶段伪造完成。

## 9. 复审记录

### 2026-11-05 / new 契约预检

- stage: new；plan_id: PLAN-062；plan_revision: 1；outcome: pass；next: work/T-01（待进入执行阶段）。
- 范围已由用户明确为建立此计划；运行/修复的实施契约已列出，本轮未执行任何实现、编译、平台测试或包安装。
- 承接AutoOS已批准四阶段路线及归档PLAN-052，未复用其已完成状态冒充平台能力。
- 现有源码/Spec观察支持列出的缺口；设计不确定性由T-02有界探针和停止条件承接；M1已安装状态由T-01只读核实承接。
- 每项AC均有owner/task/case，SD-01～SD-06由T-11准备并由T-12核验；当前测试入口与拟新增入口明确区分。
- 未指定模型或自动继续预算；不新增其他主计划/代理/聊天，不改写旧证据和已发布标签。

## 10. 待澄清事项

| 项目 | 当前处理 / owner | 对本轮建计划影响 |
|---|---|---|
| 已有测试VM是否已完成硬盘安装 | T-01读现状；已装优先复用，不重装；未装交M1环境前置处理 | 不阻止列合同，不假称已验 |
| 当前在途工具修复与Notes写入者 | T-01核最新兼容交付和触面；禁止接管WIP或从未提交副本混构建 | 不阻止；冻结前不得依赖陈旧pin |
| Linux IPC/mmap具体实现 | T-02最多两案，优先保现有协议的UDS+私有映射；真实子进程探针后定案 | 已有bounded调查owner，后续实现不得猜 |
| Notes完整Rust-rq编辑面与中文IME | T-02/T-05必须闭合；若需重大新架构具名needs_replan，不擅自改成普通文本替身或独立窗 | 是必需门，不能静默延后 |
| 精确Nix源/vendor/动态库与字体集合 | T-01/02清点，T-09纯源码构建确认；不假定复制Windows exe即可 | 不阻止；runtime缺件不可算安装通过 |
| 后续发行版/应用扩面与自有桌面 | 沿M2/M3后续Plan，不在本准入门增加支持承诺 | 无 |
