# AutoIME Windows x64（PLAN-069 / PLAN-070 / PLAN-071）

AutoIME 的 Windows x64 离线拼音输入法：TSF 文本服务 + librime 引擎 + 原生
Win32 候选窗。PLAN-069 交付了记事本上屏原型（代码 `94f1101`）；PLAN-070
在其上交付日常试用版（按键/编辑/方案/部署），行为合同见
[Design 05](../../../docs/design/05-autoime-windows.md)。PLAN-071 交付
Windows x64 离线安装包，并把方案资源与回放用例升为跨平台共享
（`../shared/`，Linux 由后续计划消费）。

## 目录

```
probe/         引擎探针：直接加载 rime.dll 回放词库、方案与候选行为
ime/           TSF 文本服务、按键路由、编辑会话、候选窗、部署与诊断
  tests/       自动化夹具（TSF 回调序列、编辑计划、emoji）与 T-04 部署套件
  tools/       autoime_deploy.exe（同步部署 + 校验 + 延迟测量）
installer/     Inno Setup 离线安装包定义与构建入口（PLAN-071）
scripts/       安装/卸载脚本（UAC；安装时预部署资源）
../shared/resources/  默认方案 autoime_pinyin、术语/别名词表、用户短语模板、
                      依赖台账（Windows/Linux 共用源数据）
../shared/tests/      平台无关按键/候选期望用例（Windows/Linux 共用）
```

## 前置

- Windows 11 x64；小狼毫（Weasel）0.17.4 已安装（提供 `rime.dll`
  = librime 1.13.1 与共享数据；路径来自注册表 `WeaselRoot`，
  可用 `AUTOIME_RIME_DLL` / `AUTOIME_RIME_SHARED_DATA_DIR` 覆盖）。
- MSVC 2022 + CMake + Ninja。

## 构建

```cmd
cd ime
build.cmd          :: build\autoime_ime.dll + verify_registered.exe
                   :: + autoime_fixture.exe + autoime_deploy.exe
cd ..\probe
build.cmd          :: build\rime_probe.exe
```

## 自动化验证

```cmd
ime\test.cmd                      :: 先跑 ime\build.cmd；此后仅运行（136 checks，<0.1s）
probe\build\rime_probe.exe --schema autoime_pinyin ^
    --resources-dir ..\..\shared\resources
                                  :: 方案/候选套件（T-03，10 用例，~3s）
probe\build\rime_probe.exe --schema autoime_pinyin ^
    --resources-dir ..\..\shared\resources ^
    --cases ..\..\shared\tests\cases.json
                                  :: 共享用例回放（PLAN-071 T-01：空白用户数据，
                                  :: 逐例排名与首选/前五命中率，只读不选词）
pwsh -NoProfile -File ime\tests\run-t04.ps1
                                  :: 部署/回退/升级/并发/延迟（T-04，~30–40s）
```

`run-t04.ps1` 覆盖：全新用户目录冷启、未变更重部署、引擎缺失直通、主方案
损坏回退朙月、坏资源升级回滚到上一可用版本、升级保留用户短语与学习库、
双宿主并发冷启、就绪后按键到候选延迟（中位数/P95）。

## 安装、部署与恢复

安装需要管理员权限（HKLM 注册）：

```cmd
scripts\install.cmd [optional\path\to\autoime_ime.dll]
```

安装流程：复制 DLL/`autoime_deploy.exe`/`resources` → **预部署** Rime 资源
（首次部署在安装时编译词表，首个按键不等待）→ `regsvr32` 注册 → 加入
当前用户 Win+Space 列表 → 清理原型遗留的 `%TEMP%\autoime-ime.log`（明文
键轨迹）。已加载的旧 DLL 无法覆盖时会被改名停放，下次安装清理。

运行期：文本服务在 `Activate` 时把资源解析与部署放到工作线程；就绪前的
按键全部直通。部署失败时依次回退：上一可用资源版本 → 朙月简化字
（`luna_pinyin_simp`）→ 引擎禁用（键盘直通）。资源与用户数据都在
`%LOCALAPPDATA%\AutoIME\rime`，版本标记 `.autoime-resources`（内容哈希），
`custom_phrase.txt` 与学习库 `autoime_user.userdb` 在升级时保留。

