use std::io::Cursor;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64::Engine;
use sha2::{Digest, Sha256};
use tauri::{AppHandle, Emitter};

use crate::types::{AppSettings, ClipboardItem};
use crate::store;

const MAX_ITEMS: usize = 1000;
const POLL_INTERVAL_MS: u64 = 700;
const SUPPRESS_WINDOW_MS: u64 = 1800;

struct ClipState {
    settings: AppSettings,
    items: Vec<ClipboardItem>,
    last_text_hash: String,
    last_image_hash: String,
    suppress_until: u64,
}

static STATE: OnceLock<Mutex<ClipState>> = OnceLock::new();
static CLIP: OnceLock<Mutex<arboard::Clipboard>> = OnceLock::new();

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let out = hasher.finalize();
    let mut hex = String::with_capacity(64);
    for byte in out.iter() {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}

fn state() -> &'static Mutex<ClipState> {
    STATE.get_or_init(|| {
        Mutex::new(ClipState {
            settings: store::load_settings(),
            items: Vec::new(),
            last_text_hash: String::new(),
            last_image_hash: String::new(),
            suppress_until: 0,
        })
    })
}

fn clipboard_lock() -> &'static Mutex<arboard::Clipboard> {
    CLIP.get_or_init(|| Mutex::new(arboard::Clipboard::new().expect("初始化系统剪贴板失败")))
}

/// RGBA → PNG 字节
fn rgba_to_png(width: u32, height: u32, rgba: &[u8]) -> Option<Vec<u8>> {
    let img = image::RgbaImage::from_raw(width, height, rgba.to_vec())?;
    let mut buf = Vec::new();
    image::DynamicImage::ImageRgba8(img)
        .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
        .ok()?;
    Some(buf)
}

/// data:image/png;base64,... → (PNG bytes)
pub(crate) fn decode_data_url(data_url: &str) -> Option<Vec<u8>> {
    let b64 = data_url.split_once(',')?.1;
    base64::engine::general_purpose::STANDARD.decode(b64).ok()
}

/// data URL → (RGBA, w, h)（用于写入系统剪贴板）
fn decode_data_url_rgba(data_url: &str) -> Option<(Vec<u8>, u32, u32)> {
    let bytes = decode_data_url(data_url)?;
    let img = image::load_from_memory(&bytes).ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    Some((rgba.into_raw(), w, h))
}

// ---- 剪贴板读写（带锁，避免轮询与写入竞争） ----

fn read_text_safe() -> String {
    let mut clip = match clipboard_lock().lock() {
        Ok(c) => c,
        Err(_) => return String::new(),
    };
    clip.get_text().unwrap_or_default()
}

fn read_image_safe() -> Option<(arboard::ImageData<'static>, Vec<u8>)> {
    let mut clip = clipboard_lock().lock().ok()?;
    let data = clip.get_image().ok()?;
    let png = rgba_to_png(data.width as u32, data.height as u32, &data.bytes)?;
    Some((data, png))
}

fn write_text_to_clipboard(text: &str) {
    if let Ok(mut clip) = clipboard_lock().lock() {
        let _ = clip.set_text(text.to_string());
    }
}

fn write_rgba_to_clipboard(rgba: &[u8], w: u32, h: u32) {
    let data = arboard::ImageData {
        width: w as usize,
        height: h as usize,
        bytes: std::borrow::Cow::Borrowed(rgba),
    };
    if let Ok(mut clip) = clipboard_lock().lock() {
        let _ = clip.set_image(data);
    }
}

// ---- 条目管理 ----

fn notify(app: &AppHandle) {
    let _ = app.emit("clipboard:updated", ());
}

fn persist_and_notify(app: &AppHandle) {
    let (dir, items) = with_state(|s| (s.settings.storage_dir.clone(), s.items.clone()));
    store::save_index(&dir, &items);
    notify(app);
}

fn with_state<T>(f: impl FnOnce(&mut ClipState) -> T) -> T {
    let mut s = state().lock().unwrap();
    f(&mut s)
}

fn remove_files(storage_dir: &str, item: &ClipboardItem) {
    for dir in [store::images_dir(storage_dir), store::thumbs_dir(storage_dir)] {
        let file = dir.join(format!("{}.png", item.id));
        if file.exists() {
            let _ = std::fs::remove_file(file);
        }
    }
}

