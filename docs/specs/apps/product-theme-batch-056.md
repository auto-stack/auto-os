# 生产力与内容产品主题批次（product-theme-batch-056）

> 来源：auto-os/PLAN-056（work 候选冻结，2026-11-03/04）。主合同：
> [shell/host-theme-policy-and-delivery](../shell/host-theme-policy-and-delivery.md)；
> 登记形态沿 [calculator-theme-adoption](calculator-theme-adoption.md) 先例。
> 消费形态（vendor/themes 固定快照 + `dep themes` + `use themes.themes: Auto`
> + design 默认腿 + follow 语义）逐产品合同见各仓 `docs/specs/theme-adoption.md`。

## 批次范围与候选（冻结源）

os-056 冻结工具链：组树 detached（auto-man `2bbd954`、auto-lang `2f558fc9`、
auto-ui `3c46104b`、auto-web `3de5524`、auto-down `7c4d5cb`）；`auto.exe`
sha256 `5858593b…cc81f68`（组树构建）；**会话子进程 = `auto-ui-tool.exe`
（`--internal-vm-exec`），缺 053 机制的旧版会使 standalone 跟随臂永不武装
——验证/部署前必须核验两二进制**。各门/探针每运行实际工具身份：门与探针
env 以 `AUTO_UI_ROOT=D:/autostack/auto-ui`（主检出）驱动工作区生成，tool
discovery（env → AUTO_UI_ROOT workspace → exe 邻接）实际解析到主检出构建
`auto-ui-tool.exe 0abe93fb`（2026-11-03 13:32，053 落地 `6f4bcfae` 之祖先
——机制在场，评审员已核谱系）；组树另构建 `3493e746…ebab0b05` 同源。
后续运行如需钉定，应记录每运行实际工具 sha256（发现序无专用钉定 env）。

| 稳定ID / 仓 | work 候选（main，未 push——push 归 merge 节奏） | design 默认腿 | VM 轨原门（冻结工具链复跑） | vue 轨 |
|---|---|---|---|---|
| 015-notes / auto-notes | `38c7c87` | dark_mode 位派生（默认暗色=原 pac 键等价） | desktop_mcp 11/2/1（A/B 基线持平）+ autotest 19/19 | follow probe 5/5 |
| 013-todo / auto-todo | `1537107` | 静态 light（纯浅色 TodoMVC，无 dark_mode 位） | verify_basic 73/73 | 78/78 |
| 016-calendar / auto-calendar | `9364239` | dark_mode 位派生（默认浅色=原 pac 键等价） | follow probe 5/5（像素级） | —（原门=CALENDAR-001/002 交付记录） |
| 017-chat / auto-chat | `378a7f2` | 静态 light | verify_basic 4/4 + identity 45/45 | 8/8 |
| 031-paint / auto-paint | `c49ef41` | 静态 dark（原 pac `theme:"dark"` 承接） | desktop_mcp 31/31 | —（canvas 像素 hex 真值内容域） |
| 026-database / auto-database | `280ddac` | 静态 light | verify_basic 59/59 | 55/55 |
| 024-charts / auto-charts | `64c9db5` | 静态 light | verify_basic 43/43 | 14/14 |
| 023-realworld / auto-blog | `beaae49`（接管停滞 blog-005 窗完成，见下） | 静态 light（路由壳无 dark_mode 位） | dual vue-rust 8/14 + native-vm 9/14——**失败集与基线 84a5514 逐场景一致**（A/B 同轮工具链；既存 works.rs CJK panic 级联+已保存断言=blog 产品线既有面非本包回归） | —（dual 双 profile 即其门） |

## 包身份与消费语义

- 每产品 `vendor/themes` = `themes` 0.2.0 固定快照，内容 digest
  `9fa07742dc68aa5377ae06cb2d966386002063f184c515b6c5dddf8a6bef838e`，
  per-file sha256 与源包校验同律；不可二次编辑、非第二权威源。
- pac `theme` 键缺省 = follow 宿主（策略完整快照 > design 默认 > legacy）；
  notes/calendar/paint 原显式键按此退役，品牌 accent（indigo）保留。
- 内容域豁免逐产品具名：paint 画布 256 像素 hex 真值、charts series 数据色、
  database 数据/DDL/查询文本、notes 四色板与编辑器语义；不因"统一"被染色。

## V-04 包源消费腿（calendar 实证，链路全体产品同源）

- **tampered/全不可用源**：`AUTOOS_THEME_PACKAGE_DIR` 指向篡改副本且无有效
  兜底 → boot 恢复**具名拒绝**（"主题包目录不可用（包缺席或校验失败）——候
  拒绝"）+ design 默认色保底（配置请求 dark 而实亮 luma 247.3），legacy 链不受影响。
- **第二有效根**：任意绝对路径 clean 副本（逐文件 sha256 校验通过）→ 照常
  驱动策略（dark 配置 boot 应用 luma 22.6）——managed channel 不绑定单一检出。
- fresh/locked/缺件/同 mtime 内容变更：同一条 per-file sha256 校验机制覆盖
  （内容身份以 hash 判定，与 mtime 无关）；053 host 侧 probe-t08 变体结论沿用。

## 具名未闭面（交 final review 裁定）

1. **auto-blog 接入完成（原具名阻点闭合）**：blog-005 窗口自 10-31 停滞
   （clone 干净已 push、无未交 WIP、owner 缺席），经目标裁决授权接管后按
   本批同构接入（`43ef2a9` adoption + SD-03 `beaae49`，已 push）。接入为
   纯增量（vendor/themes + pac dep + app.at use/design + README/SD-03），
   与 blog-005 的 vue-dev-rust 前置面正交；其计划文件未动，恢复时
   fast-forward 即可。归档分母 8/8 闭合。
2. **OS 组合 pin**：等 auto-os/PLAN-054 交付其审定的业务版本后，以本批候选
   （或其后代）fetch/detach 同 SHA 更新 gitlink（§5.3/T-05 冻结裁定）。
3. 拒绝后无间隔成功提交的立即活体重投偶发不生效（053 合同命名恢复路径为
   "重开或重连"，重开腿双产品全过）——host 侧行为注记，非产品消费面缺陷；
   merge landing 时已随本批消费面增补节提示宿主页（053 owner）知悉。
