# AutoIME 资源与依赖登记（PLAN-070 T-03；PLAN-071 T-01 迁至 shared/）

本目录是 AutoIME 随版本发布的 Rime 资源（PLAN-071 起 Windows/Linux 共用源
数据，路径 `experimental/autoime/shared/resources/`）。部署时由安装/首启流程
复制到 AutoIME 私有用户目录（Windows：`%LOCALAPPDATA%\AutoIME\rime`），不修改
小狼毫的共享目录，也不覆盖用户已编辑的 `custom_phrase.txt`。

## 本仓原创资源

| 文件 | 作用 | 字节 | SHA-256 |
|---|---|---|---|
| `autoime_pinyin.schema.yaml` | 默认方案 `autoime_pinyin`（全拼/简拼/全简混输、术语、中英混输、简体输出） | 3000 | `7012834E307DE0634F39974C235B34DBD205EC219747AD42FF35CADAE2A5CB1A` |
| `autoime_terms.dict.yaml` | 项目术语词表（原创条目，逐字拼音码） | 779 | `9B798B3DCEC8F86672BFE8F532FFA5D35C6732905E3E2ABE7239922C399A8748` |
| `autoime_terms.schema.yaml` | 术语表部署入口（依赖方案，不在切换器展示） | 716 | `B5B652FD70DAAF58BE8F0DEFF7FDC8197D95197A07AB31C95A055A24128BAEAA` |
| `autoime_en.dict.yaml` | 中英混输别名表（字母码，提交保留大小写/空格） | 668 | `0F4FD81D0ADF12862C924C35EB31711D3B955D6DD3EF17B1A2C409E793840A4F` |
| `autoime_en.schema.yaml` | 别名表部署入口（依赖方案，不在切换器展示） | 656 | `48E3A09E0A31A6F78581BBD70D150B7DD5D36DF834EEB175D75B46B8F4312397` |
| `custom_phrase.txt` | 用户可编辑的显式短语模板（已存在时不覆盖） | 370 | `FB897BF717E34B5B4751D73E4878A87AF133F11133ED0D349AF77A2F0D3C0EB4` |

依赖方案（`autoime_terms`/`autoime_en`）的作用是让 librime 的部署器编译对应
词表：部署器只为 `schema/dependencies` 中每个方案的 `translator/dictionary`
准备词典。它们不出现在方案选单中。

## 运行时依赖（本机已安装的小狼毫）

AutoIME 复用已安装的小狼毫 0.17.4 的 `rime.dll` 与共享数据；本仓不复制、
不再分发这些文件。版本与校验值来自本机 2026-11-17 实测：

| 组件 | 路径（本机） | 版本/字节 | SHA-256 |
|---|---|---|---|
| librime | `C:\Program Files\Rime\weasel-0.17.4\rime.dll` | 1.13.1（文件版本 1.13.1.0）/ 3524096 | `2D8F1BC3737635A11D9FB1BFCA4DC9E70533633930A8A0142A81CA879C39C45B` |
| 通用字词 | `...\data\luna_pinyin.dict.yaml` | 962079 | `DC15421F70E80B5D9DC0339DB79856B671A58F8E6779C1B5B7592B83A57B0817` |
| 基础方案 | `...\data\luna_pinyin.schema.yaml` | 2184 | `A80C89BD16D31842BACBEE6B17B9BA1A4F5FD862B7DB31049A229AED8628299E` |
| 拼音代数 | `...\data\pinyin.yaml` | 4965 | `F2D291166946A848FD88E541844567415F1ED3D9C5A7477757B88AEB3A69C0CA` |
| 全局默认 | `...\data\default.yaml` | 1431 | `5A438C92AA7DDD749644C7C799967074634D41ACE924859BE2A371F29565C892` |
| 按键绑定 | `...\data\key_bindings.yaml` | 3021 | `19C52D80A4BF043C16C49FDFEB58CFA4D7A9F2B94BD18C79FF2485BD7627DB5F` |
| 标点 | `...\data\punctuation.yaml` | 2212 | `C9E5147FDA24D1A13085075DF5A2E73DEECB643069BE3751C6D4D793CEB2DE5B` |
| 符号 | `...\data\symbols.yaml` | 28729 | `D3C1823F0E3BAE5FB2DAC338F4863A0D7164436D1EE132727FF5C4687F53601A` |
| 简繁转换 | `...\data\opencc\t2s.json` | 428 | `EBE32A6766BFBAEA1869F82368F54A8FC6C977A235E8A896B618714872B5DF09` |

解析顺序：`AUTOIME_RIME_DLL`/`AUTOIME_RIME_SHARED_DATA_DIR` 环境变量覆盖 →
注册表 `WeaselRoot`；缺失时按键直通（不阻塞宿主）。`rime.dll` 缺失或方案
损坏时的回退见 T-04 部署章节。

## 许可证与来源

- 本项目资源（上表前半）为本仓原创，随本仓许可证分发。
- librime 为 BSD-3-Clause；小狼毫（Weasel）为 GPL-3.0；其共享数据词表源自
  CC-CEDICT、开放词典等上游资源，逐文件许可与署名以 Weasel 0.17.4 发行包
  为准。AutoIME 仅在本机调用已安装的小狼毫，不在本仓再分发这些文件。
- 若后续改为自带共享数据，需按上述来源逐项补充许可与版本登记。

## 中英混输触发约定

数字键在组合中用于选词（`menu/alternative_select_keys: "12345"`），因此别名
码保持纯字母。默认别名例如 `ros` → `ROS 2`、`autoos` → `AutoOS`、
`tsf` → `TSF`（见 `autoime_en.dict.yaml`）。设计要求示例中的 `ros2` 形态
需要把数字纳入编码，会牺牲数字选词；本方案选择数字选词，触发条件由方案
定义（Design 05 §5 允许）。
