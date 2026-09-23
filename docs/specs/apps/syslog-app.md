# apps/syslog-app —— 039-syslog 查看器 app spec（PLAN-042 SD-02）

**状态**：plan-proposed（review 门随计划）

## 定位

桌面常驻**系统日志查看器**：宿主 syslog 环的实时 tail 开发者诊断窗。
普通注册表虚拟窗（**非 overlay 槽**）；front-only、无后端、无 daemon
（auto-term 先例：声明缺席合法）。

## pac 契约

- id = 目录名 `039-syslog`（launch/泵定位键）；`name: "syslog"`。
- `render: "vue"`（开发目标端）；`front_port: 17800`（17xxx 带）。
- `icon: "scroll-text"`；`title: "System Log"` / `title_zh: "系统日志"`；
  `category: "system"`；`desktop: "true"`（桌面注册表可见）。

## 消费契约

- 宿主泵写五个平行字符串列表
  `__syslog_seq / __syslog_time / __syslog_level / __syslog_source /
  __syslog_msg`（time 预格式化 HH:MM:SS）+ `hosted="1"`，随后宿主显式
  `call_handler("Rebuild")`。
- 平行字符串列表是**已证形态**（B12 家族：宿主注入 Obj 数组的 VM handler
  字段读失效；launcher `apps_*` 先例字符串列表下标读双端保真）。
- 单组件约束（launcher 文件头注同款）：无 store 子组件（vue TS 生成损坏）；
  行构建/过滤/暂停计数全在状态法 handler `.Rebuild`；计数进串状态（模板
  零 `.len()`）。

## 视图功能面（AC-07）

- 行形态 `HH:MM:SS 级别点 来源 文本`；error 红 / warn 琥珀 / info 天蓝。
- 级别三选 chip（error/warn/info）+ 关键字过滤（source/msg 预小写
  contains，handler 侧过滤）。
- 暂停：`paused` 期间注入照收列表、不重建 rows（一拍全量快照替换天然支
  持），积压条 `pending_label` + 恢复钮。
- 单条复制：行选中 → 详情条（全行文本）→ `clipboard_set_text`（native
  2926）。
- 独立模式（`hosted=="0"`，vue 调试轨）：Init mock 种子 + ＋测试行按钮
  自证；宿主泵首拍写 `hosted="1"` 后 mock/脚注退场。
- 展示窗尾 400 行（环 1000 全量渲染重建过重；头部 count 注明截断）。

## 端口带

17800（front only；无 back 端口）。apps.manifest 行
`{"id": "039-syslog", "repo": "apps/039-syslog", "kind": "local",
"ports": [17800], "status": "active"}`（无 daemon 对象）。

## 验收映射

AC-05（注册表可见 + launch 开普通虚拟窗非 overlay）/ AC-07（视图功能
实机截图）/ AC-08（music-scan 事件随 log 动词经本窗可见）。
