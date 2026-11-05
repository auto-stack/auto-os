# 产品主题批次 061：看板/终端/外部工作区五产品（PLAN-061）

> 来源：auto-os/PLAN-061（2026-11-05 work 执行）。宿主主合同：
> [../shell/host-theme-policy-and-delivery.md](../shell/host-theme-policy-and-delivery.md)；
> 先例批次：[product-theme-batch-060.md](product-theme-batch-060.md)
> （消费语义同源，本页不重复其机制定义）。

## 批次范围与候选（冻结源）

五个实际产品入口（apps.manifest 稳定 ID 边界）：kanban（auto-kanban）、
auto-musk、jade-garden（auto-down 嵌套 front/auto）、auto-term（app/ 根）、
jade-edit。写入检出：kanban/term/jade-edit = `D:/autostack/<repo>` 独立
clone main；musk = `D:/autostack/.wt/musk-061/auto-musk`（分支 plan-061-dev，
其 AGENTS 专属 worktree 规约）；jade-garden = `D:/autostack/.wt/os-061/auto-down`
（分支 os-061-dev，框架嵌套根 owner 树）。

产品候选 SHA（原门+主题验收通过后提交，未 push——push/OS pin 归 merge T-08）：

| 产品 | 候选 SHA | 基线 SHA | 主题套件 | 原门结果 |
|---|---|---|---|---|
| kanban | auto-kanban `a19c4ba0`（main） | 1c967c5a | tests/test_061_theme.py 5/5 | playwright（stdlib 修复在场）10 绿/7 红——红集与 base **逐项相同**（M2-M9 manual 写流 POST 400 missing body，owner=PLAN-043 walkthrough 配方×现工具链）；stdlib/auto fixture=原门复活最小修复 |
| auto-musk | musk-061 `d93d9198` | de92b909 | tests/test_061_theme.py 5/5 | 仓门=worktree 内验证；共存语义实测（下） |
| jade-garden | auto-down `2180f5a8`（os-061-dev） | 7c4d5cbb | front/tests/test_061_theme.mjs 5 腿 | **e2e 24/24 全绿**（官方投影在场） |
| auto-term | auto-term `c9e5e561`+`0677020`（main） | 3573a07a | app/tests/test_061_theme.py 5/5 | cargo workspace 测试级 **49 passed/0 failed**；cargo 进程退出 139 = engine_ffi_integration 收尾段 0xc0000005 崩（测试全绿后卸载路径）——base 3573a07 对拍逐位同败同值（plan-061/auto-os/term-cargo-gates-{run2,base2}.log），预存 owner=autoterm FFI 套件拆解路径 |
| jade-edit | jade-edit `8d2dbf86`（main） | 5fb1e9e2 | tests/test_061_theme.py 5/5 | vm_matrix merged 臂 15 PASS→check15 trash 增强子步败=base probe 对拍同域（base 更早在「回收站」钮败）——预存上游漂移 |

冻结工具链（060 同源）：`auto.exe = auto-man/target/debug/auto`
（`0.1.0+plan-890-verified-r7-t19-18-g03bb65e`）；主题包 managed 源
`auto-ui/packages/themes` 0.2.0，per-file sha256 全 match（pac.at
`2e66212b…`、themes.at `329c2d56…`），聚合 digest `9fa07742…bef838e`；
各产品 vendor 快照同律校验。

## 主题套件腿矩阵（各产品证据在 plan-artifacts/plan-061/auto-os/<app>）

| 腿 | 内容 | kanban | musk | jade-garden | term | jade-edit |
|---|---|---|---|---|---|---|
| A | 显式 AUTO_UI_THEME=dark | #0f0f12 card 精确 | dominant #09090b | --background 240 10% 4%（CSS var 读回） | 状态栏 #0f0f12 精确 | sidebar #0f0f12 精确 |
| B | 显式 light | #ffffff + dark_mode false | light + 品牌紫 177 采样点（PLAN-075 保留） | 真 popover 翻转 → 0 0% 100% + 品牌 238 55% 58% | #ffffff + standalone ◐ 真开关翻回 #0f0f12 | #ffffff 精确 |
| C | Solarized/dark 宿主完整候选 | #073642 精确 | dominant #002b36 精确 | 单族静态通道边界具名（见边界节） | #073642 精确 | #073642 精确 |
| D | 活跃业务中换肤（真实操作+活体外写翻转） | refresh 真实往返+守恒（REAPPLY-NOTE 滞后实录，E 收口） | rail 真实切换+current_view 守恒 | 开档+键入跨模式翻转守恒 | 活体翻转守恒+宿主策略权威实录（policy>app toggle） | 开档+type_text+翻转草稿守恒+保存磁盘字节 |
| E | 重开/重载追最新 | #eee8d5 精确+c_todo 3 fixture 完整 | healthy+#f7f1dd | reload 持久化 | #eee8d5 精确 | #eee8d5+标记磁盘存续 |

采样面注记：kanban 顶带=bg-card；term=状态栏 bg-card 条（终端面=引擎
palette 内容域豁免）；jade-edit=文件树 sidebar bg-card（根 col 透明，窗
dominant=原生背景非主题 token）；jade-garden=CSS var 读回+截图。

## 品牌与内容域豁免（逐产品）

