# AutoIME 离线安装包（PLAN-071 T-02）

在干净 Windows 11 x64（无小狼毫、无开发工具、断网）上安装 AutoIME 的
单个离线安装包 `AutoIME-Setup-x64.exe`。

## 构建

```cmd
cd ..\ime && build.cmd        :: 显式准备：autoime_ime.dll + 工具
cd ..\installer
build.cmd                     :: 核对随包引擎 → 暂存 → ISCC → dist\AutoIME-Setup-x64.exe
```

- 前置：MSVC 2022 + CMake + Ninja（ime 构建）、Inno Setup 6（ISCC）。
- 随包引擎与共享数据逐项核对 `bundle-manifest.json` 的锁定 SHA-256 与
  字节数后进包；来源为已安装小狼毫 0.17.4（`AUTOIME_WEASEL_ROOT` 环境变量
  → 清单默认根 → 注册表 `WeaselRoot`），不一致即命名失败，不产包。
- 产物哈希写在同目录 `AutoIME-Setup-x64.exe.sha256`；交付记录用同一值。

## 包内容

| 部分 | 内容 | 来源 |
|---|---|---|
| `autoime_ime.dll` | TSF 文本服务（070 构建，版本资源 0.1.0.0） | `windows/ime/build` |
| `autoime_deploy.exe` | 同步部署 + 校验 + 延迟测量 | 同上 |
| `verify_registered.exe` | 注册校验（路径/文件/版本） | 同上 |
| `resources\` | 默认方案/词表/短语模板 | `shared/resources`（T-01 迁移后唯一源） |
| `rime.dll` + `data\` | librime 1.13.1、朙月词库、stroke、代数/预设、OpenCC t2s | 锁定清单（`bundle-manifest.json`） |
| `licenses\` | 第三方声明 | `THIRD-PARTY-NOTICES.md` |

明确不随带 `data/essay.txt`（可选 grammar 词频，patch 带 `?`，缺席即跳过；
见 T-02 验证记录）。许可证：librime BSD-3-Clause；随包共享数据取自小狼毫
（GPL-3.0）发行包原样再分发，声明见上。

## 安装形态

- **机器级**（需要管理员）：程序文件 → `%ProgramFiles%\AutoIME`，
  TSF 注册 `regsvr32`（HKLM）→ `verify_registered.exe` 校验注册。
- **实际使用者身份**：Rime 用户数据预部署（`{userappdata}\AutoIME\rime`，
  首次编译词表）与 Win+Space 列表登记都以 `runasoriginaluser` 执行——
  标准用户以另一管理员账户授权（UAC 肩越）后，安装仍准备该用户自己的
  数据，首次输入即用就绪的默认方案。
- **升级**：宿主仍持有旧 DLL 时同卷改名停放（`autoime_ime_old_*.dll`），
  新 DLL 原位落位；旧停放文件在下次安装/卸载清理。
- **卸载**：撤销 TSF 注册与程序文件，保留 `%LOCALAPPDATA%\AutoIME`
  个人数据（短语 + 学习库）与其他输入法。

发布模式的引擎解析顺序（`rime_engine.cpp`）：`AUTOIME_RIME_DLL`/
`AUTOIME_RIME_SHARED_DATA_DIR` 环境覆盖 → 安装目录（`rime.dll` 与 `data\`
与 DLL 同目录）→ 注册表 `WeaselRoot`（开发机兼容）→ 缺失时按键直通。
