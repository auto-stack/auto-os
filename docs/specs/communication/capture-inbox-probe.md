# 文本捕获收件箱服务合同（capture-inbox-probe）

状态：**proposed**（PLAN-049 SD-04）。冻结首服务 `auto.capture.inbox`
（接口 `capture.inbox/1`）的领域合同：schema、去重/回执、事务/恢复与配额。
依据 Design02 §5；参考实现 `experimental/communication-f01/demo`（capture.rs/
store.rs）。**这是实验探针服务，不是 Notes 产品，不访问 Notes/Jade/Musk 文件。**

## 方法与 schema

- `capture`（schema `capture.inbox.capture/1`）：body `{operation_id,
  text, title?, source_uri?}`。operation_id 为 128-bit id 文本；text 1..=16384
  字节，保留原 Unicode/换行，不 trim/不规范化；title ≤256、source_uri ≤2048，
  仅作字符串元数据保存（服务不访问 URI/文件）。未知字段拒绝。
  缺省可选字段与显式空串同 digest 语义（冻结）；存储缺省记 NULL、显式空记 ""。
- `receipt`（schema `capture.inbox.receipt/1`）：body `{operation_id,
  principal}`。`principal` 必须等于 broker 绑定的可信 caller principal，
  否则 `permission_denied`（跨主体读是显式拒绝，不是静默 miss）。
  返回 `committed`（capture_id/原文/摘要/可选字段）或 `not_observed`
  （不证明在途请求永不提交）。
- 领域拒绝：`domain_error` + `domain_code=operation_conflict`（同 key 不同
  payload；无新增/无覆盖，确定性无写）。容量满（1024 条）新 key →
  `resource_exhausted` + `not_dispatched`；旧 key 的重试/查询在满库时仍可用。

## 去重与回执

- 去重键 = (可信 principal, operation_id)。digest = SHA-256，编码：
  `"f01-capture/1\0" + operation_id + \0 + u32le(title)+title + u32le(uri)+uri
  + u32le(text)+text`。capture_id = SHA-256(`"f01-capture-id/1\0" + principal
  + \0 + operation_id + \0 + digest_hex`) 前 32 hex——同 key+payload 跨进程/
  跨重启稳定同 id。
- note+digest+回执同一事务提交；成功回复在 COMMIT 之后。可重试只保证本实验
  数据库按 key 的一次逻辑插入，不推广为任意外部副作用 exactly-once。

## 存储与恢复

- 专用私有目录 + 专用 SQLite（服务独占）；启动取得生命周期 OS 文件锁
  （`inbox.lock`），第二 writer 在注册前失败退出；不用"超时后无锁继续写"。
- SQL 设置（T-01 冻结）：`journal_mode=WAL; synchronous=FULL;
  busy_timeout=2000`；主库 ≤32MiB（启动检查）。
- 崩溃模型（kill 进程级）：kill-before-commit → 无行无回执；
  commit-后-kill-before-reply → 回执持久、重试同 id 不增行。barrier 标记
  文件是杀进程时刻证据（`before_commit` / `after_commit_before_reply`），
  不用 sleep 猜事务阶段。
- 损坏/只读：坏库探测失败 → 拒绝启动退出，文件原样保留（从不重建空库）；
  只读介质（WAL pragma 失败）→ 拒绝启动；原数据保留。

## 反例绑定（必验）

capture_receipt（V-09 顺序/并发去重、冲突、跨主体拒绝与独立只读读库核对；
V-10 双 barrier 崩溃反例；V-11 第二 writer/坏库/只读/容量与旧 key 可用）、
acceptance（V-14 50×1KiB 双 caller 工作负载 + 重复运行幂等 + python stdlib
sqlite3 只读对账）。领域 oracle 是独立只读读库，不以服务自身"成功"JSON
为唯一判据。
