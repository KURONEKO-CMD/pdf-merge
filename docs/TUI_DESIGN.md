## TUI Design (ratatui)

### Layout
- Top: `Menu` (Files / Mode / Options / Help) and `Info` (Input / Depth / Selected / Output / Pages / Mode)
- Main: left `Files`, right `Selection / Order`
- Bottom: status line + 2 help lines (include `Cancel: Esc`, `Quit: q`)

### Keyboard
- Toggle top/menu focus: `g`
- Navigate top: `Tab` / `← →`; inside lists: `↑/↓/j/k`
- Select/Run: `Space` / `Enter`; Cancel: `Esc`; Quit: `q`; Rescan: `r`
- Reorder selection: `u/d/U/D`

### Files Menu
- `Input Path` / `Output Path` editors (multiline, visible caret)
- Paths support spaces, quotes and `~` expansion; relative `Output` resolves under `Input`

### Mode
- `Merge` or `Split`

### Options
- `Depth`: 1 / 2 / 3 / ∞
- `Split range`: pages per output file (default 1)
- `Overwrite`: `Force` (overwrite) or `Suffix` (default; append `_1/_2/...`)
- `Output auto‑follow`: when `Input` changes, reset Output to `merged.pdf` under `Input`

### Split
- Preflight checks page count; if estimated outputs > 20, show centered confirmation dialog (y/N)
- Ranges computed from total pages and `Split range` (e.g., 10 & 3 → `1-3,4-6,7-9,10-10`)

### Scan & Cancel
- `scan_stream` supports `max_depth=None` (∞) and cancel handle; long idle timeout cancels scanning to free resources

### Theme
- High-contrast dark theme; bold typography for readability
