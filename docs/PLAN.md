# Plan / 下一步计划

已完成（当前迭代）
- 顶部菜单：Files / Mode / Options / Help；Help 弹窗（英文）
- Options：Depth(1/2/3/∞)、Split range、Overwrite(Force/Suffix)、Output auto‑follow
- Split：>20 文件确认弹窗；Range 自动生成；输出冲突追加后缀
- 输入框：可视光标 + 自动换行；路径规范化（引号/`\ `/`~`）
- 扫描：空闲超时自动取消，释放资源
- 文档：README 与 docs/README 更新；TUI_DESIGN/PROJECT_STRUCTURE 更新

- 过滤配置：在 TUI 中编辑 include/exclude globs（弹窗），实时预览
- 配置持久化：`~/.config/pdf-ops/config.toml`（主题、深度、键位、确认阈值等）
- Windows 细节：路径/编码、OneDrive/权限更友好

- 扫描性能优化：基于 ignore::WalkBuilder 支持 `.gitignore`；大目录渐进式渲染
- 更多主题与可配置项；状态/错误弹窗规范化
- 测试补强：include/exclude/深度/隐藏文件、TUI 状态测试
