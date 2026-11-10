# RQHost 装配与保真（PLAN-063）

本页是 AutoOS 桌面域 **RQ 双 Host 装配/窗口策略/验收边界** 的 canonical 规范：
OS 侧产品装配面（host/desktop + scripts/desktop + 隔离战役基座）、两 Host 管道
语义、attach 预算井盖、与 RQ 路线的验收分工。来源：PLAN-063（r4）实施与实测、
PLAN-074/075 修复交付。未在本页写明的行为不视为已实现。

## 1. 装配面与两 Host 管道（已验）

| Host | 管道 | 客户端语义 | 装配物 |
|---|---|---|---|
| desktop-RQ（虚拟窗嵌载） | session broker：常量 `autodesk-broker`（env `AUTOOS_BROKER_PIPE` 独占井盖） | `--render-mode=desktop --desktop-endpoint=<broker> --autodesk-incubate --app386=<注册表id> --autodesk-broker=<broker>`；宿主 attach 按**注册表 id** 经 `app_resolver` 自装载（客户端不带组件语义） | ui-desktop 注册表/健康门/私 rqhost（`autodesk-rqhost-desktop-<pid>`，OS 专有，DirectChildOnly） |
| standalone-RQ（宿主原生窗） | 私 rqhost daemon pipe（OS 发布为 `AUTO_DESKTOP_ENDPOINT`） | rq/remote 采纳臂（UI063 合同 §1） | 同上 daemon |

- 注册表发现 = 容器臂（apps_dir 扫描）∪ manifest 臂 ∪ 默认 extra-roots 链
  （storage + `../auto-os-config/auto` + `../auto-os/apps` + 真 apps.manifest
  条目）——**井盖只隔离 broker 管道，不隔离注册表发现**（多桌面并存时注册表
  并集，命名冲突由 id 唯一性约束）。
- attach 预算经 `AUTO_BROKER_ATTACH_BUDGET_MS` 可配置（缺席/非法/0 = 5000 原值；
  PLAN-075）；观测行 `[ui-desktop] broker attach budget <n>ms` 仅非默认值打。
  暖轻产品实测 attach ≈1s（N=1/N=8 扫描）。

## 2. 验收边界与具名缺口（不视为通过）

- 产品嵌入扫描（隔离基座）：历史 6/8 落地（calc/clock/charts/reader/monitor/
  photos；timer 品连续流、静态品低帧、泵预算切片无饥饿）。**todo/video 覆盖
  缺口已由 UI077 关闭（2026-11-10）**：Todo 走 queue/Auto 放行（list-none），
  真实 attach 实证 adopt+frame compose；Video 产品公开形态=remote（queue 臂
  保持覆盖门拒绝=契约，防静默错绘）——详见
  [产品投影覆盖](windows-rq-product-coverage.md) 与 [最终验收](windows-rq-final-acceptance.md)。
- M-01 物理上屏终点、M-03 GPU 独占时间、高刷新适用面（本机 59/60Hz）= 具名
  pending；M-02 亚秒粒度已实证（`[P725-FRAME]` ms 单调时钟，AUTO_FRAME_BENCH=1，
  OS066 T-02/05）。代理终点=present/draw_end（呈现真相），物理终点分列不冒签。
- 测量口径：1s 日志上限分辨率已被 ms 级帧时间戳管道取代（scripts/
  rq-final-acceptance/observe.ps1 + metrics.py，OS066 交付）。

## 3. 重建身份

产品 gitlink = 父仓记录组合（OS054 交付 pin）；UI 谱系 = master（074/075 交付后）；
工具链 cargo/rustc 1.98.0。运行身份绑定（哈希+PID）随各战役收据（PLAN-063 §9）。
