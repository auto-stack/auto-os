# Notes FeaturePack v0 系统契约（featurepack-v0）

状态：**active**（PLAN-067 SD-01 交付定形；原 proposed @PLAN-065）。冻结首个
FeaturePack 的产品契约：包描述与版本、发现/启动/就绪、调用身份与授权边界、
领域样例与接线——全部按 PLAN-067 实测后 as-built 沉淀（生产启动/授权/验证
覆盖见 §3/§8）。子计划 [NOTES-006](../../../../auto-notes/docs/plans/archived/006-featurepack-service-and-editor.md)（archived
delivered）/ [READER-013](../../../../auto-reader/docs/plans/archived/013-notes-featurepack-host.md)
（archived delivered）；桌面 daemon 宿主托管合同见 auto-ui
`docs/specs/runtime/featurepack-daemon-hosting.md`（PLAN-083）。
既有边界：`broker-session` / `local-profile-v1` / `capture-inbox-probe`
（实验探针身份，不作为产品签收）。

## 1. 定位与范围

v0 = Windows 单机、构建时固定版本 UI 包、宿主负责渲染、服务按需启动。
**不在 v0**：动态加载、跨进程 UI、AutoLens、图片/公式、词典/AI、知识库加工、
ACE/AGE 切换、桌面多视口嵌入（Reader 结构性待桌面能力项，见 §6.4）。

角色与所有权：AutoOS 拥有发现、启动、调用身份、版本拒绝、受管 policy 签发
（§2.4）与真实双应用联合验收；AutoUI 拥有公开的组件参数、事件和实例生命周期
与 daemon 宿主运行时；Notes 拥有服务、组件和持久笔记；Reader 拥有出版物和
定位解释权。

## 2. FeaturePack 包契约

### 2.1 包描述符（冻结字段表）

| 字段 | v0 值（notes-fpack） | 说明 |
|---|---|---|
| `pack_id` | `notes-fpack` | 包稳定 ID；桌面注册与授权书引用键 |
| `version` | `0.1.0` | 包版本；语义化，破坏性变更升主版本 |
| `host` | `notes`（auto-notes，front 17818 / back 17819） | 主应用（服务与组件所有权方） |
| `back.interface` | `notes.fpack` | 服务接口名（方法 schema 属 NOTES-006 spec） |
| `back.version` | `notes.fpack/1` | 接口版本串——调用信封必带，见 §2.2 |
| `back.port` | 17819（notes `back_port` = manifest `daemon.port`） | 服务 HTTP 端口 |
| `ui.package` | `notesfp`（path-only dep：`../auto-notes/src/front/components`） | UI 包 ID；构建时固定版本 |
| `ui.exports` | `FpNoteEditor`（NOTES-006 交付；fp_editor） | 导出组件名表 |
| `permissions.operations` | `capture` `read` `list_by_book` `update` `receipt` | 允许操作（服务按授权书裁剪） |
| `permissions.scopes` | `book:<book_id>` 列表（**精确匹配，无通配**；空 book_id 拒绝） | 数据作用域单位 |
| `config` | 服务私有目录（`%APPDATA%/auto-notes/data`）、SQLite（inbox.db）、去重/回执参数 | 值域属 NOTES-006 spec |

### 2.2 版本与拒绝（冻结）

- 调用信封必带 `interface_version`；非 `notes.fpack/1` → 业务执行前
  `unsupported_version` 拒绝，无写入。
- UI 包版本**构建时固定**：宿主以 `dep <pkg> { path: ... }` 声明（path-only，
  version/pin/rev 键非法）；运行时不做组件热替换。
- 服务版本就绪核验（**生产合同**）：`GET /api/health` 2xx 且 body
  `interface` 逐字等于 `notes.fpack/1`（允许 ≤2 层 JSON 字符串信封——Notes
  双层编码形态）且 `status`（若存在）为 `ok` 才就绪。生产裁决由 manifest
  `daemon.health_interface` 声明驱动（auto-ui 严格健康合同：`Ready` /
  `NoListener` / `OccupiedElsewhere` / `WrongInterface` / `NotReady` /
  `BadBody` 具名裁决；错版占用者**拒复用、不 spawn、不投递配置**、占用者
  存活）。未声明 `health_interface` 的 daemon 保持旧 2xx 前缀语义（零回归面）。

### 2.3 授权书（caller authorization letter——v0 as-built 形态）

v0 服务侧 policy 文件为**唯一授权事实源**，形态：

