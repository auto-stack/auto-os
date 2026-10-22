# PLAN-050 r3：目录迁移交付前验证

状态：已获用户明确授权两项基线失败非阻塞，迁移已落主检出；完整后验和收尾见 merge-receipt.json。

## 冻结实现

- AutoOS：`24eae3eb34a4025fba59993acdb74eb2f33b6253`，工作树 `D:/autostack/.wt/os-050/auto-os`；原迁移实现为 `3e24ed1aad4766fbd6031ea239e922015b471b40`，其上仅补齐 Kanban 分支配置及对应说明。
- AutoUI：`eaea62466f84b7f911b9553026592b726f9c525b`，工作树 `D:/autostack/.wt/os-050/auto-lang`。
- 语言原提交 `97cbf25ffa8bc33af0eef349c78f0b126c98f17f` 对齐到主线 `434a61be218fa3eda0d3bebdaaff4816c159e4ee` 后改写为上述提交；range-diff 为 `=`，并发新增仅 PLAN-828 文档，代码补丁与依赖锁未变。
- AutoDown 只读依赖：`c39ac7f9dacdc769fc4a5ad3b98ec896240b28e8`。

## 迁移范围与验证

| 检查 | 结果 |
|---|---|
| 24 个产品目录重命名 | 已在 OS 工作树完成；21 导入产品 + Kanban/配置 + 内部 syslog |
| 24 个 gitlink、URL、端口与稳定 ID | 固定 SHA 全部保持；清单/子模块元数据一致，28 个注册入口仍在 |
| 元数据校验 | `verify-app-links.py --metadata-only` 通过；明确不算完整检出/业务验收 |
| 校验器反例 | 5/5；旧清单路径、错 pin、非正式目录被拒绝，完整模式不接受未初始化检出 |
| 注册表 | 28/28；覆盖 repo/local 映射、坏/缺清单、路径匹配、空检出、显式根优先和去重 |
| Vue/VM 发现 | 真实 `extra_roots_three_track_parity` 通过 |
| Vue 生成 | 新目录内容实际生成到旧 ID 的 App.vue，测试通过；恢复旧 ID 拼目录的负面对照按预期失败 |
| 已初始化子模块移动 | 隔离真实 Git 子模块 fixture 的移动/ff/HEAD/文件/clean 检查通过 |
| 当前文档与入口 | 研究资料链接、活动路径扫描、Bash/PowerShell × Vue/iced dry-run 通过 |
| 历史数据 | 导入报告、归档 Plan 无修改；研究来源保留原始路径并新增 current_path |

主检出 Explorer 当前 clean/detached，HEAD 与父 pin 一致，43 个文件已记录内容指纹；实际落地后必须再次核对。未初始化的其他 app 不克隆、不切换、不更换 pin。

## 扩大验证与基线对照

UI 工具包：修改版 161 通过 / 9 失败 / 1 忽略；同树临时恢复全部原始源码的基线为 160 通过 / 相同 9 失败 / 1 忽略。9 项为既有交付投影/生成清单/CRUD fallback/shell-pack/CSS 金样问题；新增失败 0。具体名称见 verification-summary.json。

第一次完整日常门：1132 通过、17 超时，其余因 fail-fast 未执行；一个超时探针串行复跑 72.349 秒通过。随后 `cargo t -j 4 --no-fail-fast` 完整执行 5331 项：**5329 通过、2 失败、1552 按原配置跳过**。本次没有修改测试选择、超时或跳过规则。

两个失败是：

1. `ui::iced::renderer::tests::plan488_clipboard_paste_payload_reads_os_clipboard`
2. `ui_tests::plan802_code_editor_provider::plan802_clipboard::os_clipboard_cut_copy_paste_roundtrip`

串行复验仍失败。还原本轮全部源码改动到 `434a61be218fa3eda0d3bebdaaff4816c159e4ee` 后，同条件 `cargo t -j 1 --no-fail-fast -E <这两个测试>` 得到 **0 通过 / 相同 2 失败**。完成后已恢复冻结改动。结论仅为“基线已有且本轮未新增”，不把非零门禁称为通过，也不认定具体系统原因已查明。

## 合入决策

r3 追加[子模块核对](submodule-audit.md)：24 个对应远端仓和固定提交均可访问、默认分支均为 main；元数据/原 Python 5 项复验通过。本机旧 URL 已同步。用户明确系统日志保持内部组件，未答复门禁例外；原语言源码、依赖与验证输入不变。

用户尚未授权豁免这两个失败；当前 Plan 要求语言合入前门禁通过，因此主检出尚未移动目录。已提出两种选择：记录既有失败、按本轮定向验证先迁移；或先修复剪贴板问题再迁移。时间经过或预选选项均不算授权。

审查由同一执行会话从冻结差异、实际入口与对照结果重建完成，非独立 agent/上下文。代码/定向验收可审查；完整交付、主检出 Explorer 后验、归档/清理尚未完成。

## 产物观察

主检出的 `auto.exe` 为 2026-10-21 16:39:54、`auto-ui-tool.exe` 为 2026-10-21 11:09:16；本轮未替换运行中的工具/宿主或桌面生成 bundle。目录兼容修复生效还需要按实际工具链重建对应产物。未将 dry-run 或源码回归冒称双端 GUI 实机验收。

## r4 合入更新

2026-10-22 用户明确要求记录两项基线失败、不阻塞迁移。OS 源码落地 577594e、语言源码落地 ce055a519；代码/依赖/测试配置与原冻结验证等价，rebase 并发增量仅计划文档。主检出实际目录、24 pin、Explorer 43 文件与 clean/detached 后验通过。上文“尚未授权/尚未落地”为此前检查时的历史状态，以此更新及 merge-receipt.json 为最终状态。
