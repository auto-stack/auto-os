# PLAN-050 迁移后的已知问题（2026-10-22）

## 用户批准非阻塞的两个剪贴板测试

完整日常门执行 5331 项：5329 通过、2 失败，另 1552 项按原配置跳过。恢复本次全部源码改动的基线后，同条件复现相同两个失败；未确定系统根因，未修改测试或跳过规则。用户于 2026-10-22 明确要求记录并继续迁移。

- `ui::iced::renderer::tests::plan488_clipboard_paste_payload_reads_os_clipboard`
- `ui_tests::plan802_code_editor_provider::plan802_clipboard::os_clipboard_cut_copy_paste_roundtrip`

## 额外构建观察：原生桌面 MPV 依赖

`cargo build -p auto-lang --features ui-iced,mpv-widget --example ui_desktop` 未通过：`auto-ui-runtime/src/ui/mpv/loader.rs` 使用 `libloading`，但该 crate 的 Cargo.toml 未声明此依赖，出现 E0433。本次迁移不修改这两个文件；从合入前基线到交付提交的相应 diff 为空。

该项属于额外产物构建观察，未作为已通过的原生桌面构建或 GUI 验收。CLI 与 UI 工具构建通过并已安装；原生宿主及 release/web 产物状态详见[产物记录](../plans/attachments/050/artifact-observations.json)。后续修复应按 auto-lang 独立 Plan 补齐依赖、完成相应回归并重建原生宿主。

额外扩大 UI 工具包验证发现的 9 项相同基线失败及逐项名称，保留于[验证汇总](../plans/attachments/050/verification-summary.json)，本轮新增失败为 0。
