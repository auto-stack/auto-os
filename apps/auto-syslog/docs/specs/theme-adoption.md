# 主题采纳合同（theme-adoption）

> 来源：auto-os/PLAN-057（2026-11-04 work 候选 T-03）。宿主主合同：
> auto-os `docs/specs/shell/host-theme-policy-and-delivery.md`；
> 本批登记页：`docs/specs/apps/product-theme-batch-057.md`。
> 本应用为 OS local（`apps/auto-syslog`，非独立仓、非 submodule），
> 本页与代码同在 OS 树交付。

## 消费形态

- **固定快照**：`vendor/themes` = UI 管理渠道发布的官方主题包快照
  （`themes` 0.2.0，内容 digest
  `9fa07742dc68aa5377ae06cb2d966386002063f184c515b6c5dddf8a6bef838e`，
  per-file sha256 与源包校验同律）。快照不可二次编辑、非第二权威源；
  升级 = UI 发新快照替换 + OS 树内重验。
- **pac 声明**：`dep themes { path: "vendor/themes" }`；`theme` 键保持
  缺省 = **follow 宿主**——本应用即 OS 桌面常驻诊断窗（PLAN-042），
  宿主跟随为天然主语义；pac 原无显式 theme 键，无退役项。
- **源内导入**：`use themes.themes: Auto`（C-11 收集链注册，031 合同）。
- **design 默认**：静态 light（零遗留——原无 dark_mode 位/明暗切换），
  独立 `auto run` 调试轨保现默认语义。

## 豁免与不变量

- 级别警示色（amber/红系固定类）为日志语义着色豁免，不随主题映射。
- 暂停/筛选/尾随/宿主注入（`__syslog_*` 平行列表接缝）/选中复制等
  业务面不受换肤影响；注入→Rebuild→过滤断言原样
  （`tests/desktop_mcp.py`）。

## 验证与测试设施（os-057 冻结工具链：auto.exe 8d1944dd5f66bc6d + auto-ui-tool da5ea1569500c232）

- `tests/desktop_mcp.py` harness 适配（PLAN-057 T-03，检查不变量未动）：
  `pick_free_port` 外置钉定优先——本机 9247..94xx 大段系统保留区间
  （bind 10013），`AUTOUI_MCP_PORT` 显式给钉直接采用（缺省行为不变）。
- 运行 env：`AUTO_LANG_ROOT`/`AUTO_WEB_ROOT`=主检出、`AUTO_BIN`=os-057
  组树 auto.exe、`AUTO_UI_TOOL_BIN`=组树工具。
