#[cfg(target_os = "windows")]
use std::{ffi::OsString, os::windows::ffi::OsStringExt, ptr, sync::Mutex};

#[cfg(target_os = "windows")]
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};

#[cfg(target_os = "windows")]
use winapi::shared::minwindef::DWORD;
#[cfg(target_os = "windows")]
use winapi::shared::windef::{HWINEVENTHOOK, HWND};
#[cfg(target_os = "windows")]
use winapi::um::winuser::{
    GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW, SetForegroundWindow,
    SetWinEventHook, EVENT_SYSTEM_FOREGROUND, WINEVENT_OUTOFCONTEXT,
};

#[cfg(target_os = "windows")]
static PREVIOUS_WINDOW: Mutex<Option<isize>> = Mutex::new(None);

#[cfg(target_os = "windows")]
/// 获取窗口标题
unsafe fn get_window_title(hwnd: HWND) -> String {
    let length = GetWindowTextLengthW(hwnd);

    if length == 0 {
        return String::new();
    }

    let mut buffer: Vec<u16> = vec![0; (length + 1) as usize];

    GetWindowTextW(hwnd, buffer.as_mut_ptr(), length + 1);

    OsString::from_wide(&buffer[..length as usize])
        .to_string_lossy()
        .into_owned()
}

#[cfg(target_os = "windows")]
/// 定义事件钩子回调函数
unsafe extern "system" fn event_hook_callback(
    _h_win_event_hook: HWINEVENTHOOK,
    event: DWORD,
    hwnd: HWND,
    _id_object: i32,
    _id_child: i32,
    _dw_event_thread: DWORD,
    _dwms_event_time: DWORD,
) {
    if event == EVENT_SYSTEM_FOREGROUND {
        // 保存上一个窗口
        let mut previous_window = PREVIOUS_WINDOW.lock().unwrap();
        let _ = previous_window.insert(hwnd as isize);
    }
}

#[cfg(target_os = "windows")]
/// 监听窗口切换
pub fn observe_app() {
    unsafe {
        // 设置事件钩子
        let hook = SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            ptr::null_mut(),
            Some(event_hook_callback),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        );

        if hook.is_null() {
            eprintln!("⚠️ 设置事件钩子失败");
        }
    }
}

#[cfg(target_os = "windows")]
/// 获取上一个窗口
pub fn get_previous_window() -> Option<isize> {
    PREVIOUS_WINDOW.lock().unwrap().clone()
}

#[cfg(target_os = "windows")]
/// 聚焦上一个窗口
fn focus_previous_window() {
    unsafe {
        let hwnd = match get_previous_window() {
            Some(hwnd) => hwnd as HWND,
            None => {
                // 如果没有上一个窗口，尝试获取当前前台窗口
                GetForegroundWindow()
            }
        };

        if hwnd.is_null() {
            eprintln!("⚠️ 无法获取有效的窗口句柄");
            return;
        }

        println!("📋 尝试聚焦窗口: {:?}", hwnd);

        // 使用 SetForegroundWindow 设置窗口焦点
        let result = SetForegroundWindow(hwnd);

        if result == 0 {
            eprintln!("⚠️ 设置窗口焦点失败");
        } else {
            println!("✅ 窗口焦点设置成功");
        }
    }
}

#[cfg(target_os = "windows")]
/// 等待函数（参考Lanaya实现）
fn wait(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

#[cfg(target_os = "windows")]
/// 在 Windows 上执行粘贴操作（参考Lanaya的延迟设置）
pub fn paste() -> Result<(), String> {
    println!("📋 开始粘贴操作（Windows）");

    // 先隐藏当前窗口，让焦点能够切换到其他窗口
    println!("📋 第一步：准备切换焦点");
    focus_previous_window();

    // 等待窗口切换完成（参考Lanaya的延迟）
    println!("📋 第二步：等待窗口切换...");
    wait(100);

    // 再次确保焦点设置
    println!("📋 第三步：再次确认焦点");
    focus_previous_window();
    wait(30);

    // 使用 enigo 模拟 Ctrl+V（参考Lanaya的按键延迟）
    println!("📋 第四步：模拟 Ctrl+V");

    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("初始化 Enigo 失败: {}", e))?;

    // 按下修饰键（Ctrl）后等待 30ms
    println!("📋 按下 Ctrl 键");
    enigo.key(Key::Control, Press)
        .map_err(|e| format!("按下 Ctrl 失败: {}", e))?;
    wait(30);

    // 按下并释放主键（V），按下后等待 50ms
    println!("📋 按下 V 键");
    enigo.key(Key::Unicode('v'), Click)
        .map_err(|e| format!("按下 V 失败: {}", e))?;
    wait(50);

    // 释放主键后等待 30ms
    println!("📋 释放 Ctrl 键");
    enigo.key(Key::Control, Release)
        .map_err(|e| format!("释放 Ctrl 失败: {}", e))?;
    wait(30);

    println!("✅ Windows 粘贴操作完成");

    Ok(())
}
