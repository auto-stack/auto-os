---
plan_id: PLAN-044
status: archived              # drafting → executing → execution_done → reviewed → archived
feature_name: config-write-safety
author: [agent]
created_at: 2026-09-23
updated_at: 2026-09-23
plan_revision: 1
current_step: 7
total_steps: 7

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: [docs/specs/shell/state-files.md]
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [docs/specs/shell/state-files.md, docs/specs/shell/showdesk-icons.md]
---

# [PLAN-044] config-write-safety

## 0. 变更摘要

全系统用户级配置/状态文件（`~/.config/autoos/` 下）目前**零原子写、零跨进程锁**：
所有写者都是「load-once 内存态 + `fs::write` 全量覆盖」。多桌面实例、outproc 子进程
（launcher/sysmon/native shim 全继承 `AUTO_VM_STORAGE_FILE`）、os-config daemon 与宿主
双写 config.at——任意交错都产生丢更新或撕裂文件。实机症状：重启后壁纸回退纯色、
桌面快捷方式缩水且不自愈、图标掉 lucide fallback（2026-09-23 前置诊断会话定案）。

本计划交付一个**状态文件安全工具层**（auto-lang `state_file` 模块：原子替换 + 跨进程
文件锁 + 键级合并写），应用到两个高危文件（desktop-storage.json、desktop/config.at），
并补 `shell.desktop.icons` 缺键重播种自愈；机械推广原子写到 musk 一族 / auto-ai-cli
sessions / os-config daemon 全部写点。不引入新常驻 daemon（单写者 daemon 化记为债）。

设计参照系：macOS cfprefsd（单写者 daemon）、GNOME dconf（单写者 + journal/compact）、
Windows Registry（内核仲裁 + 值级粒度）、Chromium ImportantFileWriter（temp+flush+rename
原子替换 + .backup，无 daemon）——见 §4.2。

## 1. 目标

1. **撕裂免疫**：任意写者在写盘中途被杀（断电/SIGKILL），下次读取要么完整旧值要么
   完整新值，永不出现空库/半截 JSON（"坏文件静默空库"路径根除）。
2. **并发不丢更新**：两个进程对同一状态文件交错写**不同键**，两个键都存活；
   config.at 宿主臂与 daemon PUT 交错写不同字段，双方字段都存活。
3. **快捷方式自愈**：`shell.desktop.icons` 缺键/空 → boot 自动恢复预置集；
   用户 hidden 排除不受影响。
4. **半径收敛**：desktop-storage.json 写半径从"全文件"缩到"本进程脏键"。
5. 机械面：musk / auto-ai-cli / os-config daemon 的用户级状态写点全部原子化
   （不引入跨进程锁，双实例互踩记债）。

**非目标**：不做单写者 daemon（cfprefsd/dconf 形态，D-债预留）；不解决 musk 双实例
并发；不动壁纸死路径 boot 回退链（另卡）；不动 `shell.desktop.hidden` 语义；
不改 storage 的 `AUTO_VM_STORAGE_FILE` 未钉扎时的 CWD 哈希隔离行为。

**受影响仓**：auto-lang（crates 运行时：stdlib.rs / desktop_config.rs / renderer boot；
新 util 模块）、auto-os（plan 主导仓）、auto-os-config（daemon collection.rs / core.rs）、
auto-musk（backend 5 个 writer）、auto-ai（auto-ai-cli session.rs）。跨仓计划主导仓 =
auto-os，两仓 README/计划互链。

**症状归因边界**：三症状中的「图标掉 lucide fallback」根因在启动上下文（worktree 的
AUTO_OS_ROOT/兄弟仓探测/submodule 缺席 → 未注册 id 回退 `app-window`，renderer.rs:14730
既有语义），**不是状态文件问题**——本期不改该链路，诊断结论记档另卡。本期根修的是
前两个症状（壁纸回退、快捷方式缩水）及同类全系统病灶。

## 2. 架构方案

新增 auto-lang 共享工具模块 `state_file`（放置：`crates/auto-lang/src/util/state_file.rs`
或既有 util 惯例位置，执行期按仓内布局定），三层能力：

