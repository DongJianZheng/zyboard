mod database;
mod cache;
mod utils;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

use database::ClipboardDB;
use cache::ClipboardCache;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::GlobalShortcutExt;
use tauri_plugin_autostart::ManagerExt;
use tokio::time::interval;
use utils::{md5_hash, md5_hash_bytes, rgba8_to_base64, rgba8_to_jpeg_base64};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

const MAIN_WINDOW_LABEL: &str = "main";


// 全局状态
static mut GLOBAL_CACHE: Option<Arc<Mutex<ClipboardCache>>> = None;
static mut GLOBAL_DB: Option<Arc<Mutex<ClipboardDB>>> = None;
static mut GLOBAL_APP_HANDLE: Option<tauri::AppHandle> = None;
static mut IS_PASTING: bool = false;
static mut LAST_PASTED_MD5: Option<String> = None;

fn get_global_cache() -> &'static Arc<Mutex<ClipboardCache>> {
    unsafe {
        GLOBAL_CACHE.as_ref().unwrap()
    }
}

fn get_global_db() -> &'static Arc<Mutex<ClipboardDB>> {
    unsafe {
        GLOBAL_DB.as_ref().unwrap()
    }
}

fn get_app_handle() -> &'static tauri::AppHandle {
    unsafe {
        GLOBAL_APP_HANDLE.as_ref().unwrap()
    }
}

fn get_is_pasting() -> bool {
    unsafe {
        IS_PASTING
    }
}

fn set_is_pasting(value: bool) {
    unsafe {
        IS_PASTING = value;
    }
}

fn get_last_pasted_md5() -> Option<&'static String> {
    unsafe {
        LAST_PASTED_MD5.as_ref()
    }
}

