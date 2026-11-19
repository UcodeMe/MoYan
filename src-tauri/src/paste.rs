//! Windows 前台窗口监控与粘贴模拟。
//! 原生 Win32 API 实现，替代 Electron 版的 C# 动态编译助手（无需 csc.exe）。

use std::sync::atomic::{AtomicIsize, Ordering};
use std::thread;
use std::time::Duration;

static LAST_HWND: AtomicIsize = AtomicIsize::new(0);

#[cfg(windows)]
pub fn start_foreground_watcher() {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowThreadProcessId, IsWindowVisible,
    };

    thread::spawn(|| unsafe {
        let mut last: HWND = HWND(std::ptr::null_mut());
        loop {
            thread::sleep(Duration::from_millis(150));
            let hwnd = GetForegroundWindow();
            if !hwnd.0.is_null()
                && hwnd != last
                && IsWindowVisible(hwnd).as_bool()
            {
                let mut pid: u32 = 0;
                GetWindowThreadProcessId(hwnd, Some(&mut pid));
                // 排除本应用自身的窗口（单进程多窗口，PID 比对即可）
                if pid != std::process::id() {
                    last = hwnd;
                    LAST_HWND.store(hwnd.0 as isize, Ordering::SeqCst);
                }
            }
        }
    });
}

#[cfg(not(windows))]
pub fn start_foreground_watcher() {}

/// 还原记录的前台窗口并发送 Ctrl+V（对齐 C# 助手逻辑：SwitchToThisWindow + keybd_event）
#[cfg(windows)]
#[allow(deprecated)]
pub fn paste_to_recorded_window() -> bool {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        keybd_event, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        IsIconic, IsWindowVisible, ShowWindow, SW_RESTORE, SwitchToThisWindow,
    };

    let raw = LAST_HWND.load(Ordering::SeqCst);
    unsafe {
        // 有前台记录：还原目标窗口后发送；无记录：焦点仍在外部窗口，直接发送
        if raw != 0 {
            let target = HWND(raw as *mut core::ffi::c_void);
            if IsWindowVisible(target).as_bool() {
                if IsIconic(target).as_bool() {
                    let _ = ShowWindow(target, SW_RESTORE);
                }
                SwitchToThisWindow(target, false);
                thread::sleep(Duration::from_millis(80));
            }
        }
        const VK_CONTROL: u8 = 0x11;
        const VK_V: u8 = 0x56;
        keybd_event(VK_CONTROL, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_V, 0, KEYBD_EVENT_FLAGS(0), 0);
        thread::sleep(Duration::from_millis(20));
        keybd_event(VK_V, 0, KEYEVENTF_KEYUP, 0);
        keybd_event(VK_CONTROL, 0, KEYEVENTF_KEYUP, 0);
    }
    true
}

#[cfg(not(windows))]
pub fn paste_to_recorded_window() -> bool {
    false
}