fn trim_items(storage_dir: &str) {
    // 超过 MAX_ITEMS 时移除最新的非固定条目（对齐原 Electron removable.shift() 语义）
    loop {
        let over = with_state(|s| s.items.len() > MAX_ITEMS);
        if !over {
            return;
        }
        let victim = {
            let mut s = state().lock().unwrap();
            let victim = s.items.iter().find(|i| !i.pinned).cloned();
            match victim {
                Some(v) => {
                    s.items.retain(|i| i.id != v.id);
                    v
                }
                None => return,
            }
        };
        remove_files(storage_dir, &victim);
    }
}

fn add_text_item(app: &AppHandle, content: String) {
    let created = now_ms();
    let hash = sha256_hex(content.as_bytes());
    let item = ClipboardItem {
        id: uuid::Uuid::new_v4().to_string(),
        item_type: "text".to_string(),
        size: content.len() as u64,
        content,
        hash,
        pinned: false,
        created_at: created,
        width: None,
        height: None,
    };
    with_state(|s| s.items.insert(0, item));
    let dir = with_state(|s| s.settings.storage_dir.clone());
    trim_items(&dir);
    persist_and_notify(app);
}

fn add_image_item(app: &AppHandle, data: arboard::ImageData, png: Vec<u8>) {
    let id = uuid::Uuid::new_v4().to_string();
    let (w, h) = (data.width, data.height);
    let storage_dir = with_state(|s| s.settings.storage_dir.clone());
    let images = store::images_dir(&storage_dir).join(format!("{}.png", id));
    if std::fs::write(&images, &png).is_err() {
        return;
    }
    // 缩略图：宽度固定 280（与 Electron nativeImage.resize({width:280}) 一致）
    let thumb = {
        let orig = match image::RgbaImage::from_raw(w as u32, h as u32, data.bytes.to_vec()) {
            Some(img) => img,
            None => return,
        };
        let thumb_w = 280u32;
        let thumb_h = ((h as u64 * thumb_w as u64) / w.max(1) as u64).max(1) as u32;
        let resized = image::imageops::resize(&orig, thumb_w, thumb_h, image::imageops::FilterType::Triangle);
        let mut buf = Vec::new();
        match image::DynamicImage::ImageRgba8(resized).write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png) {
            Ok(_) => buf,
            Err(_) => png.clone(), // 失败时回退原图
        }
    };
    let _ = std::fs::write(store::thumbs_dir(&storage_dir).join(format!("{}.png", id)), &thumb);
    let item = ClipboardItem {
        id: id.clone(),
        item_type: "image".to_string(),
        content: String::new(),
        hash: sha256_hex(&png),
        pinned: false,
        created_at: now_ms(),
        size: png.len() as u64,
        width: Some(w as u32),
        height: Some(h as u32),
    };
    with_state(|s| s.items.insert(0, item));
    trim_items(&storage_dir);
    persist_and_notify(app);
}

fn sync_hashes() {
    let text = read_text_safe().trim().to_string();
    let text_hash = if text.is_empty() { String::new() } else { sha256_hex(text.as_bytes()) };
    let image_hash = match read_image_safe() {
        Some((_, png)) => sha256_hex(&png),
        None => String::new(),
    };
    with_state(|s| {
        s.last_text_hash = text_hash;
        s.last_image_hash = image_hash;
    });
}

fn poll_once(app: &AppHandle) {
    let suppressed = with_state(|s| now_ms() < s.suppress_until);
    if suppressed {
        sync_hashes();
        return;
    }
    let text = read_text_safe().trim().to_string();
    if text.is_empty() {
        with_state(|s| s.last_text_hash = String::new());
    } else {
        let h = sha256_hex(text.as_bytes());
        let changed = with_state(|s| {
            if s.last_text_hash != h {
                s.last_text_hash = h.clone();
                true
            } else {
                false
            }
        });
        if changed {
            add_text_item(app, text);
        }
    }
    match read_image_safe() {
        Some((data, png)) => {
            let h = sha256_hex(&png);
            let changed = with_state(|s| {
                if s.last_image_hash != h {
                    s.last_image_hash = h.clone();
                    true
                } else {
                    false
                }
            });
            if changed {
                add_image_item(app, data, png);
            }
        }
        None => {
            with_state(|s| s.last_image_hash = String::new());
        }
    }
}

/// 启动剪贴板监控（轮询线程，700ms）
pub fn init(app: &AppHandle, settings: AppSettings) {
    store::ensure_storage(&settings.storage_dir);
    let items = store::load_index(&settings.storage_dir);
    with_state(|s| {
        s.settings = settings;
        s.items = items;
    });
    sync_hashes();
    cleanup_expired();
    let handle = app.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));
        poll_once(&handle);
    });
}