卸载默认保留用户数据：

```cmd
scripts\uninstall.cmd          :: 移除注册与程序文件，保留短语/学习库
scripts\uninstall.cmd --purge  :: 同时删除 %LOCALAPPDATA%\AutoIME
```

单独重部署（无需管理员）：

```cmd
"%ProgramFiles%\AutoIME\autoime_deploy.exe" ^
  --resources-dir "%ProgramFiles%\AutoIME\resources" ^
  --user-data-dir "%LOCALAPPDATA%\AutoIME\rime" --latency 20
```

退出码：0 主方案就绪；5 回退朙月；3 引擎禁用；2 缺少 rime.dll。

## 方案与资源

默认方案 `autoime_pinyin`：通用字词复用已安装小狼毫共享数据，术语、
中英别名与用户短语由 `resources/` 提供（版本、字节、SHA-256、许可证
来源见 [resources/README.md](../resources/README.md)）。数字保持选词键，
中英别名使用字母码（如 `ros` → `ROS 2`）。用户短语在同目录
`custom_phrase.txt`，增删后重新部署即生效。

## 隐私

默认诊断只记录身份、阶段码、计数与耗时；按键路径不记录键码、预编辑、
选区或提交正文。敏感输入范围（密码框等）与宿主禁用 IME 时整键直通，
不建立学习会话，不显示候选。原型的明文日志位置与清理入口见上。

## 集成验收记录（2026-11-17，交付构建 = 安装目录 `autoime_ime.dll`）

每个场景开始前先做 TIP 判定：在目标窗口起一段组合，枚举宿主进程中是否
出现 AutoIME 自带候选窗类 `AutoIMECandidateWindow`；小狼毫不产生该窗口，
因此可区分两者。判定失败时脚本中止，不向其他窗口发送按键。

| 宿主 | 结果 | 证据 |
|---|---|---|
| 记事本（WinUI） | 选区替换「甲旧词乙」→「甲你好乙」；Esc 取消恢复「甲旧词乙」；emoji 前缀/后缀「甲😀你好乙」；`autoos`→`AutoOS`、`ros`→`ROS 2`、`tsf`→`TSF`；半截拼音 `ni` 切到另一窗口提交 `好`，旧窗无串写且预编辑被清空；单击 Shift 切英文后 `abc` 直通，Ctrl+S 打开宿主“另存为”对话框、Esc 关闭后正文不变 | `plan-artifacts/plan-070/t05/notepad-verified-2.log` |
| 浏览器文本框（Edge/Chromium） | 选区替换「甲旧词乙」→「甲你好乙」；Esc 取消恢复「甲旧词乙」（宿主终止组合时按保存的原选区回写）；`tsf`→`TSF` | `plan-artifacts/plan-070/t05/edge-verified.log` |
| 041 AutoEdit | **待测**：验收时 AutoOS 桌面/AutoEdit 未运行（仅有 `auto-ui-tool` 进程，无 AutoEdit 窗口），不能计作通过。复验入口：启动 AutoOS 桌面并打开 041 AutoEdit，重复上面的选区替换/取消、中英混输与半截拼音切焦 | — |

候选窗：纵向原生 Win32 窗口（非小狼毫横向候选条），按引擎标签显示
1–5，`GetTextExt` 成功且随光标移动；日志记录每显示器 DPI（本次 192）与
窗口矩形。候选窗可见时每 250ms 重新查询光标并重定位，DPI 变化时重建字体；
预编辑行显示在候选之上，光标按引擎 `cursor_pos` 落在组合内部。手感与视觉
最终由用户确认。

## 探针（词库层）

```cmd
cd probe
build\rime_probe.exe                       :: 默认注册表定位，临时空用户目录
build\rime_probe.exe --keep                :: 保留用户目录
build\rime_probe.exe --rime-dll PATH --shared-data-dir DIR ^
    --user-data-dir DIR --schema ID [--resources-dir DIR]
```

退出码：0 全过；2 缺 rime.dll；3 缺方案文件；4 部署/当前方案不符；
5 有用例失败。`SUMMARY` 行输出 DLL 版本、位数、方案、部署耗时与用例表。