fn set_last_pasted_md5(md5: Option<String>) {
    unsafe {
        LAST_PASTED_MD5 = md5;
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClipboardItem {
    pub id: u64,
    pub content: String,
    pub content_preview: Option<String>,
    pub data_type: String,
    pub is_favorite: bool,
    pub create_time: u64,
}

#[tauri::command]
async fn set_text(text: String) -> Result<(), String> {
    let mut clipboard = arboard::Clipboard::new()
        .map_err(|e| format!("无法访问剪贴板: {}", e))?;

    clipboard.set_text(&text)
        .map_err(|e| format!("写入剪贴板失败: {}", e))?;

    Ok(())
}

#[tauri::command]
async fn set_image(data: serde_json::Value) -> Result<(), String> {
    let mut clipboard = arboard::Clipboard::new()
        .map_err(|e| format!("无法访问剪贴板: {}", e))?;

    // 从 JSON 中提取 base64 数据
    let base64_str = data["base64"].as_str().ok_or("无效的 base64 数据")?;
    let width = data["width"].as_u64().ok_or("无效的宽度")? as u32;
    let height = data["height"].as_u64().ok_or("无效的高度")? as u32;

            let bytes = BASE64.decode(base64_str)
                .map_err(|e| format!("Base64 解码失败: {}", e))?;

            let img_data = arboard::ImageData {
                bytes: bytes.into(),
                width: width as usize,
                height: height as usize,
            };

    clipboard.set_image(img_data)
        .map_err(|e| format!("写入图片失败: {}", e))?;

    Ok(())
}

#[tauri::command]
async fn get_all_records() -> Result<Vec<ClipboardItem>, String> {
    let cache = get_global_cache();
    let cache_records = cache.lock().unwrap().get_all_records();

    let items: Vec<ClipboardItem> = cache_records
        .into_iter()
        .map(|r| ClipboardItem {
            id: r.id,
            content: r.content,
            content_preview: r.content_preview,
            data_type: r.data_type,
            is_favorite: r.is_favorite,
            create_time: r.create_time,
        })
        .collect();

    Ok(items)
}

#[tauri::command]
async fn delete_by_id(id: u64) -> Result<bool, String> {
    // 从缓存删除
    let cache = get_global_cache();
    cache.lock().unwrap().delete_record(id);

    // 从数据库删除
    let db = get_global_db();
    db.lock().unwrap().delete_by_id(id)
}

#[tauri::command]
async fn clear_all() -> Result<(), String> {
    // 清空缓存
    let cache = get_global_cache();
    cache.lock().unwrap().clear();

    // 清空数据库
    let db = get_global_db();
    db.lock().unwrap().clear_all()?;

    Ok(())
}

#[tauri::command]
async fn paste_content(id: u64, hide_window: bool) -> Result<(), String> {
    println!("📋 开始粘贴内容，ID: {}, 粘贴后隐藏: {}", id, hide_window);

    let cache = get_global_cache();
    let item = cache
        .lock()
        .unwrap()
        .get_record(id)
        .ok_or("记录不存在")?;

    // 先设置粘贴标志，防止写入剪贴板的内容被记录
    set_is_pasting(true);

    match item.data_type.as_str() {
        "text" => {
            // 计算内容的 MD5，用于后续跳过记录
            let content_md5 = md5_hash(&item.content);
            set_last_pasted_md5(Some(content_md5.clone()));
            println!("✅ 设置粘贴标志: IS_PASTING = true, MD5: {}", &content_md5[..8]);

            {
                let mut clipboard = arboard::Clipboard::new()
                    .map_err(|e| format!("无法访问剪贴板: {}", e))?;
                clipboard.set_text(&item.content)
                    .map_err(|e| format!("写入剪贴板失败: {}", e))?;
                println!("✅ 文本已写入剪贴板");
            } // 剪贴板实例在此处释放
        }
        "image" => {
            let image_data: serde_json::Value =
                serde_json::from_str(&item.content).unwrap_or(serde_json::json!({}));
            let base64_str = image_data["base64"].as_str().unwrap_or("");
            let width = image_data["width"].as_u64().unwrap_or(0) as u32;
            let height = image_data["height"].as_u64().unwrap_or(0) as u32;

            let bytes = BASE64.decode(base64_str)
                .map_err(|e| format!("Base64 解码失败: {}", e))?;

            // 计算图片的 MD5
            let img_md5 = md5_hash_bytes(&bytes);
            set_last_pasted_md5(Some(img_md5.clone()));
            println!("✅ 设置粘贴标志: IS_PASTING = true, 图片 MD5: {}", &img_md5[..8]);

            let img_data = arboard::ImageData {
                bytes: bytes.into(),
                width: width as usize,
                height: height as usize,
            };

            // 重试机制：Windows剪贴板可能被其他程序占用
            let mut last_error = None;
            for attempt in 0..5 {
                {
                    let mut clipboard = match arboard::Clipboard::new() {
                        Ok(cb) => cb,
                        Err(e) => {
                            last_error = Some(e);
                            if attempt < 4 {
                                // 等待100ms后重试
                                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                            }
                            continue;
                        }
                    };

                    match clipboard.set_image(img_data.clone()) {
                        Ok(_) => {
                            println!("✅ 图片已写入剪贴板");
                            // 成功后等待一小段时间确保剪贴板内容已写入
                            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                            break;
                        }
                        Err(e) => {
                            last_error = Some(e);
                            if attempt < 4 {
                                // 等待100ms后重试，让其他进程释放剪贴板
                                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                            }
                        }
                    }
                } // 剪贴板实例在此处释放
            }

            if let Some(err) = last_error {
                eprintln!("图片粘贴失败，已重试5次: {}", err);
                return Err(format!("写入图片失败: {}", err));
            }
        }
        _ => return Err("不支持的数据类型".to_string()),
    }

    // 不管是否设置隐藏窗口，都需要先隐藏窗口以释放焦点
    // 这样 Ctrl+V 才能输入到目标程序
    let app_handle = get_app_handle();
    let should_restore_window = !hide_window; // 记录是否需要在粘贴后恢复窗口

    if let Some(window) = app_handle.get_webview_window(MAIN_WINDOW_LABEL) {
        println!("📋 准备隐藏窗口以释放焦点（需要恢复: {}）", should_restore_window);
        #[cfg(target_os = "macos")]
        {
            let _ = macos::hide_panel(&window);
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = window.hide();
        }
        println!("✅ 窗口已隐藏");
    }

    // macOS 特定处理：模拟 Cmd+V
    #[cfg(target_os = "macos")]
    {
        let app_handle = get_app_handle();
        if let Some(window) = app_handle.get_webview_window(MAIN_WINDOW_LABEL) {
            macos::paste(&app_handle, &window);
        }
    }

    // Windows 特定处理：模拟 Ctrl+V
    #[cfg(target_os = "windows")]
    {
        if let Err(e) = windows::paste() {
            eprintln!("❌ Windows 粘贴失败: {}", e);
        }
    }

    // 如果用户选择不隐藏窗口，粘贴后重新显示
    if should_restore_window {
        if let Some(window) = app_handle.get_webview_window(MAIN_WINDOW_LABEL) {
            println!("📋 恢复窗口显示");
            #[cfg(target_os = "macos")]
            {
                let _ = macos::show_panel(&window);
            }
            #[cfg(not(target_os = "macos"))]
            {
                let _ = window.show();
                let _ = window.set_focus();
            }
            println!("✅ 窗口已恢复");
        }
    }

    // 较短时间后清除粘贴标志，但保留 MD5 匹配
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(1500)).await;
        set_is_pasting(false);
        println!("🔓 清除粘贴标志: IS_PASTING = false");
    });

    Ok(())
}

