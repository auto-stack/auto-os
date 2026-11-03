# 计算器主题采纳（calculator-theme-adoption）

> 来源：auto-os/PLAN-053（2026-11-03 交付）。产品权威 Spec 在产品仓：
> **auto-calc `docs/specs/theme-adoption.md`**（推送后为
> `auto-stack/auto-calc` main）——本页是 OS 组合面的登记页：pin 语义、
> 组合验证口径与宿主侧消费面。宿主策略主合同见
> [../shell/host-theme-policy-and-delivery.md](../shell/host-theme-policy-and-delivery.md)。

## 组合与 pin

- 产品开发/验证使用独立 clone（`D:/autostack/auto-calc`，main 一写入者）；
  `apps/auto-calc` 是 AutoOS 组合检出，detached 在 gitlink 记录的提交，
  不作为开发目录（OS AGENTS §2.1）。
- **PLAN-053 交付 pin**：gitlink `e5247c5`（既有基线）→ 交付后由 merge
  推进到 `a1f38842`（主题采纳候选：vendor/themes 快照 + dep themes +
  design 默认 + use 导入 + 测试扩展）。工作期候选组合以 worktree 子模块
  本地 fetch+detach 到候选验证（gitlink 不动），已实测启动/采纳者
  applied/策略切换持稳。

## 宿主侧消费面

- manifest 条目 `011-calculator`（ports [17810]，无 daemon，vm in-proc
  launch 缺省）；采纳者判定 = 组件有主题 scope（`dep themes` 消费链）。
- 宿主策略提交/外写对采纳者喂 031 宿主端口（follow——pac `theme` 键保持
  缺省）；状态面行 `011-calculator=applied|pending|rejected: …`；
  legacy 控制不受影响（行 `legacy`）。
- 类目色豁免（运算符 `bg-orange-500`、科学键 `bg-indigo-700`）为 App
  命名空间固定值，不随主题映射（键位色彩编码语义保持）。

## 业务门（组合级，PLAN-053 实测口径）

calc 原义务全量（2+3=5、链式、小数、括号优先级、2^10、清零、结构负例）
+ PLAN-053 三区分用例（sqrt 语义钉定/HEX FF+1=100 pacc 256/除零错误态）
在主题采纳前后必须全绿（`tests/desktop_mcp.py`，21 检查）。sqrt 函数
求值为**具名引擎缺口**（函数 token 直通，当前语义 =9）——产品后继范围，
非宿主主题链缺陷。
