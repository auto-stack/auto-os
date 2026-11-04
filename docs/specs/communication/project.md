# 通信模块总览（project）

状态：**proposed**（PLAN-049 SD-01，随交付归档生效）。本文件是 AutoOS 通用
应用通信的模块入口：目标、支持集、包图与非目标。

## 定位

AutoOS 应用间通信的首个支持集是 **Windows 本地 F0/F1 profile**：一个独立
broker 进程连接 caller 与 provider，提供身份/发现/路由/短调用截止与有界资源；
首个真实服务是文本捕获收件箱 `auto.capture.inbox`（capture.inbox/1）。

- wire 与语义单源：[Design02](../../design/02-local-app-communication-f01.md)
  （§3 会话/接入、§4 wire/状态、§5 首服务、§6 预算）。
- 支持集：`local-profile-v1`（wire 合同）、`broker-session`（会话合同）、
  `capture-inbox-probe`（首服务合同）——见同目录三个文档。
- 参考实现：`experimental/communication-f01/`（auto-os 仓内独立 Rust
  workspace；core=wire，local=transport/broker/client，demo=服务+bin）。

## 包图（参考实现依赖方向）

```
caller ──公开wire──> broker ──provider连接──> capture-service ──事务──> 专用SQLite
core(零IO) < local(transport/broker/client) < demo(capture/store/bins)
```

broker 不内置领域服务；provider 拥有数据库与领域去重；caller 拥有显式
发现与结果核对。三个真实 PID；演示/验证以独立只读读库为领域 oracle。

## 入口与非目标

- 入口：`scripts/verify-comm-049.ps1`（验证）、`scripts/demo-comm-049.ps1`
  （演示）、`experimental/communication-f01/`（实现与测试）。
- 非目标（F1 明确拒绝，不伪造支持）：任务取消/事件/流（F2）、Auto binding、
  Notes/Launcher adapter、Linux/远程/ROS2、通用 IDL、生产自启/安装、
  完整 OS 权限系统、机器人算法。扩展位置见 local-profile-v1 的版本规则。
