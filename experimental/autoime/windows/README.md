# AutoIME Windows x64 原型（PLAN-069）

AutoIME 的 Windows x64 离线拼音输入法预览原型：TSF 文本服务 + librime
引擎 + 原生 Win32 候选窗。验收宿主为系统记事本（见 `docs/plans/069-autoime-windows-x64-tsf-preview.md`）。

## 目录

```
probe/   T-01 引擎探针：直接加载 rime.dll 复现 PLAN-069 基线行为
ime/     T-02/T-03 TSF 文本服务（后续步骤）
scripts/ 注册/注销脚本（后续步骤）
```

## 引擎探针（T-01）

前置：小狼毫（Weasel）0.17.4 x64 已安装（注册表
`HKLM\SOFTWARE\WOW6432Node\Rime\Weasel\WeaselRoot` 可解析），MSVC 2022 +
CMake + Ninja。

```cmd
cd probe
build.cmd                                  :: vcvars64 + cmake ninja，产出 build\rime_probe.exe
build\rime_probe.exe                       :: 默认：注册表定位 rime.dll/data，
                                           ::   %TEMP% 下新建空用户目录，跑完即删
build\rime_probe.exe --keep                :: 保留用户目录（调试部署产物）
build\rime_probe.exe --rime-dll PATH --shared-data-dir DIR --user-data-dir DIR --schema ID
```

探针内容（与计划「已知基线」一致）：

1. `nihao` → 组合串 `ni hao`，按 `1` 选「你好」→ 恰好一次提交，组合清空；
2. `ni` + Esc → 组合取消、无提交；
3. 双会话组合与候选相互隔离。

退出码：0 全过；2 缺 rime.dll；3 缺方案文件；4 部署/当前方案不符；
5 有用例失败。`SUMMARY` 行输出 DLL 版本、位数、方案与路径（JSON）。

已知实现注记：

- 用户目录通过写入 `default.custom.yaml` 把 schema_list 固定到所选方案。
- librime 的 `get_commit/get_context` 产物由 `free_commit/free_context` 归还
  引擎，探针不跨 CRT 释放。
- 引擎 `finalize()` 后 CRT 退出路径不可靠（静态析构卡死），探针在完整
  关停（销毁会话、finalize、清理目录）后用 `_exit` 结束。
- Debug 构建下 CRT 断言默认弹模态 MessageBox（控制台程序不可见，
  表现为"挂死"）；探针在 main 入口把报告重定向到 stderr/调试器。
