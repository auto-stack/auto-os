# D-4 决策工件：目录删除口径（PLAN-016 T-06）

**日期**：2026-09-14　**状态**：已定案

## 问题

stdlib 无递归删除 native（`File.delete` 仅文件，`File.remove_dir` 仅空
目录）。目录删除三选一：
a. 仅支持文件 + 空目录，非空目录拒绝；
b. `fs.walk` 逆序自拼递归删除；
c. 新增 `fs.remove_dir_all` native。

## 定案：a（文件 + 空目录；非空目录 toast 拒绝）

理由：
- b 的 walk 逆序删除在遍历中途失败的中间态难以回滚，误删风险与 UX 责任
  都落在 app 侧自拼逻辑；
- c 是 stdlib 面扩张（need native id + 三表登记 + 跨端声明），收益仅覆盖
  "删非空目录"一个操作，且文件管理器业界惯例本就有确认弹层兜底而非静默
  递归删除；
- 拒绝路径 UX 明确：`toast.error("目录非空，仅支持删除空目录: <名>")`。

## 落地

`app.at ExecuteDelete`：`json.parse(fs.read_dir(path)).len() > 0` → 拒绝；
否则文件 `file.delete` / 空目录 `file.remove_dir`。

## 实证

- 文件删除：testdata/mock 阶段 alert-dialog 确认 → ExecuteDelete
  `VM_HANDLER_OK`（fm-run4.log :323）+ README 行移除。
- 目录创建/删除落盘：`fm-smoke-dir` 两轮真盘创建实证
  （Desktop/Home 各一轮，`CommitNew → file.create_dir`）。

## 残留

递归删除留待 stdlib `fs.remove_dir_all` 提案（不在本计划）。
