// macOS 模块 - 仅在 macOS 上编译
use tauri::{AppHandle, WebviewWindow};
use std::process::Command;
use std::path::PathBuf;

/// 在 macOS 上执行粘贴操作（使用 AppleScript 模拟 Cmd+V）
pub fn paste(_app_handle: &AppHandle, _window: &WebviewWindow) {
    println!("📋 开始粘贴操作（macOS）");

    // 使用 AppleScript 模拟 Cmd+V
    let script = r#"tell application "System Events" to keystroke "v" using command down"#;

    println!("📋 执行 AppleScript 粘贴命令");
    let result = Command::new("osascript")
        .args(["-e", script])
        .output();

    match &result {
        Ok(output) => {
            if output.status.success() {
                println!("✅ AppleScript 执行成功");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("⚠️ AppleScript 执行失败");
                eprintln!("❌ AppleScript 错误: {}", stderr);

                // 检查是否是权限错误
                if stderr.contains("1002") || stderr.contains("不允许发送按键") {
                    show_permission_alert();
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 执行粘贴失败: {}", e);
        }
    }
}

/// 显示权限提示对话框
fn show_permission_alert() {
    println!("🔐 需要授予系统权限");

    let app_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("正元剪贴板"));
    let app_name = app_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("正元剪贴板");

    let script = format!(
        r#"
        tell application "System Events"
            activate
            display dialog "「{app_name}」需要系统权限才能使用粘贴功能

请前往：系统设置 → 隐私与安全性
在「辅助功能」和「自动化」中
如果已存在「{app_name}」，先删除再重新添加并勾选" buttons {{"确定"}} default button "确定" with title "权限提示" with icon caution
        end tell
        "#
    );

    let _ = Command::new("osascript")
        .args(["-e", &script])
        .status();
}

/// 设置 NSPanel（简化版）
#[allow(dead_code)]
pub fn setup_nspanel(_app_handle: &AppHandle, _window: WebviewWindow) {
    println!("⚠️ macOS 特定功能，需要在 macOS 上编译");
}

/// 显示 Panel（简化版）
#[allow(dead_code)]
pub fn show_panel(window: &WebviewWindow) {
    let _ = window.show();
}

/// 隐藏 Panel（简化版）
#[allow(dead_code)]
pub fn hide_panel(window: &WebviewWindow) {
    let _ = window.hide();
}