```
atomic_write(path, bytes)          L1 原子替换：同目录 <name>.tmp-<pid> 写+flush
                                   → std::fs::rename（Windows=MoveFileExW
                                   REPLACE_EXISTING，同卷原子替换）
with_lock(path, |…|)               L2 跨进程互斥：<file>.lock 独占创建
                                   （O_CREAT|O_EXCL）；锁体记 pid+时间戳；
                                   超时（缺省 2s）或 stale（持锁进程已死/超龄
                                   10s）接管；获取失败 best-effort 降级
                                   （放行+eprintln，可用性优先）
merge_write_kv(path, dirty_keys)   L3 键级合并写（desktop-storage.json 专用）：
                                   锁内重读磁盘 map → disk ∪ 本进程脏键 → 原子写。
                                   未触键以盘上新值为准，脏键本进程胜
```

应用点：

- **desktop-storage.json**：`STORAGE_MAP` 追加脏键集追踪；`storage_persist` 改为
  `with_lock → merge_write_kv → atomic_write`。所有写者（宿主 renderer 各臂、VM .at
  storage.set、native shim、outproc 子进程）经同一 stdlib 入口，自动全覆盖。
- **desktop/config.at**：宿主 `desktop_config::save` 与 daemon `write_file_module` /
  `put_entity_json` 共用同一 `<config.at>.lock`；save 前 mtime 预检——与上次热应用
  记录不一致则先 reload 再改（现有 400ms 轮询升级为写前强制检查）；两写者全部换
  原子替换；daemon 的 `.bak` 保留但改为「原子替换前锁内拷贝旧内容」，消除 bak 撕裂。
- **icons 自愈**：预置表（PLAN-018 的 11 id 现值）提升为代码常量单源；宿主 boot 时
  `shell.desktop.icons` 缺键**或空** → 播种并写回（写回走新安全写路径）；
  `hidden` 为空串是合法用户态，不播种。
- **机械推广**：musk（users/workspaces/specs/chats/professions 五 writer）、
  auto-ai-cli（sessions/<hash>.json）、os-config daemon（上条已覆盖）替换为
  各仓本地 `atomic_write` 最小替身（约 20 行，不引跨仓依赖）；统一记债后续单源化。

## 3. 技术栈

Rust（std only：`std::fs`、`OpenOptions`、`fs::rename`；不引新 crate——`tempfile`/
`fs2` 等不必要）。测试：cargo 单测（并发线程交错、锁接管）+ 集成测试（双进程交错写）
+ 既有 playwright/实机验证链。

## 4. 需求分析与背景调查

### 4.1 授权与范围

用户 2026-09-23 授权："立个小项目来处理……不止是桌面的状态并发问题；整套 AutoOS
的 config 都可能有这个问题。请检查其他的操作系统是如何解决配置文件的并发写安全的。"
→ 范围：全 `~/.config/autoos` 状态文件写安全 + 桌面前两症状根修；预算：小计划
（7 任务）；跨仓动作：auto-lang / auto-os-config / auto-musk / auto-ai 四仓代码 +
本仓 plan/spec。自动继续：work handoff 常规授权。

### 4.2 其他 OS/系统的配置并发写安全方案（调研结论）

| 系统 | 机制 | 对本计划的启示 |
|---|---|---|
| **Windows Registry** | 配置存内核管理的 hive；写入以**值（value）为粒度**，内核锁串行化写者；`RegNotifyChangeKeyValue` 提供变更通知 | ① 键级粒度把并发丢更新半径从文件缩到键（本计划 L3 同理）；② 写仲裁交给单一权威 |
| **macOS cfprefsd**（10.9 起） | 所有 plist 偏好读写经单一 daemon 串行化；daemon 持权威内存缓存，是唯一写者；**引入动机正是多进程直写 plist 的丢更新**；绕过 daemon 直改文件会被缓存回写冲掉 | 单写者 daemon 是 OS 级正解；绕过 API 直写文件被定义为未定义行为。AutoOS v1 不建 daemon，故用 L2 文件锁模拟"串行化写者" |
| **GNOME dconf / gsettings** | 部分客户/服务端：读=客户端直 mmap GVDB 二进制库；**写=全部经 dconf-service（D-Bus 单写者）**；写路径=先追加 journal 再 compact 进主库（崩溃安全两阶段） | 读路径无锁直读 + 写路径单点串行；journal/compact 思路 ≈ 本计划"原子替换"（整库换新文件） |
| **Chromium ImportantFileWriter** | temp 文件写+flush+rename 覆盖（Windows 用 `ReplaceFile`，支持备份语义）；写聚合延迟批量；`Preferences.backup` 单代备份 | **无 daemon 时的工业标准答案**：原子替换 + 备份 + 延迟聚合。本计划 L1 及 daemon .bak 改造同型 |
| **git `index.lock` / apt** | 跨进程 RMW 用独占 lockfile（O_CREAT\|O_EXCL）+ 陈旧锁清理 | L2 锁文件的便携实现范式 |
| **SQLite/WAL** | 单文件 + WAL：内建并发读者/单写者、ACID | 备选方案（状态库 SQLite 化）——重构成本高，v1 不取，记为远期选项 |

