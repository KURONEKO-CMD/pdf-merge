# Changelog / 变更日志

遵循 Keep a Changelog 精神，版本号遵循语义化（SemVer）。

## [Unreleased]
### Fixed
- 排除输出文件以避免二次运行自吞输出。
- 输出路径父目录自动创建（`create_dir_all`）。
- 错误输出改为 `eprintln!`，并附带输出路径。
- 合并函数接受 `&Path`，避免 `to_str().unwrap()` 潜在 panic。

### Added
- 重命名包与可执行文件为 `pdf-ops`。
- 子命令：`merge`（默认）、`split`。
- 分割默认行为为 `--each`（无需显式传参）。
- 页码范围：`--pages`（合并）与 `--ranges`（分割）。
- 文件过滤：`--include <GLOB>`（包含）、`--exclude <GLOB>`（排除），相对 `--input-dir` 匹配，支持重复传参。
- 集成测试：覆盖合并/范围/过滤与分割默认行为。

## [0.1.0] - Initial
### Added
- 初始版本：递归合并目录内 PDF，按路径字典序排序，默认输出为输入目录下 `merged.pdf`。
