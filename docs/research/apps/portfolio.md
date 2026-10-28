# 下一版应用组合建议

日期：2026-10-22；全篇优先级是研究建议 D，尚未替代已批准计划。版本号暂用“下一版”，避免与个别 app 自身的 v0.7 编号混淆。

2026-10-29：面向“基本可用 + Linux 移植”的进一步排序见[首批优先级](linux-mvp-priorities.md)。该补充将 AutoTerm 的 Linux 验证提前，区分首批应用、配套工具与早期媒体探针；本文保留原产品组合与研究背景。

## 1. 目标用户与范围假设

先服务个人开发者、研究/写作者与本地资料较多的知识工作者：每天整理文件、阅读资料、捕获笔记、管理任务、编写内容或代码。该人群与 AutoShell、AutoMusk、Jade/编辑器的现有方向相邻，应用间连贯性有实际价值。大众娱乐、企业组织协作、专业影像制作可保留入口或集成现成产品，不作为首批全量自研目标。

原生 app 至少能离线完成其核心任务，保存后重启可恢复，数据可导出，出错有真实状态；依赖网络的 app 说明缓存及断网行为。所有“支持”需附平台、后端轨道和功能范围。Linux 独立桌面同时需要浏览器、输入法、网络/音频/电源、文件选择与通知等基础能力，不能只看 app 列表。

## 2. 当前事实与盘点边界

盘点依据是本仓 `apps.manifest`、`.gitmodules`、README、应用导入报告及可读的 app 文档。manifest 有 **28 条 active 记录**；active 表示登记，不表示 28 个成熟产品。README 的部分历史统计与当前 manifest 不一致，本轮以 manifest 计数，不顺带改写历史叙述。

多个 app 在此检出只有空目录，未能审查当前代码；因此“需基线审计”不等于“没有功能”。文件管理器例外：本地 README 记载 PLAN-001..006 已实施、待统一复审；能力包含多选/批量操作/预览/递归搜索/分片传输。这里承认已有进展，但不把 README 的全部绿灯当本轮复现证据。

## 3. 全部 28 个注册入口的去向

2026-10-29 用户纠正：Jade Garden 为 AutoJade 的旧名。下表保留两条 manifest 接入事实，但产品只计一个；当前共 27 个去重产品，基本能力分类见[盘点](basic-capability-audit-20261029.md)。

交付角色：**核心**=下一版日用基础；**增量**=在基础上扩大场景；**专项**=保留既有产品主线；**可选**=继续保留，按需安装/成熟度展示。顺序不是取消已有范围的授权。