共识模式：**原子替换（人人都有）→ 跨进程串行化（锁或单写者仲裁）→ 粒度收敛
（registry 的值级 / dconf 的键级）→ 备份与自愈（.backup / journal 重放）**。
AutoOS 现状是四层全缺；本计划补齐前三层的轻量版 + 第四层的自愈件。

### 4.3 现状盘点（2026-09-23 实测，写点清单）

**高危（本期根修）**：
- `desktop-storage.json`：唯一磁盘写盘点 `storage_persist`（auto-lang
  `crates/auto-lang/src/vm/ffi/stdlib.rs:689`），`Mutex<STORAGE_MAP>`（:647）仅进程内；
  写者=宿主 renderer 各臂 + **所有继承 env 的 outproc 子进程**（spawn_exe_child
  session.rs:3108 不剥 `AUTO_VM_STORAGE_FILE`）+ VM .at storage.set
  （shell/desktop.at:550,562、apps/028-launcher app.at:873-877、025-sys-monitor
  sys_store.at）+ native shim（vm/native.rs:8449,8522）。全量覆盖 + load-once
  `or_insert` → 交错必丢。
- `apps/desktop/config.at`：宿主 `save`（`src/ui/desktop_config.rs:370-382`，10 个
  execute_* 触发臂全在 renderer.rs：12010/12020/12513/12533/12564/12818/12825/12838/
  12862/12968）与 daemon `write_file_module`（auto-os-config-back core.rs:52-104）/
  `put_entity_json`（collection.rs:237-280）双进程 RMW；宿主 400ms mtime 轮询
  （config_mtime :129）只护读侧；`wallpaper_request` 差分通道（desktop_config.rs:62-67）
  证明双写者是设计特性，但无任何护栏（该文件「单写方」注释已过时）。

**自愈缺口**：`shell.desktop.icons` 为 PLAN-018 一次性预置（11 id），缺键不重播种、
无运行时添加 UI——一次空库覆盖即永久缩水。

**机械面（本期原子化，不上锁）**：musk `save_users`（auth.rs:132）/`write_index`
（workspace.rs:222）/`SpecsStore::save`（specs.rs:783）/chats.rs:460/`save_professions`
（relay/profession.rs:135）；auto-ai-cli `session.rs:47-72`；daemon 写点并入 T-03。
已安全不需动：llm-rejects（唯一文件名 append）、relay handoff/task-plan（按 id 分流）、
skills/modules.d（v1 只读）。

**全系统现状**：零原子写、零跨进程锁；仅有的并发感知 = 宿主 config mtime 轮询（读侧）。

### 4.4 症状↔根因对照（前置诊断）

| 症状 | 根因 | 本期修复件 |
|---|---|---|
| 重启后壁纸回退纯色 #101014 | 宿主陈旧内存快照 save() 全量覆盖（或 daemon 交错），把图片壁纸路径冲回旧值 | T-03（锁+mtime 预检+原子写） |
| 快捷方式少了且不自愈 | storage 空库/丢更新覆盖 icons 键；预置不重播种 | T-02 + T-04 |
| 图标掉 lucide fallback | 启动上下文相关（worktree 的 AUTO_OS_ROOT/兄弟仓探测/submodule 缺席→未注册 id 回退 `app-window`）——**非状态文件问题**，本期不改代码；诊断结论记档，另卡 | 不在本期 |

## 5. 详细设计

### 5.1 `state_file` 模块（T-01）

