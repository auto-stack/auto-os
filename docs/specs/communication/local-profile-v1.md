# 本地通信 profile `json-f01/1`（local-profile-v1）

状态：**proposed**（PLAN-049 SD-02）。冻结 F0 wire 合同：帧格式、有限 JSON、
版本协商、错误分类、预算与人工 golden。语义依据 Design02 §4/§6；参考实现
`experimental/communication-f01/core`（单源）与 `tests/fixtures/`（golden）。

## 帧与 JSON

- 帧 = `u32 LE payload_len` + UTF-8 JSON；长度头先验证后分配。
  零长 → `invalid_frame`；> 65536 → `resource_exhausted`；65536 合法。
- 有限 JSON profile：object / 有界array / string / bool / **u32**。
  其他数字形态（负/浮点/指数/溢出/null）与重复键、未知字段、未知 op、
  深度 > 16 一律拒绝；分类表：语法/UTF-8 → `invalid_frame`；键/字段/类型 →
  `schema_mismatch`；深度超限 → `resource_exhausted`（分配前判定）。
- 信封：每帧一个扁平 JSON 对象，`op` 判别。op 族（ caller↔broker）：
  `hello welcome register registered discover discovered call reply error`；
  broker→provider 私有面：`pcall preply`。每个 op 的允许键表是实现单源
  （`core/src/codec.rs`）；自由 body 对象键序在编码时规范排序（BTree）。
- 版本协商三轴：协议版本 `1`、codec `json-f01/1`、接口 `capture.inbox/1`；
  无共同 profile 在业务执行前拒绝（`unsupported_version`）。
  未来扩展（task/cancel/event/stream）只登记版本位，不实现。

## 消息与身份

- 所有协议可见 ID 为 128-bit 随机数小写 hex 文本（32 字符）：broker_epoch、
  instance_id、request_id、operation_id、route_id。凭证为 256-bit hex 文本，
  只经私有文件传递；凭证不出现在命令行、日志与回执。
- `request_id` 关联一次尝试（同连接活动期复用 → `schema_mismatch`）；
  `operation_id` 是 capture 领域去重键；`corr_id` 关联 register/discover。
  三者不混用；body 自报 principal/app_id 不可信（身份来自 Hello 凭证）。

## 截止与 IO 模型

- Hello→Welcome 整体 2s；帧长度头到达后帧体 2s 内完成（残帧有界）；
  帧间空闲不受限（F1 无 keepalive）；所有写 2s 截止；关停 2s。
- call `timeout_ms ∈ [1,5000]`，缺省 2000；broker 用本地单调钟计算剩余预算
  转发（`deadline_ms`），provider 从收到调用起按剩余预算执行。
- deadline/disconnect 的 execution 判定：路由 `Pending`（转发未尝试）→
  `not_dispatched`；`Forwarded` → `may_have_run`。配额拒绝（连接内 8 /
  全局 32 / provider 队列 16）与容量拒绝携带 `not_dispatched`。
  进入转发即保守 `may_have_run`；晚到 reply/error 丢弃，不入新请求；
  写操作不自动重放。

## golden

人工 golden 帧（编码字节级与解码字段级双向核对）与全部分类反例固化于
`experimental/communication-f01/core/tests/fixtures/` + `tests/f0_contract.rs`；
修改本 profile 任一冻结项必须修订 golden 与 PLAN-049 的 plan_revision。
