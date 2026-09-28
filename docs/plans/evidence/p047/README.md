# PLAN-047 证据包（memo 档 C：动态读拦截 + per-path 版本 + computed 信号网）

> work 阶段证据索引（2026-09-28）。lang 分支 `os-047-dev`，提交链：
> 553f79a0e（T-01）→ f202d01c6（T-03）→ a27be452b（T-04）→ 27c9061ce（T-05）
> → 325a2efff（T-06）→ c47888381（T-07）→ 70595f10c（文档面）。
> 基线：os-047-dev 起点 9b5a10e51（=046 落地 tip）。

## T-01 Recorder 基建（SD-08 桥面）

- 命令：`cargo test -p auto-lang --features ui-iced --lib plan047_recorder`
- 结果：9/9 绿（memo_deps 语义 4：键形态/预算弃整集/absorb 并集+overflow
  传播/边界预算；vm_bridge 通道 5：未激活零录制/字段录制/容器 any/嵌套
  并集/Drop 恢复）。

## T-02 写点普查（决策工件）

- 产物：位点归因分类表 14 位点 + 桥 7 口（E1-E10/N1-N3/B1-B5），入计划
  §8 T-02 证据块。**附带发现**：SET_ELEM ListData 臂与 shim_hashmap_insert_str
  原无任何 seq bump（PLAN-062 遗漏的全局快路径陈旧命中窗口）——T-03 顺路
  闭合。

## T-03 per-path 版本表 + 写点归因（SD-09）

- 命令：`cargo test -p auto-lang --features ui-iced --lib plan047_attribution`
- 结果：5/5 绿——exact 定点性（root.count 前进而 root.label 零扰动）、
  列表 push 仅列表 wildcard 前进（根态 exact/wildcard 零扰动——AC-03 精度
  面）、桥写 exact、hashmap shim 直调（exact+k+wildcard+全局补齐）、C 类
  全局-only 零 path 条目。
- 回归：memo 84/engine 20/vm_bridge 50/plan046 38 全绿。

## T-04 引擎读臂拦截（SD-08 引擎面）

- 命令：`cargo test -p auto-lang --features ui-iced --lib plan047`
- 结果：16/16 绿（新增：引擎槽语义 + handler 端到端 GET 臂录制/去激活零账
  ——`.count` 读经 GET 系读臂编码被影子集捕获实证）。
- 归因注记：`vm::ffi::http_server` 全组 40/1 红 = 基线并行串扰既有 flaky
  （主检出 9b5a10e51 同型复现、单跑绿——046 在册 flaky 家族同形），非本
  计划引入。

## T-05 memo 门 check 三级判定（SD-11 前半）

- 命令：`cargo test -p auto-lang --features ui-iced --lib plan047_gate`
- 结果：4/4 绿：
  - 无关写帧 version_fast 命中（两项条目各计一次）∧ fp_slow 零调用 ∧
    check 帧求解增量 == 快路径帧（AC-02 零重解析口径——pass-1 key 规划为
    每帧合法求解，check 阶段零额外重解析）；
  - fp_slow 命中基线刷新 → 下一无关写帧回 version_fast；
  - 内容真变 → 重求值落地无陈旧（AC-07 正确性下限）；
  - 判定序 seq_fast 优先。
- **执行期正确性三补**（plan046 两测实证抓出后修复）：①静态槽/声明 deps
  估值挪入录制域（漏录 = version_fast 漏失效面 = 陈旧风险）；②extra_dyn
  派生面（sidebar nav 路由 Rust 侧读 = VM 录制盲区）跳过 version_fast；
  ③keyed-for iterable 依赖显式注入（录制域内）。

## T-06 computed 信号网（SD-10）

- 命令：`cargo test -p auto-lang --features ui-iced --lib plan047_signal`
- 结果：3/3 绿——inline 通道（无关写命中/deps 变化重算保真）、block 通道
  （隐藏 VM fn 合成在册预检 + 端到端）、级联闭合（memo 条目收编信号 dep
  面——条目自身不直读该状态亦失效）。

## T-07 降级面收敛（SD-11 后半）

- 命令：`cargo test -p auto-lang --features ui-iced --lib plan047_convergence`
- 结果：1/1 绿——computed-widget menubar memo 化产物与原始路径**逐字节
  一致**（AC-05 对拍）+ 条目在册确证（非静默降级）+ deps 变化经信号级联
  保真（trigger text 闭合态可见面观察）。

## 全量门（AC-01，满载对拍归因）

- 命令：`cargo test -p auto-lang --features ui-iced --lib`（两跑同条件并行）
  - 本侧 os-047-dev@70595f10c：**5504 绿 / 329 红**（gate-output-tip.log）
  - 基线 9b5a10e51（=046 落地 tip）：5498 绿 / 311 红（gate-output-baseline.log）
- **差集归因**（new-reds-full-load.txt）：
  - 新红 22 = **20 条 plan047 新测**（满载红全数同根因：
    `plan492-pkg-repro-m1-canary` 临时目录污染——bar_chart.at 解析错经
    VmBridge::new 串入无关测试；基线同证 29 条既有 vm_bridge 测试同根因
    同报错文本）+ **2 条漂移**（plan632-f4 computed 模块 fn / ffi_dual-019
    i64 pin——**单跑双绿实证**，满载合成/编码串扰）。
  - 反向漂移 4（基线独红本侧绿——负载序翻转）。
  - **零真回归**（AC-01 相对零增量达成；单跑绿口径：plan047 24/24、
    memo 86、plan04 76、vm_bridge 52、engine 20、scoped 全绿在档）。

## 语料 opt-in 现场（auto-os）

- `widgets-gallery/src/front/app.at:701` `outlet (memo: true)`（046 SD-07）
- `widgets-gallery/src/front/components/filetree.at:45` `for r in .ftRows
  key: r.id`（046 SD-05）
- bar_chart.at 多处 `path (key: ...)`（组件内键化列表）
- **档 C 零语料改动**：机制在既有 opt-in 下自动拾取（非 memo 语料零行为
  零开销红线由 T-01 未激活零录制测试承载）。

## 部署观察项（landing≠deployment）

- 本次落地为源码/语料面：桌面宿主经 scripts/desktop.sh 启动时按主检出现
  场构建（debug ui_desktop），下次桌面启动自动拾取档 C 机制；widgets-gallery
  /syslog 语料 App 启动时现读。交互式 A/B 计时留观沿 046 Q-04 先例（测量
  会话与用户实机使用冲突，计数器断言为验收口径）。
