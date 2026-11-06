# 产品主题批次 060：桌面游戏三产品（PLAN-060）

> 来源：auto-os/PLAN-060（2026-11-05 work 执行）。宿主主合同：
> [../shell/host-theme-policy-and-delivery.md](../shell/host-theme-policy-and-delivery.md)；
> 先例批次：[product-theme-batch-059.md](product-theme-batch-059.md)
> （消费语义同源，本页不重复其机制定义）。

## 批次范围与候选（冻结源）

三个实际产品入口（apps.manifest 稳定 ID 边界）：037-klondike（auto-solitaire）、
036-tetris（auto-tetris）、038-minesweeper（auto-minesweeper），全部在
`D:/autostack/<repo>` 独立 clone main（一写入者，实现/独立 review 同检出）。

产品候选 SHA（原门+主题验收通过后提交，主合同 T-05 冻结）：

| 产品 | 候选 SHA | 基线 SHA | 主题套件 | 原门结果 |
|---|---|---|---|---|
| 037-klondike | auto-solitaire `54c0051` | 91192ef5 | tests/test_060_theme.py 5/5 | rules_golden 100%；desktop_mcp 22/26（=base 逐项一致，4 项预存）；rust_golden 12/12（G5-7 按测试内已记录债跳过）；test_vm_klondike exit 0 |
| 036-tetris | auto-tetris `d252f36` | 06b2e67f | tests/test_060_theme.py 5/5 | playwright smoke 3/3（vue 轨）；desktop_mcp exit 0；vm_rules_golden PASS（opening/lock+7×4 旋转+1..4 消行）；gallery_contract 陈旧路径预存（独立 clone 布局）；rust 轨/persistence 借 a2r 具名红（见下）；native_physical 环境阻塞（SetForegroundWindow 拒绝） |
| 038-minesweeper | auto-minesweeper `6873674` | ae6dcbfd | tests/test_060_theme.py 5/5 | desktop_mcp 25/0；rust_smoke 借 a2r 具名红（见下） |

os-060 冻结工具链：`auto.exe = auto-man/target/debug/auto`
（`0.1.0+plan-890-verified-r7-t19-18-g03bb65e`，036 交付+053 机制）；
主题包 managed 源 `auto-ui/packages/themes` 0.2.0，per-file sha256 全 match，
聚合 digest `9fa07742…bef838e`（053 宿主策略固定值逐位一致；三产品 vendor
快照同律校验，交付渠道 root1/root2 staged 包逐文件同值）。

## 主题套件腿矩阵（每产品 5/5，证据在 plan-artifacts/plan-060/auto-os/<app>）

| 腿 | 内容 | solitaire | tetris | minesweeper |
|---|---|---|---|---|
| A | 显式 AUTO_UI_THEME=dark | #09090b 角点精确 | token 占比 40% | card #0f0f12 占比 23.8% |
| B | 显式 AUTO_UI_THEME=light | #ffffff 角点精确 | 12.9% | #ffffff 占比 14.4% |
| C | Solarized/dark 完整候选跟随 | #002b36 角点精确 | 12.8% | card #073642 占比 65.8% |
| D | 活跃局中换肤（真实操作+活体外写翻转） | 必胜局存续；翻转滞后按 REAPPLY-NOTE 记录、重开（E）闭合 | 开始→已暂停存续；活体翻转实测生效 9.2% | 中级盘存续；活体翻转实测生效 67.9% |
| E | 重开追最新 | #fdf6e3 角点精确 | 12.3% | card #eee8d5 占比 60.3% |

业务哨兵全腿守恒：solitaire 发牌/步数/撤销；tetris 就绪/进行/暂停状态机；
minesweeper 雷数 LCD/难度盘面。内容域豁免零重染（牌面红黑/白底、七方块分类色、
雷盘 1-8 色阶、💣/🚩/🙂、LCD 仪表）。

## 组合与宿主（T-04 实录）

- **双产品相反策略隔离腿**：solitaire@Solarized/dark 与 tetris@Auto/light
  同时运行、各自独立 `AUTOOS_DESKTOP_CONFIG`——solitaire 角点 #002b36 精确、
  tetris #ffffff 占比 12.9%，互不串扰；**owned 退出**：taskkill 树 + 会话内
  自收割后 app 进程数=0（本会话基线先行清场，收割计数留证）。