```json
{"entries": [{"cred": "<256-bit hex>", "principal": "fp-reader",
  "operations": ["capture","read","update","receipt","list_by_book"],
  "book_scopes": ["book:<id>"]}]}
```

- 请求自报的 caller/principal **不可信**——主体只来自 policy 绑定；PLAN-065
  草案的 `caller`/`issued_by`/`epoch` 字段 **v0 未实现**（epoch 换绑语义由
  「文件整体替换 + 服务重启」承担；服务重启后合法主体对**原已提交操作**的
  receipt 查询不受影响——实测跨重启 committed）。
- 校验序（业务执行前）：凭据 → principal 命中 → operation ∈ 授权表 →
  book_id ∈ book_scopes → 接口版本 → source 闭集 schema。任一拒绝 →
  `permission_denied` / `unsupported_version` / `invalid_request`，无写入。
- **书籍范围授予/撤销**（OS 管理面）：`scripts/fp_provision_policy.{sh,ps1}`
  `add-book`/`revoke-book BOOK_ID`（幂等）；授予/撤销即时生效（policy 文件
  每请求读取），撤销后该书 capture 拒绝、**已提交操作 receipt 仍可查**。

### 2.4 可信身份传入后端（冻结——v0 as-built 通道）

- 凭据为 256-bit hex 文本（64 hex 字符）；**请求信封字段 `cred`**（每次调用；
  .at API 面无 header 通道，单用户边界下信封字段与头等价）。凭据不出现在
  公开 manifest、命令行、日志与回执（供给脚本默认输出仅含 sha256:12 指纹，
  `--reveal-cred` 显式私有传递窗）。
- **受管 policy 位置**（服务私有，固定）：`%APPDATA%/auto-notes/data/
  fp-policy.json`（`NOTES_FP_POLICY` env 覆盖 / `NOTES_DATA_DIR` 即数据目录
  本体；文件缺席 = deny-all，FeaturePack 关闭态）。policy 文件 **UTF-8 无
  BOM**（运行时 serde_json 与服务 fs_read 均不容忍 BOM；供给脚本双侧强制
  无 BOM 写 + 历史文件归一）。
- **传递通道（生产）**：manifest `daemon.host.credential` 受管引用（env 键
  `NOTES_FP_CRED` + path + principal + field）——auto-ui 运行时在宿主后端
  **独立子进程 spawn 时**注入（`apply_host_env`：先剔除继承同名键），凭据
  **不写宿主进程全局**；in-proc 后端拿不到凭据（`Env.get` 进程全局接口缺口，
  具名诊断，非静默降级）。地址键 `015_NOTES_DAEMON`（`daemon_env_key(manifest
  id)` 派生）同通道投递；`NOTES_DAEMON` 保留为测试/独立直连覆盖键（Reader
  消费优先级：`015_NOTES_DAEMON` > `NOTES_DAEMON` > 缺省端口，键序实证：
  015 指死端口 → 调用 500 且 Notes 零接触）。
- **签发 owner**：AutoOS 供给入口 `scripts/fp_provision_policy.{sh,ps1}`
  （fp-reader 条目创建 + add-book/revoke-book）；运行时只验证声明并精确
  传递，不生成凭据、不解释授权内容。

### 2.5 后端直连的授权边界（冻结）

- 绕过宿主进程、直接对服务端口（17819）的调用：仅当携带有效凭据且请求
  落在其授权书（操作+作用域）内才被接受——直连不豁免校验序。
- 无凭据 / 未知凭据 / 越权作用域 → `permission_denied`（业务前，无写入）。
  实测锚：空/错 cred 拒绝、被拒操作零回执（receipt `not_observed`）。
- v0 诚实边界（沿 broker-session 单用户立场）：同用户本地恶意代码不在
  防御模型内；独立进程不是沙箱；跨用户/网络面不在 v0。

## 3. 发现、启动与就绪（生产 as-built）

