//! 全屏截图捕获与截图编辑器窗口管理（替代 Electron desktopCapturer）。

use std::io::Cursor;
use std::sync::Mutex;

use base64::Engine;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::types::ScreenshotInit;

static PENDING: Mutex<Option<ScreenshotInit>> = Mutex::new(None);

pub fn is_editor_open(app: &AppHandle) -> bool {
    app.get_webview_window("screenshot").is_some()
}

pub fn take_pending() -> Option<ScreenshotInit> {
    PENDING.lock().ok()?.clone()
}

fn clear_pending() {
    if let Ok(mut p) = PENDING.lock() {
        *p = None;
    }
}

/// 获取光标所在显示器的物理 bounds（x, y, w, h）
#[cfg(windows)]
fn cursor_monitor_bounds() -> (i32, i32, u32, u32) {
    use windows::Win32::Foundation::{POINT, RECT};
    use windows::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    };
    use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

    unsafe {
        let mut pt = POINT::default();
        if GetCursorPos(&mut pt).is_err() {
            return (0, 0, 1920, 1080);
        }
        let monitor = MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST);
        let mut info = MONITORINFO::default();
        info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
        if GetMonitorInfoW(monitor, &mut info).as_bool() {
            let RECT { left, top, right, bottom } = info.rcMonitor;
            (left, top, (right - left) as u32, (bottom - top) as u32)
        } else {
            (0, 0, 1920, 1080)
        }
    }
}

#[cfg(not(windows))]
fn cursor_monitor_bounds() -> (i32, i32, u32, u32) {
    (0, 0, 1920, 1080)
}

/// 捕获光标所在显示器画面并打开全屏编辑器窗口
pub fn capture_screen(app: &AppHandle) -> Result<(), String> {
    if is_editor_open(app) {
        if let Some(win) = app.get_webview_window("screenshot") {
            let _ = win.set_focus();
        }
        return Ok(());
    }
    let (mx, my, mw, mh) = cursor_monitor_bounds();

    // xcap：遍历所有显示器，找包含光标的那个（xcap 0.4 属性返回 Result）
    let monitors = xcap::Monitor::all().map_err(|e| e.to_string())?;
    let monitor = monitors
        .into_iter()
        .find(|m| match (m.x(), m.y(), m.width(), m.height()) {
            (Ok(x), Ok(y), Ok(w), Ok(h)) => mx >= x && my >= y && mx < x + w as i32 && my < y + h as i32,
            _ => false,
        })
        .ok_or_else(|| "未获取到屏幕画面".to_string())?;

    let rgba = monitor.capture_image().map_err(|e| e.to_string())?;
    let (w, h) = rgba.dimensions();
    let mut png = Vec::new();
    image::DynamicImage::ImageRgba8(rgba)
        .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
        .map_err(|e| e.to_string())?;
    let b64 = base64::engine::general_purpose::STANDARD.encode(png);
    let init = ScreenshotInit {
        data_url: format!("data:image/png;base64,{}", b64),
        width: w,
        height: h,
    };
    if let Ok(mut p) = PENDING.lock() {
        *p = Some(init);
    }
    open_editor(app, mx, my, mw, mh)
}

fn open_editor(app: &AppHandle, x: i32, y: i32, w: u32, h: u32) -> Result<(), String> {
    let window = WebviewWindowBuilder::new(app, "screenshot", WebviewUrl::default())
        .title("MoYan 截屏")
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .visible(false)
        .build()
        .map_err(|e| e.to_string())?;
    // 物理像素级精确定位与尺寸（覆盖目标显示器全屏，置于任务栏之上）
    let _ = window.set_position(tauri::PhysicalPosition::new(x as i32, y as i32));
    let _ = window.set_size(tauri::PhysicalSize::new(w, h));
    let _ = window.show();
    let _ = window.set_focus();
    Ok(())
}

pub fn close_editor(app: &AppHandle) {
    clear_pending();
    if let Some(win) = app.get_webview_window("screenshot") {
        let _ = win.destroy();
    }
}