/// 设置更新后重新加载索引（目录可能已迁移）
pub fn update_settings(next: AppSettings) {
    store::ensure_storage(&next.storage_dir);
    let items = store::load_index(&next.storage_dir);
    with_state(|s| {
        s.settings = next;
        s.items = items;
    });
    cleanup_expired();
    sync_hashes();
}

pub fn current_settings() -> AppSettings {
    with_state(|s| s.settings.clone())
}

/// 写入剪贴板后抑制采集窗口（防止自记录）
pub fn suppress_capture() {
    let until = now_ms() + SUPPRESS_WINDOW_MS;
    with_state(|s| s.suppress_until = until);
    sync_hashes();
}

/// 过期清理（非固定条目）
pub fn cleanup_expired() {
    let (dir, removed) = {
        let mut s = state().lock().unwrap();
        let max_age = if s.settings.expiration_days > 0 {
            s.settings.expiration_days as u64 * 24 * 60 * 60 * 1000
        } else {
            u64::MAX
        };
        let now = now_ms();
        let removed: Vec<ClipboardItem> = s
            .items
            .iter()
            .filter(|i| !i.pinned && now.saturating_sub(i.created_at) > max_age)
            .cloned()
            .collect();
        if removed.is_empty() {
            (s.settings.storage_dir.clone(), removed)
        } else {
            s.items.retain(|i| !removed.iter().any(|r| r.id == i.id));
            (s.settings.storage_dir.clone(), removed)
        }
    };
    if !removed.is_empty() {
        for item in &removed {
            remove_files(&dir, item);
        }
        let items = with_state(|s| s.items.clone());
        store::save_index(&dir, &items);
    }
}

/// 固定优先 + 创建时间倒序
pub fn get_items() -> Vec<ClipboardItem> {
    let mut items = with_state(|s| s.items.clone());
    items.sort_by(|a, b| {
        if a.pinned != b.pinned {
            return if a.pinned { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater };
        }
        b.created_at.cmp(&a.created_at)
    });
    items
}

pub fn get_image_data_url(id: &str, thumb: bool) -> Option<String> {
    let (storage_dir, is_image) = with_state(|s| (s.settings.storage_dir.clone(), s.items.iter().any(|i| i.id == id && i.item_type == "image")));
    if !is_image {
        return None;
    }
    let dir = if thumb { store::thumbs_dir(&storage_dir) } else { store::images_dir(&storage_dir) };
    let file = dir.join(format!("{}.png", id));
    let bytes = std::fs::read(file).ok()?;
    let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
    Some(format!("data:image/png;base64,{}", b64))
}

/// 复制文本到系统剪贴板（并抑制自采集）
pub fn copy_text_to_clipboard(text: &str) {
    write_text_to_clipboard(text);
    suppress_capture();
}

/// 复制图片（data URL）到系统剪贴板（并抑制自采集）
pub fn copy_image_to_clipboard(data_url: &str) {
    if let Some((rgba, w, h)) = decode_data_url_rgba(data_url) {
        write_rgba_to_clipboard(&rgba, w, h);
    }
    suppress_capture();
}

pub fn delete_item(id: &str) {
    let (dir, removed) = {
        let mut s = state().lock().unwrap();
        let target = s.items.iter().position(|i| i.id == id);
        match target {
            Some(idx) => {
                let item = s.items.remove(idx);
                (s.settings.storage_dir.clone(), Some(item))
            }
            None => (s.settings.storage_dir.clone(), None),
        }
    };
    if let Some(item) = removed {
        remove_files(&dir, &item);
        let items = with_state(|s| s.items.clone());
        store::save_index(&dir, &items);
    }
}

pub fn clear_items() {
    let (dir, removed) = {
        let mut s = state().lock().unwrap();
        let removed: Vec<ClipboardItem> = s.items.iter().filter(|i| !i.pinned).cloned().collect();
        s.items.retain(|i| i.pinned);
        (s.settings.storage_dir.clone(), removed)
    };
    for item in &removed {
        remove_files(&dir, item);
    }
    let items = with_state(|s| s.items.clone());
    store::save_index(&dir, &items);
}

pub fn toggle_pin(id: &str) {
    let dir = with_state(|s| s.settings.storage_dir.clone());
    let changed = with_state(|s| {
        if let Some(item) = s.items.iter_mut().find(|i| i.id == id) {
            item.pinned = !item.pinned;
            true
        } else {
            false
        }
    });
    if changed {
        let items = with_state(|s| s.items.clone());
        store::save_index(&dir, &items);
    }
}
