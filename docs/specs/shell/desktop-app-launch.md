# Spec: 桌面 app 启动方式声明（apps.manifest `launch` 字段）

> Source of truth for the per-app launch-mode declaration introduced by
> PLAN-043（2026-09-26 落地，lang `1d597e3f4`）。Host implementation:
> auto-lang `ui/app_registry.rs`（`manifest_launch_lookup`）+
> `ui/session.rs`（`launch_app` 分派门）。Data: auto-os `apps.manifest`。

## LD-01 声明与查表

- `apps.manifest` 条目增 `"launch": "vm" | "native"`（可选；缺席 =
  向后兼容旧行为：desktop_exe 存在即原生附着）。
- `manifest_launch_lookup(parent, name)`：单名查表（daemon lookup 同款
  独立读取，不依赖注册表构建）；**键 = 注册表 id**——manifest 条目 id
  必须与注册表 id 对齐（036-tetris 先例：目录名/manifest id 错位整条
  查询落空）。
- 语义：声明 `vm` 的 app **恒 inproc 解释渲染**（编译产物存在也不走
  原生附着）；`native` 或缺席维持原判定链（`outproc_native_exe` 探测 →
  SHM 像素桥附着，Plan 020 血统通道）。

## LD-02 版本裁定（2026-09-25 用户）

- 本版**纯 VM 集成模式**：apps.manifest 10 条目全声明 `vm`；RQHost /
  desktop 方言统一、中文渲染、图标字体等原生形态完善**下版本再做**
  （PLAN-043 移交清单 11）。
- 动机分离：开发 RQ 仍可生成 exe（a2r 编译链不受影响）；打包测验以
  VM 声明为准——启动形态从「产物存在即切换」改为「按声明分派」。

## LD-03 原生形态已知债（native 声明的前置门槛）

- a2r codegen 两债（lang/auto-man 深水，独立立项）：①store 数组 push
  静默失败（klondike 原生 exe 明牌牌面全空——发牌 down 计数同源赋值
  生效、七列数组视图期空）；②视图 if 条件位对 record 字段访问按字符
  串降链（应 bool→as_bool；klondike main.rs 盘上直修在案，再生回退）。
- 旧 exe 与桌面二进制合成协议可能不匹配（进程活窗不出）——原生 exe
  需随桌面协议重建。

## 验证

- 实机（0925/0926，VM-only 桌面）：tetris/klondike inproc 解释渲染可玩
  （.at 游戏 UI 桌面内完整）；launch 查表按注册表 id 命中（tetris id
  对齐前后对照）。
- 回归：缺省（无 launch 字段）行为与旧判定链一致（outproc-native 探测
  语义不变）。
