use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::types::AppSettings;

static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

/// 初始化应用数据目录（%APPDATA%/MoYan，与 Electron userData 一致，无缝继承旧配置）
pub fn init(app_data: PathBuf) {
    let _ = fs::create_dir_all(&app_data);
    let _ = DATA_DIR.set(app_data);
}

pub fn data_dir() -> PathBuf {
    DATA_DIR.get().cloned().unwrap_or_else(|| {
        let fallback = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        fallback.join("MoYan")
    })
}

fn default_settings() -> AppSettings {
    let base = dirs::document_dir().unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")));
    AppSettings {
        storage_dir: base.join("MoYan").to_string_lossy().to_string(),
        expiration_days: 30,
        clipboard_hotkey: "Shift+C".to_string(),
        screenshot_hotkey: "F1".to_string(),
        launch_at_login: false,
    }
}

fn settings_file() -> PathBuf {
    data_dir().join("settings.json")
}

pub fn load_settings() -> AppSettings {
    let file = settings_file();
    if !file.exists() {
        return default_settings();
    }
    let raw = fs::read_to_string(&file).unwrap_or_default();
    match serde_json::from_str::<AppSettings>(&raw) {
        Ok(s) => s,
        Err(_) => default_settings(),
    }
}

pub fn save_settings(settings: &AppSettings) {
    let file = settings_file();
    let _ = fs::create_dir_all(file.parent().unwrap_or(Path::new(".")));
    if let Ok(json) = serde_json::to_string_pretty(settings) {
        let _ = fs::write(&file, json);
    }
}

// ---- 存储目录结构 ----

pub fn index_file(dir: &str) -> PathBuf {
    Path::new(dir).join("index.json")
}
pub fn images_dir(dir: &str) -> PathBuf {
    Path::new(dir).join("images")
}
pub fn thumbs_dir(dir: &str) -> PathBuf {
    Path::new(dir).join("thumbs")
}
pub fn screenshots_dir(dir: &str) -> PathBuf {
    Path::new(dir).join("screenshots")
}

pub fn ensure_storage(dir: &str) {
    for d in [
        PathBuf::from(dir),
        images_dir(dir),
        thumbs_dir(dir),
        screenshots_dir(dir),
    ] {
        let _ = fs::create_dir_all(d);
    }
    let idx = index_file(dir);
    if !idx.exists() {
        let _ = fs::write(&idx, "[]");
    }
}

pub fn load_index(dir: &str) -> Vec<crate::types::ClipboardItem> {
    ensure_storage(dir);
    let raw = fs::read_to_string(index_file(dir)).unwrap_or_default();
    match serde_json::from_str(&raw) {
        Ok(items) => items,
        Err(_) => Vec::new(),
    }
}

pub fn save_index(dir: &str, items: &[crate::types::ClipboardItem]) {
    ensure_storage(dir);
    let file = index_file(dir);
    if let Ok(json) = serde_json::to_string_pretty(items) {
        let _ = fs::write(file, json);
    }
}

/// 更换存储目录时迁移数据：复制 images/thumbs/screenshots 与 index.json（不覆盖已存在文件）
pub fn migrate_storage(from: &str, to: &str) {
    let src = Path::new(from).canonicalize().unwrap_or_else(|_| PathBuf::from(from));
    let dst = Path::new(to).canonicalize().unwrap_or_else(|_| PathBuf::from(to));
    ensure_storage(to);
    if src == dst {
        return;
    }
    if !src.exists() {
        return;
    }
    for name in ["images", "thumbs", "screenshots"] {
        let source = src.join(name);
        let target = dst.join(name);
        if !source.exists() {
            continue;
        }
        let _ = fs::create_dir_all(&target);
        if let Ok(entries) = fs::read_dir(&source) {
            for entry in entries.flatten() {
                let s = entry.path();
                let d = target.join(entry.file_name());
                if !d.exists() {
                    let _ = fs::copy(s, d);
                }
            }
        }
    }
    let src_idx = index_file(from);
    if src_idx.exists() {
        let dst_idx = index_file(to);
        if !dst_idx.exists() {
            let _ = fs::copy(src_idx, dst_idx);
        }
    }
}
