# 时钟主题采纳（clock-theme-adoption）

> 来源：auto-os/PLAN-055（2026-11-05 交付）。产品侧 `docs/specs/theme-adoption.md`
> 为后继项（本批候选未建，SD-02 预告，owner=auto-clock 后继主题计划）；
> 本页为 OS 组合面登记：pin 语义、采纳形态、业务门与宿主侧消费面。宿主
> 策略主合同见
> [../shell/host-theme-policy-and-delivery.md](../shell/host-theme-policy-and-delivery.md)。

## 组合与 pin

- 产品开发/验证使用独立 clone（`D:/autostack/auto-clock`，main 一写入者）；
  `apps/auto-clock` 是 AutoOS 组合检出，detached 在 gitlink 记录的提交，
  不作为开发目录（OS AGENTS §2.1）。
- **PLAN-055 交付 pin**：gitlink `551d05e` → **`3495671`**（已推
  `auto-stack/auto-clock` main；候选=被测=origin/main 同源。候选内容：
  vendor/themes 0.2.0 固定快照——逐文件 sha256 对源包清单校验后发布——
  + `dep themes` + `use themes.themes: Auto` + README `apps/auto-clock`
  反链与 PLAN-055 主合同节 + 四tab测试旧路径头/就绪判定修）。

## 采纳形态与语义边界（as-built）

- pac `render=vue`、`front_port 17812`、无 daemon；业务单 `src/front/app.at`
  四 tab（秒表/计时器/世界时钟/闹钟，storage `clock.alarms.N`）。
- **follow-only**：pac `theme` 键缺省 = follow 宿主策略完整快照（053/031
  合同）。CLI `--theme` 对未声明 `dark_mode` 的 VM 采纳者无主题内容面
  （C-03 §3 VM 边界「启动播种只写已声明 dark_mode/accent_color；未声明
  即无主题面」，UI038 口径；PLAN-055 修复轮实测：只翻窗口 chrome，内容
  照随宿主策略）——scope App override 档运行时无生产者（`HostThemeInputs`
  唯一生产者 = 策略快照）。显式选择档由有声明面的产品承担（反选腿 calc
  显式受控启动钉死 × clock follow，角色等价记录于宿主页 PLAN-055 节）。
- 独立默认 = Auto/light（无宿主 env 时 design 默认，像素级 luma oracle
  224=light）。

## 宿主侧消费面

- manifest 条目 `012-clock`（ports [17812]，无 daemon）；a2r 生成 exe 嵌入
  官方包身份/族表（与宿主 catalog 同一聚合 digest `9fa07742…`），outproc
  真载体四腿（attach 推送/变更广播/重连最新/stale 分列）+ 具名拒绝保旧
  恢复 + 真实 OS 事件窗口腿实证——**G-1/G-2 闭合**（证据链见宿主页
  SD-05/G-2 节与 PLAN-055 归档页）。
- broker 多实例隔离：管道 env `AUTOOS_BROKER_PIPE` 覆盖（无 env 零行为差）。

## 业务门（组合级，PLAN-055 实测口径）

`tests/desktop_mcp.py` 四 tab 全量原条件 15/15：T0 表盘（含 dial_mode
切换）/T1 秒表（走表≥1000ms、MM:SS.cc、计圈、复位）/T2 计时器（1s 到零
banner、Dismiss）/T3 世界时钟（八城、北京/伦敦 ±60s 原容差）/T4 闹钟
入列 + storage 跨重启。换肤/attach/reconnect 不重置 elapsed、不倍速/
重复 Tick（载体 push 不触 inner 结构保证：双次换肤业务字段保持单测 +
同场腿实测）。
