# PLAN-050 子模块核对（2026-10-22）

## 已确认

- 父仓已有 24 条真正的 `160000` gitlink；与 `.gitmodules` 一一对应，不是普通目录伪装的应用仓。
- 24 个 URL 均与 GitHub 返回的 `auto-stack/<产品名>` clone URL 一致，仓库均可访问且未归档，默认分支均为 `main`。
- 逐一读取对应远端仓的 Git commit 对象，24 个固定 SHA 全部可取回；本次不推进任何 gitlink。
- Kanban 是唯一未显式声明 `branch = main` 的子模块。已在迁移工作树补齐；其余 23 项原本就有此配置。
- 主检出的本地 Kanban URL 原为 SSH，已通过 `git submodule sync` 与受版本管理的 HTTPS URL 对齐；Explorer URL 同步后保持原值。
- Explorer 检出仍 clean/detached，HEAD 保持 `59a99b9f42896728aa32a7ff3e38fa08261c7d18`。没有初始化其余空子模块。
- 用户明确选择“暂时保留为内部组件”，系统日志继续由父仓维护。`common` / `autostack` 为辅助目录。

## 配置与检出的含义

迁移后的物理路径为 `apps/auto-*`；`.gitmodules` 逻辑节名保留旧名，Git 内部存储沿用原位置。这是合法的子模块配置，避免重新克隆已有检出。应用启动 ID 同样保持原值。

`branch = main` 指明远端更新的目标分支，日常组合仍按父仓 gitlink 的固定提交检出；不使用 `update --remote` 自动追最新版本。

元数据检查和现有 Python 5 项测试通过。该结论不等于 24 个应用已全部检出或已完成功能验收。逐项仓库/路径/默认分支/SHA 记录见 [submodule-audit.json](submodule-audit.json)。

## 落地状态

本地 URL 同步、受版本管理的 `branch = main` 补齐及 24 个目录迁移均已完成并合入主线。用户随后明确授权两项既有剪贴板失败非阻塞；实际主检出和 Explorer 后验通过，见 merge-receipt.json。
