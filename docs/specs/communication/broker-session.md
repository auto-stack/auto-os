# broker 会话与路由合同（broker-session）

状态：**proposed**（PLAN-049 SD-03）。冻结 F1 会话/接入/路由语义：principal
绑定、epoch/实例、注册/发现/转发、截止/断线与接入控制。依据 Design02 §3–4；
参考实现 `experimental/communication-f01/local`（broker.rs/policy.rs/registry
语义内联）。

## 会话与身份

- 每次 broker 启动产生新 `broker_epoch`；每次注册成功由 broker 分配新
  `instance_id`。发现返回 `ServiceRef={broker_epoch,instance_id,service_id,
  interface}`；引用永不静默跳到新实例（旧 epoch/旧 instance → `stale_instance`，
  服务无活跃 provider → `service_not_found`）。
- broker 只监听随机实验管道名（`\\.\pipe\autoos-comm-f01-<random>`），不占
  固定名；首个监听 `FILE_FLAG_FIRST_PIPE_INSTANCE`（同名第二 broker 在创建处
  失败退出）；所有实例显式保护 DACL（仅当前用户 + SYSTEM，读写；
  SE_DACL_PROTECTED，无 Everyone/匿名继承 ACE）+ `PIPE_REJECT_REMOTE_CLIENTS`。
  DACL 以真实句柄枚举核验（恰好两条允许 ACE）。
- 凭证→(principal, role, allow) 的绑定只在 broker 侧 policy（运行配置，
  私有目录）；provider 仅可注册其 allow 内服务；caller 仅可发现/调用其
  allow 内服务；无凭证/越权 → `permission_denied`（业务执行前）。
- 受控单用户实验 profile：不提供安装身份、跨用户授权、网络安全或同用户
  恶意代码隔离；独立进程不是沙箱。

## 注册与发现

- 同 profile 只允许一个活跃 provider 持有 `auto.capture.inbox`：
  同连接相同 descriptor 重复注册幂等返回原 instance；同连接改变 descriptor
  → `already_registered` 拒绝；其他连接抢占 → `already_registered`，不覆盖。
  原连接断开释放后，新注册取得新 instance。
- provider 断线：其注册撤销；该连接上已转发路由 → `disconnected +
  may_have_run` 通知 caller。

## 路由与资源

- broker 校验顺序（业务前拒绝）：权限 → ServiceRef(epoch/instance) →
  schema/method 对与 body 形状 → timeout 范围 → 活动 request_id 查重 →
  剩余预算 → 配额。任何拒绝不改数据库、不触 provider handler。
- 配额：8 已认证连接 / 每连接 8 在途 / 全局 32 / provider 队列 16；
  超限 → `resource_exhausted` + `not_dispatched`。不无界 spawn。
- 路由生命周期：Pending →（转发写尝试）Forwarded → 回复/错误/超时移除；
  回复按私有 route 精确回达原 caller 连接；伪造 route（caller 发 provider
  方向帧、provider 发未知 route）→ 连接终止或丢弃，不影响其他请求。

## 反例绑定（必验）

会话合同的可观察行为由以下测试族固化（`experimental/communication-f01/
demo/tests/`）：f1_route（V-04 权限/伪造主体、V-06 注册生命周期与抢占、
V-07 截止两态与晚到结果、V-08 双caller隔离与伪造route、V-12 连接配额）、
resources（V-05 管道安全位、V-12 在途/队列配额）、lifecycle（V-06 broker
重启 epoch 更替、V-13 受管生命周期）。
