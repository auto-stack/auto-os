# AutoIME Windows x64（PLAN-069 / PLAN-070）

AutoIME 的 Windows x64 离线拼音输入法：TSF 文本服务 + librime 引擎 + 原生
Win32 候选窗。PLAN-069 交付了记事本上屏原型（代码 `94f1101`）；PLAN-070
在其上交付日常试用版（按键/编辑/方案/部署），行为合同见
[Design 05](../../../docs/design/05-autoime-windows.md)。

## 目录

```
probe/    引擎探针：直接加载 rime.dll 回放词库、方案与候选行为
ime/      TSF 文本服务、按键路由、编辑会话、候选窗、部署与诊断
  tests/  自动化夹具（TSF 回调序列、编辑计划、emoji）与 T-04 部署套件
  tools/  autoime_deploy.exe（同步部署 + 校验 + 延迟测量）
resources/ 默认方案 autoime_pinyin、术语/别名词表、用户短语模板、依赖台账
scripts/  安装/卸载脚本（UAC；安装时预部署资源）
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
ime\test.cmd                      :: 按键路由/编辑计划/隐私（136 checks，~1.5s）
probe\build\rime_probe.exe --schema autoime_pinyin ^
    --resources-dir ..\resources  :: 方案/候选套件（T-03，10 用例，~3s）
pwsh -NoProfile -File ime\tests\run-t04.ps1
                                  :: 部署/回退/升级/并发/延迟（T-04，~24s）
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
