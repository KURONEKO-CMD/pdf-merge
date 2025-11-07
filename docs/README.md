## pdf-ops

A simple Rust CLI that merges/splits PDFs; recursively collects PDFs in a folder and merges them, or splits a single PDF. Supports page-range selection and file filtering via globs.

## Usage

- Merge current directory:
  - `pdf-ops`
- Specify input directory:
  - `pdf-ops merge -i ./docs`
- Specify output file:
  - `pdf-ops merge -i ./docs -o output.pdf`
- Merge with pages (applies to each input):
  - `pdf-ops merge -i ./in --pages "1-3,5,10-"`
- Split defaults to one page per file:
  - `pdf-ops split -i ./input.pdf -d ./out`
- Split by ranges:
  - `pdf-ops split -i ./input.pdf -d ./out --ranges "1-3,4-6,7-"`

## Behavior & Filtering
- `--output` default `merged.pdf` writes to `<input-dir>/merged.pdf`.
- Recurses into subdirectories; `.pdf` files (case-insensitive) are considered.
- Sorting is lexicographic by path; use prefixes like `001-...` to control order.
- Filtering with globs (relative to `--input-dir`):
  - `--include <GLOB>`: include only matching files; repeatable; empty means include all.
  - `--exclude <GLOB>`: exclude matching files; repeatable.
  - Example: `--include "**/*.pdf" --exclude "backup/**" --exclude "**/*draft*.pdf"`

## Development
- Format: `cargo fmt --all`; Lint: `cargo clippy --all-targets --all-features -D warnings`
- Run: `cargo run -- merge -i ./samples -o merged.pdf`
- Test: `cargo test`
