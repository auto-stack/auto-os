# AutoOS 应用通信与 AutoAI 调用协议（v0.6 RFC）

状态：Draft RFC；日期：2026-10-04。用户已确认系统级应用通信/AI互调是v0.6重点。本文推荐技术分层，尚未冻结名称、wire格式、语言语法或具体后端；不宣称实现，不占用未同步的全局计划号。

## 1. 用户目标与现状证据

Auto语言作为应用表达与调用的统一入口，扩展task/msg，连接不同应用、系统服务、AI Agent与第三方应用。业务动作不应靠重复读取整个界面再模拟点击才能执行；外部Agent仍需要可兼容接入。系统DevTools消费同一协议。

本地证据（完整v0.5恢复后复核）：auto-lang `crates/auto-lang/src/vm/task_system.rs` 有TaskRegistry/TaskHandle/TaskInstance，以tokio mpsc与进程内Value传递actor消息；这不是已存在的跨进程TaskHandle。`docs/design/autoui/desktop-protocol-v1.md` 的桌面协议涵盖shell/app事件，不等于通用应用服务协议。`docs/specs/auto-lang/mcp/design/dual-mcp-servers.md` 记录源码VM stdio MCP与应用进程内AutoUI HTTP MCP，职责/进程不同，不能仅合并两个server就完成统一系统通信。

Atom/Batom v2在auto-lang `docs/design/strategy/atom-batom-v2-design.md` 仍是草案；现有auto-atom/VM Value/Shell Batom不能当作已冻结v2编码。AutoOS现有Spec分散，当前没有覆盖本议题的稳定current-state模块；本文为新增设计，不覆盖既有Spec。

## 2. 分层决策：并非 actor / DDS / IPC / COM 四选一

| 层 | 推荐 | 为什么 |
|---|---|---|
| 语言/执行模型 | actor风格task/msg、typed request/reply、async任务 | 保留Auto现有表达与状态隔离；跨进程失败显式 |
| 能力模型 | 有身份和版本的Service/Capability/Method，类似COM的“接口可发现”思想 | 跨语言描述功能，不绑定COM ABI、Windows对象指针和线程模型 |
| 交互语义 | call、send、subscribe、stream、task/cancel/status | 覆盖短请求、事件、长时任务和AI流式结果 |
| 数据模型 | Schema约束的Atom值/record/variant与Entity/AssetRef | 同一接口可以生成Auto/Rust/C等绑定；不把执行器Value当wire对象 |
| 数据编码 | 首个可调试profile；JSON投影用于接入，Batom后续协商 | RPC语义与序列化是两回事；大数/bytes等不能丢类型 |
| 本地transport | Windows named pipe / Linux Unix domain socket候选；Web gateway；具体经探针选择 | IPC是搬运层，可替换，不决定actor或RPC |
| 分布式/ROS2 | RMW/DDS等桥接provider | QoS/发现用于真正有分布式需要的场景；不迫使每个本地便签应用引入DDS |

