# 产品主题批次 058：媒体三应用（PLAN-058）

> 来源：auto-os/PLAN-058（2026-11-04 work 执行）。宿主主合同：
> [../shell/host-theme-policy-and-delivery.md](../shell/host-theme-policy-and-delivery.md)；
> 先例批次：[product-theme-batch-057.md](product-theme-batch-057.md)
> （消费语义同源，本页不重复其机制定义）。

## 批次范围与候选（冻结源）

三个实际产品入口（apps.manifest 稳定 ID 边界）：020-music-player、
030-video-player、029-photo-gallery。

os-058 冻结工具链（组树 detached，全部 clean）：

| 树 | pin |
|---|---|
| auto-os | 31bdcfd（分支 codex/plan-058-product-themes） |
| auto-man（auto.exe 来源） | 2bbd954（组树构建，sha256 前 16 位 `491d568b00473899`，版本 `0.1.0+plan-890-verified-r7-t19-17-g2bbd954`） |
| auto-lang | 2f558fc93 |
| auto-ui | 3c46104b（包源 per-file sha256 与 056/057 快照同律） |
| auto-web | 3de5524 |
| auto-down | 7c4d5cbb |

**video 例外线（具名红②）**：video 的 VM 轨在冻结线 fatal
（`playlist.at:124` `caption_text`——子模块 `use stylekit.styles:` 导入，
plan-446 C1），主检出链（lang main 内嵌构建）boot 正常；本批 video 门
（主题反控+vm-smoke）跑主检出链 auto.exe（auto-man main 检出构建）。
两臂 A/B 同源同证，owner=auto-lang 解析线；冻结线兼容由该 owner 处置。

主题包源（managed channel 唯一可编辑源 `auto-ui/packages/themes`）：
pac.at `2e66212b…`、themes.at `329c2d56…`、聚合 digest
`9fa07742dc68aa53…6bef838e`——三产品 vendor 快照逐字节校验同律。

产品候选表（候选 SHA 于各产品原门+主题验收通过后回填）：

| 产品 | 独立 clone 候选 SHA | design 默认腿 | VM 门 | 原门 |
|---|---|---|---|---|
| 020-music-player | auto-music `4b9e446` | dark_mode 位派生（pac dark 退役；现 UI 无明暗按钮=458 env 链驱动） | 反控**双腿像素精确**（dark #09090b / OS 跟随 #ffffff）+ 受管夹具真实 scan/播放哨兵 | test_020_vm.py **exit 0**（walkthrough 容错） |
| 030-video-player | auto-video `0922b44` | dark_mode 位派生（pac dark 退役；顶栏 ToggleDarkMode 原样） | 反控**双腿像素精确**（队列面板 card #0f0f12 / #ffffff）+ 诚实降级不变量 | vm-smoke.mjs **ALL PASS**（A-F 诚实降级） |
| 029-photo-gallery | auto-photos `7ce8bd2` | dark_mode 位派生（历来无 theme 键；ToggleDark/SetAccent 原样） | 反控**双腿像素精确**（画布 #09090b / #ffffff）+ 真实 scan 哨兵 + **真实 full 链**（OpenPhoto→`/api/photos/full/:id`→回网格） | **本仓首个永久回归门**（原无 tests 根，058 §4 裁定补齐） |

## 具名预存红（A/B 双臂同证，非本批引入；owner 域不在本批）

1. **media 能力臂后端生成红（video）**：`auto service --server rust` 与
   boot 期后端拉起均报 `backend Cargo.toml missing under
   rust-workspace\auto-video-db65e00f-back (generate_api reported
   success?)`——media_root 能力臂（无 src/back/api.at 形态，PLAN-617
   T-11 承诺面）在冻结线与主检出线两臂同败；photo 能力臂（photos
   17833）同机制正常。**owner=auto-lang 后端生成（media 能力臂）**；
   关闭条件=该臂生成完整工作区 + video 真实 scan/播放门
   （tests/smoke.spec.ts）绿。本批不削减该承诺、不以空视图代 PASS。
2. **冻结线子模块 dep-style 导入解析红（video）**：见上节例外线。

## 组合与宿主（T-04 实录）

- **V-03 双产品相反策略腿**（standalone follow 臂，AUTOOS_DESKTOP_CONFIG
  隔离 config + 冻结包根 AUTOOS_THEME_PACKAGE_DIR）：
  music@Solarized/dark → 主区像素 **#002b36（Solarized dark 精确值）**；
  video@Auto/light → 队列面板 **#ffffff**；双产品同时运行相反策略
  不串（像素各自保持）；坏 digest 候选 → **具名拒绝保旧**（"内容 digest
  不符——候选拒绝（保 legacy 链）"实录，回 design/legacy 链 #ffffff）；
  策略文件改写 Auto/dark → **重开追最新 #09090b 精确**、对侧 video 零串扰。
  **OS056-REAPPLY-NOTE 复现**（live 外写臂 #051a21 混合帧滞留不落定，
  053 owner 注记维持，非本批回归——重开恢复腿实测过）。
  逐腿像素证据 `plan-artifacts/plan-058/auto-os/hosted/`。
- V-04 包源消费腿：per-file sha256 准入机制全体产品同源（036 交付）；
  vendor 快照不可二次编辑纪律见各产品 SD-03 页。

## 与其他批次的边界

- 055 拥有 clock 与真实 OS 事件/outproc 载体（G-1/G-2）；本批不重复接。
- 054 拥有八基础产品业务装配；本批三产品非其分母，OS pin 按本批自身门
  更新（fetch/detach 同 SHA + 健康门 + 实际启动后提交）。
- UI037/038 教学线不抵本批产品门。
