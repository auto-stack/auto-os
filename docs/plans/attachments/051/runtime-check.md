# Plan 051 合入后三仓 VM 实测（2026-10-28）

结论：新启动脚本确实完成了 Man CLI、UI 工具与语言仓桌面宿主的真实构建和启动。桌面内计算器，以及独立 `auto run -r vm` 计算器，均实测 `2+3=5`。独立启动首次暴露 UI 工具 Windows 主线程栈不足，已修复并复测；视频解析与媒体接口仍有未闭合问题。因此基础消费链路可用，不能据此宣称所有应用或完整拆分回归通过。

## 交付与范围

- Plan 051 定向回归 13/13；复审 pass。复审由本会话重新核验工件，未另开独立模型/会话，保留上下文独立性限制。
- OS `main` ff-only 交付提交 `3088dea1db735c294ec9a6fe22f5b3522ee82209`；Spec、ledger、归档和工作树 guard 清理完成，归档收尾 `f86dff3f98fa69499e062e9c99f0345637900fd3`。
- 本报告属于用户额外授权的合入后真实运行检查，不扩大已归档脚本合同，也不以脚本替身替代真实运行。
- 无远端 push；未运行 Cargo 整档、docs_gen 或 Vue/VM 像素对拍。框架构建存在原有 warning 输出，不作全仓零 warning 声明。

## 冻结输入和真实构建

| 仓 | 首次运行提交 | 最终检查提交 |
|---|---|---|
| auto-os | `f86dff3f98fa69499e062e9c99f0345637900fd3` | 同左（随后仅补本报告等簿记） |
| auto-lang | `df54eb58370fce2a036d7da8ff3e21f9944aa07d` | 同左 |
| auto-ui | `7095f536747e5180338a470a7d73452c6b550ed1` | `5a89bfd88a773b33a50c757c709e56a9e4a0bc73` |
| auto-man | `873fa234f301cf248c2335e97c392255f1e6adec` | 同左 |
| auto-down | `c3464d0a0d36e4f8a7988b0c0c26448fc52361b4` | 同左 |

依赖固定在 `D:/autostack/.wt/os-051/{auto-lang,auto-ui,auto-man,auto-down}` 同级 detached 工作树；未消费各主检出的未提交改动，未建立链接。UI 最终只增加工具包 build.rs 主栈策略，常规 Cargo 重建工具；原 CLI/桌面使用的库源码未变。

实际调用新脚本：

```powershell
$env:AUTOUI_ACCEPTANCE = '1'
$env:AUTOUI_MCP_PORT = '4331'
$env:AUTO_VM_STORAGE_FILE = 'D:/autostack/.verify/os-051-20261028/desktop-storage.json'
$env:CARGO_BUILD_JOBS = '4'
pwsh -NoProfile -File D:/autostack/auto-os/scripts/desktop.ps1 -Track iced `
  -LangRoot D:/autostack/.wt/os-051/auto-lang `
  -UiRoot D:/autostack/.wt/os-051/auto-ui `
  -ManRoot D:/autostack/.wt/os-051/auto-man `
  -BuildRoot D:/autostack/.verify/os-051-20261028/build
```

用户明确要求启动真实 GUI；辅助控制台隐藏启动。三次 Cargo 均使用 `--locked`，按包 owner 的工作目录读取配置，共用显式 target，均 exit 0：

| 构建 | 参数 | 首轮耗时 |
|---|---|---|
| Man | `-p auto --bin auto --features mpv-widget` | 3m33s |
| UI | `-p auto-ui-tools --bin auto-ui-tool --features auto-ui-runtime/mpv-widget` | 4m32s |
| 语言桌面 | `-p auto-lang --features ui-iced,mpv-widget --example ui_desktop` | 3m13s |

没有修改系统 PATH。产物位于专用 `build/target/debug/`，桌面在其 `examples/` 下；CLI 和工具邻接，`AUTO_UI_TOOL_BIN` 明确指向该工具。应用目录为 OS 的 `apps`，存储隔离至验证目录。

## 实际进程和操作

1. 新脚本启动真实桌面，MCP 4331 可响应，桌面背景、dock、窗口与组件可见。第一次过早截图遇到窗口尺寸尚未就绪，重试后截图成功。
2. 从桌面通过 `autoui_desktop` 的 `bus launch\t011-calculator` 启动注册表应用，状态可见 `8:auto-calc`；通过桌面真实 handler 路径操作 Clear、2、+、3、Equals。状态 `display="5"`、`expr_line="2+3 ="`、`error=""`，截图可见结果。没有注入计算结果。
3. 再从 `D:/autostack/auto-os/apps/auto-calc` 运行新 CLI 的 `auto.exe run -r vm`（同组根和工具，MCP 4181），验证 CLI→UI 工具→VM 子进程。修复后读取实际渲染按钮 ID，依次按 C、2、+、3、=，状态与截图同样为 5。
4. 使用仓库 autoui-verifier 标准 MCP 客户端；每次检查返回错误标识，不把接口返回成功当截图或应用行为证据。

2026-10-28 08:17 UTC 观察到的存活进程链：桌面 launcher 69752→`ui_desktop.exe` 61092→`auto.exe` rqhost 69148；独立 CLI 32640→`auto-ui-tool.exe` 52308→VM 工具子进程 62068。各 exe 路径均来自上述专用 target。保留桌面和计算器窗口及运行所需的冻结依赖树；这些运行树与已清理的 OS 实施工作树不是同一对象。

