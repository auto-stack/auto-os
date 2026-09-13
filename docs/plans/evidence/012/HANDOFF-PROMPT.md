# PLAN-012 F2 实机走查 · 新会话继续 Prompt

把下面整段复制到新会话即可继续：

---

继续 PLAN-012（shell-ux-feedback-batch）的 F2 实机走查收口。用
/auto-plan:work 技能（needs_fix→work 续修状态，计划文件
docs/plans/012-shell-ux-feedback-batch.md §9/§10 是唯一事实源，先读它）。

## 环境与状态（已全部提交，勿重复劳动）

- worktree 组：`.wt/os-012/` 三检出，分支均 os-012-dev：
  - auto-os @ 88b18e1（shell pack 修复）
  - auto-lang @ a2f4a1763（宿主/渲染器修复，含 F1 协议文档 v1.6）
  - auto-os-config @ 0f82628（os-config 前端修复——本仓已获走查扩仓授权）
  - auto-down @ bd21ef6（纯 path 依赖，未动）
- 主检出（本仓）计划文件已落账至 work F2 走查续二（c96911b 之后的续记提交）。
- vm 桌面启动命令（隔离 F2 档案，验证用）：
  `Start-Process D:/autostack/.wt/os-012/auto-lang/target/debug/examples/ui_desktop.exe -WorkingDirectory 'D:/autostack/.wt/os-012/auto-os'`
  env：`AUTOOS_DESKTOP_CONFIG=%TEMP%/os012-f2/config.at`、
  `AUTO_VM_STORAGE_FILE=%TEMP%/os012-f2/storage.json`、
  `AUTO_OS_ROOT=D:/autostack/.wt/os-012/auto-os`。
  （CWD 必须 = os worktree 根；calculator 直挂 `use` 解析已有 lang 仓根
  回退基 ac849f63f。）

## 已解决（用户实机确认，勿重做）

AC-01/02 打开提速 ✓、主题切换 ✓、AC-11 三态 ✓、侧栏滚动/压缩 ✓、
「目录枚举失败」✓、底部圆角 ✓、显示页缺省+主题/accent 置顶 ✓、
⚙️ 二态关闭 ✓（已修待复验）。

## Open items（本会话核心工作，按序）

1. **O1 AC-10 字形居中**：真实任务栏图标偏左上，headless 探针通过但真实
   链路不一致——需真实应用 bounds 探针（devtools/MCP；MCP :9247 被
   at-app.exe 占用需先处理）。
2. **O2 dock 重复图标**：固定后运行窗未去重（用户截图：固定图标+运行图
   标并存）——去重条件 `__dock_pinned_csv.contains(w.app)` 未命中，疑
   w.app 与 pinned id 不一致，需活体状态探针；用户 UX 裁定：同类 app
   共享一图标。
3. **O3 通知面板贴顶**：mt-auto 在 headless 探针通过但真实 overlay 挂载
   链未生效——查 notification overlay 装配臂（renderer 装配层）高度链。
4. **O4 复验**：真拖拽（按住拾起→拖动→松手磁吸落格/占位挤推）、⚙️ 二态、
   错误❗、dock/桌面分离——运行实例 23708 已含全部修复，前端 .at 每次
   ⚙️ 点击重读。
5. 收口：证据归档 docs/plans/evidence/012/ → 复审（/auto-plan:review）
   → merge（双仓 + auto-os-config 三仓收口，PLAN-011 先例）。

## 纪律

- auto-lang crates/ 改动 = Category A 门档（cargo t scoped + 全量定性）。
- shell pack 改动必跑 `python scripts/shell-pack-sync.py`（hash-lock）。
- worktree 红线：删除前 wt-guard；junction/symlink 禁止。
- 修复循环上限 3 轮；不 weaken AC；未验证判据不记 pass。

---

补充说明（不用复制）：本会话上下文确实接近极限——上面这段 prompt 已包含新会话所需的全部关键指针，计划文件 §9/§10 有完整证据链（每项修复的提交号、测试名、根因分析都在）。三个 worktree 全部已提交干净，桌面实例 23708 还在跑着（含全部修复），新会话可以直接接着验证或继续修 O1-O3。
