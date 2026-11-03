# 宿主主题策略与正式包交付（host-theme-policy-and-delivery）

> 来源：auto-os/PLAN-053（2026-11-03 交付）。supersedes：无（新增域页，旧
> 暗亮/内置名链保留为 legacy 支持面）；关联：[showdesk-ux-polish](showdesk-ux-polish.md)
> P041 双路径契约、[../apps/calculator-theme-adoption.md](../apps/calculator-theme-adoption.md)。

## 产品批次消费面（PLAN-056 增补）

本页策略模型/提交序/拒绝保旧/交付通道不变；PLAN-056 为其**实际消费者扩展
批次**（七个生产力/内容产品 + blog 待其写窗，登记与逐产品合同见
[../apps/product-theme-batch-056.md](../apps/product-theme-batch-056.md)）：

- 产品经固定 `vendor/themes` 快照 + `dep themes` + `use themes.themes: Auto`
  （031 合同）消费；pac `theme` 键缺省 = follow 宿主，原显式键退役。
- **standalone 跟随臂实证约束**：会话子进程 = `auto-ui-tool.exe`
  （`--internal-vm-exec`），hosted 钩子路径的 arm 依赖**配套工具宿主二进制
  含 053 机制**——验证/部署前必须核验两二进制（旧工具宿主 = 跟随臂永不
  武装，全部 family 候选具名拒绝，仅 legacy 链生效）。
- 生成工作区一致性：产品 `auto run` 再生成 back workspace 时，组树工具链
  需 `AUTO_LANG_ROOT`/`AUTO_WEB_ROOT` 指主检出，否则混入组树路径触发
  `links` 冲突（消费侧运行事实，非策略语义变更）。
- V-04 包源消费腿（tampered 全不可用具名拒绝+design 默认保底 / 第二有效根
  验证后照常驱动）在 calendar 实证，机制全体产品同源（per-file sha256）。
- **宿主页 owner 注记（053 交付遗留观察，非本批缺陷）**：拒绝后无间隔成功
  提交的立即活体重投偶发不生效（standalone 轮询臂）；合同命名恢复路径
  "重开或重连"双腿双产品像素级全过。若日后需命名立即重投为必需语义，
  属本页新合同项（053 owner），非 056 消费面。

## 策略模型（SD-01）

桌面单源配置（`~/.config/autoos/apps/desktop/config.at`，env
`AUTOOS_DESKTOP_CONFIG` 覆盖）在既有 legacy 键（`dark_theme`/`theme_source`/
`theme_name`）之上承载**宿主主题策略组**：

- `theme_family`：官方主题包 family 标识（`Auto`/`Solarized`）。缺省缺键 =
  未选，行为与升级前逐字节一致（legacy 链原样）。
- `theme_family_version` + `theme_family_digest`：固定包内容锁（version
  `0.2.0` + 全文件聚合 sha256），与宿主目录快照比对，任一不符整候选拒绝。
- `theme_mode`：`light`|`dark`|`system`（运行派生值不落盘）。
- `theme_rev`：宿主分配的单调 revision（有效提交一次 +1）。

**提交序（唯一入口）**：完整候选校验（family 在目录、version/digest 相符、
mode 被 family 声明）→ 候选字段改写（含 legacy 兼容映射：mode light/dark →
`dark_theme` + `theme_source=manual`；system → `theme_source=system`）→
`config.at` 原子落盘 → 发布内存 revision → 通知（in-proc 喂端口 + outproc
广播 + shell 状态面）。写失败零内存变更零成功 ack；全等候选 `Unchanged`
零落盘（杀外写/重复写回环）。**bus 动词与 config 外写轮询走同一入口**
（P041 双路径等价的策略层）。

**拒绝与保旧**：未知 family、版本/digest 不符、mode 未声明、包目录不可用、
落盘失败 → 具名原因回写 shell 状态面（`__theme_error`），内存 policy 与
盘面保旧。已拒绝候选的 legacy `dark_theme` 翻转不得改色采纳者。

