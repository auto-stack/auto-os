# App工作位置约定与子模块文档同步复核（2026-10-04）

授权：用户确认所有app操作在auto-os/apps子目录；日常detached，修改时v0.6-dev，完成提交/更新父仓指针后恢复detached。本次只改操作文档和计划执行位置，不实施功能。

复核方法：同会话独立检查实际提交、旧/新计划任务及AC、格式与链接。没有独立agent；不是12个代码计划的实现验收。

- apps/015-notes：`7ecfa5d0cb61c3be624bb43d8afabeb02c51e028` → `8d06d233ec03535e4802b8789d0f001fc12fe8a8`；计划revision 2，仍drafting；T/AC逐行与旧版本相同。
- apps/028-launcher：`2c23b8baca51043edafe5d324e4061e2cd51a52f` → `94abb2386c5d1dbdcda53ed93662529613b129ff`；计划revision 2，仍drafting；T/AC逐行与旧版本相同。
- apps/018-book-reader：`d43fa417df4621a65a606a096a6495647cbc24d6` → `4e36f6b6916062f086b169f25fdcb05f96158ed7`；计划revision 2，仍drafting；T/AC逐行与旧版本相同。
- apps/023-realworld：`6cfd7ebd9e25aa13e9a5df0451cd66c254c373d9` → `6d819c4edda9f7e1c3e52843b1e060c3df9d0a34`；计划revision 2，仍drafting；T/AC逐行与旧版本相同。

结果：4仓均从实际apps检出提交，source/pac/vendor未改变；旧外部worktree示例及“文档不更新指针”规则已修正。先推送app、再提交/推送OS gitlink、最后显式detach；只同步这4个已复核文档SHA；在途app不切换不暂存，实际并发状态见下节。

OS AGENTS和操作指南记录最新用户约定，语言/框架核心worktree规约继续保留。纯文档无需cargo/UI功能测试。后续并发同app需协调单写者，已有WIP不自动detach。

结论：文档范围pass；适合同步4个app固定指针，代码计划未执行。最后以实际远端SHA、父仓gitlink和子模块detached状态复核落地。

## 落地时并发状态补充

上述复核绑定四个明确的文档SHA，重新从提交对象检查12个计划的T/AC一致性、revision与diff --check通过。Notes随后由另一Agent开始Plan 001；其分支已推进并有未提交代码。本次只固定到上述已复核的文档SHA，不纳入其后实现提交、不切换或detach Notes。其分支推送时已包含另一个Agent的两条提交，不据此视为代码验收。Launcher、Reader、Blog在干净且HEAD仍等于固定SHA时才恢复detached；weather/file-manager保留原状。