ROS2官方说明其RMW可接不同middleware，除DDS也有Zenoh；DDS不是ROS2全部语义的同义词。来源：[ROS2 middleware](https://docs.ros.org/en/ros2_documentation/kilted/Concepts/Intermediate/About-Different-Middleware-Vendors.html)。借鉴COM的接口思想不等于采用COM实现，参考：[Microsoft COM](https://learn.microsoft.com/en-us/windows/win32/com/the-component-object-model)。

## 3. v0.6 建议拓扑

```text
Auto程序 / AI Agent / 人用DevTools
        ↓ 同一Service/Capability接口
  AutoOS session broker（发现、会话、授权、路由、trace）
       ↙                    ↓                  ↘
应用进程adapter        系统服务adapter       外部桥接adapter
   ↓ task/msg           ↓ install/资产等      ↓ MCP / ROS2 / Qt
```

先用用户会话级broker协调发现和控制面。broker退出或应用崩溃应显示明确状态；重新连接不假定旧对象仍活着。是否直接peer传输大流量，v0.7再由测量决定；首版不建自定义分布式服务网格。broker不要求整个AutoOS虚拟桌面UI运行，独立Windows Launcher可使用同一用户会话服务。

app_id来自安装/注册身份；instance_id每次进程启动改变；endpoint_id和service_id分别识别连接与服务。远程引用包括目标instance generation和接口版本，失效返回stale_instance，不能把运行时地址/函数指针序列化。TaskRegistry内部handle保留进程内语义；RemoteTaskRef是不同类型，跨边界没有共享内存可变状态的承诺。

首个broker提供schema/capability注册、查询、调用路由及trace；不默认导出本机全部进程，也不为未知程序生成不存在的业务API。没有接入协议的应用只能由后续accessibility/窗口adapter提供有限能力。

## 4. 公共协议与可靠性

Envelope候选：`protocol_version, session_id, request_id, trace_id, target(app_id,instance_id,service_id), method_id, schema_id/version, deadline, body, permission_context`。Reply：`request_id, status, result/error, task_ref?, artifact_refs?, state_revision?`。StreamChunk：`stream_id, seq, kind, payload/ref, final`。Event：`topic, publisher_instance, event_seq, body, schema_version`。

| 模式 | 明确语义 |
|---|---|
| call | 有deadline的request/reply；失败分 transport、schema、permission、domain；请求已送达不等于业务完成 |
| send | 单向消息；可区分accepted与业务receipt，不承诺无回执可靠执行 |
| subscribe | 显式topic、过滤与队列策略；slow consumer有背压/丢弃计数；不能无界积压 |
| stream | 顺序seq、取消、完成标记、背压；断线与取消分开，不擅自断线重跑生成任务 |
| task | accepted/running/completed/failed/cancel_requested/cancelled；取消是否及时/可恢复由能力描述声明 |

mutation的重试采用request_id+服务端幂等收据，约束保留期；不能只把网络响应缓存一下就承诺exactly-once。超时/断线后的结果可能unknown，客户端先查status/receipt，禁止无条件重放安装/发布/保存。read可按声明重试；业务函数是否pure/idempotent由服务声明并测试，不从名称猜测。

同一目标的排序、多个sender的并发、batch事务、事件重放和持久性单独声明；不把进程内actor队列顺序自动推广到跨机器全局顺序。服务需要序列执行还是并发执行由service/task定义；可能重入和死锁的同步循环调用需trace检测、deadline与明确调度策略。

大图片/视频/VTree全文作为Asset/ArtifactRef按需读取，不能每次调用都base64重发。共享内存/GPU句柄必须是受控transport专有能力，不是普通Atom可持久化标量。首版建议中等数据用chunk，后续零拷贝必须含寿命/释放/租约规范。

## 5. Atom、Batom 与 RPC 的关系

Atom是可交换数据模型；Batom是编码；RPC是请求/应答等调用语义。三者共同使用，而非选择“JSON还是RPC”。Auto普通值类型通过显式Schema/生成codec映射，拒绝闭包、裸指针、活体句柄等不可序列化内容。

首轮先定义消息语义、service schema及少量golden样例；用已有可验证编码提供实现。JSON兼容profile给bytes/宽整数/variant/ref等定义保真标签或拒绝超出profile的类型，不能默转浮点/null。Atom v2稳定后添加Batom negotiation，协议版本、接口版本与codec版本独立。双方无公共codec明确报错，不能靠“读得差不多”继续执行。

Auto语言调用面建议生成typed service binding，让调用者使用普通类型与await/task流；第一版不修改语法/parser。以下是语义伪码，非已可执行语法：

```text
notes = discover(service="auto.notes.capture", version=1)
receipt = await notes.capture(envelope, request_id)
task = await installer.install(package_ref)
subscribe(task.progress)
cancel(task)
```

## 6. AutoAI：共同能力协议上的 Agent profile

AutoAI暂作工作名，定位是面向Agent的调用profile/SDK，不是大模型推理或Agent间自由聊天的单一协议。AI Agent与普通应用调用同一业务能力；自然语言意图先编排成有类型动作，默认不把任意Auto代码当远程执行请求。

每个能力除Schema还描述：用途、输入输出、effect(read/write/external等)、幂等/取消、授权范围、可能错误、运行状态、结果artifact与来源。能力描述首次发现/版本变化时传递，后续用短method_id；支持projection、分页、子树、增量与缓存，避免反复输送全树/全部tool描述。复杂任务可提交有限steps DAG，逐步收据、状态条件和trace；batch不自动有全局事务或失败回滚。

优先调用业务能力 `notes.capture`、`reader.open_at`、`blog.export`；未知/仅界面操作场景才走DevTools action。Agent可读摘要和定位后按需取证，结果包含可验证状态而不只返回一段“成功了”。保存、安装、外部发布等effect按既有用户授权/系统会话策略处理，同一协议也供人用UI调用。

要减少Agent开销，重点是合理能力粒度、稳定引用、增量查询、结果投影、复用会话和任务管理；Batom缩小传输字节不自动缩小LLM token。外部模型仍可能通过JSON tool调用/文本视图接入，须测完整流程的token、轮数、延迟和失败率。

MCP采用JSON-RPC并定义stdio/Streamable HTTP，也允许custom transport；不能把MCP笼统说成HTTP-only或先天不能复用会话。[MCP transports](https://modelcontextprotocol.io/specification/2025-11-25/basic/transports)。Auto原生应用之间可用新协议取代逐次MCP/CLI胶水；对外保留MCP/CDP/CLI适配，CLI也复用同一业务SDK，不断掉已有Agent工具。首版MCP桥只映射明确能力子集，并校验错误/取消/结果语义不丢失。

权限绑定用户会话和服务范围；诊断读取、UI动作、代码求值分别授权。输入网页/模型文本不能自己提升调用权限；本机socket存在不等于调用已授权。此边界是协议正确运行的必要部分，不是要求所有操作弹窗。

## 7. v0.6 范围与实验

必需最小交付：本机会话注册/发现；typed call与receipt；事件订阅；长时task/取消/状态；Auto调用binding与一条非Auto接入路径；Notes捕获+Launcher调用；DevTools复用；MCP兼容桥。实际发布范围由独立计划固定，不要求28app全改造。

实验：用10–100个capability、文本/嵌套值/100KiB artifact引用、并发/背压、断线重连、实例更替、取消/未知结果、codec兼容等fixture；与当前MCP/CLI工作流在同机同数据测latency p50/p95、字节、Agent token/轮数、CPU/内存和成功率。首版没有测量前不承诺比所有方案快。

DDS bridge、分布式发现、远程多用户、共享内存、第三方SDK广覆盖为v0.7+。先把语义和服务注册做成独立新模块/fixture；不趁主力机失联直接改VM task调度或旧桌面协议。

## 8. 分解与后续决策

顺序：协议/错误/codec golden契约 → isolated broker与两个测试进程 → Auto binding和一项真实capture → 流/task/MCP bridge → 系统DevTools接入。每步另建独立计划/worktree，恢复主力机后正式取号。

待探针决定：broker宿主/进程生命周期、Windows/Linux IPC实现、Web gateway、Schema与生成器、Batom首个profile、权限/receipt存储。先做设计和小原型，避免现在修改旧core带来v0.5合并压力。关联：[DevTools](system-devtools-v0.6.md)、[基础设施分期](v0.6-foundations-roadmap.md)。