## 目录（managed channel）

宿主/standalone 进程的包目录解析为 env 双源、有序尝试：
`AUTOOS_THEME_PACKAGE_DIR` → `AUTO_UI_ROOT/packages/themes`。每个候选先
verify（owner/package/version/schema + per-file sha256 + 家族模式可用）
再算聚合 digest；失败跳过下一个候选；**全失败 = 降级**（family 链具名
拒绝，legacy 链不受影响）。**无主检出 fallback**：managed channel 缺席 =
降级，绝不静默换源到编译期锚定的检出目录。

## 交付通道（SD-01）

- **in-proc（desktop 宿主内 vm 应用）**：提交/启动/挂载三路对有主题
  scope 的组件喂 031 宿主端口（`HostThemeInputs{follow_host:true,
  host_family, host_mode}`）；发布失败整帧保旧（031 publish 语义）。
  采纳者的 `dark_mode` 模型位由**已提交策略的派生 mode**驱动（视图类
  分支消费），legacy `cfg.dark_theme` 翻转不越权。
- **outproc（协议客户端）**：`Hello` 尾段声明能力 `theme-policy/1`；
  attach/重连/策略变更三路推送完整快照帧
  （`ThemePolicyPush{revision, record}`，record =
  `rev␟family␟version␟digest␟mode`）；客户端喂 031 端口、帧构建评估后
  回 `ThemePolicyResult{revision, applied|rejected|unsupported|stale,
  reason}`；低于本连接已推 revision 的结果记 `stale` 不覆盖新状态。
  **已声明能力才推送**（旧客户端零未知帧）。零装载采纳（`adopt` 外部
  rqhost 客户端，D1 客户端权威）后同路推送。
- **standalone（`auto run -r vm`）**：env `AUTOOS_DESKTOP_CONFIG` 在场 =
  显式跟随宿主桌面配置——400ms 轮询差异臂与宿主同一提交入口；
  env 缺席零开销零行为差（应用 design 默认/legacy 链）。

## shell 状态面

`desktop.at` 空白菜单主题策略区（Auto/Solarized × 浅/深/跟随系统六直选）
+ 宿主注入 `__theme_family/__theme_mode/__theme_rev/__theme_status`（摘要
`family@mode revN` + 每应用 `applied|pending|rejected: 具名|unsupported|
stale|legacy` 行）与 `__theme_error`（具名拒绝/落盘失败诊断）。pending
自续拍：eval 落定在帧构建路径，待定行下拍再查，落定即停。

## 真实系统事件与平台边界（SD-05，具名 hold）

- 受控腿（已实证）：`theme_mode=system` 经 `system_prefers_dark()`
  （Windows 注册表只读查询）在 boot/load/帧评估消费。
- **真实腿具名 hold**：真实 OS 主题偏好变更→监听/轮询→policy resolved→
  桌面/运行应用的端到端实证，需用户授权的验证窗口（备份原值、finally
  精确恢复）。**PLAN-053 交付时未获授权，真实腿 pending**——owner:
  user/merge；closure: 获授权窗口后补 V-07 真实腿。本页不声明真实腿
  完整通过；受控绿不代替真实腿。
- 平台边界：provider 仅 Windows（`reg query`）；Linux/Mac 未测，不冒
  跨平台全绿。

## outproc 载体后继闭环条件（G-2 遗留，源自 PLAN-053 review）

桌面组合式 outproc 应用的策略推送端到端（attach→推→结果帧）已实现且
协议两端点有单测（帧 17/18 roundtrip、caps、零装载采纳、stale 判定），
但本环境无 outproc app 二进制载体（calc 为 vue/vm 单产品），外部自采纳
客户端的 Hello 未达桌面 broker（Plan-500/694 代 spawn 机制，非 PLAN-053
代码）。**闭环条件：存在 outproc 载体 app 后按 probe-t05 重测四腿
（attach 推送/applied 回读/重连重推/stale 分列）。**
