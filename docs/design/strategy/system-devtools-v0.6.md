# AutoOS 系统级 DevTools（v0.6 RFC）

状态：Draft RFC；日期：2026-10-04。用户提出把单app DevTools提升为系统协议/服务，由OS提供人用界面与Agent调用，各应用按能力接入。技术边界尚待探针确认。

## 1. 判断与现状

方向可行，且可成为AI版OS的基础：共同观察协议让不同backend使用同一检查、比较和操作工具；共同业务协议让Agent不必逐app学习一套MCP/CLI。目标是跨应用、跨渲染后端的诊断/自动化底座，不要求每个程序自己嵌一套F12面板。

Chrome已有分离的远程协议与Agent接入，不能把协议化/AI调用本身说成其缺失；AutoOS的新范围是统一多种native/web应用、系统任务和应用间因果关系。[Chrome DevTools for agents](https://developer.chrome.com/docs/devtools/agents/get-started/configuration)。

现有AutoLang Spec `docs/specs/auto-lang/mcp/design/dual-mcp-servers.md` 已记录活体VTree和截图双信道、scope/depth/字段开关、F12或MCP连接捕获门；这些不是从零重新发明。`crates/auto-lang/src/ui/mcp_types.rs` 有UiSnapshot/UiNode/action类型，`ui/mcp_server.rs` 提供工具路由。当前同名工具/协议不证明VM与Rust模式都有布局/state/source等同等能力。

现有降级规则是未测量字段可省略；新协议在此基础上增加capability、availability与原因，防止Agent把“未知bounds”当0或无差异。Musk等运行时测试中已有pointer合成编码Spec，adapter必须保留真实坐标/事件语义，不能另拼看似相同的整数/Double字段绕过原路径。

## 2. 服务与应用的责任

| 系统服务负责 | 应用/backend adapter负责 |
|---|---|
| target发现、会话、授权、能力协商 | target身份、活体数据采集、允许的动作和能力说明 |
| 人用Inspector、target/树/日志/任务/比较UI | 框架原始树到公共观察树的映射、快照一致性 |
| Agent SDK、增量/投影、trace和artifact管理 | UI线程调度、布局/输入/源代码等真实backend行为 |
| diff、记录、报告和可复现测试场景 | 不能实现的能力显式unsupported，限制与错误清楚 |

应用不必自行提供面板，但必须有instrumentation或OS accessibility adapter才能导出内部信息。系统无法从任意exe凭空读取Rust变量、Qt内部虚拟树或浏览器DOM。接入SDK与被动accessibility是两条路径，能力不同。

F12保留为便利入口：通知系统Inspector打开当前target；脱离完整AutoOS时可运行独立Inspector companion或轻量fallback，不能使独立app失去原本调试能力。统一的是协议与工具体验，不强求所有backend暴露全部内部数据。

## 3. 分层能力

| 层 | 能力 | 首版说明 |
|---|---|---|
| C0 | targets、capabilities、logs、tasks、trace | 不需要图形树的后台服务也能接入 |
| C1 | semantic_tree、inspect、actions、state projection | 标准角色/label/值/状态；定义是观察树，不强迫Qt实现Auto VTree |
| C2 | layout、computed style、screenshot、frame、hit-test | 明确逻辑/像素坐标、DPI、surface和frame；缺失字段unknown |
| C3 | runtime state、source mapping、performance、message trace | VM/Rust按真实能力提供，字段可筛选并脱敏 |
| C4 | breakpoints、step、variable inspection、runtime evaluate | 独立调试adapter；VM/编译Rust能力不对称，v0.6仅探针 |

capability有协议版本、操作集合、支持状态与限制；read-tree和press/type等动作分开。snapshot并非隐式授予mutation/debug/evaluate权限。能力不支持时客户端不展示伪按钮，Agent收到明确错误和可用替代方法。

Qt可经QAccessible接口提供角色/文本/几何及部分动作，不能等同完整QObject树、布局树或源码映射。官方说明：[QAccessibleInterface](https://doc.qt.io/qt-6/qaccessibleinterface.html)、[Qt accessibility](https://doc.qt.io/QT-6/accessible-qwidget.html)。Rust不是UI框架名称，要按iced/egui/GTK等adapter实现。Web通过DOM/accessibility/CDP等已有接口映射。

## 4. Snapshot 与树协议

TargetRef：`app_id, instance_id, target_id, generation, backend, surface_id`。Snapshot：`snapshot_id, tree_revision, frame_id?, timestamp, capabilities, coordinate_space, viewport, nodes, artifact_refs, completeness`。Node：`node_id, parent_id, role, label?, value?, states, actions, test_id?, source_ref?, layout?, style?, children`。

node_id只承诺target generation内身份；跨进程重启/不同backend用test_id/语义key匹配，而不拿局部数字相等当同组件。没有稳定键时可作role/label/path匹配，标置信与不确定，不能输出“精确差异”。虚拟化列表注明未实例化/折叠子树，不能把没导出的内容当删除。

布局字段至少有bounds、空间(screen/window/local)、单位(logical_px/device_px)、transform、DPI和clip；未测量是null+reason。截图单独artifact，可带所属frame/surface；布局、树、截图不同采样时间必须标明，不能把乱帧组合宣称像素一致。

get_snapshot支持scope/depth/fields/projection，get_delta使用base_revision；base不在缓存返回need_full_snapshot。events有seq，丢包标gap，不静默从不完整diff继续推状态。bulk state按用户选择字段导出，避免所有变量默认暴露。

动作：`target_ref, node_id, expected_tree_revision, action, args, request_id`；前置节点失效返回stale_target，促使重新定位。语义press/type不保证覆盖真实输入法/鼠标hit-test链，所以分别记录semantic-action与physical-input adapter能力；坐标测试必须取实际layout并走原运行时输入规则。结果包含receipt与可观察后置状态，不用一句“已点击”替代业务成功。

## 5. 三种 diff 与比较规则

1. 语义diff：结构、role、文本/值、可用动作、状态；先按test_id与角色匹配。不同产品没有共同测试场景时，只报告可比较项，不能宣布功能等价。
2. 布局/style diff：归一坐标/DPI，比较边界、间距、字体/主题、clip；指定容差、字体/渲染环境和缺失项。
3. 像素diff：对齐viewport/scale/theme、相同fixture/state及稳定frame后比较截图；抗锯齿/字体差异单列。VTree相等不代表画面相等，截图相似也不证明handler行为相同。

VM与Rust parity使用同fixture、相同动作序列、可复现初始状态及公共语义键；同时报告“不同”和“无法比较”。日志/trace时间轴把call→msg→state→render关联起来，方便定位一个跨应用动作在哪一步失败。系统diff组件复用现有verifier实践，不在每app维护单独截图工具。

## 6. Agent 与人使用同一服务

人用Inspector的目标列表、节点选择、日志筛选、任务跟踪、diff均调用公共服务；Agent可取小投影、订阅状态变化、等待具体条件、执行动作、获取证据并输出报告。这样“给人看的组件”和“给AI的接口”不会长成两套互不一致逻辑。

优先业务Service调用；UI动作服务用于体验测试、未知应用或需要真实交互路径的任务。inspect并不自动具备app业务API，业务调用也不替代UI回归。支持一步条件等待、批量取证及任务取消；录制动作是测试脚本/trace，不许把非幂等外部效果自动回放。

debug与runtime evaluate分开，默认记录状态/日志不会开代码执行。AI可按授权设置探针、查变量和追踪消息；C4执行/暂停能力需backend支持及开发会话允许。第三方进程权限遵守平台规则，不能因为OS面板出现便绕过用户会话隔离。

## 7. v0.6 最小可交付与远期

v0.6：统一target/capability/snapshot/action/error协议；一个系统Inspector；VM与a2r/Rust两个AutoUI adapter至少完成共同C1与已可用C2字段；Agent profile/MCP bridge；一组真实跨backend语义+布局+截图比较；一个Qt或accessibility adapter最小样板。第三方样板优先择已有应用/SDK可控范围，不要求全部Rust/Qt应用全功能。

完整source debugger、任意第三方自动接入、跨机器调试、系统级时间旅行/确定性回放、完整性能剖析为v0.7+。首版功能按共同子集验证，unsupported字段明确；不能用VM的深树去要求每个第三方绘图库暴露同样内部结构。

顺序：统一schema/fixture → 现有MCP数据adapter（兼容输出） → runtime snapshot边界/两backend能力探针 → 系统Inspector与Agent SDK → diff/动作证据 → 第三方最小adapter。只在前序跨应用通信最小层具备时接transport，不等待完整Batom/DDS/知识系统。

## 8. 验收案例与待定事项

案例A：两个AutoUI backend运行相同简单应用，取同状态快照、点击后等待条件，输出结构/布局/像素差异与缺失字段。案例B：一个Qt应用导出role/label/bounds和允许动作，系统面板展示其有限能力。案例C：Launcher调用Notes时在时间轴看到broker call、capture receipt与UI状态变化；断开target后旧node操作明确失败。

报告含代码/工具版本、fixture hash、frame/timestamp、viewport/DPI/字体、动作与截图artifact。测任务成功率、token/轮数与完整流程时延；不以协议包更小等同Agent更可靠。

待决定：Inspector是否使用独立系统组件仓、服务部署位置、SDK接口、对不同backend的最低C2字段集、第三方样板对象。先隔离做schema与fixture，恢复v0.5后分别正式取号；不此时改旧F12/renderer/桌面事件分发。关联：[通信协议](auto-app-communication-v0.6.md)、[基础设施分期](v0.6-foundations-roadmap.md)。
