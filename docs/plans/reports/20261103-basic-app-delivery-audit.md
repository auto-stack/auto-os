# 原B类应用交付核对与下一轮安排

核对日期：2026-11-03。性质：源码/Plan/Spec/提交与远端快照核对，**不是本轮新执行的业务/GUI最终复审**。归档中的原始测试记录有适用版本和review上下文边界，不据历史绿总数宣布当前公共工具链已兼容全部应用。

## 实际状态

| 应用/计划 | 本地main | 远端main | 实际阶段 | 证据与下一步 |
|---|---|---|---|---|
| [NOTES-004](D:/autostack/auto-notes/docs/plans/archived/004-standalone-native-editing.md) | `5c9fa5d2f4eb5e44844200adfac0379a38d8bb63` | `5c9fa5d2f4eb5e44844200adfac0379a38d8bb63` | 已交付归档，8/8 | review 59226ed为HEAD祖先，src/tests/pac之后零差异；已有Vue31/31、原生19/19、VM反例11/11、重开后再保存记录。当前未重跑。 |
| [CALENDAR-002](D:/autostack/auto-calendar/docs/plans/archived/002-recovery-revision-and-deletion.md) | `0a1bce447b700f2bb8d9d6f9b3d45fb03c60a509` | `0a1bce447b700f2bb8d9d6f9b3d45fb03c60a509` | 已交付归档，8/8 | 修复复审1d30fa9为HEAD祖先，src/tests/pac之后零差异；高水位/墓碑/拒写保全与受控恢复已入Spec，保留灾难恢复残余边界。 |
| [PAINT-002](D:/autostack/auto-paint/docs/plans/archived/002-standalone-project-and-svg.md) | `4b6021324f116cd288c7d4fedc88b211294adc36` | `4b6021324f116cd288c7d4fedc88b211294adc36` | 已交付归档，7/7 | 最终复审fca907b与merge回执在档；VM50/Vue45/desktop31，跨重启与后端断开错误已覆盖，两个storage profile不自动同步。 |
| [CHAT-002](D:/autostack/auto-chat/docs/plans/archived/002-standalone-identity-and-isolation.md) | `db7638392ef8acb6c823d402726afa9c3e994309` | `db7638392ef8acb6c823d402726afa9c3e994309` | 有最终复检/交付归档；计数仍7/9 | 最终复检2dc7836为HEAD祖先，src/tests/pac之后零差异；identity45/45、Vue8/8、VM4/4有记录。T-07仍未勾/计数未对账，不能由本轮直接补成9/9。 |
| [CHARTS-002](D:/autostack/auto-charts/docs/plans/archived/002-standalone-unicode-and-decimal-charts.md) | `30436a7f44649dc05350d7649cd74c2525d6a200` | `30436a7f44649dc05350d7649cd74c2525d6a200` | 已交付归档，8/8 | 复审b6e6bcf为HEAD祖先，之后仅tests/basic/verify_basic.py有端口隔离修正；merge绑定最终统一门VM43/43、Vue14/14、probe零失败，保留非独立复审上下文局限。 |
| [TODO-002](D:/autostack/auto-todo/docs/plans/002-standalone-process-isolation.md) | `8a65e08295e8147f82b59f5ac89ec42ccffa16e9` | `8a65e08295e8147f82b59f5ac89ec42ccffa16e9` | 未实施，drafting/0/7 | 本地及远端仍8a65e08规划提交，worktree仅main；不能视为002完成，不创建003绕过。复用原002继续work→review→merge。 |
| [DATABASE-002](D:/autostack/auto-database/docs/plans/002-standalone-native-user-flow.md) | `1c2f24a5418a603a3c6305fb775e9eb8a0da1e04` | `74b6d03175d7d20d8982d4033ffdbe5426c62b40` | work完成，6/8；最终review/merge待做 | 最近receipt已记原生崩溃owner修复入auto-ui master 76b0340并重建共享tool；原生59/59×2与Vue55/55有记录。frontmatter仍drafting、README仍旧阻塞，状态与回执滞后。远端仅74b6d03规划tip，8个本地提交未推，不能算完整交付。 |
| [BLOG-005](D:/autostack/auto-blog/docs/plans/005-standalone-dev-startup.md) | `84a551412cc8220c37fa34bbc8d5117f4a7acde4` | `84a551412cc8220c37fa34bbc8d5117f4a7acde4` | 实施中/上游阻塞，3/8 | 普通vue-dev-rust门有dayjs CJS诚实红与pending；等待实际auto-ui-tools/src/vue.rs配置发射最小前置。已有preview/native绿不替代dev门；继续原005，不新编号跳过。 |