```rust
pub fn atomic_write(path: &Path, bytes: &[u8]) -> io::Result<()> {
    // 1. tmp = <dir>/<name>.<ext>.tmp-<pid>（同目录 ⇒ 同卷，rename 原子性成立）
    // 2. File::create(tmp) + write_all + sync_all（flush 落盘）
    // 3. drop 后 fs::rename(tmp, path)（Windows=MoveFileExW REPLACE_EXISTING）
    // 4. 失败清理 tmp；rename 后不追 sync 目录（v1 取舍，注释声明）
}

pub fn with_lock<T>(target: &Path, timeout: Duration, f: impl FnOnce() -> T) -> LockOutcome<T> {
    // lock = <target>.lock；OpenOptions new+write+create_new 独占创建
    // 成功：写 pid+epoch；执行 f；释放删锁
    // AlreadyExists：重试至 timeout；锁体 pid 已死或 age>10s → 接管
    // 超时：Proceed（best-effort 放行 + eprintln）——降级只跳过互斥不跳过合并
}
```

并发测试用 `std::thread` 交错覆盖进程内语义；跨进程语义用集成测试 spawn 两个测试
子进程对同一文件交错写（见 §6）。

### 5.2 storage 脏键合并写（T-02）

- `STORAGE_MAP: Mutex<HashMap<String,Value>>` 追加 `DIRTY: Mutex<HashSet<String>>`；
  所有 `storage_set` 臂同步 insert 键名进 DIRTY。
- `storage_persist`：锁内 `disk = storage_read_fresh_unlocked()`（直读盘，绕过
  load-once 缓存）→ `merged = disk ∪ {k∈DIRTY → memory[k]}` → `atomic_write` →
  `DIRTY.clear()`。
- `storage_host_read` 的 load-once + `or_insert` 语义**保持不变**（读侧进程内写恒胜是
  既有契约，PLAN-024 R7）；合并写在写侧兜住丢更新，读侧不感知。
- 失败路径：锁超时降级 Proceed 时**仍用 merge 语义**（重读磁盘再并）；读盘失败
  （JSON 坏）→ 以内存 map 全量重建 + `<file>.corrupt.bak` 留证。
- 迁移注意：`LEGACY_STORAGE_KEYS`（desktop_config.rs:32-41）只读回退不受影响。

### 5.3 config.at 双写者护栏（T-03）

- 宿主：`DesktopConfig::save` → `with_lock(<config.at>.lock)` 内：stat mtime ≠ 上次
  热应用 mtime → 先 `load()` 重读（external diff 热应用同款路径）再应用本次字段改动
  → 序列化 → `atomic_write`。修正 desktop_config.rs 过时的「单写方」注释。
- daemon：`write_file_module` / `put_entity_json` / `delete_block_json` 已是"读盘 AST
  merge 写回"，补同一把 `.lock`（路径约定与宿主一致：`<target>.lock`）+ 原子替换；
  `.bak` 改为锁内、原子替换前 `fs::copy(target, bak)`。
- 跨仓约定：锁文件名与 stale 规则在 state_file 模块 doc 注释单源；auto-os-config
  是独立 Rust 仓不依赖 auto-lang → 本地最小替身（同路径约定），doc 注释互指。

### 5.4 icons 缺键重播种（T-04）

- 预置表常量 `DEFAULT_DESKTOP_ICONS: [&str; 11]`（盘上现值 = PLAN-018 预置集实况）：
  `011-calculator, 012-clock, 013-todo, 014-weather, 015-notes, 020-music-player,
  028-launcher, 029-photo-gallery, 030-video-player, jade-garden, auto-musk`。
  跨仓 id 注册缺失只影响图标回退、不影响条目存在（renderer.rs:14730 既有语义）。
- boot 链（renderer 桌面快照装载点，`build_desktop_surface_snapshot` 前）：icons 键
  **缺**或空串 → 以常量播种 → 走 T-02 安全写路径落盘（后续重启不再触发）。
  `hidden`/`positions` 不播种。

### 5.5 机械推广（T-05）

五仓写点换各仓本地 `atomic_write`（同 §5.1 算法 20 行替身，不引新依赖）：
musk auth.rs/workspace.rs/specs.rs/chats.rs/profession.rs、auto-ai-cli session.rs；
daemon 两个文件已在 T-03。**不**给这批上锁（单进程内存态为主；musk 双实例互踩 =
已知限制，记 D-债）。

### 5.6 规范增量