首次桌面宿主曾正常退出（exit 0），随后通过新脚本 warm rebuild 重启；没有证据将该次退出归因为崩溃或特定用户动作。

## 已闭合：UI 工具主线程栈溢出

首次独立 VM 启动真实失败：`thread 'main' ... has overflowed its stack`，内部进程退出 `0xc00000fd`。实际 PE header：Man 主栈 64 MiB，桌面 32 MiB，UI 工具只有 1 MiB。UI 的 `RUST_MIN_STACK` 只设置新建 Rust 线程，不能设置必须承载 winit 的 OS 主线程。

在 UI 工具 owner `crates/auto-ui-tools/build.rs` 增加 target OS/ABI 条件化链接策略，默认主栈 64 MiB，沿用 `AUTO_STACK_SIZE` / `auto_stack_size` 覆盖。MSVC 发 `/STACK`，GNU 发 `--stack`，其他 target 不发 Windows 参数。无启动脚本绕过。

修复提交 `5a89bfd88a773b33a50c757c709e56a9e4a0bc73` 已 ff-only 合入 auto-ui `master`，未 push。定向审查、rustfmt/diff check、四种输出控制（MSVC/GNU/Linux/覆盖值）通过；控制仅证明链接策略输出，不冒称完成 Linux/MinGW 实际编译。普通 Cargo 工具重建 19.21s、exit 0，PE 主栈实际变为 64 MiB；相同独立入口和应用随后渲染、运算成功。旧 1 MiB 工具副本仅本地保留用于对照。

## 未闭合：应用层观察

| ID | 现象 | 后续验收 |
|---|---|---|
| OS051-RUN-VIDEO | 桌面启动 `030-video-player`，`apps/auto-video/src/front/playlist.at:124:8` undefined variable，应用构建失败；现场为 `style: caption_text`，文件存在 stylekit import | 复现并定位导入/样式解析原因；视频可启动、真实媒体交互通过 |
| OS051-RUN-MEDIA | Music `/api/media/scan` 与 Photos `/api/photos/scan` 的 back proxy 响应 404 no-route；照片空集合不能视为扫描成功 | 核对应用后端路由及消费契约，实际扫描/展示有内容，异常语义明确 |

两项尚无旧组合对照，不能判定为迁移新回归或既有基线；也没有足够证据将视频错误直接归因为某条 import。已登记到 `docs/plans/KNOWN-DEBT-AND-RISKS.md`。其余应用、媒体播放、完整多窗口生命周期和发布包/resource 完整性未覆盖。

## 本地证据收据

根目录 `D:/autostack/.verify/os-051-20261028/`。原始日志/截图/JSON/二进制不入库，仅此 Markdown 摘要入库。完整输入、进程和内容 SHA-256 清单见该目录 `runtime-final-receipt.json`（08:17 UTC）；原始输入保留 `runtime-inputs.json`。持续输出日志另复制为 `receipt-*`，下表 hash 对应截取时副本，不声称 live 日志永不变化。

| 本地路径（相对证据根） | SHA-256 |
|---|---|
| `runtime-inputs.json` | `bd00d1685b567eced31e2f3199a482c44630e53f1f776cb2c20c66bcd749651e` |
| `desktop-calculator-result.png` | `7fc7f31a0b7b5b5e3570b49c24dd5c8559127bdf35557a6cc880de4e763df11d` |
| `standalone-calculator-result.png` | `5f92bff71ae33e69769f65eac11c8dc0d7da628b75a84caa96d5ab1996495f92` |
| `calculator-after.json` | `d10e8014fe8c1eef99daa44c856c3fee7675fc252c515a98a1690e557ce67826` |
| `standalone-fixed-result.txt` | `d7fa135db0f037968dc3e2a54cb432667a0eeebe6b849ff829a62fbb62d14fae` |
| `receipt-desktop.stderr.log` | `f01302c90d3448ca6c7279827e37d7ccbaa68c6874f3d069550d77a5e00cf5e9` |
| `receipt-desktop-restart.stderr.log` | `c9066eaeede4f10d6711a310179bfe6b801eee10d086606975eb310fed55066c` |
| `receipt-standalone.stderr.log` | `50fb4a7019a801b41f336d91b178a170d4d76686fc9a6df133d1865241f674b0` |
| `receipt-standalone-fixed.stderr.log` | `666469119d967a1900eac52c07c3a9b00f2b947c98a762cc81df478802fc11b1` |
| `build/target/debug/auto.exe` | `ce2f0db8e75f1b5a95d3c10775e821207f033f4f3353b29d326f9e7f103a9d90` |
| `build/target/debug/auto-ui-tool.exe` | `e8bee6aaf4bd6fd71d2a1465afa0203a85947c4431245123b52f8a1fd5a6cf7a` |
| `build/target/debug/examples/ui_desktop.exe` | `4b0102e3a99c44736e424f9b86abd468da56233c3663c031dec8057d94ee4abe` |

启动脚本 SHA-256 `eb6d0ac62a6c9828991ab47fe45e8ac7c9a390908017ed96161f6bd91254d12c`，与复审/交付版本一致。构建日志和 `stack-policy-controls.json` 的 hash 也在本地完整收据中。