## 本轮结论与调度

1. **先关闭既有计划**：TODO-002从T-00实施；DATABASE-002在现有clone完成T-06独立最终复审/T-07归档与push；BLOG-005消费真实上游交付后关闭dev门、复审和归档。三者不以新编号替代，也不在本轮顺手推送尚待复审的Database实现历史。
2. **已交付应用转统一组合候选**：Notes/Calendar/Paint/Chat/Charts可进入只读版本盘点。Chat的旧计数/勾选及Notes旧README失效归档链接由app owner依据真实回执对账，不由OS伪造任务完成，不因此重开功能合同。
3. **后续统一集成**：[auto-os/PLAN-054](../054-basic-apps-unified-assembly.md)覆盖上述8个应用。T-00只读准备可独立进行；所有应用交付门和共用SDK兼容门满足后，才允许升级候选OS gitlink、做桌面用户任务，再交付组合。当前只起草，不执行。
4. **Reader排除**：按[规划 AutoReader 下一步](thread://01a1964c-8973-7421-beb3-062a3cf05f51?hostId=local)单独跟踪Readest新目标。该会话最新安排Reader-007，等待UI029真实交付，本轮不改其源码/设计/Plan/README/组合pin。
5. **Linux仍等待**：本地主检出 `E:/AutoOS/src/auto-os-distro/docs/plans/001-nixos-cosmic-iso.md`仍executing/r2、5/10，未见M1版本化验收回执。单独应用/Windows桌面集成不以Linux已支持作声明。

## AutoOS与共享工作区边界

AutoOS main当前`65fc70d`，本轮只新增核对报告、054合同与总表指针。已有053主题/计算器计划（执行中）及其.next-id预留、autos-desktop-program、设计/研究WIP不接管；054取号保留053，不把它的正文或WIP带入提交。

子模块只读健康检查：27个manifest repo条目、missing_pac_at=0；`apps/auto-charts`实际检出`d5f5535`与父仓gitlink不一致，和前次相同。**本轮未修复或更新子模块**。当前健康非绿不能成为054候选通过证据；054必须在独立候选组合中校正并验证完整注册集，不以只初始化8个目标导致其他app消失。

### 已核对的集成风险

- manifest中Calendar/Paint/Database/Charts仍只列旧前端端口，不能凭清单存在认为新后端会自动可用。
- 现iced桌面`session.launch_app → ensure_backend → back_provision.plan_backend`可默认把`#[api]`装载为VM proxy。Chat凭据依赖Rust secure_hex，不能为图标可启动就统一降为VM；Notes/Todo/Calendar/Chat/Blog/Database的Rust服务与Paint/Charts的VM HTTP分别验证。
- CLI/tool/desktop宿主需选定同一可重建兼容组合，不能只依赖各app过去不同日期的exe缓存。Vue桌面与iced桌面分别验，app单独运行的Vue/VM记录不能替代桌面内真实任务。
- 053的theme/auto-calc与054的8app组合互不增加整份计划依赖；实际共享文件/提交窗口须协调，054不修改theme业务，也不接管Reader UI029。

本轮未运行桌面、构建应用、迁移用户数据、创建worktree、修改app源码/Spec或写旧ledger；没有合入应用gitlink。
