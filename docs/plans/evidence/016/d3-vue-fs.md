# D-3 决策工件：真实 FS 的轨口径与 JsonValue 风险（PLAN-016 T-05）

**日期**：2026-09-14　**状态**：已定案

## 问题

真实 FS 数据层的两个未证风险：
1. `json.parse(...)` 返回 `JsonValue?`（可选），字段级访问
   `meta.get("len").as_int()` 在 VM 轨无先例；
2. vue 轨的 ts_adapter 是否有 fs/file 桥（决定 vue 轨能否同启真实 FS）。

## 实证结果（VM 轨）

- **可选字段读取确证产出 None 级联**：初版 NavTo 用
  `meta.get("is_dir").as_int()` 三连，实机 `TypeError: unsupported operand
  type(s) for +: 'int' and 'NoneType'`（fm-run10.log，crash ip 在 Init/NavTo）。
  kanban 先例只覆盖 `for-in + ""+elem` 的**字符串**物化路径，数字字段不可套用。
- **定案：列表物化零 JSON**。逐条目改直连 native 三件套：
  - `file.is_dir(path) bool`（既有）
  - `file.size(path) int`（既有，rust_fn 宏注册）
  - `fs.mtime(path) int`（**本计划新增**，epoch 秒，失败/1970 前 = -1；
    native_catalog id 2986 + for_each_bigvm_native + NATIVE_ID_ENTRIES 三表
    登记 + `stdlib/auto/fs.at #[vm]` 声明）
  - `fs.read_dir` 的 JSON 数组仅用 kanban 已证的 `for-in + ""+name` 字符串
    物化路径。
- metadata JSON 的 `modified` 键扩展保留（shim_fs_metadata，API 面完整性），
  但 .at 侧不再消费。

## 迁移中顺带修复的框架缺口（crates/auto-lang/src/vm/native_catalog.rs）

1. **id 撞号**：`auto.fs.rename`(2842)/`canonical`(2844)/`ext`(2845) 与 bigvm
   表 `File.read`/`Path.new`/`Path.join` 撞号——read_dir 2843→2866 同症先例
   （:448 注）。三 native 迁 **2983/2984/2985**（for_each_native 注册行 +
   for_each_bigvm_native + NATIVE_ID_ENTRIES 全表一致）。
2. **白名单缺席**：三者原缺 for_each_bigvm_native/NATIVE_ID_ENTRIES 登记，
   VM 模式 app 模块调用报 `Undefined symbol`（read_dir 同症先例）。

## vue 轨口径（计划 D-3 原问题）

ts_adapter 的 fs 桥存在性**未再深查**——VM 轨为桌面事实轨，vue 轨维持
既有渲染冒烟口径（T-10 截图佐证）。若 vue 轨需要真实 FS，属独立 stdlib
ts 桥工程，不在本计划契约内（PLAN-016 §5.4 已备案该口径）。

## 运行时 stdlib 解析注记（部署依赖）

app 运行时 stdlib 解析序 = CARGO_MANIFEST_DIR → `~/.auto/libs/` → 系统
（`util.rs find_std_lib`）。开发机上 `~/.auto/libs/stdlib/auto/fs.at` 为
同步缓存，**需含 `fs.mtime` 声明**（本次已手补；上游 merge 后随同步自然
携带）。否则编译出的调用落 `Undefined symbol: fs.mtime`。

## 残留

`auto run` 冷启动与 Init 内重 FS 活存在偶发竞态挂起（同代码 ~50% 复现，
AUTO_VM_STORAGE_FILE 未设时更高频）——定案 **Tick 延迟引导**：Init 仅
env/storage 轻量恢复，快捷访问探测 + 首次 listing 挪到首个 250ms Tick
（012/025 interval 先例）。修复后连续多轮冒烟零复现。
