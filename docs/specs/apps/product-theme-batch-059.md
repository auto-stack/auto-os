# 产品主题批次 059：Reader 阅读产品（PLAN-059）

> 来源：auto-os/PLAN-059（2026-11-04 work 执行）。宿主主合同：
> [../shell/host-theme-policy-and-delivery.md](../shell/host-theme-policy-and-delivery.md)；
> 先例批次：[product-theme-batch-058.md](product-theme-batch-058.md)
> （消费语义同源，本页不重复其机制定义）。

## 批次范围与候选（冻结源）

一个实际产品入口（apps.manifest 稳定 ID 边界）：018-book-reader
（auto-reader，独立 clone main）。

os-059 冻结工具链（组树 detached，全部 clean）：

| 树 | pin |
|---|---|
| auto-os | 0c715dc（分支 codex/plan-059-product-themes） |
| auto-man（auto.exe 来源） | 2bbd9542（安装版 `0.1.0+plan-890-verified-r7-t19-17-g2bbd954`） |
| auto-lang | 2f558fc93（036 交付线） |
| auto-ui | e06b65b2（036 交付线）+ 本批 vue 轨 design 投影门控修复（分支 codex/plan-059-vue-design-theme-gate，见下"框架投影修复"） |
| auto-web | 3de5524 |
| auto-down | 7c4d5cbb |

主题包源（managed channel 唯一可编辑源 `auto-ui/packages/themes`）：
0.2.0，per-file sha256 全 match，聚合 digest
`9fa07742dc68aa5377ae06cb2d966386002063f184c515b6c5dddf8a6bef838e`
（OS053 宿主策略固定值逐位一致；vendor 快照同律校验）。

## 前置核验（开工时点，2026-11-04）

- U036（auto-ui/PLAN-036 正式包/生成输入收口）：**archived 10/10**
  （merge pass：UI e06b65b2 / Man 2bbd954 / lang 2f558fc93）。
- READER-007（正式 EPUB 阅读面/Locator v2/来源支持）：**archived 12/13**
  （T-11 独立 review pass；auto-reader main @ 4c93d1da clean，写窗释放）。
  计划 §4 观察值（executing 2/13）为立项时点快照，开工前已重核。

## 框架投影修复（本批具名，owner=auto-ui）

vue 轨 `design { theme }` 投影（PLAN-031 F-03 合同）被生成器注入门控
（`has_accent_color || has_theme_name`）静默截断：无 accent/theme_name
状态的产品（本批 reader、056 todo 同形）App 级 design 不产生任何运行时，
官方包不上身（vue 轨停留 scaffold 静态表——`gen/**/index.css` 亦不消费
包树）。**056/057/058 的像素 oracle 均在 VM 轨执行，该缺陷在其交付时已
存在且未被其门覆盖。** 本批修复 = auto-ui `ui_gen/vue/script.rs` 注入门
扩三臂（accent_color / theme_name / design.theme 投影任一在场即注入；
三无 app 维持零注入，零变化优先裁定不变），配套编译器回归测试 +
reader 腿 A vue 轨像素反证（修复前 #030712 静态表值、修复后 #09090b
官方包值）。auto-ui 落地按其正常 push + 受影响职责门。

## 产品候选表（候选 SHA 于各产品原门+主题验收通过后回填）

| 产品 | 独立 clone 候选 SHA | design 默认腿 | Vue 轨门 | VM 轨门 | 原门 |
|---|---|---|---|---|---|
| 018-book-reader | auto-reader `<T-05 回填>` | dark_mode 位派生（无 accent 声明；独立默认 light=原纯浅色语义） | test_059_theme 七腿（A/B/F/FB/FR/S1/S2，像素+读回+哨兵） | MCP 像素腿 + verify_standalone vm | verify_standalone 28/27、verify_local 50、verify_epub 22、auto test 7、verify_epub_product |

## 阅读面接线（经 READER-007 正式引擎端口）

- 优先级：显式书级选择（reading-state/选择器）> 应用模式派生 > 缺省
  light；sepia 恒显式。派生经 `/api/reader/command` `op:"settings"` 主题
  字段下发，`window.__reader.appliedSettings.theme` 读回。
- 本批修复既有缺口：冷恢复不向引擎端口补发主题（stored dark 重开引擎
  面回浅色）——Init 末段按 SetTheme 同式补发，反例入 S1/S2 与腿 A。
- 内容域豁免：出版物正文/图片/封面/批注色语义归 007 owner；批注高亮
  `bg-primary/10` 随 token（合法公共面）。

## 组合与宿主（T-04/T-05 实录，待回填）

- V-03 腿：reader standalone follow 臂（AUTOOS_DESKTOP_CONFIG 隔离
  config + 冻结包根 AUTOOS_THEME_PACKAGE_DIR）+ 已交付产品相反策略
  并行不串扰；坏 digest 候选具名拒绝保旧；重开追最新。
- V-04 包源消费腿：per-file sha256 准入（036 交付机制）；vendor 快照
  不可二次编辑纪律见 reader SD-03 页。
- OS 组合 pin：fetch/detach 同 SHA + 健康门（check-app-submodules）+
  实际启动后提交。

## 与其他批次的边界

- 055 拥有 clock 与真实 OS 事件/outproc 载体（G-1/G-2）；本批不重复接。
- 054 拥有八基础产品业务装配；本批产品非其分母，OS pin 按本批自身门
  更新。
- UI037/038 教学线不抵本批产品门。
- OS056-REAPPLY-NOTE（live 外写翻转臂滞留）为 053 owner 注记；本批
  恢复路径按合同命名"重开"实测。
