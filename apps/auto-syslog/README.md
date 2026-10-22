# 039-syslog（系统日志 / System Log）

桌面常驻**系统日志查看器**（PLAN-042）：宿主 syslog 环
（auto-lang `crates/auto-lang/src/ui/syslog.rs`，cap 1000 FIFO）的实时
tail 视图。普通注册表虚拟窗（非 overlay 槽）；窗开 = 宿主 ServiceTick
注入泵 ≥500ms 一拍全量快照下行，窗关 = 泵零扫描零注入。

## 三层采集（宿主侧）

1. **log crate trap**（`HostLogger`，ui_desktop boot 安装）：error/warn/info
   入环，source=`host`；
2. **诊断家族双写**（`syslog!` 宏）：renderer/back_provision/shell-pack 等
   eprintln 载重站点——环 + stderr 文件双层（归档层不退役）；
3. **App `log` 动词**（协议 v1.9，`log␟level␟text`）：任意注册表 app 上行，
   source = 发件方 registry_id（notify_source 分段归因）。

## 运行

```sh
# 桌面轨（注册表可见「系统日志」，launch 开窗）
cargo run -p auto-lang --features ui-iced --example ui_desktop -- --apps-dir <auto-os>/apps

# vue 调试轨（独立模式，mock 数据自证）
cd apps/039-syslog && auto run
```

## 消费契约

- 注入面：`__syslog_seq/__syslog_time/__syslog_level/__syslog_source/__syslog_msg`
  平行字符串列表（宿主 `write_state_vec`；time 已预格式化 HH:MM:SS）+
  注入后宿主显式 `call_handler("Rebuild")`（宿主写状态不触发 handler，
  launcher RebuildNotes 同规）。
- app 侧：单组件（store 子组件 vue TS 生成损坏——launcher 约束同款），
  行重建/级别×关键字过滤/暂停积压计数全在 `.Rebuild` 状态法 handler。

## 测试

```sh
cd tests && python desktop_mcp.py   # VM 轨 MCP 冒烟（注入→Rebuild→过滤断言）
```
