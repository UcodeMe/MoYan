use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use tauri::{AppHandle, Manager};

use crate::types::{AppSettings, CompleteScreenshotOptions, PastePayload, SettingsPatch};
use crate::{clipboard, paste, screenshot, store};

/// 单独声明：main 窗口 keep-open 标记由 main.rs 维护
#[tauri::command]
pub fn settings_get() -> AppSettings {
    clipboard::current_settings()
}

#[tauri::command]
pub fn settings_update(app: AppHandle, patch: SettingsPatch) -> Result<AppSettings, String> {
    let prev = clipboard::current_settings();
    let storage_changed = patch
        .storage_dir
        .as_deref()
        .map(|d| d != prev.storage_dir)
        .unwrap_or(false);
    let next = AppSettings {
        storage_dir: patch.storage_dir.unwrap_or_else(|| prev.storage_dir.clone()),
        expiration_days: patch.expiration_days.unwrap_or(prev.expiration_days),
        clipboard_hotkey: patch.clipboard_hotkey.unwrap_or_else(|| prev.clipboard_hotkey.clone()),
        screenshot_hotkey: patch.screenshot_hotkey.unwrap_or_else(|| prev.screenshot_hotkey.clone()),
        launch_at_login: patch.launch_at_login.unwrap_or(prev.launch_at_login),
    };
    if storage_changed {
        store::migrate_storage(&prev.storage_dir, &next.storage_dir);
    }
    store::save_settings(&next);
    clipboard::update_settings(next.clone());
    crate::register_shortcuts(&app, &next);
    // 开机自启动
    use tauri_plugin_autostart::ManagerExt;
    if next.launch_at_login {
        let _ = app.autolaunch().enable();
    } else {
        let _ = app.autolaunch().disable();
    }
    Ok(next)
}

#[tauri::command]
pub async fn dialog_directory(app: AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let handle = app.clone();
    let picked = tauri::async_runtime::spawn_blocking(move || {
        handle.dialog().file().blocking_pick_folder()
    })
    .await
    .map_err(|e| e.to_string())?;
    Ok(picked
        .and_then(|p| p.into_path().ok())
        .map(|pb| pb.to_string_lossy().to_string()))
}

#[tauri::command]
pub fn clipboard_items() -> Vec<crate::types::ClipboardItem> {
    clipboard::get_items()
}

#[tauri::command]
pub fn clipboard_image(id: String, thumb: Option<bool>) -> Option<String> {
    clipboard::get_image_data_url(&id, thumb.unwrap_or(true))
}

#[tauri::command]
pub fn clipboard_copy_text(text: String) {
    clipboard::copy_text_to_clipboard(&text);
}

#[tauri::command]
pub fn clipboard_copy_image(data_url: String) {
    clipboard::copy_image_to_clipboard(&data_url);
}

#[tauri::command]
pub async fn clipboard_paste_to_focus(app: AppHandle, payload: PastePayload) -> Result<bool, String> {
    match payload.item_type.as_str() {
        "text" => {
            if let Some(text) = payload.text {
                clipboard::copy_text_to_clipboard(&text);
            }
        }
        "image" => {
            if let Some(data_url) = payload.data_url {
                clipboard::copy_image_to_clipboard(&data_url);
            }
        }
        _ => {}
    }
    // 隐藏主窗口后还原原前台窗口并模拟 Ctrl+V
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.hide();
    }
    tauri::async_runtime::spawn_blocking(|| {
        thread::sleep(Duration::from_millis(180));
        paste::paste_to_recorded_window()
    })
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn clipboard_delete_item(id: String) {
    clipboard::delete_item(&id);
}

#[tauri::command]
pub fn clipboard_clear() {
    clipboard::clear_items();
}

#[tauri::command]
pub fn clipboard_pin(id: String) {
    clipboard::toggle_pin(&id);
}

#[tauri::command]
pub fn window_hide(app: AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.hide();
    }
}

#[tauri::command]
pub fn window_keep_open(keep: bool) {
    crate::KEEP_OPEN.store(keep, std::sync::atomic::Ordering::Relaxed);
}

#[tauri::command]
pub fn navigate(app: AppHandle, route: String) {
    use tauri::Emitter;
    let _ = app.emit_to("main", "navigate", route);
}

#[tauri::command]
pub fn screenshot_trigger(app: AppHandle) -> Result<(), String> {
    screenshot::capture_screen(&app)
}

#[tauri::command]
pub fn screenshot_init() -> Option<crate::types::ScreenshotInit> {
    screenshot::take_pending()
}

#[tauri::command]
pub fn screenshot_close(app: AppHandle) {
    screenshot::close_editor(&app);
}

#[tauri::command]
pub async fn screenshot_complete(app: AppHandle, options: CompleteScreenshotOptions) -> Result<String, String> {
    let settings = clipboard::current_settings();
    let dir = store::screenshots_dir(&settings.storage_dir);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let file = dir.join(format!("screenshot-{}.png", iso_stamp()));
    let bytes = clipboard::decode_data_url(&options.data_url).ok_or_else(|| "截图数据解码失败".to_string())?;
    std::fs::write(&file, &bytes).map_err(|e| e.to_string())?;

    if options.save_to_file {
        use tauri_plugin_dialog::DialogExt;
        let handle = app.clone();
        let default_name = file.file_name().unwrap_or_default().to_string_lossy().to_string();
        let saved = tauri::async_runtime::spawn_blocking(move || {
            handle
                .dialog()
                .file()
                .set_file_name(default_name)
                .add_filter("PNG 图片", &["png"])
                .blocking_save_file()
        })
        .await
        .map_err(|e| e.to_string())?;
        if let Some(picked) = saved {
            if let Ok(path) = picked.into_path() {
                let _ = std::fs::copy(&file, &path);
            }
        }
    }

    clipboard::copy_image_to_clipboard(&options.data_url);
    screenshot::close_editor(&app);
    Ok(file.to_string_lossy().to_string())
}

#[tauri::command]
pub fn app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ---- 工具：ISO 风格时间戳（与 Electron new Date().toISOString() 行为对齐，:. 替换为 -） ----

fn iso_stamp() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = now.as_secs();
    let millis = now.subsec_millis();
    let days = (secs / 86400) as i64;
    let sod = secs % 86400;
    let (y, mo, d) = civil_from_days(days);
    format!(
        "{:04}-{:02}-{:02}T{:02}-{:02}-{:02}-{:03}Z",
        y,
        mo,
        d,
        sod / 3600,
        (sod % 3600) / 60,
        sod % 60,
        millis
    )
}

/// Howard Hinnant 的 civil_from_days 算法
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}
