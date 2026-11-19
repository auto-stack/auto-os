# AutoIME 第三方组件与数据声明（PLAN-071 T-02 离线安装包）

AutoIME 的方案资源（`autoime_pinyin`、术语/别名词表、用户短语模板）为
AutoIME 仓原创，随 AutoIME 分发。安装包同时随带以下第三方运行时与数据，
供无网络、无开发工具的干净 Windows 11 x64 系统使用；来源为已安装小狼毫
（Weasel）0.17.4 发行包，逐文件来源、版本与 SHA-256 见
`installer/bundle-manifest.json` 与 `shared/resources/README.md`。

## librime

- 文件：`rime.dll`（随包，安装于 `C:\Program Files\AutoIME\rime.dll`）
- 版本：1.13.1（文件版本 1.13.1.0，3,524,096 字节）
- 许可证：BSD-3-Clause（librime 上游）
- 来源：小狼毫 0.17.4 发行包内随附的 librime 构建版
- 版权：Copyright (c) 伤脑筋 (lotem) 等 librime 作者

## 朙月拼音词库与基础方案数据

- 文件：`data/luna_pinyin.dict.yaml`、`data/luna_pinyin.schema.yaml`、
  `data/luna_pinyin_simp.schema.yaml`、`data/stroke.dict.yaml`、
  `data/stroke.schema.yaml`、`data/pinyin.yaml`、`data/default.yaml`、
  `data/key_bindings.yaml`、`data/punctuation.yaml`、`data/symbols.yaml`
- 许可证：随小狼毫发行包分发（GPL-3.0）；其中词表上游源自 CC-CEDICT、
  开放词典网等开放资源，逐项署名以 Weasel 0.17.4 发行包与其文档为准
- 用途：`autoime_pinyin` 默认方案的通用字词（`dictionary: luna_pinyin`）、
  全简混输代数（`pinyin.yaml`）、按键/标点/符号预设，以及引擎禁用前的
  最后回退方案（`luna_pinyin_simp`）

## OpenCC 简繁转换数据

- 文件：`data/opencc/t2s.json`、`data/opencc/TSCharacters.ocd2`、
  `data/opencc/TSPhrases.ocd2`
- 许可证：随小狼毫发行包分发（OpenCC 为 Apache-2.0/LGPL 系，数据随
  Weasel 发行包许可为准）
- 用途：`autoime_pinyin` 的简体输出过滤（`simplifier: t2s.json`）

## 明确不随带的文件

- `data/essay.txt`（八股文语言模型词频，仅可选 grammar 配置引用；
  `grammar:/hant?` patch 缺席时静默跳过，部署与候选行为不受影响，
  见 T-02 验证记录）。

## 许可证合规说明

随包的 librime 与 Rime 共享数据取自小狼毫（Weasel，GPL-3.0）发行包并
原样再分发，未作修改；本文件即对应声明。AutoIME 原创资源（方案、词表、
模板、宿主代码）的许可随 AutoIME 所在仓声明，不适用上述第三方许可。
安装包完全离线：构建时逐文件核对 SHA-256 后打包，安装与运行均不访问
网络，不调用任何在线服务。