- **错误拒绝腿**：错 digest 完整候选 boot → 具名拒绝保旧实录：
  `[PLAN-053] boot 恢复拒绝：内容 digest 不符（候选与目录快照不一致）——
  候选拒绝（保 legacy 链，family 候选需重新选择）`；boot 存续（设计默认）。
- **包源消费腿（V-04，solitaire 实证、机制全体产品同源）**：缺 manifest /
  字节篡改 / 同 mtime 内容变更（内容锁捕获）三种坏包均具名拒绝
  （`主题包目录不可用（包缺席或校验失败）——候选拒绝（保 legacy 链…）`）、
  boot 存续；字节恢复后 family 重应用（#002b36 角点精确）。
- **OS056-REAPPLY-NOTE**：solitaire 活体外写翻转腿一臂像素滞后（#f9f2e0）；
  tetris/minesweeper 活体翻转均实测生效。合同命名恢复路径"重开"双腿全过；
  053 owner 注记维持，非本批回归。
- **dark_mode 位同步注记**：standalone 跟随臂渲染层应用已提交策略派生
  mode（像素 oracle 过），state 内 dark_mode 位不同步——宿主通道行为归
  053 owner 注记（与 057 frame-0 注记同族），显式 AUTO_UI_THEME 轨位同步
  正常（A/B 腿断言过）。

## 交付渠道（036 安装/生成输入，V-04 渠道腿）

- `man-install.py install --profile full --install-root <scope>/root1`：
  exit 0（9704 文件，install-manifest.json 落盘，sources 记录
  auto-ui 25d4ea5d/auto-man 03bb65e/auto-lang 5f673746/auto-down 7c4d5cbb/
  auto-web 37d32c22）；`verify` exit 0；staged `sdk/packages/themes`
  per-file sha256 与 managed 源逐位一致。
- 第二绝对根 root2：install+verify exit 0。
- 910 边界：安装/交付全程在 plan-artifacts scope，产品 boot 在安装后照常
  （RECOV 腿即安装后 boot），无 target/lease 退化表现。

## 具名预存红（owner=auto-lang 后端生成，非主题回归，均 base 复现）

1. **a2r `??` codegen（tetris）**：`src/back/db.at` `raw[i].to_int() ?? 0` →
   生成 `.parse::<i64>().unwrap_or(0).unwrap_or(0)`（E0599 ×3）——基线
   06b2e67f 同败（probe/tetris-base）。tetris `auto run -r vm`（默认 a2r
   back）受阻；本批主题门走 `auto run -r vm --server vm`（source-README
   既有支持模式）。关闭条件=该模式 a2r 修复。
2. **a2r 前端 codegen（minesweeper）**：store `fc.adjacent` 数值比较生成
   `__at_num(&(… as i32))`（E0308 ×5，&i32 vs &Value）——基线 ae6dcbfd
   同败（probe/mines-base）。rust_smoke（编译 exe MCP 驱动）受阻；VM 轨
   desktop_mcp 25/0 覆盖产品行为。关闭条件同上。
3. **tetris tracked golden fixture 缺席**：`rust-workspace/036-tetris/
   tests/rules_golden.rs` 不在 main 树（allowlist 存在、文件未提交）——
   cargo rules_golden 无从执行；与 1 同 owner 收口。
4. **solitaire desktop_mcp 4 项**（自动收牌 (Auto)/撤销 (Undo) 标签漂移、
   K 占位数、T5 moves 复位）——base 91192ef5 逐项同败（probe/solitaire-base
   22/26 同集），v0.5 交互改动后的陈旧测试面；base=候选零回归证据。
5. **gallery_contract 陈旧路径**：`OS_ROOT=parents[1]` 假设 apps/ 内嵌布局，
   独立 clone 下解析到盘根——独立 clone 时代预存；manifest 登记实质由
   T-01 核验（三产品 id/ports/status/launch=vm）+ 合入健康门覆盖。

## 与其他批次的边界

- 055 拥有 clock 与真实 OS 事件/outproc 载体（G-1/G-2）；本批不重复接。
- 054 拥有八基础产品业务装配；本批产品非其分母，OS pin 按本批自身门更新。
- 056 拥有八生产力/内容产品；auto-charts 现存组合 mismatch 为他会话 WIP
  （非本批产品），不代处置。
- UI037/038 教学线不抵本批产品门。
