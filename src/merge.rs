use globset::{Glob, GlobSet, GlobSetBuilder};
use lopdf::{Dictionary, Document, Object, ObjectId};
use std::path::{Path, PathBuf};
use indicatif::{ProgressBar, ProgressStyle};
use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::spec;

pub fn run(
    input_dir: &Path,
    output: &Path,
    pages_spec: Option<&str>,
    includes: &[String],
    excludes: &[String],
    force: bool,
) -> Result<()> {
    // Resolve output directory
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("创建输出目录失败: {}", parent.display()))?;
    }

    // Build glob sets (relative to input_dir)
    let include_set = build_globset(includes).with_context(|| "包含规则无效".to_string())?;
    let exclude_set = build_globset(excludes).with_context(|| "排除规则无效".to_string())?;

    // Scan pdf files
    let mut pdf_files: Vec<_> = WalkDir::new(input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map(|ext| ext.eq_ignore_ascii_case("pdf")).unwrap_or(false))
        .filter(|e| e.path() != output)
        .filter(|e| {
            let rel = e.path().strip_prefix(input_dir).unwrap_or(e.path());
            let rel_path = rel;
            // include logic: if include_set is empty, include by default; else must match one
            let include_ok = if include_set.is_empty() { true } else { include_set.is_match(rel_path) };
            // exclude logic: if matches any exclude, drop
            let exclude_hit = if exclude_set.is_empty() { false } else { exclude_set.is_match(rel_path) };
            include_ok && !exclude_hit
        })
        .map(|e| e.path().to_owned())
        .collect();
    pdf_files.sort();

    if pdf_files.is_empty() {
        anyhow::bail!("未在目录中找到 PDF: {}", input_dir.display());
    }

    // Progress bar per file
    let pb = ProgressBar::new(pdf_files.len() as u64);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
        .unwrap()
        .progress_chars("##-"));
    pb.set_message("准备合并...");

    merge_selected_pages(&pdf_files, output, pages_spec, &pb, force)?;
    pb.finish_with_message("合并完成");
    Ok(())
}

fn merge_selected_pages(files: &[PathBuf], output: &Path, pages_spec: Option<&str>, pb: &ProgressBar, force: bool) -> Result<()> {
    // Overwrite protection handled here to ensure we fail early
    if output.exists() && !force {
        anyhow::bail!("输出文件已存在: {} (使用 --force 覆盖)", output.display());
    }
    let mut doc = Document::with_version("1.5");
    let mut page_ids: Vec<ObjectId> = Vec::new();

    for path in files {
        pb.set_message(path.file_name().and_then(|s| s.to_str()).unwrap_or("加载中..."));
        let mut pdf = Document::load(path)
            .with_context(|| format!("加载 PDF 失败: {}", path.display()))?;
        let total_pages = pdf.get_pages().len();
        let indices: Option<Vec<usize>> = if let Some(spec_str) = pages_spec {
            let ranges = spec::parse_spec(spec_str)
                .with_context(|| format!("解析页码范围失败: {}", spec_str))?;
            Some(spec::expand_to_indexes(&ranges, total_pages))
        } else { None };

        let offset = doc.max_id + 1;
        pdf.renumber_objects_with(offset);
        doc.max_id = pdf.max_id;

        let pages_map = pdf.get_pages();
        // Collect in natural order
        let mut current: Vec<ObjectId> = Vec::new();
        for (i, (_, pid)) in pages_map.into_iter().enumerate() {
            if let Some(ref idxs) = indices {
                if !idxs.contains(&i) { continue; }
            }
            current.push(pid);
        }
        page_ids.extend(current);
        doc.objects.extend(pdf.objects);
        pb.inc(1);
    }

    let pages_id = doc.new_object_id();
    for &pid in &page_ids {
        let page_obj = doc.objects.get_mut(&pid).expect("page not found");
        let page_dict = page_obj.as_dict_mut().expect("page not a dict");
        page_dict.set("Parent", Object::Reference(pages_id));
    }
    let kids: Vec<Object> = page_ids.iter().map(|&id| Object::Reference(id)).collect();
    let mut pages_dict = Dictionary::new();
    pages_dict.set("Type", "Pages");
    pages_dict.set("Kids", Object::Array(kids));
    pages_dict.set("Count", page_ids.len() as i64);
    doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

    let catalog_id = doc.new_object_id();
    let mut catalog_dict = Dictionary::new();
    catalog_dict.set("Type", "Catalog");
    catalog_dict.set("Pages", Object::Reference(pages_id));
    doc.objects.insert(catalog_id, Object::Dictionary(catalog_dict));

    doc.trailer = Dictionary::new();
    doc.trailer.set("Root", Object::Reference(catalog_id));
    doc.compress();
    doc.save(output)
        .with_context(|| format!("写入输出失败: {}", output.display()))?;
    Ok(())
}

fn build_globset(patterns: &[String]) -> anyhow::Result<GlobSet> {
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
