# 产品主题批次 057：桌面工具与设置六入口（PLAN-057）

> 来源：auto-os/PLAN-057（2026-11-04 work 执行）。宿主主合同：
> [../shell/host-theme-policy-and-delivery.md](../shell/host-theme-policy-and-delivery.md)；
> 先例批次：[product-theme-batch-056.md](product-theme-batch-056.md)
> （消费语义同源，本页不重复其机制定义）。

## 批次范围与候选（冻结源）

六个实际产品入口（apps.manifest 稳定 ID 边界）：014-weather、028-launcher、
025-sys-monitor、027-file-manager、os-config、039-syslog（OS local，
`apps/auto-syslog` 非 submodule）。

os-057 冻结工具链（组树 detached，全部 clean）：

| 树 | pin |
|---|---|
| auto-os | 1910c222（分支 codex/plan-057-product-themes） |
| auto-man（auto.exe 来源） | 2bbd9542 |
| auto-lang | 2f558fc93 |
| auto-ui | 3c46104b（056 实证 pin；含 036 交付 e06b65b2 ancestor） |
| auto-web | 3de5524 |
| auto-down | 7c4d5cbb |

二进制身份：auto.exe `0.1.0+plan-890-verified-r7-t19-17-g2bbd954`
（sha256 前 16 位 `8d1944dd5f66bc6d`）；auto-ui-tool `0.1.0`
（sha256 前 16 位 `da5ea1569500c232`）。

产品候选表（候选 SHA 于各产品原门+主题验收通过后回填）：

| 产品 | 独立 clone 候选 SHA | design 默认腿 | VM 门 | vue 门 |
|---|---|---|---|---|
| 014-weather | auto-weather `8c13952` | dark_mode 位派生（默认暗色） | vm_smoke **183 PASS/0 FAIL**（T5 主题翻转过） | run+admission ✓（receipt digest 实录） |
| 028-launcher | auto-launcher `134d047` | 静态 light（零遗留） | desktop_mcp **A/B 同集 20P/4F**（4F=001 r7 provider 行为 vs 前置套件预期，预存） | vue_verify **ALL PASS**（admission digest 实录） |
| 025-sys-monitor | auto-monitor `6ff5646` | 静态 dark（pac dark 退役） | desktop_mcp **13 passed/0 failed**（含重启持久 T6） | playwright smoke 被**预存** back E0432 阻（A/B 两臂同，具名 hold owner=monitor 产品线） |
| 027-file-manager | auto-explorer `8505ef2` | dark_mode 位派生（pac dark 退役） | desktop_mcp **78/78 PASS** | verify_p2 12/0、p3 10/0、p4 11/0、p6 21/0；p5 AC-04 菜单挂载 flake **A/B 两臂同分布**（12/1、11/2）；perf P-2/P-4 过，P-5 last_snapshot_ms=-77ms **A/B 两臂逐字节同值**（预存仪器伪值） |
| os-config | auto-os-config `0018066` | Theme store dark_mode 派生 | e2e-vm **14 PASS**（accent 切换/重启持久/重启后导航；connection 项=运行臂无 daemon 环境项，back/api 零改动；2 GAP=plan010 台账预存） | npm run build 绿（scoped 部署）；**策略卡实机点击全过**（隔离根：Auto 写入 family+mode、浅色切换、未选清除、刷新读回、其余字段零触碰）；e2e vue 轨两臂预存红（详见 SD-03）；theme-switch 两臂 networkidle 预存不兼容 |
| 039-syslog（OS 本地） | OS 树内 | 静态 light（零遗留） | desktop_mcp **8/8 PASS** | 桌面组合启动（registry=9 见下） |

## 组合与宿主（T-04/T-05 实录）

- **独立跟随臂像素探针**（weather+launcher，隔离 config + 冻结包，056 同型）：
  **12/13 PASS** —— boot@完整候选=暗（luma 50/11）→ 外写翻 light 跟随（234/253）→
  未知 family "Noir" 具名拒绝保旧 → **恢复腿 1/2 偶现不生效 = OS056-REAPPLY-NOTE
  hold 复现**（053 owner 注记维持）；命名恢复路径"重开"双腿全过（reopen@dark
  luma 50/11）；vendor 快照往返 sha256 不变。