#[tauri::command]
async fn register_toggle_shortcut(shortcut: String) -> Result<(), String> {
    use tauri_plugin_global_shortcut::Shortcut;

    println!("🎯 [register_toggle_shortcut] 收到快捷键配置: {}", shortcut);

    // 保存快捷键配置到文件
    let app_handle = get_app_handle();
    let config_dir = app_handle.path()
        .app_config_dir()
        .map_err(|e| format!("获取配置目录失败: {}", e))?;

    std::fs::create_dir_all(&config_dir)
        .map_err(|e| format!("创建配置目录失败: {}", e))?;

    let config_path = config_dir.join("toggle_shortcut.json");
    let config = serde_json::json!({ "shortcut": shortcut });
    std::fs::write(&config_path, serde_json::to_string_pretty(&config).unwrap())
        .map_err(|e| format!("保存配置失败: {}", e))?;

    println!("✅ 快捷键配置已保存到文件: {:?}", config_path);

    let global_shortcut = app_handle.global_shortcut();

    // 先注销旧的快捷键
    let _ = global_shortcut.unregister_all();

    // macOS 上将 Alt 替换为 Option
    #[cfg(target_os = "macos")]
    let shortcut = shortcut.replace("Alt", "Option");

    println!("🎯 [register_toggle_shortcut] 转换后的快捷键: {}", shortcut);

    // 获取窗口
    let window = app_handle.get_webview_window("main").ok_or("获取窗口失败")?;
    let window_toggle = window.clone();

    // 解析快捷键字符串
    let shortcut_obj = Shortcut::try_from(shortcut.as_str())
        .map_err(|e| format!("快捷键格式无效: {}", e))?;

    println!("✅ [register_toggle_shortcut] 快捷键对象创建成功");

    // 注册新的快捷键
    global_shortcut.on_shortcut(shortcut_obj, move |_app, _shortcut, event| {
        if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
            let is_visible = window_toggle.is_visible().unwrap_or(false);
            let is_focused = window_toggle.is_focused().unwrap_or(false);

            println!("🎯 [快捷键触发] 窗口可见: {}, 焦点: {}", is_visible, is_focused);

            if is_visible && is_focused {
                #[cfg(target_os = "macos")]
                {
                    let _ = macos::hide_panel(&window_toggle);
                }
                #[cfg(not(target_os = "macos"))]
                {
                    let _ = window_toggle.hide();
                }
                println!("✅ 隐藏窗口");
            } else {
                #[cfg(target_os = "macos")]
                {
                    let _ = macos::show_panel(&window_toggle);
                }
                #[cfg(not(target_os = "macos"))]
                {
                    let _ = window_toggle.show();
                    let _ = window_toggle.set_focus();
                }
                println!("✅ 显示窗口");
            }
        }
    }).map_err(|e| format!("注册快捷键失败: {}", e))?;

    println!("✅ 快捷键已注册: {}", shortcut);
    Ok(())
}

