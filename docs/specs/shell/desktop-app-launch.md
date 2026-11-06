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

## LD-04 产品目录与启动 ID 分离（PLAN-050）

应用物理目录使用正式产品名（如 `apps/auto-tetris`），manifest `id` 继续是 `036-tetris`。AutoUI 主根扫描、VM/iced 容器展开、Vue 容器展开按容器父级 `apps.manifest` 的 active repo/local 条目的真实目录路径匹配 ID；未匹配或清单损坏时回退目录名。启动 lookup、图标、桌面 pin 和配置仍使用稳定 ID，不能把新目录名作为第二个入口重复注册。空子模块保持跳过。此能力需要包含 PLAN-050 注册表变更的 AutoUI 工具/宿主；更新后重建旧二进制再验证启动。

## LD-05 原生共享 Rust HTTP 服务的桌面归属（PLAN-054，待本计划最终复审）

`apps.manifest` 的 `native_http: true` 显式选择需要独立共享HTTP服务的原生应用，目前为017-chat。`launch: native` 本身不意味着独立服务；其他应用已有内嵌后端和VM供给保持原路径。启动声明必须active/native，实际聚合注册表选中的应用根必须存在pac.at、`api: "rust"`和唯一字面量back_port，端口必须包含在manifest ports中。基址仍由正式app构建绑定该pac profile，不向宿主或其他app写入HTTP基址。

AutoOS iced宿主在展示启动器前，以既有auto service Rust入口准备这些服务；使用既有auto-tool-execution的无shell/自有进程树设施。只认本次进程stdout的实际AUTO_SERVICE_READY及loopback健康响应中同一个instance/config/fingerprint，不收编已有监听者。端口占用、缺源/错误profile、提前退出、READY超时或身份不符都使启动失败；不换端口、不解释降级、不杀占用者。

持有者是桌面实例，最后一个应用窗口关闭后仍保留服务，便于重新打开/多客户端共享。桌面事件循环正常返回时，核对当前服务仍为本次READY身份，调用既有loopback shutdown并等待实际服务CLI正常退出。身份变化时不向未知监听者发送关闭；失败只取消本次自有进程树，返回具名错误，不记作正常释放。Vue沿其既有desktop_backends供给实现，不由本原生入口重复启动。

## LD-06 产品私有 rqhost 的有限归属（PLAN-054 r3）

AUTO_DESKTOP_RQHOST=1时，OS入口通过现有auto CLI启动本次私有PID端点的rqhost，持有DirectChildOnly执行句柄。启动前拒绝已占用端点，就绪后发布AUTO_DESKTOP_ENDPOINT并抑制共享框架的重复boot-spawn；禁用时不启动或改端点。它不收编全局/手工-q服务。

正常桌面事件循环返回后，OS取消并有界回收自己的空闲helper，再drain共享HTTP后端。helper没有正常shutdown接口，记录实际controlled cancellation和已回收状态，不能称为helper正常退出0；helper已自然退出0可正常收尾，意外非零退出或回收失败使产品退出具名失败。DirectChildOnly不创建KILL_ON_JOB_CLOSE Job，保留外部终止跳过析构时helper不级联的兼容边界；全局/手工-q生命周期未修改。

资源验证包含真实子进程就绪/关闭/Drop、占用者保存、提前退出与观测过监听后的超时回收，以及父process exit跳过析构后子监听仍存活的等价类（测试专用quit回收）。该等价类不冒称用户实际外部强杀。正式桌面须另外证明本次helper只有一个、normal退出后自有进程/端点消失且旁观者和应用数据不受影响。
