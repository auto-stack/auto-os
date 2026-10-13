# 2026-10-04：应用规划与系统RFC文档复核

范围：4app的32份规划文档/12个草案计划、AutoOS通信与DevTools两RFC及共同分期、AutoLang roadmap v2。该复核允许文档落v0.6-dev，不是技术架构冻结或任何代码计划的实现验收。

复核限制：在同一会话另做一次基于实际Git提交与文件的检查，没有另一个agent/独立会话。没有把作者摘要作为运行证据；所有实施计划仍drafting，全部AC未执行。

## 提交基线

| 仓库 | 被复核commit | 结果 |
|---|---|---|
| auto-notes | `7ecfa5d0cb61c3be624bb43d8afabeb02c51e028` | 9个Markdown路径；diff --check通过 |
| auto-launcher | `2c23b8baca51043edafe5d324e4061e2cd51a52f` | 9个Markdown路径；diff --check通过 |
| auto-reader | `d43fa417df4621a65a606a096a6495647cbc24d6` | 9个Markdown路径；diff --check通过 |
| auto-blog | `6cfd7ebd9e25aa13e9a5df0451cd66c254c373d9` | 9个Markdown路径；diff --check通过 |
| auto-os | `86eff4696f89bd870ec1cab32e10c04bb49944cf` | 4个Markdown路径；diff --check通过 |
| auto-lang | `6bd64072ecd173ac8d7964f8c597f7580da05911` | 1个Markdown路径；diff --check通过 |

## 检查与结果

- 4个app各3个计划，filename为仓内001–003；frontmatter版本1/drafting，11个编号章节齐备，60个AC均未伪勾完成；计划Spec delta为提案而非已发布规范。
- 实际文件与准备文档hash对照通过；本轮所有相对链接、代码块闭合、UTF-8/LF和提交格式检查通过。计划目录原为空，取号独占锁创建后只删除本次锁，不占全局.next-id。
- 调研使用官方帮助/规范；Smartisan旧页面只说明该示例能力，不推断当前同步；手写区分设备与语言；没有引用竞品价格/全平台一致性或未测性能排名。
- Notes仓库数据库注释不能证明持久化；Reader占位章节与真实导入区分；Launcher独立mock不是Windows发现；Blog演示鉴权不作生产社区基础。这些差距已进入首批任务。
- 通信分层不把actor/IPC/RPC/Atom混为同一选择；JSON profile不得丢类型，Batom尚未冻结；可靠性含unknown、幂等收据、实例更替、背压和取消语义。
- DevTools复用现有活体VTree/MCP，并明确backend能力/第三方instrumentation限制；snapshot/frame/坐标与三类diff分开；未把Chrome说成没有协议/AI支持。
- 业务领域DTO保留独立本地闭环，注册/发现/trace等接公共层；不存在必须等待完整知识系统或新编译器才可记录真实数据的前置。
- 无运行代码/包版本/子模块指针改动；未运行cargo/应用功能测试，符合文档范围。所有性能指标是目标，非实测。
- 新增系统重点已写roadmap十条方向，但transport、编码、最低能力集与容量仍属建议，后续独立Plan决定；原主打目标未擅自删除或延期。

## 尚未解决的事项（后续实施前核定）

1. 各运行时存储与原子提交能力、Windows全局入口、EPUB/AutoDown双端能力由每计划T-00给证据；真实阻塞不以mock关闭。
2. service/schema/codec、broker生命周期、权限/receipt、DevTools最低字段及第三方样板仍需RFC讨论；不得直接称为稳定规范。
3. app当前无goal/模块Spec账本，frontmatter的目标引用/Spec路径是候选；实施合入时登记真实goal与实际Spec delta。未代替执行后独立复审。
4. 主力机v0.5隐藏增量须恢复后从source-sync三方吸收；目前不触碰旧核心调度/F12与四大app。

## 冻结的RFC内容hash

- `docs/design/strategy/auto-app-communication-v0.6.md`：SHA256 `afcba715b7ae69437784811216e850d1a3cdd65d7169f3df59044a81d3477ac8`。
- `docs/design/strategy/system-devtools-v0.6.md`：SHA256 `37eb30811d57edf71edf9f6adcd30b14478c22fcaad0f66391da2f1a3fcbe889`。
- `docs/design/strategy/v0.6-foundations-roadmap.md`：SHA256 `5704f2efff71a6d941efd41c8114cd632bbd3049f36b39a1575c24b550a54d86`。

结论：文档范围pass；适合提交/合入v0.6-dev继续讨论与分计划。应用代码计划未执行，不能因此把状态翻reviewed/archived。worktree与现有应用源码保留，未执行磁盘清理或UI重启。