#[tauri::command]
async fn set_autostart(enable: bool) -> Result<(), String> {
    let app_handle = get_app_handle();
    let autostart_manager = app_handle.autolaunch();

    if enable {
        autostart_manager.enable()
            .map_err(|e| format!("启用自启动失败: {}", e))?;
    } else {
        autostart_manager.disable()
            .map_err(|e| format!("禁用自启动失败: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
async fn is_autostart_enabled() -> Result<bool, String> {
    let app_handle = get_app_handle();
    let autostart_manager = app_handle.autolaunch();

    let enabled = autostart_manager.is_enabled()
        .map_err(|e| format!("检查自启动状态失败: {}", e))?;

    Ok(enabled)
}

#[tauri::command]
async fn set_data_dir(data_dir: String) -> Result<(), String> {
    // 保存自定义数据目录到配置
    let app_handle = get_app_handle();
    let config_dir = app_handle.path()
        .app_config_dir()
        .map_err(|e| format!("获取配置目录失败: {}", e))?;

    std::fs::create_dir_all(&config_dir)
        .map_err(|e| format!("创建配置目录失败: {}", e))?;

    let config_path = config_dir.join("data_dir.json");

    let config = serde_json::json!({ "data_dir": data_dir });
    std::fs::write(&config_path, serde_json::to_string_pretty(&config).unwrap())
        .map_err(|e| format!("保存配置失败: {}", e))?;

    // 提示用户重启应用以应用更改
    Ok(())
}

#[tauri::command]
async fn get_data_dir() -> Result<Option<String>, String> {
    let app_handle = get_app_handle();

    // 尝试读取配置文件
    let config_dir = app_handle.path()
        .app_config_dir()
        .map_err(|e| format!("获取配置目录失败: {}", e))?;

    let config_path = config_dir.join("data_dir.json");

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("读取配置失败: {}", e))?;

        let config: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("解析配置失败: {}", e))?;

        if let Some(data_dir) = config.get("data_dir").and_then(|v| v.as_str()) {
            return Ok(Some(data_dir.to_string()));
        }
    }

    // 返回默认目录
    let default_dir = app_handle.path()
        .app_data_dir()
        .map_err(|e| format!("获取默认数据目录失败: {}", e))?;

    Ok(Some(default_dir.to_string_lossy().to_string()))
}

#[tauri::command]
async fn set_max_history(limit: usize) -> Result<(), String> {
    // 保存最大记录数到配置
    let app_handle = get_app_handle();
    let config_dir = app_handle.path()
        .app_config_dir()
        .map_err(|e| format!("获取配置目录失败: {}", e))?;

    std::fs::create_dir_all(&config_dir)
        .map_err(|e| format!("创建配置目录失败: {}", e))?;

    let config_path = config_dir.join("max_history.json");
    let config = serde_json::json!({ "max_history": limit });
    std::fs::write(&config_path, serde_json::to_string_pretty(&config).unwrap())
        .map_err(|e| format!("保存配置失败: {}", e))?;

    println!("✅ 最大记录数已保存: {}", limit);
    Ok(())
}

#[tauri::command]
async fn get_max_history() -> Result<usize, String> {
    let app_handle = get_app_handle();

    // 尝试读取配置文件
    let config_dir = app_handle.path()
        .app_config_dir()
        .map_err(|e| format!("获取配置目录失败: {}", e))?;

    let config_path = config_dir.join("max_history.json");

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("读取配置失败: {}", e))?;

        let config: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("解析配置失败: {}", e))?;

        if let Some(limit) = config.get("max_history").and_then(|v| v.as_u64()) {
            return Ok(limit as usize);
        }
    }

    Ok(100) // 默认值
}

