# Notes FeaturePack v0 系统契约（featurepack-v0）

状态：**proposed**（PLAN-065 SD-01）。冻结首个 FeaturePack 的产品契约：包
描述与版本、发现/启动/就绪、调用身份与授权边界、领域样例与 UI 组件接线。
依据 PLAN-065 §5；子计划 [NOTES-006](../../../../auto-notes/docs/plans/006-featurepack-service-and-editor.md) /
[READER-013](../../../../auto-reader/docs/plans/013-notes-featurepack-host.md)。
既有边界：`broker-session` / `local-profile-v1` / `capture-inbox-probe`
（**实验探针身份，不作为产品签收**）与桌面 daemon 启动链（apps.manifest，
AGENTS §3）。组件实例合同见 auto-ui
`docs/specs/runtime/component-and-store-instances.md`。

## 1. 定位与范围

v0 = Windows 单机、构建时固定版本 UI 包、宿主负责渲染、服务按需启动。
**不在 v0**：动态加载、跨进程 UI、AutoLens、图片/公式、词典/AI、知识库加工、
ACE/AGE 切换。

角色与所有权：AutoOS 拥有发现、启动、调用身份和版本拒绝；AutoUI 拥有公开的
组件参数、事件和实例生命周期；Notes 拥有服务、组件和持久笔记；Reader 拥有
出版物和定位解释权。

## 2. FeaturePack 包契约

### 2.1 包描述符（冻结字段表）

包 ID 声明主应用、后台接口及版本、UI 包及其导出组件、允许的操作与配置：

| 字段 | v0 值（notes-fpack 样例） | 说明 |
|---|---|---|
| `pack_id` | `notes-fpack` | 包稳定 ID；桌面注册与授权书引用键 |
| `version` | `0.1.0` | 包版本；语义化，破坏性变更升主版本 |
| `host` | `notes`（auto-notes，front 17818 / back 17819） | 主应用（服务与组件所有权方） |
| `back.interface` | `notes.fpack` | 服务接口名（方法 schema 属 NOTES-006 spec） |
| `back.version` | `notes.fpack/1` | 接口版本串——调用信封必带，见 §2.2 |
| `back.port` | 17819（notes `back_port`） | 服务 HTTP 端口（manifest daemon 对象同源） |
| `ui.package` | `fp-ui`（现探针形态 `fpv0` 0.1.0） | UI 包 ID；构建时固定版本 |
| `ui.exports` | `FpProbeCard`（探针）→ `NoteEditor`（NOTES-006 T-03 起） | 导出组件名表 |
| `permissions.operations` | `capture` `read` `list_by_book` `update` `receipt` | 允许操作（服务按授权书裁剪） |
| `permissions.scopes` | `book:<book_id>` 列表 | 数据作用域单位 |
| `config` | 服务私有目录、SQLite、去重/回执参数 | 值域属 NOTES-006 spec |

### 2.2 版本与拒绝（冻结）

- 调用信封必带 `interface_version`；非 `notes.fpack/1` → 业务执行前
  `unsupported_version` 拒绝，无写入（沿 local-profile-v1 版本协商轴的
  产品化最小面：协议/codec 轴由 HTTP+JSON 承担，接口轴显式校验）。
- UI 包版本**构建时固定**：宿主以 `dep <pkg> { path: ... }` 声明（path-only，
  version/pin/rev 键非法——blueprint contract Q5；跨仓对齐由 git 承担）；
  运行时不做组件热替换，动态加载留给后续计划。
- 服务版本就绪核验：`GET /api/health` 200 且 body `interface` 字段等于
  `notes.fpack/1` 才视为就绪（仅探活 200 不够）。

### 2.3 授权书（caller authorization letter，冻结字段表）

| 字段 | 说明 | 样例 |
|---|---|---|
| `caller` | 宿主应用 manifest id | `018-book-reader` |
| `principal` | 调用主体（服务侧绑定的可信身份） | `fp-reader` |
| `operations` | 允许操作 ⊆ 包 `permissions.operations` | `["capture","read","list_by_book"]` |
| `book_scopes` | 允许书目 | `["book:abc123"]` |
| `issued_by` | 签发方 | `autoos-desktop` |
| `epoch` | 签发批次（服务重启换绑不跨 epoch 复用） | 128-bit hex |

- Reader 仅访问授权书记录；主应用（notes）管理用户自己的笔记——**数据
  归属**：宿主读写按授权书裁剪，主应用全量。