| 项 | 冻结语义 |
|---|---|
| manifest 声明 | `015-notes` 条目 `daemon { port: 17819, bin, env_port: AUTO_HTTP_PORT, health_interface: "notes.fpack/1", data_root: "%APPDATA%/auto-notes/data", host.credential {…} }`（schema 属 auto-ui 宿主托管合同 §2） |
| **bin 语义（T-02 定形）** | **路径语义**（相对 manifest repo 目录 + Windows `.exe` 补全）：`../../../auto-notes/target/release/notes-service`——**稳定命名发布件**（生成件名含路径哈希 id8 随检出路径变，不能进 manifest；对齐 musk/jade-garden 先例）。发布入口 `scripts/build_notes_service.{sh,ps1}`：auto service 生成链构建 → 稳定名复制 → 绑定回执（**成员名取自本构建日志 + 本缓存根唯一定位 + 零跨根回退**；构建失败即发布失败）。发布开关 `AUTO_ALLOW_PARTIAL_DB=1`（asset_upload=NOTES-002/PLAN-730 混合态，notes 仓自有 battery 同款接受面；fp v1 端点全部 db.rs 匹配） |
| 端口注入 | spawn env 仅 `env_port` 键（`AUTO_HTTP_PORT=<port>`）——生成后端端口解析链 `AUTO_HTTP_SERVICE_JSON` > `AUTO_HTTP_PORT` > 8080；**整链跟随 manifest 端口权威**（17919 变体实测零 17819 回退） |
| 桌面启动发现 | pac `daemon: "015-notes"` 声明（notes/reader 双侧）→ 桌面 boot 启动器：**共享 provider 判定**（back_port == daemon.port）→ Shared 臂经跨进程门 ensure（取锁→锁内二次探测→spawn）→ 路由到已核验身份的 daemon（不起第二份后端）；**非共享消费者**（如 reader）按 pac 声明名查表（DEBT-007 修复），宿主 env（地址+受管凭据）在其后端子进程 spawn 时投递 |
| 单实例 | 跨进程门（auto-ui PLAN-083）：身份 = user + instance_key + port + 规范化 data_root；锁文件独占 + 锁内二次探测 → 仅无监听才 spawn；实例标记按端口定位。两独立真实宿主并发冷启 → 恰一 LISTENING（实测）；错端口占用不杀占用者 |
| 退出恢复 | v0 合同：**恢复动作 = 重开 App 窗口**（launch → ensure 重入 → 服务恢复；原已提交操作回执跨重启持久可查——实测 committed 跨 respawn）。**named 局限（2026-11-16）**：宿主进程死亡会连带终止其 spawn 的共享 daemon（观测复现；机制=宿主进程线系，daemon 非 proc_host owned 但仍被捎杀，机制判定 owner=auto-ui 运行时）——影响多桌面场景「关启动宿主后另宿主可调用」；单桌面常规路径数据持久不受影响。会话内自动重 ensure 不在 v0 |
| 就绪探测 | 见 §2.2 生产合同；`ready_timeout_ms ∈ [200,60000]` 缺省 5s（生产档）；spawn 后同接口轮询，仅 `Ready` 结束 |
| 启动预算 | 生产 ensure 5s 档（发布件秒级就绪，实测 0.2s 冷启）；600s 窗口=开发 profile（首启含工具链构建，发布入口 `--wait-secs` 承载） |
| 调用超时 | 请求信封 `timeout_ms ∈ [1, 5000]` 缺省 2000；v0 服务内联执行，超时由 HTTP 客户端截止承担 |
| 重试身份 | `attempt_id`（每次尝试新）/ `operation_id`（跨重试稳定）分离——同 (principal, operation_id) 同 payload 返回原结果（`replayed:true`）、异 payload `request_conflict`；服务被杀后同 op 重试 replayed + receipt committed（实测） |

## 4. UI 组件契约（交付态）

- 组件 `FpNoteEditor`（fp_editor，NOTES-006 交付；props/事件/生命周期沿
  auto-ui 组件实例合同）。Reader 以库形态 dep 消费（`dep notesfp { path }`，
  READER-013 T-02 落地）；Notes 主应用同源挂载（创建/编辑双输入；其 VM 条件
  输入按用户裁定边界单列）。
- 桌面 v1 嵌入约束（2026-11-15 实测）：Reader 路由工作区结构性不兼容桌面
  v1 单视口（`has router pages` 具名跳过）；notes 经 app 侧 vue gen 产出
  `gen/front/vue/src/lib/api.ts` glue 后可嵌入（Shared 臂已实测触发）。双端
  桌面内全链待桌面多视口/嵌入能力项（auto-ui 桌面路线）。

## 5. 领域样例（合法/拒绝——NOTES-006 r2 冻结闭集）

方法 schema 属 NOTES-006 spec；本节冻结 OS 层信封与授权样例。服务基址
`<015_NOTES_DAEMON>`；全部请求带信封字段 `cred`（§2.4）与 `interface_version`。