- **musk**：PLAN-075 结构化品牌主题（purple primary/ring）=合法 overlay。
  实测共存语义：默认 boot 品牌紫生效（#6466f1 族）；宿主完整候选
  （family+mode）整体接管——pac theme{} 不压宿主 family/mode。
- **jade-garden**：.theme-<accent> 品牌 accent 预设（默认 indigo
  238 55% 58%）+ --af-* 别名/状态色 + 模式 popover/localStorage 业务全保留；
  核心 role 变量 = 官方投影值。
- **term**：PTY/VT ANSI 调色 = terminal 原生组件内容域豁免不映射；
  ◐ scheme 开关语义保留（统一翻 dark_mode 位，scheme 派生）。
- **jade-edit**：未存稿/草稿保护/撤销/编辑引擎业务面不随肤重染
  （D 腿草稿跨翻转守恒+磁盘字节直证）。
- **kanban**：看板列/卡片区/语言计划数据为业务面；manual 写流属预存红
  （见预存红节），非本批触面。

## 组合与宿主（T-04 实录，kanban+musk 双产品）

- **ISO 隔离腿**：kanban@Solarized/dark 与 musk@Auto/light 同时运行、
  各自独立 AUTOOS_DESKTOP_CONFIG——kanban 角点 #073642 精确、musk
  dominant #ffffff，互不串扰。
- **EXIT owned 退出腿**：taskkill 树收割后 auto.exe 进程数回落基线
  （before=5/during=6/after=5；全程 scoped PID，无宽域清扫）。
- **REJ 错误拒绝腿**：缺件/字节篡改/同 mtime 内容变更三种坏包
  （staged 包目录）逐腿 boot——候选均被拒、回落 legacy/种子链
  （Auto/light 白），boot 存续；字节恢复后 family 重应用
  （#073642 重现）。
- **宿主策略权威注记**：active policy 下 app toggle（term ◐）被
  policy re-assert 压回——053 优先级（宿主快照 > design 默认）的
  正确行为，已实录；toggle 权威域 = standalone/override 轨
  （B 腿实测翻回）。

## 交付渠道（036 安装/生成输入，T-05 V04）

- `man-install.py install --profile full --install-root <scope>/root1`：
  install+verify（结果录 plan-artifacts/plan-061/auto-os/install-root1.log
  /verify-root1.log）；第二绝对根 root2 同流程。
- 910 边界：安装/交付全程在 plan-artifacts scope；staged sdk
  packages/themes per-file sha256 与 managed 源逐位一致。
- jade-garden 生成输入：`.theme-inputs.json` receipt（digest
  9fa07742…bef838e，identity 41ea2af4…）——036 生成输入消费凭证。

## 具名预存红（base 对拍同域，非本批回归）

1. **kanban manual 写流**（M2-M9）：store HTTP POST 配方被 VM back 拒
   （400 missing param `body`）——base 1c967c5 probe 逐项同败；owner=
   PLAN-043 walkthrough HTTP 配方 ×现工具链。stdlib/auto fixture 缺失使
   原门 17/17 环境死（"selected SDK stdlib missing"）——本批提交
   testdata stdlib 复活（现工具链 SDK stdlib 解析要求 AUTO_LANG_ROOT 下
   stdlib/auto）。
2. **jade-edit vm_matrix 深检查**：check15 trash 增强子步（TrashR1/R2
   行未现）——base 5fb1e9e2 probe 同域失败（base 更早在「回收站」菜单
   钮即败）；连同 ws_name 漂移 = wiki-demo fixture 上游演进 vs 锁定清单；
   owner=jade-edit 维护×上游语料。另 arm MCP 口 9381 落 WinNAT 排除段
   9300-9399（netsh 实测）整门环境死——本批以仓内 pick_port.mjs 复活
   （probe 族同款避让件）。
3. **term CLI VM split 形态**：shim extern `term/get_lines` 500
   ("future_all/race: empty or invalid future list") → 独立 CLI 轨无
   PTY、mux/NewTab 不可驱（owner=VM 宿主平台 shims；桌面形态=宿主进程内
   装载不受影响）；app/rust-workspace a2r 构建断（auto-lang `ui-iced`
   feature 漂移，owner=auto-term/a2r）；boot 种子（系统主题）与 Init
   scheme 派生同步缝（◐一次闭合，REAPPLY-NOTE 同族）。原门=cargo
   workspace 49/0 全绿。
4. **jade-garden 单族静态投影**：生成 index.css 四块固定取默认族 Auto
   （tree_index_css_for_app 默认族策略）——第二族（Solarized）需运行时
   切换通道（`__AUTO_TREE_THEMES__`/主题 JS），本批不新增业务；族维度
   = 具名边界（owner=auto-ui 投影扩展 / jade-garden 运行时消费），
   C 腿以负向对照实录（非伪造族腿）。变体构建 derivation 尝试未走通，
   如实记录。

## 与其他批次的边界

- 055 拥有 clock 与真实 OS 事件/outproc 载体（G-1/G-2）；本批不重复接。
- 054 拥有八基础产品业务装配；本批产品非其分母。
- 056-060 拥有各自批次产品；auto-charts 现存组合 mismatch 为他会话 WIP
  （056 域），不代处置。
- UI037/038 教学线不抵本批产品门。
- 056 组合门等 U054 维持原判；本批五产品 OS pin 更新归 merge T-08
  （干净组合 fetch/detach 到候选 SHA、健康门+实际启动后才更 pin）。