- 校验序（业务执行前，沿 broker-session 校验序产品化）：身份 → 操作 ∈
  授权书 → 作用域 ∈ book_scopes → 接口版本 → schema/method 形状。任一
  拒绝 → `permission_denied` / `unsupported_version` / `schema_mismatch`，
  无写入、不触 provider 业务。

### 2.4 可信身份传入后端（冻结）

- 凭据为 256-bit hex 文本，**只经私有凭据文件传递**（每宿主一份，桌面
  ensure 链签发；per-user DACL，沿 broker-session 管道 DACL 语义）；凭据
  不出现在命令行、日志与回执。
- 传递：请求头 `X-FP-Credential: <256-bit hex>`（每次调用）；服务侧
  policy 将 凭据 → (principal, operations, book_scopes) 绑定（运行配置，
  服务私有目录）。body 自报 caller/principal **不可信**（沿
  local-profile-v1 身份来自凭证绑定）。
- 桌面 ensure 链把凭据文件路径经 `<NAME>_FP_CRED_FILE` env 注入宿主 App
  会话（与既有 `<NAME>_DAEMON=<url>` 注入同链同权）；独立启动见 §3。

### 2.5 后端直连的授权边界（冻结）

- 绕过宿主进程、直接对服务端口（17819）的调用：仅当携带有效凭据且请求
  落在其授权书（操作+作用域）内才被接受——直连不豁免校验序。
- 无凭据 / 未知凭据 / 越权作用域 → `permission_denied`（业务前，无写入）。
  该边界使「普通 Reader 进程伪造调用」不可绕过书籍授权（AC-01 反例锚）。
- v0 诚实边界（沿 broker-session 单用户立场）：同用户本地恶意代码不在
  防御模型内；独立进程不是沙箱；跨用户/网络面不在 v0。

## 3. 发现、启动与就绪

| 项 | 冻结语义 |
|---|---|
| 桌面启动发现 | apps.manifest 条目 `daemon { port: 17819, bin, env_port }`；ensure 链探活 `GET /api/health` → 未在线按 `bin` spawn → 退避重试；在线后注入 `NOTES_DAEMON=<url>` + `NOTES_FP_CRED_FILE=<path>` 到宿主 App 会话 |
| 独立启动发现（dev） | 服务以 notes `back_port` 直启；宿主以显式 env `NOTES_DAEMON` / `NOTES_FP_CRED_FILE` 指向（无桌面注入时的等价面）；两 env 缺失且服务不可达 → 宿主进降态（阅读/本地功能不受阻） |
| 单实例 | 同一服务实例对全部宿主服务；并发首次调用**合并启动**（ensure 幂等：探活竞争中只允许一个 spawn 赢家，输家复用赢家实例） |
| 退出恢复 | 主应用（notes 窗口）退出后服务**不随退**：daemon 生命周期独立，另一宿主仍能调用（冷启动场景：主应用关闭时从 Reader 冷启动服务，AC-01） |
| 就绪探测 | `GET /api/health`，超时 2s；200 + body `interface == notes.fpack/1` = 就绪 |
| 启动退避 | ensure 链既有探活→spawn→退避节奏；spawn 后最顺便绪等待窗口 30s，超时按服务不可达降态，不阻塞宿主 |
| 调用超时 | 请求信封 `timeout_ms ∈ [1, 5000]` 缺省 2000（沿 local-profile-v1 截止轴）；deadline 语义：转发未尝试 `not_dispatched`，已触业务 `may_have_run` |
| 重试身份 | **调用尝试 ID（每次尝试新）与保存操作 ID（跨重试稳定）分离**——重试携带同 operation_id + 新尝试 ID；服务按 (principal, operation_id) 去重幂等（沿 capture-inbox 去重键产品化），同 key 同 payload 返回原结果、同 key 异 payload `operation_conflict` |

## 4. UI 组件契约与 T-01 可行性结论

- 组件参数/事件/实例生命周期沿 auto-ui 组件实例合同（PLAN-041/045/057）：
  props 每帧重播种、子→父 emit 回调、卸载=帧末释放、重挂=全新 Init、
  迟到回调不重定向。组件代码由 Notes 发布（NOTES-006），宿主只消费。
