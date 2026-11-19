// MoYan - Tauri 主入口（替代 Electron main.ts）
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // 无控制台窗口

mod clipboard;
mod commands;
mod paste;
mod screenshot;
mod store;
mod types;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use tauri::{
    AppHandle, Emitter, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, WindowEvent,
};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::types::AppSettings;

static KEEP_OPEN: AtomicBool = AtomicBool::new(false);
static CLIP_HK: Mutex<Option<Shortcut>> = Mutex::new(None);
static SHOT_HK: Mutex<Option<Shortcut>> = Mutex::new(None);

// ---- 快捷键 ----

/// Electron accelerator → Tauri/global-hotkey 格式
fn normalize_accelerator(accel: &str) -> String {
    let is_mac = cfg!(target_os = "macos");
    accel
        .split('+')
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .map(|p| match p {
            "CommandOrControl" | "Control" => {
                if is_mac {
                    "cmd".to_string()
                } else {
                    "ctrl".to_string()
                }
            }
            "Command" | "Meta" | "Super" => "cmd".to_string(),
            "Alt" | "Option" => "alt".to_string(),
            "Shift" => "shift".to_string(),
            other => other.to_lowercase(),
        })
        .collect::<Vec<_>>()
        .join("+")
}

pub fn register_shortcuts(app: &AppHandle, settings: &AppSettings) {
    let gs = app.global_shortcut();
    let _ = gs.unregister_all();
    *CLIP_HK.lock().unwrap() = None;
    *SHOT_HK.lock().unwrap() = None;
    let clip = normalize_accelerator(&settings.clipboard_hotkey);
    let shot = normalize_accelerator(&settings.screenshot_hotkey);
    match clip.parse::<Shortcut>() {
        Ok(hk) => {
            if gs.register(hk.clone()).is_ok() {
                *CLIP_HK.lock().unwrap() = Some(hk);
            }
        }
        Err(_) => eprintln!("剪贴板快捷键注册失败: {}", settings.clipboard_hotkey),
    }
    match shot.parse::<Shortcut>() {
        Ok(hk) => {
            if gs.register(hk.clone()).is_ok() {
                *SHOT_HK.lock().unwrap() = Some(hk);
            }
        }
        Err(_) => eprintln!("截图快捷键注册失败: {}", settings.screenshot_hotkey),
    }
}

// ---- 主窗口 ----

fn create_main_window(app: &AppHandle) -> Option<WebviewWindow> {
    WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("MoYan")
        .inner_size(440.0, 640.0)
        .min_inner_size(400.0, 520.0)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .visible(false)
        .build()
        .ok()
}

/// 窗口定位到光标附近（工作区内钳制，物理像素精确对齐 Win32）
#[cfg(windows)]
fn position_near_cursor(window: &WebviewWindow) {
    use windows::Win32::Foundation::{POINT, RECT};
    use windows::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    };
    use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

    unsafe {
        let mut pt = POINT::default();
        if GetCursorPos(&mut pt).is_err() {
            return;
        }
        let monitor = MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST);
        let mut info = MONITORINFO::default();
        info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
        if !GetMonitorInfoW(monitor, &mut info).as_bool() {
            return;
        }
        let RECT { left, top, right, bottom } = info.rcWork;
        let (w, h) = window
            .outer_size()
            .map(|s| (s.width as i32, s.height as i32))
            .unwrap_or((440, 640));
        let x = (pt.x + 14).clamp(left, right.saturating_sub(w).max(left));
        let y = (pt.y + 14).clamp(top, bottom.saturating_sub(h).max(top));
        let _ = window.set_position(tauri::PhysicalPosition::new(x, y));
    }
}

#[cfg(not(windows))]
fn position_near_cursor(window: &WebviewWindow) {
    if let Ok(Some(monitor)) = window.current_monitor() {
        let size = window.outer_size().unwrap_or_default();
        let _ = window.set_position(tauri::PhysicalPosition::new(
            monitor.position().x + 100,
            monitor.position().y + 100,
        ));
        let _ = size;
    }
}

pub fn show_main(app: &AppHandle, route: Option<&str>) {
    let window = match app.get_webview_window("main") {
        Some(w) => w,
        None => match create_main_window(app) {
            Some(w) => w,
            None => return,
        },
    };
    if let Some(r) = route {
        let _ = app.emit_to("main", "navigate", r);
    }
    position_near_cursor(&window);
    let _ = window.show();
    let _ = window.set_focus();
}

