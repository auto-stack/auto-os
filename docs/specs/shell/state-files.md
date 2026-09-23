# Spec: 用户级状态文件写安全（state files）

> Source of truth for `~/.config/autoos/` 下配置/状态文件的并发写安全契约
> （PLAN-044 config-write-safety）。设计参照系：Chromium ImportantFileWriter
> （temp+flush+rename + .backup）、git index.lock（独占 lockfile）、
> cfprefsd/dconf（单写者 daemon——远期形态，v1 以文件锁等效串行化）。
> 实现：auto-lang `src/state_file.rs`（跨仓锁路径约定单源）+
> `vm/ffi/stdlib.rs`（storage 合并写）+ `ui/desktop_config.rs`（config.at
> 双写者护栏）；auto-os-config `auto-os-config-back/src/{core,collection}.rs`
> （daemon 侧同锁）；musk 五写点与 auto-ai-cli 会话文件（原子替换）。

## SD-01 三层写安全

- **L1 原子替换（所有状态文件写，强制）**：同目录 `.名.tmp-<pid>-<seq>` 写 +
  sync + rename 覆盖（Windows 上 `std::fs::rename` =
  `MoveFileExW(REPLACE_EXISTING)`，同卷原子）。任何对 `~/.config/autoos`
  状态文件的新写点禁止 `fs::write` 直写。
- **L2 跨进程互斥（多写者文件，强制）**：`<目标>.lock` 独占创建
  （O_CREAT|O_EXCL），锁体 pid+epoch_ms 两行；stale = 锁龄
  max(mtime 龄, 锁体 epoch 龄) > 10s 接管（锁体空/不可解析不参与 epoch
  龄判定、stat 失败不改判 stale——防 create_new→写锁体空窗期活锁误删）；
  获取超时 2s **best-effort 降级放行**（可用性优先；降级只跳过互斥，
  消费方合并语义照常）。锁路径约定单源 = auto-lang `state_file` 模块头，
  daemon 与宿主对同一目标持同一把锁文件。
- **L3 键级合并写（desktop-storage.json）**：persist = 锁内重读磁盘 →
  盘 ∪ 本进程脏键（`STORAGE_DIRTY`，写路径统一登记；脏键含删除胜）→
  原子替换。未触键以盘上新值为准——多进程（宿主 × outproc 子进程）交错
  写不同键两键均存活。坏 JSON 盘文件 → `.corrupt.bak` 留证 + 内存重建。
  读侧 load-once / `or_insert` 契约不变（PLAN-024 R7）。

## SD-02 config.at 双写者协议

- 宿主 save 与 os-config daemon put/delete 共用 `<config.at>.lock`；两边的
  read→merge→write **整段 RMW** 锁内完成（不只是写点）。
- 宿主 save 锁内重读盘上最新值做**字段级 diff 合并**（`LAST_SNAPSHOT`
  基线 = boot load 与每次成功 save 后刷新；外部热应用不刷新是有意的——
  热应用字段与盘上现值同值覆盖无害）：仅调用方相对快照变过的字段采纳
  调用方值，daemon 轮询间隙外写的字段不被陈旧快照整体冲掉。无快照基线
  （boot 迁移首写）或盘上无文件 → 调用方整份为准（历史行为）。
- `.bak` 单代保留，锁内从当前盘值原子拷贝，目标原子替换（备份与替换均
  不可撕裂）。

## SD-03 适用范围

- **多写者强制面（L1+L2+L3/字段合并）**：`desktop-storage.json`、
  `apps/desktop/config.at`（及后续登记的宿主/daemon 共写文件）。
- **单写者原子面（仅 L1，不上锁）**：musk users/workspaces/specs/chats/
  professions、auto-ai-cli `sessions/<cwd-hash>.json`、daemon collection
  实体写与模板创建。双实例互踩为已知限制（PLAN-044 §10.3 记债）。
- **天然安全面（无需改造）**：唯一文件名 append（llm-rejects）、按 id
  分流（relay handoff/task-plan）、v1 只读（skills/modules.d）。

## 验证

- lang：`tests/storage_cross_process.rs`（AC-01 双进程交错红→绿——修复前
  `{"xproc.a":"v0","xproc.b":"v199"}` 实录、修复后两键 v199；AC-02
  SIGKILL×5 轮读回恒合法 JSON）；stdlib 单测
  `storage_persist_merges_disk_with_dirty_keys` /
  `storage_persist_rebuilds_from_corrupt_disk`；desktop_config 单测
  `save_merges_external_field_changes`（daemon 外写字段在宿主陈旧快照
  save 下保住）。
- 实机（os-044 执行期）：ui_desktop 隔离三轮（损坏态自愈/并发交错/重启
  零漂移）+ 真库无扰动轮（icons/壁纸/主题不漂移、探针键存活）全过。
