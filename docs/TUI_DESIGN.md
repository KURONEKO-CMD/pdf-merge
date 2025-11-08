## TUI Architecture & Plan (ratatui, gitui-style)

### Goals & Scope
- 提供键盘优先的 TUI（gitui 风格）用于合并/分割 PDF。
- 不改 CLI 行为；TUI 为可选子命令：`pdf-ops tui`。
- 后端继续使用 `lopdf`，当前阶段不引入 tokio。

### UX & Layout
- 顶部 Tabs + 信息：Tabs 为 `Files | Dirs | Mode`；信息行展示 `Input/Depth/Selected/Output/Pages/Mode`，扫描中显示 `Scanning...`。
- 左栏（Files）：PDF 列表；Space 选择；j/k/↑/↓ 导航；u/d/U/D 调整右栏顺序；p 编辑页码范围；o 编辑输出；F 覆盖；Enter 运行。
- 右栏（Selection/Order）：展示/调整合并顺序。
- 目录栏（Dirs，后续）：显示子目录；h/Backspace/← 返回上层；l/Enter/→ 进入；切换目录后重扫 Files。
- 底部 Footer：状态行 + 帮助行（`Quit:q  Focus:Tab  Select:Space  Move:↑/↓/j/k  Reorder:u/d/U/D  Rescan:r  Depth:[ ] \  Output:o  Pages:p  Force:F  Run:Enter`）。
- Mode 子菜单：在 `Mode` Tab 上 Enter 弹出（Merge/Split），↑/↓/j/k 选择，Enter 确认，Esc 取消。

### Architecture
- Feature：`tui`（可选编译）。依赖：`ratatui`、`crossterm`（仅在 feature 启用时）。
- 模块（新式）：
  - `src/tui/app.rs`：`AppState`（模式、焦点、过滤、选择、任务、日志）
  - `src/tui/ui.rs`：绘制（blocks、lists、tabs、popups）
  - `src/tui/events.rs`：输入与 tick 循环（crossterm + std::time）
  - `src/tui/components/`：List、Prompt、Help、Confirm 等组件
  - `src/tui/jobs/`：后台 `Job`（MergeJob、SplitJob）+ `JobProgress`
  - `src/progress.rs`：`ProgressSink` 抽象；CLI 用 indicatif 适配，TUI 用 channel 适配

#### ProgressSink（抽象进度）
```rust
pub trait ProgressSink: Send + Sync {
    fn set_len(&self, len: u64);
    fn inc(&self, n: u64);
    fn set_message(&self, msg: impl Into<String>);
    fn finish(&self, msg: impl Into<String>);
}
```
- 合并/分割接受 `impl ProgressSink` 报告进度；CLI 继续显示 indicatif；TUI 通过 channel 刷新 Gauge。

#### 并发模型
- 不用 tokio。使用 `std::thread::spawn` + `mpsc`（或 crossbeam-channel）。
- UI 线程渲染；后台 Job 发送 `JobProgress { pos, len, msg }` 与最终 `Result`。

#### 扫描与过滤
- 复用 `walkdir` + `globset`；已抽出 `ScanConfig`。
- TUI 默认深度=1，可用 `[`/`]`/`\` 调整并重扫；采用流式 `scan_stream()` + 取消句柄以避免阻塞与线程堆积。
- 目录浏览：在 `scan.rs` 增加 `list_dirs(input_dir, follow_links, show_hidden)`；导航更新 `input_dir` 并重扫。

### Theming（gitui 风格）
- `tui/theme.rs` 提供 Theme 结构与风格方法（借鉴 gitui）：`block(focus)/title(focus)/tab(selected)/text(enabled,selected)` 等。
- 外部主题（TOML）：
```toml
[theme]
name = "gitui-dark"
[colors]
bg="#0c0c0c"; fg="#c9d1d9"; accent="#58a6ff"; border="#30363d"
selected_bg="#1f6feb"; selected_fg="#ffffff"
warn="#d29922"; error="#f85149"; ok="#2ea043"
```
- CLI：`pdf-ops tui --theme gitui-dark` 或 `--theme-file path.toml`。
- 许可：若复用 gitui 调色/结构，请在第三方声明中保留 MIT 许可与来源链接。

### CLI 集成
- 新子命令：`tui`（受 `tui` feature 控制）。支持 `--theme`、`--theme-file`。
- 共享现有过滤（`--include/--exclude`）、语义（`--force`、页码范围、模板）。

### Plan / Milestones
1) Tabs 与 Mode 子菜单；主题接线到边框/高亮/标题/状态/帮助。
2) Dirs 面板与目录导航；`list_dirs()` 与重扫集成。
3) Split 模式接入；模板编辑弹窗；帮助弹窗（?）。
4) 键位配置与持久化（可选）。

### 非目标（初期）
- 不做 PDF 渲染预览；不引入 tokio；不支持远程来源。
- 不实现书签/大纲编辑；聚焦合并/分割编排与可视化。

### 风险
- 大规模合并的内存占用；必要时文档限制并探索增量/流式策略。
- 复杂 PDF 兼容性取决于 `lopdf`；出现异常时明确报错并给出指引。
