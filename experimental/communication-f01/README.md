# autoos-comm-f01 — 本地应用通信 F0/F1 与文本捕获收件箱（PLAN-049）

独立 Rust 实验workspace：Windows 命名管道连接 **comm-broker / capture-inbox /
comm-client** 三个真实进程，验证发现→短调用→持久领域回执的最小闭环。
设计与执行合同：[Design02](../../docs/design/02-local-app-communication-f01.md)
· [PLAN-049](../../docs/plans/049-local-communication-f01.md)。
伞形归属：[auto-os](../../README.md)（`experimental/` 实验区，不进 apps.manifest，
不改桌面/产品接线）。

## 结构

```
core/   autoos-comm-core  — 有限JSON wire（手写严格解析器+allowed-key表）、DTO、
                           错误模型（含 execution 判定）、profile 限额。零IO。
local/  autoos-comm-local — 显式DACL命名管道（用户+SYSTEM、first-instance、
                           reject-remote）、有界帧IO、broker（注册表/路由/配额/
                           watchdog）、client、provider、Job对象进程遏制。
demo/   autoos-comm-demo  — capture.inbox/1 服务（专用SQLite WAL+FULL、生命周期
                           文件锁、(principal,operation_id) 去重、冲突/容量/
                           崩溃barrier）、三个bin、集成测试族。
```

依赖方向 core ← local ← demo；不依赖 auto-lang/UI/man/AI/renderer，无手写 Rust
以外的语言面。依赖版本以 Cargo.lock 为准（tokio/serde/rusqlite(bundled)/
windows-sys/sha2/getrandom）。

## 运行（演示，只展示已验证行为）

```powershell
pwsh -NoProfile -File scripts/demo-comm-049.ps1          # 50×1KiB 双caller负载
                                                          # + python只读对账 + 幂等重跑
```

## 测试（验证入口）

```powershell
pwsh -NoProfile -File scripts/verify-comm-049.ps1 -Scope acceptance
# scope ∈ admission|routing|receipt|lifecycle|resources|acceptance
# 或直接：cargo test --workspace --locked（在本目录）
```

测试族与设计反例对应：f0_contract（V-02/03/04）、frame_io（拆/粘/残帧）、
f1_route（V-04/06/07/08/12）、capture_receipt（V-09/10/11）、lifecycle
（T-01全路径门 + V-13）、resources（V-05/12）、acceptance（V-14 工作负载）。

## 边界（非目标）

不做：Auto binding/TaskRegistry 扩展、真实 Notes/Launcher 接线、F2 任务取消/
事件/流、跨平台（Linux UDS）、生产安装/自启、通用IDL、跨用户授权或同用户
恶意代码隔离。实验凭证不是安装app身份；capture 服务不是 Notes 产品。