pub fn toggle_main(app: &AppHandle) {
    if screenshot::is_editor_open(app) {
        return;
    }
    if let Some(win) = app.get_webview_window("main") {
        if win.is_visible().unwrap_or(false) {
            let _ = win.hide();
            return;
        }
    }
    show_main(app, None);
}

// ---- 托盘 ----

fn create_tray(app: &AppHandle) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "open", "打开剪贴板", true, None::<&str>)?;
    let shot = MenuItem::with_id(app, "shot", "截屏", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "退出 MoYan", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &shot, &settings, &separator, &quit])?;

    let icon = tauri::image::Image::from_bytes(include_bytes!("../icons/tray.png"))?;

    TrayIconBuilder::with_id("moyan-tray")
        .icon(icon)
        .tooltip("MoYan")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "open" => show_main(app, Some("clipboard")),
            "shot" => {
                if let Err(e) = screenshot::capture_screen(app) {
                    eprintln!("截图失败: {}", e);
                }
            }
            "settings" => show_main(app, Some("settings")),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main(tray.app_handle(), None);
            }
        })
        .build(app)?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        // 单实例（必须第一个注册）：第二实例唤起主窗口
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            show_main(app, None);
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state != ShortcutState::Pressed {
                        return;
                    }
                    let is_clip = CLIP_HK
                        .lock()
                        .map(|g| g.as_ref() == Some(shortcut))
                        .unwrap_or(false);
                    let is_shot = SHOT_HK
                        .lock()
                        .map(|g| g.as_ref() == Some(shortcut))
                        .unwrap_or(false);
                    if is_clip {
                        toggle_main(app);
                    } else if is_shot {
                        if let Err(e) = screenshot::capture_screen(app) {
                            eprintln!("截图失败: {}", e);
                        }
                    }
                })
                .build(),
        )
        .on_window_event(|window, event| {
            // 主窗口失焦自动隐藏（设置页保持打开 / 截图编辑器打开时除外）
            if let WindowEvent::Focused(false) = event {
                if window.label() == "main" {
                    let app = window.app_handle();
                    if !KEEP_OPEN.load(Ordering::Relaxed)
                        && window.is_visible().unwrap_or(false)
                        && !screenshot::is_editor_open(app)
                    {
                        let _ = window.hide();
                    }
                }
            }
        })
        .setup(|app| {
            let handle = app.handle().clone();

            // 应用数据目录（%APPDATA%/MoYan，与 Electron userData 一致，无缝继承旧配置）
            let data_dir = handle
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| dirs::data_dir().unwrap_or_default().join("MoYan"));
            store::init(data_dir);

            let settings = store::load_settings();
            clipboard::init(&handle, settings.clone());

            // 主窗口（隐藏创建，唤起时定位到光标附近）
            create_main_window(&handle);

            // 托盘
            if let Err(e) = create_tray(&handle) {
                eprintln!("托盘创建失败: {}", e);
            }

            // 全局快捷键
            register_shortcuts(&handle, &settings);

            // 开机自启动
            use tauri_plugin_autostart::ManagerExt;
            if settings.launch_at_login {
                let _ = handle.autolaunch().enable();
            } else {
                let _ = handle.autolaunch().disable();
            }

            // 前台窗口监控（粘贴助手）
            paste::start_foreground_watcher();

            // 过期数据清理（每小时）
            {
                let h = handle.clone();
                thread::spawn(move || loop {
                    thread::sleep(Duration::from_secs(3600));
                    clipboard::cleanup_expired();
                    let _ = h.emit("clipboard:updated", ());
                });
            }

            // 启动即显示主窗口
            show_main(&handle, None);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::settings_get,
            commands::settings_update,
            commands::dialog_directory,
            commands::clipboard_items,
            commands::clipboard_image,
            commands::clipboard_copy_text,
            commands::clipboard_copy_image,
            commands::clipboard_paste_to_focus,
            commands::clipboard_delete_item,
            commands::clipboard_clear,
            commands::clipboard_pin,
            commands::window_hide,
            commands::window_keep_open,
            commands::navigate,
            commands::screenshot_trigger,
            commands::screenshot_init,
            commands::screenshot_complete,
            commands::screenshot_close,
            commands::app_version,
        ])
        .run(tauri::generate_context!())
        .expect("MoYan 启动失败");
}
