use anyhow::{Context, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn collect_pdfs(
    input_dir: &Path,
    includes: &[String],
    excludes: &[String],
    extra_exclude_paths: &[PathBuf],
) -> Result<Vec<PathBuf>> {
    let include_set = build_globset(includes).with_context(|| "包含规则无效".to_string())?;
    let exclude_set = build_globset(excludes).with_context(|| "排除规则无效".to_string())?;

    let mut out: Vec<PathBuf> = WalkDir::new(input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map(|ext| ext.eq_ignore_ascii_case("pdf")).unwrap_or(false))
        .filter(|e| !extra_exclude_paths.iter().any(|p| e.path() == p))
        .filter(|e| {
            let rel = e.path().strip_prefix(input_dir).unwrap_or(e.path());
            let include_ok = if include_set.is_empty() { true } else { include_set.is_match(rel) };
            let exclude_hit = if exclude_set.is_empty() { false } else { exclude_set.is_match(rel) };
            include_ok && !exclude_hit
        })
        .map(|e| e.path().to_owned())
        .collect();

    out.sort();
    Ok(out)
}

fn build_globset(patterns: &[String]) -> Result<GlobSet> {
    if patterns.is_empty() {
        return Ok(GlobSetBuilder::new().build()?);
    }
    let mut builder = GlobSetBuilder::new();
    for pat in patterns {
        let g = Glob::new(pat).with_context(|| format!("无效的 GLOB: {}", pat))?;
        builder.add(g);
    }
    Ok(builder.build()?)
}