| delta_id | add/modify/retire | docs/specs/ 目标 | before/after 规则 | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/shell/state-files.md（新模块 spec） | before:（无 spec——状态文件写安全无契约）；after: `~/.config/autoos` 状态文件写契约——全部经原子替换；desktop-storage.json 与 config.at 跨进程写必须持 `<file>.lock`；storage 持久化=键级合并写（盘∪脏键）；icons 缺键播种；锁超时 best-effort 降级语义 | 四层全缺的根修；对齐 OS 级先例（§4.2） | AC-01..04 |
| SD-02 | modify | docs/specs/shell/showdesk-icons.md | before: 桌面图标集=icons−hidden，icons 为一次性预置、缺键=空桌面；after: icons 缺键或空 → boot 以 `DEFAULT_DESKTOP_ICONS` 播种并落盘；hidden 空串为合法用户态不播种 | 自愈缺口 | AC-04 |

## 6. 测试设计

1. **单测（state_file 模块）**：原子写 happy path / tmp 清理 / rename 覆盖已存在文件；
   锁独占创建、超时降级、stale 接管（伪造 pid+旧时间戳锁体）；8 线程 × 100 轮交错
   atomic_write 无撕裂。
2. **单测（storage 合并写）**：预置磁盘 map{A,B} + 内存脏{A',C} → persist 后
   {A',B,C}；坏 JSON 磁盘文件 → 重建 + `.corrupt.bak`。
3. **集成测试（双进程交错）**：spawn 两个测试进程循环写不同键各 N 次 → 终态两键均为
   最后写值（修复前必丢一，作为红→绿对拍）；config.at 同型（宿主臂模拟 + daemon
   PUT 模拟交错）。
4. **撕裂注入**：子进程 atomic_write 循环中 SIGKILL → 父进程读回恒为合法 JSON
   （100 轮）。
5. **icons 自愈**：storage 删 icons 键 → boot 装载 → 键恢复 11 id；hidden="a,b" +
   icons 空 → 播种 icons 且 hidden 保留。
6. **回归门**：auto-lang worktree `cargo t`（改 crates ⇒ Category A 允许且要求在
   lang worktree 跑）；本仓既有 app e2e 抽样（launcher/sysmon）红不增；实机桌面 +
   launcher + sysmon 并行跑一轮后图标数/壁纸/主题不回退（截图三轮对拍，沿 PLAN-042
   验证链）。

## 7. 验收标准

- **AC-01 并发不丢键**：双进程交错写不同键的集成测试终态两键均存活（§6.3），
  命令 `cargo t -p auto-lang --test storage_cross_process`（名以执行期为准），预期绿。
- **AC-02 撕裂免疫**：SIGKILL 注入 100 轮读回全部为合法 JSON（§6.4）。
- **AC-03 config.at 双写者**：宿主/daemon 交错模拟测试双方字段均存活（§6.3）。
- **AC-04 icons 自愈**：删键→boot→恢复预置集；hidden 不被误清（§6.5）。
- **AC-05 机械面收口**：musk/auto-ai-cli/daemon 写点 grep 门——目标清单内
  `fs::write` 直写归零（豁免：append-only 与按 id 分流两类）。
- **AC-06 实机不回退**：实机三轮对拍（桌面图标数=11−hidden、壁纸字段稳定、主题
  manual/stella 稳定），红不增。

## 8. 执行步骤

> 布局：worktree 组 `D:/autostack/.wt/os-044/{auto-os,auto-lang}`（Plan 529；
> 本计划改 auto-lang crates ⇒ 组内补依赖 lang worktree，cargo t 只在 worktree 跑）。
> 先在 main 提交 `.next-id` 与本 plan，再建组。
> **执行期实建组（6 成员）**：auto-os（plan-044-dev）、auto-lang（os-044-dev）、
> auto-down（detached 只读——lang workspace 路径依赖）、auto-os-config/auto-musk/
> auto-ai（各 os-044-dev——musk 的 auto-lang/auto-ai 路径依赖经组内兄弟解析命中）。

- **T-01** `state_file` 工具模块（auto-lang）：`atomic_write` + `with_lock` + 单测
  （§6.1）。依赖：无。验证：`cargo t state_file` 绿。→ AC-01,02 基座
  [✅ 已完成] lang os-044-dev 1df3b6597；cargo t state_file 6/6 绿。执行期修一真
  bug：create_new→写锁体空窗期空体被判极老→**活锁误删**（4×25 RMW 计数 99/100
  实录）——空体不参与 epoch 龄判定、stat 失败不改判 stale。
- **T-02** storage 脏键合并写（auto-lang stdlib.rs）：DIRTY 追踪 + `storage_persist`
  改造 + §6.2 单测 + 双进程集成测试（§6.3 storage 腿）。依赖：T-01。→ AC-01,02
  [✅ 已完成] lang os-044-dev 4f4c6952e；新增 tests/storage_cross_process.rs——
  AC-01 修复前红实录 `{"xproc.a":"v0","xproc.b":"v199"}`（B 全量覆盖冲掉 A 的键）
  →修复后两键 v199 绿；AC-02 SIGKILL×5 读回恒合法 JSON 绿；stdlib 单测 2
  （合并语义/坏库 .corrupt.bak 自愈）绿；既有 storage 测试族 100/100 绿。
  执行期勘定：`cargo t` 别名钉 4 二进制白名单（--lib + schema_drift/docs_gen/
  component_registry_test），新增集成测试目标须裸 `cargo nextest run --test X`
  直跑。
- **T-03** config.at 双写者护栏：auto-lang desktop_config.rs（锁+mtime 预检+原子写+
  注释修正）+ auto-os-config core.rs/collection.rs（同名锁+原子替换+.bak 锁内拷贝）
  + 交错测试（§6.3 config 腿）。依赖：T-01。→ AC-03
  [✅ 已完成] lang 侧 ffa4838b2（save() 锁内字段级 diff 合并——LAST_SNAPSHOT 基线，
  daemon 轮询间隙外写字段不被陈旧快照冲掉；save_merges_external_field_changes 绿）；
  daemon 侧 core.rs/collection.rs 已改（put/delete 整段 RMW 锁内 + .bak 锁内原子
  拷贝）。**执行期修正：auto-os-config-back 与 musk 本就 path 依赖 auto-lang（组内
  兄弟解析命中 os-044 worktree），直接复用 `auto_lang::state_file`，比计划预设的
  「本地最小替身」更单源**。daemon cargo check 在途。
- **T-04** icons 缺键播种：常量单源 + boot 播种 + §6.5 测试。依赖：T-02。→ AC-04
  [✅ 已完成] lang 侧（同 ffa4838b2）：DEFAULT_DESKTOP_ICONS 11 id 常量 +
  ensure_desktop_icons_seeded 挂 renderer boot（hole_mode 装载点后）；
  desktop_icons_seed_only_when_missing 三态绿。
- **T-05** 机械推广：auto-musk 五 writer + auto-ai-cli session.rs 换本地 atomic_write。
  依赖：T-01（算法范式）。→ AC-05
  [✅ 已完成] musk os-044-dev 5ed25be（五写点→`auto_lang::state_file::atomic_write`，
  cargo check -p musk 过）+ auto-ai os-044-dev 75bbf19（session.rs 本地 20 行同算法
  替身，cargo check -p auto-ai-cli 过）；AC-05 grep 门：目标清单生产写点 fs::write
  直写归零（豁免：musk 测试 fixture、append-only/按 id 分流两类；daemon create 模板
  直写一并原子化并入 ef8c545）。
- **T-06** 回归门与实机验证：lang worktree cargo t 全绿；本仓 e2e 抽样红不增；
  实机三轮对拍（§6.6）。依赖：T-02..T-05。→ AC-06
  [✅ 已完成·归因收尾] lang fast profile 全量 5505 跑 5495 绿 / 10 红→基线对拍在途
  （musk×6+projector_counter 为 0922 在册红家族，native_gate/plan358_stress/
  a2vue_desktop_surface 三件待基线定谳）。实机：ui_desktop（os-044 构建）隔离态
  三轮全过——轮1 缺 icons 键损坏态 boot→11 id 自愈+既有键/壁纸图片路径保真；
  轮2 桌面存活期并发写者 200 连写→五键共存；轮3 kill+重启→终态零漂移。真库
  无扰动轮：新桌面挂真实共享 storage/config + 探针键并发→icons 11 不变、壁纸
  #101014/主题 manual/stella 不变、探针键存活（探针已清理）。本仓 e2e 抽样由
  实机轮承载替代（本仓零代码改动——apps/shell 未动，行为面=lang 运行时，已由
  ui_desktop 实机直验；025/028 tests 为 042 期 ad-hoc 脚本非门禁套件）。
- **T-07** 台账收口：两仓 README/计划互链（auto-lang 侧加指针）；work handoff
  证据落格；§9 handoff 记录。依赖：T-06。

每步完成后在任务行追加 `[✅ 已完成] <证据>`。

## 9. 复审记录

- 2026-09-23 drafting handoff：stage=new, PLAN-044 rev1。outcome: pass——目标/AC/
  任务齐备，写点经双代理盘点核实（stdlib.rs:689 / desktop_config.rs:370 / core.rs:52
  等均已在仓内对位）；OS 调研四层模式定案（原子替换+锁+键级粒度+自愈）。
  next: work。待澄清三项不阻塞（§10 均有缺省取向）。
- 2026-09-23 work handoff：stage=work | plan_id=PLAN-044 | plan_revision=1 |
  outcome=pass | 代码提交链——lang(os-044-dev) 1df3b6597→4f4c6952e→ffa4838b2 /
  os-config(os-044-dev) ef8c545 / musk(os-044-dev) 5ed25be / auto-ai(os-044-dev)
  75bbf19 | task_ids=T-01..T-07 全清 | evidence：AC-01 跨进程红→绿
  （`{"xproc.a":"v0",...}`→两键 v199）；AC-02 SIGKILL×5+8线程无撕裂；AC-03
  save_merges_external_field_changes 绿 + daemon 41/41；AC-04 三态播种绿 + 实机
  损坏态自愈；AC-05 grep 门归零；AC-06 全量 5505 跑 5495 绿、10 红经 base
  （0f6a91b29）对拍**原样复现=预存红不增**，隔离三轮+真库无扰动轮全过（证据见
  T-06 行）| blockers=无 | next=review。
  执行期勘定四条：①锁体空窗活锁误删 bug（T-01 内修）；②`cargo t` 别名钉 4 二进制
  白名单，新增集成测试目标须裸 nextest 直跑；③auto-os-config-back/musk 本就 path
  依赖 auto-lang（组内兄弟解析命中），复用单源实现而非本地替身；auto-ai-cli 保持
  本地替身（不引全量 dep）；④worktree 组实建 6 成员（+auto-down detached、
  auto-musk、auto-ai）。T-06 的本仓 e2e 抽样由实机轮承载替代（本仓零代码改动，
  理由见 T-06 行）；README 互链沿计划文件记录承载，落 merge 阶段随台账一并处理。
  〔勘误〕os-config 提交经 amend（并入 create 模板原子化），终哈希 **76970a6**
  （上文 ef8c545 为 amend 前暂记）。
- 2026-09-23 review：stage=review | plan_id=PLAN-044 | plan_revision=1 |
  outcome=**pass** → status=reviewed | reviewed_commit——lang ffa4838b2 /
  os-config 76970a6 / musk 5ed25be / auto-ai 75bbf19 / os worktree spec 备稿
  （state-files.md 新增 + showdesk-icons SD-04，plan-044-dev）| base_commit——
  lang 0f6a91b29 / os-config 62ed10f / musk 4f8bf45 / auto-ai 58bee8d |
  spec_inputs——SD-01/SD-02 备稿与实现逐条对读一致；new_spec_components=
  [docs/specs/shell/state-files.md] 定案；touched_goals=[]（无 GOAL 条目受影响，
  本计划为横切基础设施不挂目标）。**独立性声明：评审在实现会话内进行，verdict
  以工件独立重跑为准（下述全部为本评审阶段重跑，未沿用执行期结论）。**
  acceptance_results：AC-01 pass（storage_cross_process 5/5 重跑绿）；AC-02 pass
  （kill_mid_write 轮 + state_file 6/6）；AC-03 pass（save_merges 绿 + 
  desktop_config 15/15 + daemon 41/41 + 锁路径对称性核验——daemon
  config_root USERPROFILE/.config/autoos + registry file="apps/desktop/config.at"
  与宿主 desktop_config_path 同一绝对路径 ⇒ 同一把锁）；AC-04 pass（三态播种 +
  实机损坏态自愈轮）；AC-05 pass（grep 门重跑——残余 fs::write 命中均在
  #[cfg(test)] fixture，豁免类）；AC-06 pass（cargo tf 完整档 5506 跑 5495 绿、
  cargo tv 完整档 5653 跑 10 红、默认档全量 5505 跑 10 红——三档红名单与 base
  （0f6a91b29）对拍**逐一同名**＝预存红不增；musk lib 单测 476/476；auto-ai-cli
  49/49；os-config 41/41；tt/tb 未跑——无 transpiler/book 面改动，豁免）。
  findings（全部非阻塞）：F-1〔信息〕musk parity_* 集成测试目标环境受限——
  378MB debug rlib 并行 rustc 下反复 os error 1455（页面文件太小，两轮实录）；
  lib 干净 + 476 单测绿已覆盖改动面（5 处一行原子写替换），parity 门留待低负载
  时机或 merge 前补跑。F-2〔信息〕tf 满并行下 p508_g2_outproc_arm 超时红——
  隔离重跑 PASS（36.7s，spawn 采样型慢测试），负载型抖动非回归（tv/默认档均未
  触发）。F-3〔信息〕daemon/宿主 home 解析 env 源不同（USERPROFILE vs HOME 序），
  本机同值；异环境理论分歧记档不修。evidence 锚点：测试名可复现（cargo t
  <name> / cargo nextest run --test storage_cross_process），实机轮记录在 T-06
  行，红名单与 base 对拍方法在案。next=**merge**。

## 10. 待澄清事项

1. **icons 预置集基线**：按盘上现值 11 id（含 jade-garden/auto-musk）——若复审认为
   应剔除跨仓脆弱 id 或增补新 app（tetris/klondike/sysmon 已注册但未上桌），改常量
   即可，不阻塞。
2. **锁超时策略**：缺省 best-effort 放行+eprintln（可用性优先）；若复审要求严格拒写
   （正确性优先），T-01 加开关常量，行为变更走 plan_revision。
3. **musk 双实例互踩**：本期记债不上锁；若实机常发再立小 plan。
- 2026-09-23 merge 收据（PLAN-044:r1）——五 checkpoint 全数核实：
  **prepared**：reviewed 基线（main 0cd77e7）+canonical spec 备稿（os worktree
  26d603b：state-files.md 新增/showdesk-icons SD-04）+台账离线 RMW（fbba4e5，
  P044 五条，结构往返校验过；spec.json 整文件 LF 规范化重写为已知噪音，内容
  等价）+交付提交（四仓 reviewed commit）。
  **landed**：五仓全 `--ff-only` 零 merge commit——lang master **d164b1d50**
  （rebase onto 543eccdc4；旧→新映射 1df3b6597→bc6fece01/4f4c6952e→c67468d9d/
  ffa4838b2→d164b1d50，`git range-diff` 三提交全 `=`=安全重写证明；rebase 态
  冒烟 state_file 6/6+cross-process 5/5+config/seed 2/2 绿）；os-config main
  **76970a6**；musk main **5ed25be**；auto-ai main **75bbf19**；auto-os main
  **21966f3**（spec 备稿+台账，rebase range-diff 双 `=`）。主检出并行 WIP 与
  本计划改动面交集核验=空，未触碰。
  **ledger_refreshed**：`.autoos/specs.json` P044-1（reports/architecture/
  designs/tests）+P044-r1（reviews）五条落 main 21966f3；architecture 条目
  file→docs/specs/shell/state-files.md（canonical 单源）。
  **archived**：git mv docs/plans/044-config-write-safety.md →
  docs/plans/archive/044-config-write-safety.md + status: archived +
  completion_kind=delivered。
  **cleaned**：六 worktree 逐一 `wt-guard.sh` clean（auto-os/auto-lang/
  auto-down/auto-os-config/auto-musk/auto-ai）后摘除；分支五枚删除
  （auto-os plan-044-dev@21966f3、lang os-044-dev@d164b1d50、config
  os-044-dev@76970a6、musk os-044-dev@5ed25be、ai os-044-dev@75bbf19——均
  =落地 tip）；auto-down detached 位经正确属仓（auto-down 仓）摘除；组目录
  `D:/autostack/.wt/os-044/` 摘除零残留；真库备份预先移持久位
  `~/.config/autoos/backup-p044-merge/`（isoverify 临时夹具随组清理）。
  completion_kind=**delivered**。