- **V-04 包源腿**（weather）：vendor 篡改一字节 → **具名拒绝
  `主题包生成准入失败…themes.at`**、启动中止不静默消费；恢复后逐文件
  sha256 等值（themes.at 329c2d56…）+ 干净 boot（interpreter ok=true）；
  第二有效根（任意绝对路径拷贝）boot 成功零拒绝。fresh/locked/缺件/同 mtime
  由 per-file sha256 准入机制覆盖（036 机制全体产品同源）。
- **宿主桌面 boot**（os-057 组树 ui-desktop + 隔离 config + 冻结包根）：
  `[ui-desktop] theme package: themes v0.2.0 families=2 digest=9fa07742..6bef838e`
  宿主目录装载成功；候选 gitlink fetch/detach 后 registry=9 desktop_visible=9
  （其余 19 空壳=worktree 混合形态默认，AGENTS §2 约定）；sys-monitor 桌面
  拉起（back-proxy lazy-start + dashboard face）。
- **半候选拒绝实录**：仅 family+mode（缺 version/digest）的 config → boot
  恢复具名拒绝 `候选 "(缺席)" ≠ 目录 0.2.0（保 legacy 链）` —— 宿主回填
  语义与 os-config 卡写侧分工（写两键，宿主 stage 回填）一致。
- **健康门**：`check-app-submodules.ps1 -OsRoot` = 本批 5 模块 detached 于
  候选（+SHA，待本提交落 pin）、其余 21 未初始化（worktree 混合形态默认）。

## 包身份与消费语义

- `vendor/themes` 固定快照：`themes` 0.2.0 / schema 1，内容 digest
  `9fa07742dc68aa5377ae06cb2d966386002063f184c515b6c5dddf8a6bef838e`，
  与 UI 源包（3c46104b `packages/themes/`）逐文件 cmp 一致
  （pac.at `2e66212b…`、themes.at `329c2d56…`）；六产品 vendor 快照与
  056 交付快照（auto-notes/vendor/themes）逐字节相同。
- 消费形态同 056：pac `dep themes { path: "vendor/themes" }` +
  `use themes.themes: Auto`（C-11 收集链注册）+ pac `theme` 键缺省 =
  follow 宿主。weather/launcher/syslog 原本即无显式 theme 键；
  monitor/explorer 的 pac `theme: "dark"` 显式键本批退役（原语义由
  design 默认腿承接）。
- 生成准入 receipt：weather vue `gen/front/vue/.theme-inputs.json`
  实录 `{schema:1, owner:auto-ui, name:themes, version:0.2.0,
  digest:9fa07742…bef838e, identity:41ea2af4…6245d}`（2026-11-04 实跑）。

## 本批特有面

- **os-config 053 policy 收敛**（SD-02 消费面扩展）：Desktop 模块页
  「主题包策略」卡（family 未选/Auto/Solarized × mode 浅色/深色/跟随系统）
  经既有 `PUT /api/config/desktop` 链（Plan 504 config-plugin-architecture
  同一文件）写 `theme_family`/`theme_mode` 两键；宿主 config 外写轮询
  （P041 双路径）送 053 唯一提交入口 stage 校验→version/digest 回填→
  publish revision。拒绝具名保旧（宿主 shell `__theme_error`；
  os-config 刷新读回呈现保旧值）。`theme_rev` 只读回显，外部写方不手改
  （desktop_config.rs 字段契约）。既有 mode/accent（autoos-ui.json）保持
  独立默认腿与 legacy 字段。
- **syslog 跟随宿主**：OS 桌面常驻诊断窗，follow 为天然主语义；独立
  `auto run` 静态浅色默认。暂停/筛选/尾随/注入业务不变。

## V-04 包源消费腿

同 056 calendar 实证机制（per-file sha256 全体产品同源）；本批在
weather 实证：tampered → 具名拒绝 + design 默认保底；第二有效根照常
驱动（结果回填）。

## 具名未闭面 / 传hold

- **U036 SDK/delivery-root vue-render 最终消费门**：auto-ui specs 具名
  待办（owner=Man 发行/U036 谱系），非本批阻塞——本批与 056 同用
  vendored 固定快照通道（已交付合法消费面），V-04 包源腿在产品实证。
- 主检出观察：`apps/auto-charts` 组合臂漂移（056 F-5 已登记，非本包）；
  本批零触碰。