| manifest ID / 现有 app 路径 | 建议角色 | 下一版应交付的最小真实闭环 | 竞品研究 |
|---|---|---|---|
| `027-file-manager` / `apps/auto-explorer` | 核心 | 定位→预览→批量复制/移动→冲突处理→错误/取消可解释 | [文件](file-manager.md) |
| `028-launcher` / `apps/auto-launcher` | 核心 | 搜索应用/文件/命令→看懂结果→完成动作；失效结果可恢复 | [工具](system-developer.md#launcher) |
| `015-notes` / `apps/auto-notes` | 核心 | 快速捕获文字/附件/来源→重启仍在→检索→导出 | [知识](knowledge-productivity.md#notes) |
| `018-book-reader` / `apps/auto-reader` | 核心 | 导入自有 EPUB/PDF→持续阅读→标注→摘录及来源回跳 | [阅读](reader.md) |
| `auto-term` / `apps/auto-term/app` | 核心 | 可靠终端会话→中文输入/选择复制→退出/重连可理解 | [工具](system-developer.md#terminal-shell) |
| `os-config` / `apps/auto-os-config/auto` | 核心 | 修改设置→验证生效→错误可见→可还原/导出 | [工具](system-developer.md#system) |
| `025-sys-monitor` / `apps/auto-monitor` | 核心 | 查看真实资源/进程→定位负载；操作前确认对象身份 | [工具](system-developer.md#system) |
| `039-syslog` / `apps/auto-syslog` | 核心内部组件 | 聚合诊断→筛选→关联 app/任务→脱敏导出 | [工具](system-developer.md#system) |
| `011-calculator` / `apps/auto-calc` | 核心小工具 | 输入表达式→准确结果/单位→历史/复制→解释错误 | [工具](system-developer.md#utilities) |
| `012-clock` / `apps/auto-clock` | 核心小工具 | 计时/闹钟/世界时钟→关闭窗口仍有明确提醒语义 | [工具](system-developer.md#utilities) |
| `020-music-player` / `apps/auto-music` | 日用样板 | 本地库→播放队列→切歌/媒体键→恢复队列/进度 | [媒体](media-creative.md#music) |
| `013-todo` / `apps/auto-todo` | 增量 | 收件箱→项目/今日→到期/重复→完成及恢复 | [知识](knowledge-productivity.md#tasks) |
| `016-calendar` / `apps/auto-calendar` | 增量 | 创建事件→跨时区/重复例外→提醒→ICS 往返 | [知识](knowledge-productivity.md#calendar) |
| `029-photo-gallery` / `apps/auto-photos` | 增量 | 浏览真实目录→大图/EXIF→筛选→不破坏原图的编辑/导出 | [媒体](media-creative.md#photos) |
| `030-video-player` / `apps/auto-video` | 增量 | 打开本地片源→字幕/音轨/seek→续播→错误诊断 | [媒体](media-creative.md#video) |
| `014-weather` / `apps/auto-weather` | 增量小工具 | 地点→真实预报/更新时间→缓存及失效说明 | [工具](system-developer.md#utilities) |
| `kanban` / `apps/auto-kanban` | 专项/工作管理 | 项目→卡片→状态流转→持久化/导出；与 Todo 的关联后续确认 | [知识](knowledge-productivity.md#kanban) |
| `023-realworld` / `apps/auto-blog` | 专项、保留既有承诺 | 本地草稿→预览→带资源的可迁移导出；远端发布显式操作 | [知识](knowledge-productivity.md#blog) |
| `auto-musk` / manifest 外部定位 | 专项 | 一项变更的计划→实现→独立验证→知识沉淀可追溯 | [工具](system-developer.md#flagships) |
| `jade-garden` / manifest 外部定位 | AutoJade 的旧名入口，非独立产品 | 与 jade-edit 归同一 AutoJade；保留接入事实，历史位置/数据兼容另核 | [知识](knowledge-productivity.md#notes) |
| `jade-edit` / manifest 外部定位 | 专项、基线待对齐 | 本地 README 定位 AutoJade 文档/知识库：可靠写作、链接、检索与恢复 | [工具](system-developer.md#flagships) |
| `026-database` / `apps/auto-database` | 可选开发工具 | 首先 SQLite：只读浏览→SQL→分页→显式事务编辑/导出 | [工具](system-developer.md#data-tools) |
| `024-charts` / `apps/auto-charts` | 可选数据工具 | 导入 CSV→类型校验→图表映射→保存规格/导出 | [工具](system-developer.md#data-tools) |
| `031-paint` / `apps/auto-paint` | 可选创作工具 | 截图/图片→裁剪/标注/撤销→导出副本 | [媒体](media-creative.md#paint) |
| `017-chat` / `apps/auto-chat` | 可选；先定产品身份 | 明确 AI 对话还是人与人通信，再定义真实历史/重试/协议 | [知识](knowledge-productivity.md#chat) |
| `036-tetris` / `apps/auto-tetris` | 可选休闲 | 完整规则、键盘、存档/战绩；维持已有交付 | [工具](system-developer.md#games) |
| `037-klondike` / `apps/auto-solitaire` | 可选休闲 | 完整规则、撤销、存档/战绩；维持已有交付 | [工具](system-developer.md#games) |
| `038-minesweeper` / `apps/auto-minesweeper` | 可选休闲 | 完整规则、键盘、难度/战绩；维持已有交付 | [工具](system-developer.md#games) |

AutoShell 是用户明确的主打方向，但不在这 28 个 manifest ID 中；作为生态/产品能力单列研究，不能把它与桌面 Shell 或 AutoTerm 合并统计。外部定位的日常路径按 AGENTS 的 `apps/` 纪律处理；本表只如实记录现状，不新建检出或迁移仓库。

## 4. 下一版还缺什么

| 类别 | 市场/开源参考 | 建议交付方式 | 本轮判断 |
|---|---|---|---|
| 浏览器 | [Firefox](https://www.firefox.com/en-US/)，开源产品；Chromium 留待另轮对照 | 集成外部浏览器，统一 open URL/下载/默认关联 | Linux 日用必需；先验证窗口/输入/剪贴板接入，不自研浏览器内核 |
| 办公文档/表格/演示 | [LibreOffice](https://www.libreoffice.org/)，开源，许可有组件差异 | 集成外部套件 | “系统能支持”有价值；完整 OOXML/排版兼容不适合首批重写 |
| 邮件/联系人 | [Thunderbird](https://www.thunderbird.net/en-US/) | 可选外部集成 | 真实邮箱身份、OAuth、离线索引与日历邀请复杂，先复用 |
| RSS/稍后读 | [Folo](https://github.com/RSSNext/Folo)，AGPL 元数据 | Reader/知识流的后续入口 | 不急于新增桌面 app；阅读格式、来源与导出可以先共享 |
| 下载/传输 | [aria2](https://github.com/aria2/aria2)、[qBittorrent](https://www.qbittorrent.org/) | 可选引擎/外部集成 | 有断点续传/任务队列价值；先核对 AutoDown 的真实定位，不能望文生义当下载器 |
| 密码管理 | [KeePassXC](https://github.com/keepassxreboot/keepassxc) | 外部集成优先 | 系统级凭据能力必需；不新增自研密码保险箱 |
| 同步与备份 | [Syncthing](https://github.com/syncthing/syncthing)、[restic](https://github.com/restic/restic) | 外部工具/适配服务 | 同步与备份分开；文件同步不能直接当正在写入的数据库同步协议 |
| 截图、归档、PDF/打印 | Portal；Reader/Photos/Paint 相关引擎 | 系统动作 + 按需面板 | 先完成共同能力，不为每项新增顶级 app |

第三方 Wayland/X11 客户端集成仍是本仓下一阶段路线，未证明当前桌面已完整支持。Windows/虚拟桌面阶段可显式启动宿主应用；Linux 独立会话需另验生命周期与窗口桥接。外部集成不能冒充已完成 AutoUI 双端原生化。

## 5. 推荐推进顺序与取舍

1. **R0 基线与技术探针**：盘点各 app 实际 pin/功能/测试；优先验证 Reader 排版表面、媒体播放、后台任务/提醒、文件操作语义。产出可行/不支持的能力表。
2. **R1 两条最短流程**：Launcher→Explorer→Reader→Notes；Launcher→AutoTerm/AutoShell→Musk/编辑器。文件与摘录可以先显式导出/import；通信接口以已有 F0/F1/F2 计划实际交付为准。
3. **R1 日用样板并行候选**：音乐队列/媒体键/重启恢复；Blog 保留本地草稿/导出闭环。是否同时启动由容量评估决定，不能靠增加研究条目自动增加承诺。
4. **R2 扩大覆盖**：Todo/Calendar、Photos/Video、Kanban 数据关联；复用已经验证的存储/任务/搜索/媒体接缝。
5. **R3 长期增强**：多端同步、协作、插件生态、语义搜索、复杂数据库/绘图。每一项独立判断价值和维护成本。

优先级依据：文件/启动/持久化是所有场景的地基；阅读与知识流能体现 AutoOS 连贯性；Shell/Musk 已有差异化资产；音乐能以低学习成本验证真实日用。多人聊天、全办公、专业绘画的协议/内容/兼容成本很大，先用集成方式补齐。以上是定性决策，尚无统一工期或人力估算。

**建议的发布声明**：保留现有应用发现能力，按“日用已验收 / 试用 / 示例或专项 / 外部集成”标注真实成熟度。既有已完成游戏等可直接保留，不为突出主线而降级它们的已验证行为。是否隐藏未达门应用属于后续明确的产品决定。