#[tauri::command]
async fn check_accessibility_permission() -> Result<bool, String> {
    #[cfg(target_os = "macos")]
    {
        use tauri_plugin_macos_permissions::check_accessibility_permission;
        let permitted = check_accessibility_permission().await;
        println!("🔐 辅助功能权限状态: {}", permitted);
        Ok(permitted)
    }

    #[cfg(not(target_os = "macos"))]
    {
        Ok(true) // 非 macOS 平台默认返回 true
    }
}

#[tauri::command]
async fn request_accessibility_permission() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use tauri_plugin_macos_permissions::request_accessibility_permission;
        request_accessibility_permission().await;
        println!("🔐 已请求辅助功能权限");
        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    {
        Ok(()) // 非 macOS 平台无需操作
    }
}

#[tauri::command]
async fn start_dragging() -> Result<(), String> {
    let app_handle = get_app_handle();
    if let Some(window) = app_handle.get_webview_window(MAIN_WINDOW_LABEL) {
        println!("🎯 尝试启动窗口拖拽");
        #[cfg(target_os = "macos")]
        {
            use tauri_nspanel::ManagerExt;
            if let Ok(panel) = app_handle.get_webview_panel(MAIN_WINDOW_LABEL) {
                let _ = panel.set_movable_by_window_background(true);
                println!("✅ 已设置 movable_by_window_background = true");
            } else {
                println!("❌ 无法获取 panel");
            }
        }
        window.start_dragging().map_err(|e| format!("启动拖拽失败: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn start_clipboard_monitor() -> Result<(), String> {
    let app_handle = get_app_handle();

    tokio::spawn(async move {
        let mut interval = interval(Duration::from_millis(1000));
        let mut last_text_md5 = String::new();
        let mut last_img_md5 = String::new();
        let mut clipboard = arboard::Clipboard::new().unwrap();

        println!("开始监听剪贴板...");

        loop {
            // 动态读取最大记录数配置
            let max_history = {
                let config_dir = app_handle.path().app_config_dir();
                if let Ok(config_dir) = config_dir {
                    let config_path = config_dir.join("max_history.json");
                    if config_path.exists() {
                        if let Ok(content) = std::fs::read_to_string(&config_path) {
                            if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                                config.get("max_history").and_then(|v| v.as_u64()).unwrap_or(100) as usize
                            } else {
                                100
                            }
                        } else {
                            100
                        }
                    } else {
                        100
                    }
                } else {
                    100
                }
            };
            interval.tick().await;
            let mut need_notify = false;

            // 监听文本变化
            match clipboard.get_text() {
                Ok(text) => {
                    let content_len = text.len();
                    let content = text.trim();

                    if !content.is_empty() {
                        let md5 = md5_hash(&text);

                        if md5 != last_text_md5 {
                            // 如果正在粘贴操作中，或者是刚粘贴的内容，跳过数据库插入
                            if get_is_pasting() {
                                println!("检测到粘贴操作（标志），跳过记录");
                                last_text_md5 = md5;
                                continue;
                            }

                            // 检查是否是刚粘贴的内容
                            if let Some(last_md5) = get_last_pasted_md5() {
                                if md5 == *last_md5 {
                                    println!("检测到刚粘贴的内容，跳过记录");
                                    last_text_md5 = md5.clone();
                                    set_last_pasted_md5(None); // 清除已匹配的 MD5
                                    continue;
                                }
                            }

                            println!("检测到新文本，长度: {}", content_len);

                            let content_preview = if content.len() > 100 {
                                Some(content.chars().take(100).collect())
                            } else {
                                Some(content.to_string())
                            };

                            let now = chrono::Local::now().timestamp_millis() as u64;
                            let record = database::ClipboardRecord {
                                id: 0,
                                content: text.clone(),
                                content_preview,
                                data_type: "text".to_string(),
                                is_favorite: false,
                                create_time: now,
                            };

                            let cache = get_global_cache();
                            if let Ok(db) = get_global_db().lock() {
                                match db.insert(record.clone()) {
                                    Ok(id) => {
                                        let mut record_with_id = record.clone();
                                        record_with_id.id = id;

                                        // 检查是否超过最大记录数限制
                                        match db.delete_over_limit(max_history) {
                                            Ok(deleted) => {
                                                if deleted {
                                                    // 删除了旧记录，重新加载所有记录到缓存
                                                    if let Ok(records) = db.get_all_records() {
                                                        cache.lock().unwrap().clear();
                                                        for r in records {
                                                            cache.lock().unwrap().add_record(r);
                                                        }
                                                        println!("✅ 缓存已更新");
                                                        // 通知前端刷新列表
                                                        let _ = app_handle.emit("clipboard-refresh", ());
                                                    }
                                                } else {
                                                    // 没有删除，直接添加到缓存
                                                    cache.lock().unwrap().add_record(record_with_id.clone());
                                                }
                                            }
                                            Err(e) => {
                                                eprintln!("删除超出限制的记录失败: {}", e);
                                                // 出错时也添加到缓存
                                                cache.lock().unwrap().add_record(record_with_id.clone());
                                            }
                                        }
                                        need_notify = true;

                                        println!("成功插入记录 ID: {}", id);

                                        let item = ClipboardItem {
                                            id: record_with_id.id,
                                            content: record_with_id.content,
                                            content_preview: record_with_id.content_preview,
                                            data_type: record_with_id.data_type,
                                            is_favorite: record_with_id.is_favorite,
                                            create_time: record_with_id.create_time,
                                        };
                                        let _ = app_handle.emit("clipboard-change", item);
                                    }
                                    Err(e) => {
                                        eprintln!("插入记录失败: {}", e);
                                    }
                                }
                            } else {
                                eprintln!("获取数据库锁失败");
                            }

                            last_text_md5 = md5;
                        }
                    }
                }
                Err(_e) => {
                    eprintln!("读取剪贴板文本失败");
                    if let Ok(new_clipboard) = arboard::Clipboard::new() {
                        clipboard = new_clipboard;
                    }
                }
            }

            // 监听图片变化
            match clipboard.get_image() {
                Ok(image) => {
                    let img_md5 = md5_hash_bytes(&image.bytes);
                    if img_md5 != last_img_md5 {
                        // 如果正在粘贴操作中，跳过数据库插入
                        if get_is_pasting() {
                            println!("检测到粘贴操作，跳过图片记录");
                            last_img_md5 = img_md5;
                            continue;
                        }

                        println!("检测到新图片，尺寸: {}x{}", image.width, image.height);

                        let rgba_image: image::RgbaImage = match image::ImageBuffer::from_raw(
                            image.width as u32,
                            image.height as u32,
                            image.bytes.to_vec()
                        ) {
                            Some(img) => img,
                            None => {
                                eprintln!("无法创建图像缓冲区");
                                continue;
                            }
                        };

                        let full_base64 = rgba8_to_base64(&rgba_image);
                        let preview_base64 = rgba8_to_jpeg_base64(&rgba_image, 75);

                        let now = chrono::Local::now().timestamp_millis() as u64;
                        let record = database::ClipboardRecord {
                            id: 0,
                            content: full_base64.clone(),
                            content_preview: Some(preview_base64),
                            data_type: "image".to_string(),
                            is_favorite: false,
                            create_time: now,
                        };

                        let cache = get_global_cache();
                        if let Ok(db) = get_global_db().lock() {
                            match db.insert(record.clone()) {
                                Ok(id) => {
                                    let mut record_with_id = record.clone();
                                    record_with_id.id = id;

                                    // 检查是否超过最大记录数限制
                                    match db.delete_over_limit(max_history) {
                                        Ok(deleted) => {
                                            if deleted {
                                                // 删除了旧记录，重新加载所有记录到缓存
                                                if let Ok(records) = db.get_all_records() {
                                                    cache.lock().unwrap().clear();
                                                    for r in records {
                                                        cache.lock().unwrap().add_record(r);
                                                    }
                                                    println!("✅ 缓存已更新");
                                                    // 通知前端刷新列表
                                                    let _ = app_handle.emit("clipboard-refresh", ());
                                                }
                                            } else {
                                                // 没有删除，直接添加到缓存
                                                cache.lock().unwrap().add_record(record_with_id.clone());
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("删除超出限制的记录失败: {}", e);
                                            // 出错时也添加到缓存
                                            cache.lock().unwrap().add_record(record_with_id.clone());
                                        }
                                    }
                                    need_notify = true;

                                    println!("成功插入图片记录 ID: {}", id);

                                    let item = ClipboardItem {
                                        id: record_with_id.id,
                                        content: record_with_id.content,
                                        content_preview: record_with_id.content_preview,
                                        data_type: record_with_id.data_type,
                                        is_favorite: record_with_id.is_favorite,
                                        create_time: record_with_id.create_time,
                                    };
                                    let _ = app_handle.emit("clipboard-change", item);
                                }
                                Err(e) => {
                                    eprintln!("插入图片记录失败: {}", e);
                                }
                            }
                        } else {
                            eprintln!("获取数据库锁失败");
                        }

                        last_img_md5 = img_md5;
                    }
                }
                Err(_e) => {
                    // 这是正常的，当剪贴板包含文本时会失败
                }
            }

            if need_notify {
                let _ = app_handle.emit("clipboard-refresh", ());
            }
        }
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--hidden"])
        ))
        .setup(|app| {
            // 检查启动参数，如果有 --hidden 则隐藏窗口（自启动时）
            let args: Vec<String> = std::env::args().collect();
            let should_hide_window = args.iter().any(|arg| arg == "--hidden");
            
            if should_hide_window {
                println!("🚀 自启动模式，隐藏窗口");
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            } else {
                println!("🚀 正常启动，显示窗口");
            }

            // 保存 app_handle
            let app_handle = app.handle().clone();
            unsafe {
                GLOBAL_APP_HANDLE = Some(app_handle);
            }

            // 初始化数据库
            let db = ClipboardDB::new(&app.handle(), None)
                .expect("初始化数据库失败");
            unsafe {
                GLOBAL_DB = Some(Arc::new(Mutex::new(db)));
            }

            // 初始化缓存
            unsafe {
                GLOBAL_CACHE = Some(Arc::new(Mutex::new(ClipboardCache::new())));
            }

            // 从数据库加载记录到缓存
            if let Ok(db) = get_global_db().lock() {
                if let Ok(records) = db.get_all_records() {
                    let cache = get_global_cache();
                    for record in records {
                        cache.lock().unwrap().add_record(record);
                    }
                }
            }

            let window = app.get_webview_window("main").unwrap();

            println!("🚀 应用启动，获取窗口: main");

            // macOS 特定设置：使用 NSPanel
            #[cfg(target_os = "macos")]
            {
                println!("🍎 macOS 平台，初始化 NSPanel...");
                // let _ = app.handle().plugin(tauri_nspanel::init());
                macos::setup_nspanel(app.handle(), window.clone());
            }

            // Windows 特定设置：监听窗口切换
            #[cfg(target_os = "windows")]
            {
                println!("🪟 Windows 平台，初始化窗口监听...");
                windows::observe_app();
            }

            // 获取全局快捷键管理器
            let global_shortcut = app.global_shortcut();

            // 先注销所有快捷键
            let _ = global_shortcut.unregister_all();

            // 尝试从配置文件中读取快捷键设置
            let config_dir = app.path().app_config_dir().unwrap();
            let config_path = config_dir.join("toggle_shortcut.json");

            let shortcut_str = if config_path.exists() {
                println!("📋 读取快捷键配置文件: {:?}", config_path);
                match std::fs::read_to_string(&config_path) {
                    Ok(content) => {
                        if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                            if let Some(shortcut) = config.get("shortcut").and_then(|v| v.as_str()) {
                                println!("✅ 从配置文件读取到快捷键: {}", shortcut);
                                shortcut.to_string()
                            } else {
                                println!("⚠️ 配置文件中没有找到快捷键设置，使用默认值");
                                "Alt+C".to_string()
                            }
                        } else {
                            println!("⚠️ 配置文件解析失败，使用默认值");
                            "Alt+C".to_string()
                        }
                    }
                    Err(e) => {
                        println!("⚠️ 读取配置文件失败: {}，使用默认值", e);
                        "Alt+C".to_string()
                    }
                }
            } else {
                println!("📋 配置文件不存在，使用默认快捷键: Alt+C");
                "Alt+C".to_string()
            };

            // macOS 上将 Alt 替换为 Option
            #[cfg(target_os = "macos")]
            let shortcut_str = shortcut_str.replace("Alt", "Option");

            println!("🎯 启动时注册快捷键: {}", shortcut_str);

            // 注册快捷键显示/隐藏窗口
            use tauri_plugin_global_shortcut::Shortcut;
            let window_toggle = window.clone();
            let shortcut_obj = Shortcut::try_from(shortcut_str.as_str())
                .expect("无效的快捷键格式");

            let registered_shortcut = shortcut_str.clone();
            global_shortcut.on_shortcut(shortcut_obj, move |_app: &tauri::AppHandle, _shortcut: &Shortcut, event: tauri_plugin_global_shortcut::ShortcutEvent| {
                if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                    let is_visible = window_toggle.is_visible().unwrap_or(false);

                    println!("🎯 快捷键 {} 触发，窗口可见: {}", registered_shortcut, is_visible);

                    if is_visible {
                        // 窗口可见，隐藏它
                        #[cfg(target_os = "macos")]
                        {
                            let _ = macos::hide_panel(&window_toggle);
                            println!("✅ 隐藏窗口");
                        }
                        #[cfg(not(target_os = "macos"))]
                        {
                            let _ = window_toggle.hide();
                            println!("✅ 隐藏窗口");
                        }
                    } else {
                        // 窗口不可见，显示它
                        #[cfg(target_os = "macos")]
                        {
                            let _ = macos::show_panel(&window_toggle);
                            println!("✅ 显示窗口");
                        }
                        #[cfg(not(target_os = "macos"))]
                        {
                            let _ = window_toggle.show();
                            let _ = window_toggle.set_focus();
                            println!("✅ 显示窗口");
                        }
                    }
                }
            }).expect("注册快捷键失败");

            println!("✅ 快捷键已成功注册: {}", shortcut_str);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_text,
            set_image,
            get_all_records,
            delete_by_id,
            clear_all,
            paste_content,
            start_clipboard_monitor,
            register_toggle_shortcut,
            set_autostart,
            is_autostart_enabled,
            set_data_dir,
            get_data_dir,
            start_dragging,
            set_max_history,
            get_max_history,
            check_accessibility_permission,
            request_accessibility_permission
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