- **T-01 实测结论（2026-11-09，fpv0 0.1.0 样例）**：跨工程组件导入机制
  成立——`dep fpv0 { path }` + `use fpv0.fp_probe: FpProbeCard` 在
  015-notes 与 018-reader 两个真实工程以同一挂载面完成参数/回调/卸载/
  重挂核对；同一最小双参数组件（`label: str` 数据参数 + `on_acked: msg`
  回调参数）行为一致。实测矩阵：notes Vue 9/9、notes VM 10/10（真实
  iced 窗 + UI MCP，PLAN-041 重挂=全新 Init 实证：子 taps 1→0、宿主计数
  保留）、reader Vue 9/9（vite 绕行面）、reader VM **blocked**（预先存在
  的 reader 后端装配失败 `library.at` 转译，stash 干净树复现，非组件
  归因；owner=reader 适配线，最小前置=reader main 通过当前工具链装配）。
- 引擎事实：449-3.1（VM 回调 props 整组件 fallback）已在当前引擎修复；
  449-3.2（组件子树快照不可见）已不再阻塞（子树全量入快照）；一次
  Acked 派发伴随 `handler not found` 单行后由回落送达（行为正确，记录
  为派发内部路径细节）。
- 可行性裁定：**无需为 v0 提独立 AutoUI 引擎子计划**；FeaturePack 组件
  合同在双端可用（reader VM 格随 reader 适配线解除后以同探针补测）。

## 5. 领域样例（合法/拒绝）

方法 schema 属 NOTES-006 spec；本节冻结 OS 层信封与授权样例。服务基址
`<NOTES_DAEMON>`；全部请求带头 `X-FP-Credential` 与 `interface_version`。

**合法**——Reader（授权 `book:abc123`）捕获：

```json
POST /fp/v1/capture
{ "interface_version": "notes.fpack/1", "caller": "018-book-reader",
  "attempt_id": "<128-bit hex, 每次尝试新>",
  "operation_id": "<128-bit hex, 跨重试稳定>",
  "timeout_ms": 2000,
  "body": { "source": { "book_id": "abc123", "book_version": "v3",
             "author": "…", "quote": "原句原文…", "chapter": 7,
             "locator": { "kind": "cfi", "v": 1, "value": "epubcfi(…)" } },
             "text": "可编辑正文…" } }
→ 200 { "note_id", "revision", "receipt": { "committed": true } }
```

**拒绝**（全部业务前、无写入）：

| 样例 | 结果 |
|---|---|
| 无 `X-FP-Credential` 直连 | `permission_denied`（§2.5） |
| 凭据有效但书不在 `book_scopes` | `permission_denied`（错作用域） |
| `operations` 未含 `update` 调 update | `permission_denied`（越操作） |
| `interface_version: "notes.fpack/2"` | `unsupported_version` |
| 同 `operation_id` 不同 payload | `operation_conflict`（确定性无写） |
| `expected_revision` 过期的 update | `revision_conflict`（保留用户输入，供重读比较再提交） |

领域正文字段（quote/text/title/author）保留原始 Unicode，不 trim / 不
规范化（沿 capture-inbox 语义产品化）；来源快照不可变，锚的解释权在
Reader。

## 6. 候选接线（候选——子计划落地时定稿，非冻结）

- **Reader 首宿主**（READER-013）：reading_page 真实选区 → 原句/CFI/
  上下文/book_id/source_hash 校验 → 挂载编辑组件（NOTES-006 T-03 发布的
  `NoteEditor`，探针形态同构）→ capture；回访事件 → locator 重验，失效
  显示原摘录+需重定位；服务不可达仍可阅读并恢复未提交草稿。
- **Notes 主应用**（NOTES-006）：列表/导航沿用自有 Store；编辑组件同源
  挂载（创建/编辑双输入、已保存/回访双事件）；登录用户全量视角。
- **服务**（NOTES-006 T-02 + OS-T-02）：notes back 扩展 `notes.fpack/1`
  入口；apps.manifest `015-notes` 行补 `daemon` 对象；单实例注册 + 退出
  恢复实测（scripts/tests/verify_featurepack_host.py）。

## 7. 验收映射

- AC-01（work）：§2.5 直连拒绝 + §3 冷启/单实例/退出恢复 + §5 拒绝表
  实测（verify_featurepack_host.py）。
- AC-02（work）：§6 接线落地后，双端 V-02（选句保存→改文→重开回访同一
  处），记录应用/包提交、服务身份、note_id、revision 与定位。
- AC-03（review）：同一实现版本独立复演 V-01/02，证据绑定三方版本。
- AC-04（merge）：按评审版本交付，核对本文与实测边界，归档回执。
