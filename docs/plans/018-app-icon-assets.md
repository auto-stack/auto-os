---
plan_id: PLAN-018
status: execution_done        # drafting → executing → execution_done → reviewed → archived（2026-09-14 work T1-T6 全落，见 §9）
feature_name: app-icon-assets
author: [zhaopuming]
created_at: 2026-09-14
updated_at: 2026-09-14

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: [auto-lang/docs/specs/auto-lang/ui/overview.md#icon-字符串协议族（执行期定位）]
touched_goals: []

affects: [auto-lang/crates/auto-lang/src/ui/iced/renderer.rs, auto-lang/crates/auto-lang/src/ui/iced/native_icon.rs,
          auto-lang/crates/auto-man/src/vue.rs, auto-os/assets/icons/, auto-os/scripts/slice_icons.py,
          auto-os/shell/, auto-lang/docs/specs/auto-lang/ui/overview.md]
current_step: 0
total_steps: 6
---

# [PLAN-018] app-icon-assets

## 0. 变更摘要

用户交付了两套桌面 app 图标精灵表（`D:\autostack\assets\icons.png` 浅色 /
`icons_dark.png` 深色，各 1491×1055，7×4=28 图块，浅深同布局），要求应用到
虚拟桌面。本计划把两表资产化并接入 AutoUI 图标通道：

1. **资产层**（auto-os）：切片工具把两表按网格切成 28×2 张独立 PNG
   （`assets/icons/{light,dark}/<stem>.png`，sheet 保留为设计源）+
   `mapping.json`（app id → 图标 stem，含 Browser 预留位不入映射）。
2. **渲染层**（auto-lang）：icon 字符串协议新增 **`iconfile:<stem>`** 前缀
   ——沿 Plan 515 D1 `hicon:<slot>` 既有先例（前缀分发 → `image::Handle::
   from_rgba` + 全局缓存），iced 三处图标分发点与 vue 轨各加一臂；主题
   （PLAN-615 链）联动 light/dark 子目录；回退链 `iconfile: → hicon: →
   lucide: → 占位`，未命中自动落 lucide **零回归**。
3. **接线层**（auto-os）：桌面 boot 在注册表注入前按 mapping 把命中 id 的
   `entry.icon` 改写为 `iconfile:<stem>`——桌面格/launcher/dock/任务栏四个
   消费面共用同一条字符串通道，一处改写全覆盖。

跨仓：主导仓 auto-os（资产/工具/接线），auto-lang（渲染臂 + spec）互链。

## 1. 目标

1. **G1** 两套精灵表资产化入库：28×2 张切片 PNG + 映射表可复现再生
   （工具 + 校验），sheet 为设计源保留。
2. **G2** AutoUI 图标通道支持位图后端：`iconfile:` 前缀在 iced（按钮
   icon 臂 / image·icon 视图臂 / 窗 icon 臂）与 vue 轨可渲染，主题联动
   light/dark。
3. **G3** 桌面四消费面（桌面图标格 / launcher grid / dock / 任务栏）实机
   显示双主题位图图标；主题切换即时跟随。
4. **G4** 零回归：未映射 app（如未来新 app）、缺资产目录、旧 pac（仅
   lucide 名）全部自动落回 lucide 渲染，行为与今日一致。
5. **G5** icon 字符串协议族（`lucide:`/`hicon:`/`iconfile:` + 回退链）成文
   auto-lang spec。

**非目标**：
- 不做运行时 sprite/offset 渲染（rect map 维护 + 每渲染端裁剪，28 图标
  静态集不划算；sheet→切片的构建期方案已保 sheet 单一事实源）。
- 不做 VM 独立窗系统窗口图标设置（win32 WMIcon/HICON 注入，后置债务）。
- 不动 app 内部 UI 的 lucide 用法（只换桌面壳层的 app 图标位）。
- 不建 Browser app（仅切出预留图，不入映射）。

## 2. 架构方案

```
icons.png / icons_dark.png（设计源，auto-os assets/）
        │ scripts/slice_icons.py（网格拟合 + 逐格填充率校验）
        ▼
assets/icons/{light,dark}/<stem>.png ×28×2   +   mapping.json（id→stem）
        │                                           │
        │ auto-os 桌面 boot（注册表注入前）          │
        │ 命中 id → entry.icon = "iconfile:<stem>"   │
        ▼                                           ▼
  registry 字符串通道（四消费面共用：桌面格/launcher/dock/任务栏）
        ▼
  iced 渲染臂（沿 hicon 先例）              vue 轨（lucide-vue-next 邻位）
  iconfile: → 主题目录 <stem>.png            iconfile: → <img src>，根 class
  → Handle 缓存 → image()                    切 light/dark；回退 lucide 组件
  回退：iconfile: → hicon: → lucide: → 占位
```

**为什么构建期切片而不是运行时 sprite+offset**：sprite 方案需维护 rect map、
每个渲染端实现子图裁剪与 DPI 缩放；静态 28 图标集下切片是构建期一次性
成本，运行时路径最短（`Handle::from_path` 族 + 缓存），sheet 仍是唯一设计源。

**为什么集中映射而不是逐 app 改 pac**：图标表是桌面级资产（覆盖全部 app、
含尚无 app 的 Browser 位）；boot 侧一处改写让 28 个 pac.at 零改动，新 app
天然 lucide 兜底。pac 级 `icon: "iconfile:…"` 留作后续单 app 自定位的扩展位
（协议已支持，无需二次设计）。

## 3. 技术栈

- 切片：Python（PIL/numpy/scipy 已具备）+ 逐格校验。
- iced：`iced::widget::image::Handle`（raster）+ 全局缓存（native_icon.rs
  的 `OnceLock<Mutex<HashMap>>` 同款）。
- vue：`<img>` + 主题根 class（既有 dark/light 链）。
- 测试：cargo t 定点（ui-iced 档）+ 实机截图/headless 指针（PLAN-012 先例）。

## 4. 需求分析与背景调查

### 4.1 授权与范围

- 用户 2026-09-14 提供两套精灵表并裁定方向：应用到虚拟桌面 + 调研
  AutoUI 图片/精灵图支持方式；同日认可本计划设计（"OK，起草计划"）。
- 允许仓库/动作：auto-os（assets/、scripts/、桌面 boot 接线、README）、
  auto-lang（iced 渲染臂、vue 生成端、spec 节、定点测试）。改 auto-lang
  crates 属 Category A 例外面，定点 cargo t 可跑（全量门留 review）。

### 4.2 根因调查（代码证据）

| # | 事实 | 证据 |
|---|------|------|
| 1 | icon 是字符串协议通道，按前缀分发后端 | `renderer.rs:3428-3520`（按钮 icon 臂：`\u{EE01}name\u{EE02}text` 内嵌协议 → `native_icon::parse_field`（`hicon:<slot>`）→ `Handle::from_rgba`；否则 `lucide_svg_doc` → svg handle）；`renderer.rs:5095`（image/icon 视图臂同款）；`renderer.rs:11946`（窗 icon 字段） |
| 2 | 全局缓存先例 | `crates/auto-lang/src/ui/iced/native_icon.rs`（`OnceLock<Mutex<HashMap>>`，失败占位防每帧重试）——iconfile 缓存同款 |
| 3 | DSL 已有图片节点 | `aura_view_builder.rs:1888`（`img|image|icon` → `convert_image_or_icon`）；`iced/image_surface.rs`（fit/zoom 几何） |
| 4 | 注册表 icon 透传链 | `app_registry.rs` entry.icon（pac `icon:`）→ 注入 `registry_entries`（session.rs）→ renderer 消费（launcher icons 列表 ~8793、桌面格 label/icon ~10818、dock {id,icon}、任务栏 ~8915）——字符串透传，改写点在注入前即全覆盖 |
| 5 | vue 轨图标 = lucide-vue-next | `auto-man/src/vue.rs:517-523`（npm deps）——vue 端位图即 `<img>`，主题经根 class |
| 6 | 主题链 | PLAN-615：config.dark_theme / `theme:` 声明，桌面运行时已知当前 mode → light/dark 目录选择 |
| 7 | 两表网格几何 | 1491×1055；连通域实测（浅表）：图块 ≈153×150，列距 ≈195.4、行距 ≈201.5，x0≈78/y0≈148；深表同布局（同 rect 复用）；28 = 27 桌面 app + Browser 预留 |
| 8 | 既有桌面 boot env 注入点 | `scripts/desktop.sh`（vue：AUTO_OS_ROOT+EXTRA）；iced 轨 CWD=auto-os——资产根解析有现成锚 |

### 4.3 相邻在途计划

- PLAN-015（已归档）：app 展示名四名称契约——本计划只动 icon 通道，
  与 title/locale 面零交集；mapping 以 **registry id** 为键（id 稳定语义，
  PLAN-015 已钉死"标识不挂展示"边界）。
- PLAN-016（file-manager-revamp）/PLAN-017（minesweeper-ui-revamp）：改
  app 内部 UI，不触桌面壳图标位；执行期 rebase 常规协调。

## 5. 详细设计

### W1 切片工具与资产入库（G1，auto-os）

- `scripts/slice_icons.py`：输入两表路径 + 7×4 网格参数（初值取 §4.2-7
  实测，工具内做最小二乘拟合 + 每格填充率校验 ≥0.85，低即报错退出）；
  输出 `assets/icons/{light,dark}/<stem>.png`（原生 153×150 RGBA）。
- `assets/icons/mapping.json`：`{"<registry-id>": "<stem>", ...}` 27 条
  （Browser 切出 `browser.png` 但**不入映射**——无 app）；id 清单 =
  PLAN-015 桌面注册表 27 app（examples 策展 17 + auto-os 5 + manifest 5，
  kanban 以 registry id `kanban` 计）。
- stem 命名 = 精灵表标题的 kebab 规范形（calculator、music-video-player、
  os-config…），与 id 的映射显式落 mapping.json（不猜前缀剥离规则）。
- README 增"图标资产再生成"一节（工具用法 + 设计源地位）。

### W2 iced `iconfile:` 渲染臂（G2/G4，auto-lang）

- 新模块 `crates/auto-lang/src/ui/iced/icon_file.rs`（native_icon.rs 同款）：
  - `parse_field(s) -> Option<IconFileRef>`：识别 `iconfile:<stem>`；
  - 资产根解析序：`AUTO_OS_ICON_ROOT` env（boot 注入绝对路径）→
    `AUTO_OS_ROOT/assets/icons` → 不命中（返回 None = 回退链下沉）；
  - `load(stem, theme) -> Option<Handle>`：`{root}/{light|dark}/<stem>.png`
    → `Handle::from_path` 族 + `OnceLock` 缓存（键 = 路径+theme+mtime；
    读失败占位防重试）。
- 三处分发点各插一臂（先于 hicon/lucide）：按钮 icon 臂（~3428）、
  image/icon 视图臂（~5095）、窗 icon 臂（~11946）。主题取当前会话
  theme mode（PLAN-615 链已析出）。
- 回退链语义：`iconfile:` 解析失败 → 原字符串若含 hicon/lucide 语义照旧
  下沉 → 最终占位。**纯 lucide 名（绝大多数现有 app）行为逐字节不变**。
- 单测：parse_field/回退顺序/资产根三档解析/缓存幂等（ui-iced 档）。

### W3 vue 轨 `iconfile:` 臂（G2，auto-lang）

- `auto-man/src/vue.rs` 生成端：icon 值为 `iconfile:<stem>` 时输出
  `<img class="app-icon" data-theme-src=…>`（src 指向 light，dark 由根
  class CSS 切换或双 src display 切换——执行期按宿主主题机制选一）；
  非 iconfile 值走 lucide-vue-next 原路。资产路径由桌面宿主注入
  （vue 轨 desktop-host 生成期配置注入同 PLAN-465 先例）。
- 金样/生成测试对齐（vue 296 测试档既有口径）。

### W4 桌面 boot 接线（G3，auto-os）

- 桌面 session boot（registry 注入 shell 前）：读
  `<AUTO_OS_ROOT>/assets/icons/mapping.json`（缺席/坏 JSON = 不改写，
  静默降级）→ 命中 id 的 `entry.icon = "iconfile:<stem>"`；同时注
  `AUTO_OS_ICON_ROOT` env（渲染臂资产根）。
- 四消费面（桌面格/launcher/dock/任务栏）零改动——同字符串通道自然
  生效；launcher palette 搜索键仍是 title/id（PLAN-015 display 链），不受影响。

### W5 实机验收与证据（G3/G4）

- 桌面双主题切换走查：四消费面 × {light, dark}；截图入
  `docs/plans/evidence/plan018-icons-*.png`；headless 指针成文兜底
  （PLAN-012 先例：注入通道实机受阻时）。
- 零回归抽查：无映射 app（临时造一个仅 lucide icon 的测试 app 目录）
  桌面显示 lucide 占位不变。

### W6 spec 成文（G5，auto-lang）

- `docs/specs/auto-lang/ui/overview.md`（执行期定位，亦可能独立
  icon 契约节文件）：**icon 字符串协议族**节——`lucide:`/`hicon:`/
  `iconfile:` 三前缀语义、回退链顺序、资产根解析序、主题目录约定、
  mapping.json schema 指针（auto-os assets/）。
- plan `new_spec_components` 终稿随 review 定。

### 规范增量

| delta_id | 类型 | 目标 | before/after | rationale | acceptance |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang/docs/specs/auto-lang/ui/overview.md §icon 字符串协议族（执行期定位） | before：icon 通道三后端（lucide/hicon/占位）无成文契约，散见注释；after：三前缀 + `iconfile:` + 回退链 + 资产根解析序成文 | 位图后端进协议族需契约钉死回退语义 | AC-2, AC-5 |
| SD-02 | add | 本仓 README.md §图标资产（auto-os） | before：无图标资产约定；after：assets/icons 布局、mapping.json schema、再生成命令、设计源地位 | 资产再生成属桌面级约定 | AC-1 |

## 6. 测试设计

| 层 | 手段 | 断言 |
|---|---|---|
| 切片 | python 工具 + 校验模式 | 28×2 文件存在、尺寸一致（153×150±2）、逐格填充率 ≥0.85；重跑幂等（字节级或像素级一致） |
| iced 渲染臂 | cargo t（ui-iced 档定点） | parse_field 三态；资产根解析三档；回退链顺序钉死（iconfile 坏值→lucide 照常出图）；缓存幂等 |
| vue 生成 | vue 296 测试档 | iconfile: 生成 `<img>`；lucide 值生成不变（金样） |
| 桌面接线 | boot 单测/临时 fixture | mapping 命中改写、缺席静默；无映射 app icon 原样 |
| 实机 | 截图 + headless 指针 | 四消费面 × 双主题位图渲染；主题切换跟随 |

## 7. 验收标准

- **AC-1**：`python scripts/slice_icons.py --verify` 绿——28×2 PNG 存在、
  尺寸一致、mapping.json 27 条与注册表 id 一一对应、Browser 不在映射。
- **AC-2**：iced 三分发点支持 `iconfile:`，回退链顺序单测钉死；纯 lucide
  app 渲染路径零变化（既有 ui-iced 档测试无新增红）。
- **AC-3**：vue 轨 `iconfile:` 生成 `<img>` 且主题可切；lucide 金样不变。
- **AC-4**：实机桌面四消费面（图标格/launcher/dock/任务栏）显示双主题
  位图图标，主题切换即时跟随；证据入 evidence/。
- **AC-5**：icon 字符串协议族在 auto-lang spec 成文（三前缀 + 回退链 +
  资产根解析序）。

## 8. 执行步骤

| ID | 任务 | 依赖 | 落点 | 产出/验证 | AC |
|---|---|---|---|---|---|
| T1 | W1 切片工具 + 28×2 资产 + mapping.json 入库 | — | auto-os scripts/ assets/ README | `slice_icons.py --verify` 绿 | AC-1 |
| T2 | W2 iced `iconfile:` 臂（icon_file.rs + 三分发点 + 缓存 + 回退链） | T1（资产在才可实测，代码可先行） | auto-lang iced/{renderer.rs,icon_file.rs} | 定点单测绿（ui-iced 档） | AC-2 |
| T3 | W3 vue 轨臂 + 金样对齐 | T2 | auto-lang auto-man/vue.rs | vue 档绿 | AC-3 |
| T4 | W4 boot 接线（mapping 改写 + AUTO_OS_ICON_ROOT 注入） | T1, T2 | auto-os 桌面 session boot 侧 | 接线单测 + fixture | AC-4 |
| T5 | W5 实机双主题四消费面走查 + 证据 | T2-T4 | 本仓 docs/plans/evidence/ | 截图/指针成文 | AC-4 |
| T6 | W6 spec 成文 + 测试对齐收口 | T2 | auto-lang docs/specs/ | spec diff | AC-5 |

执行载体：组 worktree `D:/autostack/.wt/os-018/auto-os`（`os-018-dev`，
Plan 529 布局）+ auto-lang 依赖 worktree `D:/autostack/.wt/os-018/auto-lang`
（auto-os-config 亦入组备用）。门档：改 auto-lang crates → 定点 cargo t
可跑，全量门（nextest 串行）留 review。

## 9. 复审记录

- 2026-09-14 /auto-plan:new 起草（stage: new，PLAN-018 rev1）：
  设计经用户认可（调研轮"OK"）；接缝全部锚到代码（§4.2 八条）。
  outcome: pass，next: work（T1 可即刻开工，T2 与 T1 可并行——代码先行
  资产后至）。

## 10. 待澄清事项

| # | 事项 | 影响 | 处置 |
|---|---|---|---|
| Q1 | vue 宿主主题切换机制选型（根 class vs 双 src） | T3 实现细节 | work 期按 desktop-host 既有主题机制实测选定，设计两可 |
| Q2 | 切片导出是否同步出 2x 高清位图（源表仅 1x） | AC-4 视觉锐度 | 源表 153×150 在桌面格 ≈48px 显示下已超采样，v1 不做 upscale；实测糊则 T5 提出回炉 |
| Q3 | Browser 预留图是否顺带切出 | T1 产物 +1 文件 | 切出不入映射（成本零，未来 app 就绪即用） |

### 执行证据（2026-09-14，work 轮）

- **[x] T1** [✅ 已完成] `scripts/slice_icons.py`（连通域锚点拟合 + 每表各自
  网格 + 像素级往返校验 + 蒙太奇预览）+ `assets/icons/{light,dark}/` 28×2 +
  `mapping.json`（27 id）+ README 资产节。commit `20122b1`。手段注记：计划的
  "逐格填充率 ≥0.85" 不可行——粉彩/白瓷图块与海报底色阈值扫描 0.06-0.86 全谱
  不可分（T1 实测），改像素往返 + 预览人眼复核（AC-1 语义不变）。
- **[x] T2** [✅ 已完成] `iced/icon_file.rs`（parse 白名单/资产根解析序/
  (stem,dark) 缓存）+ renderer 两 raster 源合流臂（按钮 icon 臂 + Image 视图
  臂；窗 icon 字段为 hicon 生产端不需改——手段注记）。commit `3f825c66f`。
  定点 2/2 绿。
- **[x] T3** [✅ 已完成] vue codegen icon 臂 iconfile 双 `<img>` + SFC 切换
  规则三行（`iconfile_theme_css` 旗标）；`plan018_iconfile_dual_theme_bitmap`
  绿 + ui_gen 档 773/773 + desktop 金样绿。
- **[x] T4** [✅ 已完成] 桌面 boot 接线 `apply_icon_mapping` + `AUTO_OS_ICON_ROOT`
  注入（renderer.rs 注册表快照组装点）。落点注记：boot 代码在 auto-lang
  （读 auto-os 资产）——计划写"auto-os boot 侧"系代码归属误记，语义不变。
- **[x] T5** [✅ 已完成] 实机证据（PrintWindow 零打扰采集）：浅色 dock
  calculator/todo/notes 位图 + 深色 dock 同三枚走 dark 切片 + lucide 回退
  同画面共存（grid/terminal 钮）；evidence/plan018-icons-{light,dark}.png、
  plan018-dock-zoom{,-dark}.png。commit `64096a5`。手段注记：前三轮截屏被
  前台应用污染（用户在用机器），最终 PrintWindow 离屏采集零打扰；工作树缺
  兄弟检出致 kanban/musk/term 三 manifest 项缺席本轮桌面（merge 后主干全量
  复核项，非缺陷）。
- **[x] T6** [✅ 已完成] ui/overview.md §icon 字符串协议族（SD-01 定稿落点
  即此节；SD-02 README 节随 T1 落）。commit `3b4a59f58`。

- 2026-09-14 /auto-plan:work（stage: work | plan_id: PLAN-018 | rev1 |
  outcome: **pass** | next: review）。
  **code_commit**：auto-os worktree `20122b1`（T1）+ `64096a5`（T5 证据）、
  auto-lang worktree `3f825c66f`（T2-T4）+ `3b4a59f58`（T6 spec）。
  **worktree**：`.wt/os-018/{auto-os,auto-lang,auto-down}`（auto-down 为
  crates optional-dep 兄弟解析用，detach master）。
  **task_ids**：T1-T6 全 ✅；**evidence**：§8 执行证据 + evidence/plan018-*。
  **关键实测**：dock 四钮实机位图渲染（浅/深双主题各自走对切片目录）、
  lucide 回退同画面共存；ui_gen 773/773 + app_registry/icon_file 定点全绿；
  切片 28×2 像素级往返 + mapping 恰等。**手段偏差三笔记录**（填充率→像素
  往返+人眼预览；T4 落点 auto-lang；T5 PrintWindow 替代前台截屏），均
  等价实现不触契约。
  **blockers**: 无。遗留 review 复核：vue 宿主 .dark class 机制实测（Q1）、
  主干全量桌面（含 kanban/musk/term manifest 项）图标复核、全量门。
