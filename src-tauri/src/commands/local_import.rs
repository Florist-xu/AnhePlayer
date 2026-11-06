// 本地音乐：把用户选中的文件拷贝进 App 沙盒的 LocalMusic 目录。
//
// iOS 无法扫描任意目录，只能通过文件选择器逐个导入。拷贝后该目录即可作为普通「扫描文件夹」
// 交给既有的扫描 / 元数据解析流程处理（见前端 localMusicStore.importFiles）。

use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
    /// 导入目录（需要加入扫描文件夹）
    pub dir: String,
    pub imported: usize,
    pub failed: usize,
}

/// 文件选择器在 iOS 上可能返回 `file://` URL（带百分号编码），统一还原为本地路径。
fn to_local_path(raw: &str) -> PathBuf {
    let trimmed = raw.trim();
    let Some(rest) = trimmed.strip_prefix("file://") else {
        return PathBuf::from(trimmed);
    };
    // file:///private/var/... → /private/var/...；忽略可能的 host 部分
    let path_part = match rest.find('/') {
        Some(idx) => &rest[idx..],
        None => rest,
    };
    PathBuf::from(percent_decode(path_part))
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok();
            if let Some(v) = hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// 目标目录下不重名的路径：已存在同名文件时追加 " (n)"。
fn unique_dest(dir: &Path, file_name: &str) -> PathBuf {
    let candidate = dir.join(file_name);
    if !candidate.exists() {
        return candidate;
    }
    let path = Path::new(file_name);
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "track".to_string());
    let ext = path
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    let mut n = 1;
    loop {
        let next = dir.join(format!("{stem} ({n}){ext}"));
        if !next.exists() {
            return next;
        }
        n += 1;
    }
}

#[tauri::command]
pub fn local_import_files(files: Vec<String>) -> Result<ImportResult, String> {
    let dir = super::paths::user_data_dir().join("LocalMusic");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let mut imported = 0usize;
    let mut failed = 0usize;
    for raw in files {
        let src = to_local_path(&raw);
        let Some(name) = src.file_name().map(|n| n.to_string_lossy().into_owned()) else {
            failed += 1;
            continue;
        };
        // 已在导入目录内（重复选择同一文件）视为成功，不再拷贝
        if src.parent() == Some(dir.as_path()) {
            imported += 1;
            continue;
        }
        let dest = unique_dest(&dir, &name);
        match fs::copy(&src, &dest) {
            Ok(_) => imported += 1,
            Err(err) => {
                log::warn!("[local-import] copy {:?} failed: {err}", src);
                failed += 1;
            }
        }
    }

    Ok(ImportResult {
        dir: dir.to_string_lossy().into_owned(),
        imported,
        failed,
    })
}
