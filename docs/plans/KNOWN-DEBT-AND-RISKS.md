# Known debt and verification limits


## v0.6 应用独立化（2026-10-04）

- **V06-APP-001（既有基线/范围外）**：当前开发组缺少旧 manifest 引用的 Musk 与 Jade Garden 外部应用根；注册表产生对应跳过警告。四大主力应用按用户决定暂不迁移；完整 v0.5 恢复后对齐实际仓与入口。新 21 项产品入口不受此项影响。
- **V06-APP-002（运行验证范围）**：未执行 21 项完整 GUI/后端/媒体交互；本机无可用 Auto CLI/桌面二进制。本轮验收为仓库组织与接线，不把路径探针称为双端运行验收。下一阶段逐 app 使用 autoui-verifier 验证。
- **V06-APP-003（知识/发布工程）**：Notes 暂以固定 AutoDown 子模块携带 engine 源码，StyleKit 为固定 vendor 快照。统一 package/install 成熟后可改为版本依赖；本轮不改包管理协议。终端历史证据日志很大， fresh-clone 验证以浅/过滤/稀疏模式获取同一 pin，正式配置不改。
- **V06-APP-004（恢复任务）**：五个 OS 目录转 gitlink，隐藏 v0.5 修改须先导入对应产品 source-sync，再合产品开发线并更新 OS pin；禁止用旧目录覆盖新产品，父仓文件/目录冲突按应用处理。临时计划编号待主机恢复后登记，避免占用不可见计划 ID。