**来源快照闭集（source_json，7 键）**：`book_id`、`source_hash`（真实内容
哈希，禁固定哨兵）、`title`、`author`、`quote`、`locator{kind,value}` 必需，
`chapter` 可选；长度界 1..=128 / 1..=256 / kind 1..=64 / value 1..=1024；
未知字段超计数拒（前向冲突显式拒绝）。**`book_version` 已退役**（固定 "v1"/
空 source_hash 均拒）。

**端点形态表（全部 POST + JSON body；生成服务器形态实测）**：

| 端点 | body 信封 | 返回（仓储信封） |
|---|---|---|
| `POST /api/fp/v1/capture` | `cred, interface_version, attempt_id, operation_id, source_json, text` | `ok, err_code, note{note_id,revision,title=quote,body=text,…}, receipt{status:committed}, replayed` |
| `POST /api/fp/v1/get` | `cred, interface_version, note_id` | `ok, note{…+book_id+source}, receipt(空占位)` |
| `POST /api/fp/v1/list` | `cred, interface_version, book_id, cursor(0=首页), limit(1..100)` | `ok, items[note…], next_cursor(翻页面才出现)` |
| `POST /api/fp/v1/update` | `cred, interface_version, attempt_id, note_id, expected_revision, text, operation_id` | `ok/revision_conflict(+current_revision)，仅 body 可变` |
| `POST /api/fp/v1/receipt` | `cred, interface_version, operation_id` | `status: committed(含原收据) \| not_observed` |
| `GET /api/health` | — | `{"status":"ok","interface":"notes.fpack/1"}`（JSON 字符串信封一层，就绪探针解包语义见 §2.2） |

**合法**——授权 `book:abc123` 捕获：

```json
POST /api/fp/v1/capture
{ "interface_version": "notes.fpack/1", "cred": "<256-bit hex>",
  "attempt_id": "<每次尝试新>", "operation_id": "<128-bit hex, 跨重试稳定>",
  "source_json": "{ \"book_id\": \"book:abc123\", \"source_hash\": \"<64 hex>\", \"title\": \"…\", \"author\": \"…\", \"quote\": \"原句原文…\", \"locator\": { \"kind\": \"epub.cfi\", \"value\": \"epubcfi(…)\" }, \"chapter\": 7 }",
  "text": "可编辑正文…" }
→ 200 { "ok": true, "note": { "note_id": "n-N", "revision": 1, … },
        "receipt": { "status": "committed" }, "replayed": false }
```

**拒绝**（全部业务前、无写入；均实测）：

| 样例 | 结果 |
|---|---|
| 无/未知 `cred` | `permission_denied`（被拒操作零回执） |
| 凭据有效但书不在 `book_scopes` / 授予后撤销 | `permission_denied`（原 receipt 仍可查 committed） |
| `interface_version: "notes.fpack/2"` | `unsupported_version` |
| 同 `operation_id` 不同 payload | `operation_conflict`（确定性无写） |
| `expected_revision` 过期 update | `revision_conflict`（+current_revision） |
| source 闭集违例（缺 source_hash/哨兵 "v1"/超长/未知字段） | `invalid_request`（具名 err_detail） |

Reader 代理面（`POST /api/fp/note/{capture,get,list,update}`，reader 5a6415a）：
客户端携 `book_id/quote/chapter/locator/text/para_ch/para_pi`——reader 自派
7 键 source（managed-book lookup + source_hash 真实哈希 + 空作者占位「未知」）；
TXT 段锚路径服务端**逐字复核 quote 在章正文**（夹具违例被正确拒绝实测），
`revision_conflict` 语义经代理同构。

领域正文字段保留原始 Unicode，不 trim / 不规范化；来源快照不可变，锚的
解释权在 Reader（locator = reader-locator canonical JSON，value ≤1024）。

## 6. 接线（as-built）

### 6.1 服务与发布（auto-os T-02/T-04）

- 服务：notes back `notes.fpack/1`（NOTES-006 交付；NOTES-007 代际）——
  发布件经 `scripts/build_notes_service.{sh,ps1}`（§3 bin 语义）落
  `<notes>/target/release/notes-service.exe`，回执绑定 notes HEAD/member/exe
  sha256。
- manifest：`015-notes` daemon 对象（§3 表；083 schema）。

### 6.2 Notes 主应用（NOTES-006/007）

- `pac.at daemon: "015-notes"`（6a7b0e7）；列表/导航自有 Store；编辑组件
  同源挂载；本地草稿 scratch 端点（`/api/fp/draft/*`，不走 policy 门）。
- 主应用 VM 条件 edit 输入按用户裁定边界单列（create 真实输入 + 状态面验收）。

