# 阅读器：Readest 的阅读体验 + 可靠来源与知识流

日期：2026-10-22；对应 `018-book-reader`。本地 app README 未检出，不推断当前功能。建议 D；未安装竞品。

## 市场与候选

电子书阅读大体有四种诉求：购买/消费内容、阅读自有文件、管理大型书库、研究标注与引用。AutoOS 首先覆盖后面三者中的个人日用闭环；书店授权、DRM、跨设备付费内容和发行渠道不随一个阅读 UI 自然获得。

| 产品 | 开源/实现观察 | 优点与吸收内容 | 不直接跟随的范围 |
|---|---|---|---|
| [Readest](https://github.com/readest/readest) | AGPL-3.0 元数据；Next.js + Tauri；README、manifest、类型/平台服务抽样 B | 多格式、排版/滚动、标注、对照阅读、词典/TTS、OPDS、同步等，阅读体验主参考 | 跨端/云同步、翻译/AI/书库服务会扩大产品和运维面，按需分期 |
| [Foliate](https://github.com/johnfactotum/foliate) | GPL-3.0 元数据；桌面阅读器；A | 专注阅读的产品定位；底层 foliate-js 的模块化适合单独评估 | GTK 容器不是 AutoUI 的通用原生组件 |
| [KOReader](https://github.com/koreader/koreader) | AGPL-3.0；Lua；仓库描述 A | 多格式、电子墨水设备场景；研究低资源阅读及跨设备进度的好参照 | 电子纸手势/刷新/设备适配不作为桌面首版需求 |
| [calibre](https://github.com/kovidgoyal/calibre) | GPL-3.0；Python；仓库描述 A | 书库管理定位与格式生态值得第二轮专题阅读 | 格式转换/设备管理和海量元数据工作台不塞进阅读主界面；本轮未复核其每项功能 |
| [Apple Books](https://www.apple.com/apple-books/) | 商业生态产品；官方页 A | 阅读/听书与内容发现有一贯流程 | 不模仿书店规模，不宣称可读它的全部购买内容 |

表中的“优点”是适合研究的产品价值，未比较实际排版质量或资源消耗。

## Readest 实现抽样

抽样版本固定为 `6b2229ea5fa18ece2d6b90f1ba33c88ae13f4b6e`。官方 README 既有当前 Features，又有 Planned Features；AI 摘要等路线不能全部当已完成能力。源码里出现模块也不证明已经向用户稳定发布。

- [`package.json`](https://github.com/readest/readest/blob/6b2229ea5fa18ece2d6b90f1ba33c88ae13f4b6e/apps/readest-app/package.json) 与 README 支持 Web/Next.js、Tauri 和平台测试等实现观察；不代表 AutoUI 原生能直接复用其 React 视图。
- [`.gitmodules`](https://github.com/readest/readest/blob/6b2229ea5fa18ece2d6b90f1ba33c88ae13f4b6e/.gitmodules) 指向 **readest/foliate-js fork**，也有其他定制依赖。因此“直接用上游 foliate-js”不等于取得 Readest 所有能力，应比较 fork 的增量和许可。
- [`types/book.ts`](https://github.com/readest/readest/blob/6b2229ea5fa18ece2d6b90f1ba33c88ae13f4b6e/apps/readest-app/src/types/book.ts) 与 [`annotator.ts`](https://github.com/readest/readest/blob/6b2229ea5fa18ece2d6b90f1ba33c88ae13f4b6e/apps/readest-app/src/types/annotator.ts) 把格式、阅读状态和标注工具分开；可借鉴领域分层，不复制全部枚举当新需求。
- [`appService.ts`](https://github.com/readest/readest/blob/6b2229ea5fa18ece2d6b90f1ba33c88ae13f4b6e/apps/readest-app/src/services/appService.ts) 的平台服务抽象与库/字体/词典等服务分离，值得借鉴。云同步冲突和数据库迁移尚未深读，不能声称已经找到完整可移植方案。

独立排版候选 [`johnfactotum/foliate-js`](https://github.com/johnfactotum/foliate-js) 为 MIT 元数据，提供浏览器模块，README 明确 API 尚不稳定、PDF 支持为实验性且依赖 PDF.js。固定抽样 `78914aef4466eb960965702401634c2cb348e9b1` 的 [`view.js`](https://github.com/johnfactotum/foliate-js/blob/78914aef4466eb960965702401634c2cb348e9b1/view.js) 可见打开书、位置移动、标注、CFI 接口；[`epubcfi.js`](https://github.com/johnfactotum/foliate-js/blob/78914aef4466eb960965702401634c2cb348e9b1/epubcfi.js) 提供位置/范围相关转换。这只是入口抽样，不构成排版兼容验证。

## 综合设计建议

**书库面**以最近阅读、在读、集合/标签、导入和搜索为中心；每个文件记录来源、内容指纹及导入方式，区分“引用原文件”和“复制入库”。路径改变与内容更新分别处理。合并重复书籍前让用户理解依据。

**阅读面**以文本为主，侧栏在目录、书内搜索、标注间切换；字体/字号/行距/页边距/主题/分页与滚动可调。PDF 固定版式和 EPUB 重排分别设计。中英文、竖排、脚注、图片/表格、代码与辅助功能进入真实语料验证。

**摘录面**保存 `书籍身份 + 文件版本/指纹 + 格式定位器 + 原文/上下文 + 用户批注`。EPUB 可以 CFI 为定位候选；PDF 用页/坐标/文本锚点，不能用一个“百分比”定位所有格式。修改排版后应能回跳，源书内容变化则显示重定位/失配，不静默指到错误文字。

摘录导出 Markdown/JSON，带来源和可恢复定位信息；接通 Notes/Jade 后保存稳定引用与可迁移快照。AI 总结、翻译、问答是明确触发的辅助动作，输出带引文且与作者原文区分；不上传整本书作为默认行为。TTS、双书对照、OPDS 与多端同步属于后续增强。

## 引擎方案比较

| 方案 | 收益 | 主要风险/前提 | 初步选择 |
|---|---|---|---|
| AutoUI 外壳 + 浏览器排版引擎（foliate-js 等） | 复用现有 EPUB HTML/CSS 排版 | 原生 VM 是否有可嵌阅读表面、输入/选区/焦点/无障碍/资源访问通道未证 | 优先做双端整体探针，不仅做 Web 演示 |
| AutoUI + 原生文档引擎 | 更直接接原生渲染与性能 | EPUB CSS/CJK、选区、链接、标注难度高；候选引擎、许可和可绑定能力需另选 | 保留为原生接缝失败后的比较方向 |
| 集成 Readest/Foliate 独立应用 | 尽快具备真实阅读能力 | 外部窗口/关联/摘录桥、AGPL/GPL 及分发方式需评估 | 可作为过渡支持方式，成熟度标签写清楚 |
| 重写完整排版系统 | 栈统一 | 高投入且已有成熟领域，兼容缺口大 | 初期不推荐 |

优先 EPUB + PDF 两种真实格式；TXT/Markdown 是否复用已有编辑/预览引擎由基线审计决定。未达到的格式显示不支持，不能列一串扩展名但打开后丢内容。

## 可行性与验收任务

用无版权争议/自建书籍语料：CJK 长文、脚注、目录、多栏/竖排、代码、图片表格、异常 ZIP；PDF 加可复制文本与扫描页两类。跑导入→重排→选区标注→退出重开→回跳→导出→Notes 接收；同一任务在 Vue/VM 端验证。

首个探针必须回答：阅读表面如何进入 VM/桌面合成、字体/中文输入/选区与快捷键如何传递、URI/文件访问如何授权、资源是否泄漏。给出启动/翻页/内存实测和 unsupported 清单。若原生不可行，先明确采用外部集成或缩小原生格式承诺，再立实施 Plan，不用纯文本伪装 EPUB 完整支持。
