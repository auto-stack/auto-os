# PLAN-041 T-01 决策件：VM 动态臂 `Undefined symbol` 家族

日期：2026-09-22 晚 · 调查基线：lang 工作树 os-041-dev（master 80f96a95a + 本批）
· 实测：隔离实例 :9350（AUTO_OS_ROOT=os 工作树）

## 三问裁定

### ① back 模块为何不进 VM 环境——根因（非设计意图，是解析缺口）

桌面 inproc launch 链：`launch_app` → `build_dynamic_component(spec.code,
source_path)` → `build_dynamic_component_inner`（lib.rs 4050）：

1. use-import 束按**基目录表**逐试探文件系统路径：`back.api → back/api.at`
   （lib.rs ~4208 注释实例）。基目录表 = `[entry 文件父目录（= src/front/）,
   lang_root 回退]`（lib.rs 4160-4170）。
2. **`src/back/api.at` 不在任何基目录下**：`use back.api` 探
   `src/front/back/api.at`（miss）→ `read_to_string` `if let Ok` **静默跳过**
   （lib.rs ~4216）——零告警零日志。
3. 符号悬空至 VmBridge `new_from_decls`：单模块合成（"single module → no
   cross-module relocation"，vm_bridge.rs 450）+ `Linker::link()` →
   `Undefined symbol: api.kill_process in module App` → launch 失败死窗
   （"应用暂不可用"）。

**结论：不是 daemon 分离的设计意图，是 import 解析基目录缺 `src/` 档的装载
缺口。** auto-os app 布局约定（AGENTS §3：src/front + src/back）自 PLAN-013
起就是登记 app 的标准形态，动态臂从未覆盖之。

### ② 修复形态——推荐臂已实施（可复审翻案）

**选定：装载链补 back 符号注册**（计划 §5 A 轨首选预期）。实现 = base_dirs
增补 entry 父目录（仅当其名为 `src`，守卫防误伤平铺布局）：`back.api →
src/back/api.at` 命中，back 模块随 import 束正常编译链接。零契约变更、
无 storage/pac 面改动。代码：lib.rs 4160 区（PLAN-041 T-02 注记块）。

翻案备选（若复审否决）：「需后端」声明语义前置——注册表标 daemon/back 依赖，
桌面 launch 前置拦截给明确提示窗（不进死窗）。回滚面 = 该 2 行 + 备选另立。

### ③ 五 app 符号家族定性

| app | 符号 | 定性 |
|---|---|---|
| 025-sys-monitor | `api.kill_process` | 同根因①（`use back.api`），修复臂直接覆盖——本次实测复验 |
| kanban | `api.list_boards` | 同根因①预判（back 布局同型）；隔离实例无 kanban checkout，切用户桌面后复验（T-03 残项） |
| 017-chat | `api.send_message` | 同根因①预判；examples/ui 布局（app.at 平铺 + api 在同目录？）——T-03 复测定性 |
| ui-gallery | `fmt.pct1` | front 兄弟模块，首基应命中且历史可跑——15:55 失败疑并行会话 WIP 中间态（ui-gallery 修改未提交在案），T-03 复测 |
| auto-term | `term.config_spawn_program` | term 模块 + pac 在 `app/` 子目录布局，基目录解析路径不同——隔离实例无 checkout，切桌面后复验（T-03 残项） |

## 影响面与风险

- 守卫 `src_dir.file_name() == "src"`：examples/ui 平铺 app（011-calculator
  等）parent 名非 src，零行为变化；shell/特权 pack 不走此链（宿主内编译）。
- back/api.at 若含 front 解析不识别的语法，将在 import 编译期显式报错
  （比静默跳转链接失败友好）；实测见下节回执。
- 首基序不变：front 兄弟模块优先于 src/back（无遮蔽回归）。

## 实测回执（补记）

见 §9 执行记录与本文件提交同批证据（.auto/iso041/）。