### 6.3 Reader 宿主（READER-013，archived delivered @5a6415a）

- `pac.at daemon: "015-notes"` + 代理四端点（`/api/fp/note/*`）+ 键序消费
  （§2.4）+ 双层信封解码；真实 7 键来源（书目回查 + quote 复核）、每意图
  op/attempt、fp_host 状态机、Reader 本地草稿检查点（64KiB slots）、
  51 条分页、来源回访（locator/verify oracle）、持久高亮。
- 宿主侧验收（013 T-03 23/23 + T-08 用户裁定过点）：直接消费 OS 生产回执
  形态（受管凭据 + 发布件 + 仅生产键注入）。

### 6.4 已知边界（named）

- **桌面多视口/嵌入**：Reader 桌面内全链待 auto-ui 桌面能力项（§4）；
  桌面 boot 对 notes 已实测（Shared 臂），对 reader 结构性跳过。
- **宿主死亡捎杀共享 daemon**（§3 退出恢复行）：多桌面场景 named 局限，
  owner=auto-ui 运行时。
- **框架锁漂移**：PLAN-980 战役期框架主检出 Cargo.lock 与 manifest 失配
  （构建需工作树内归一化）——框架卫生债，非本合同面。

## 7. 验收映射（PLAN-067 实测闭合）

- **AC-01/AC-04**（daemon 级）：§3 全表 + §5 拒绝表实测——冷启/单实例/
  双宿主复用/错版占用具名拒/杀后恢复回执持久/非默认端口权威（真桌面
  gate 腿 + host 19/19 + V1/V2 探针 + combo 占用负控 exit3）。
- **AC-02**（双端产品流）：Reader 侧按 READER-013 T-08 边界消费（R-01
  完整链全绿 + R-01～03 全绿 + T-07 分页/回访/高亮）；桌面内全链待 §6.4
  能力项；Notes 主应用 VM 边界单列。
- **AC-03**（同组件同适配双端消费 + 无环）：fp_editor 双端消费（notes 挂载
  + reader notesfp dep）；依赖图无环（PLAN-067 §5 M1/M4）。
- **AC-05**：本规范 as-built 沉淀（本节 + §3/§8）；ACE/AGE 可复用输入 =
  闭集 schema 样例 + 拒绝表 + 生产注入形态。

## 8. 生产装配固定版本与验证覆盖（T-04 联合验收定形）

### 8.1 固定三角（2026-11-16 联合验收）

| 件 | 版本/SHA |
|---|---|
| notes 服务发布件 | sha256 f57d004d…（NOTES-007 代际，notes HEAD 2b47583，member `auto-notes-30db9907-back`，A-01 绑定回执） |
| reader | 5a6415a（013 delivered pin） |
| manifest（本仓 worktree） | apps.manifest daemon 对象 §3 表（68c8632+） |
| 框架 | auto-man 54fe4070 / auto-ui 15b9c4c6（⊇ DEBT-007 修复 c78208de）/ auto-lang 64ed621a3 / auto-web 0adebc0（锁归一化 caveat 见 §6.4） |

### 8.2 验证覆盖入口（全部实测在案，原始件 plan-artifacts/plan-067）

| 入口 | 覆盖 | 最新成绩 |
|---|---|---|
| `scripts/tests/verify_featurepack_host.py` | 服务级 ensure 形状（隔离数据/端口/自有 PID；H-01～05） | 19/19 |
| `scripts/tests/verify_notes_featurepack.py --profile vue\|vm` | API/代理组合（C-2 代理捕获→C-3 冲突语义→C-4 重启恢复；占用负控 exit3） | 11/11 ×3 profile |
| `probe_t02_v1/v2.py`（plan-artifacts） | 生产 spawn 形状契约（严格健康/授权面/数据根） | 11/11、9/9 |
| 真实桌面 gate 腿（desktop.sh + 断言） | G1 单实例冷启 / 双宿主复用 / G4 错版占用拒 / G5 杀后恢复回执持久 / G6 端口权威 | r8（6939bafa 代际）+ r10（f57d004d 代际）双代际全绿 |
| READER-013 侧（其仓在案） | T-03 23/23（生产输入形态 + 故障对照）；T-08 产品 UI（R-01 36/0/0、R-01～03 59/0/0） | archived delivered |

边界纪律：API/代理层证据不冒充 UI；必需 case 缺失/skip 非零（C-05）；
原始材料在 plan-artifacts，耐久摘要归 PLAN-067。
