use tauri::{AppHandle, Manager, WebviewWindow};

#[tauri::command]
pub fn exit_app(app: AppHandle) {
    app.exit(0);
}

// ── 窗口类命令：仅桌面端有实际行为；移动端为空实现，保持前端 invoke 契约不报错 ──

#[tauri::command]
#[allow(unused_variables)]
pub fn minimize_window(window: WebviewWindow, skip_task_bar: Option<bool>) {
    #[cfg(desktop)]
    {
        let _ = window.minimize();
        if let Some(true) = skip_task_bar {
            let _ = window.set_skip_taskbar(true);
        }
    }
}

#[tauri::command]
#[allow(unused_variables)]
pub fn show_main_window(app: AppHandle) {
    #[cfg(desktop)]
    if let Some(window) = app.get_webview_window("main") {
        // 关闭到托盘时窗口可能被最小化并隐藏任务栏按钮，恢复显示时一并还原
        let _ = window.set_skip_taskbar(false);
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[tauri::command]
#[allow(unused_variables)]
pub fn toggle_maximize(window: WebviewWindow) {
    #[cfg(desktop)]
    if let Ok(is_max) = window.is_maximized() {
        if is_max {
            let _ = window.unmaximize();
        } else {
            let _ = window.maximize();
        }
    }
}

#[tauri::command]
#[allow(unused_variables)]
pub fn toggle_visible(window: WebviewWindow) {
    #[cfg(desktop)]
    if let Ok(visible) = window.is_visible() {
        if visible {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

#[tauri::command]
#[allow(unused_variables)]
pub fn enter_minimode(app: AppHandle) {
    #[cfg(desktop)]
    {
        if let Some(main) = app.get_webview_window("main") {
            let _ = main.hide();
        }
        if let Some(mini) = app.get_webview_window("minimode") {
            let _ = mini.show();
            let _ = mini.set_focus();
        }
    }
}

#[tauri::command]
#[allow(unused_variables)]
pub fn exit_minimode(app: AppHandle) {
    #[cfg(desktop)]
    {
        if let Some(mini) = app.get_webview_window("minimode") {
            let _ = mini.hide();
        }
        if let Some(main) = app.get_webview_window("main") {
            let _ = main.show();
            let _ = main.set_focus();
        }
    }
}

#[tauri::command]
#[allow(unused_variables)]
pub fn toggle_minimode(app: AppHandle) {
    #[cfg(desktop)]
    if let Some(mini) = app.get_webview_window("minimode") {
        if let Ok(is_vis) = mini.is_visible() {
            if is_vis {
                exit_minimode(app);
            } else {
                enter_minimode(app);
            }
        }
    }
}

#[tauri::command]
pub fn open_external(app: AppHandle, url: String) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_path(path: String) -> Result<(), String> {
    let p = std::path::Path::new(&path);
    if !p.exists() {
        let _ = std::fs::create_dir_all(p);
    }
    #[cfg(desktop)]
    return open::that(path).map_err(|e| e.to_string());
    // iOS：沙盒目录无法用系统程序打开
    #[cfg(mobile)]
    Err("open_path is not supported on mobile".to_string())
}

#[tauri::command]
pub fn show_item_in_folder(path: String) -> Result<bool, String> {
    let path = path.trim().to_string();
    // 文件已被移动/删除时不要拉起资源管理器，交由前端提示失败
    if path.is_empty() || !std::path::Path::new(&path).exists() {
        return Ok(false);
    }

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        use std::process::Command;

        // explorer 的 /select 解析对引号位置敏感：必须只给路径加引号。
        // 若整个参数被引号包裹（std::process::Command 对含空格参数的默认行为），
        // explorer 会把它当成未知项并退回「文档」目录。
        Command::new("explorer")
            .raw_arg(format!("/select,\"{}\"", path))
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(true)
    }
    #[cfg(all(desktop, not(target_os = "windows")))]
    {
        open::that(path).map_err(|e| e.to_string())?;
        Ok(true)
    }
    // iOS 沙盒内无法在「文件」App 中定位条目
    #[cfg(mobile)]
    {
        Ok(false)
    }
}

/// 忽略鼠标事件（歌词窗口穿透点击等）
#[tauri::command]
#[allow(unused_variables)]
pub fn ignore_mouse_event(window: WebviewWindow, ignore: bool) -> Result<(), String> {
    #[cfg(desktop)]
    return window
        .set_ignore_cursor_events(ignore)
        .map_err(|e| e.to_string());
    #[cfg(not(desktop))]
    Ok(())
}

/// 前端日志落点：输出到 stderr（env_logger 接管）
#[tauri::command]
pub fn log_entry(entry: serde_json::Value) {
    log::info!("[frontend] {}", entry);
}

/// WebView2 用户数据目录大小（字节）。目录不存在或统计失败返回 0。
#[tauri::command]
pub fn get_cache_size() -> u64 {
    let candidates = [
        dirs::data_local_dir().map(|d| d.join("com.anhe.player")),
        dirs::config_dir().map(|d| d.join("AnhePlayer").join("EBWebView")),
    ];
    for candidate in candidates.into_iter().flatten() {
        if candidate.is_dir() {
            return dir_size(&candidate, 0);
        }
    }
    0
}

fn dir_size(dir: &std::path::Path, depth: u32) -> u64 {
    if depth > 12 {
        return 0;
    }
    let mut total = 0u64;
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    total += dir_size(&entry.path(), depth + 1);
                } else {
                    total += meta.len();
                }
            }
        }
    }
    total
}

/// 清除 WebView 浏览数据（cookies/localStorage/IndexedDB/缓存）。
/// 注意：Tauri 未提供按类型清除的 API，此处为全量清除。
#[tauri::command]
#[allow(unused_variables)]
pub fn clear_cache(window: WebviewWindow) -> Result<(), String> {
    #[cfg(desktop)]
    return window.clear_all_browsing_data().map_err(|e| e.to_string());
    #[cfg(mobile)]
    Ok(())
}
