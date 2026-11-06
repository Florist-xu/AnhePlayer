//! 跨平台目录。
//!
//! 桌面端沿用原有位置（`<config_dir>/AnhePlayer`、系统下载目录），保证老用户数据不变。
//! iOS 上 `dirs` 返回的路径不在 App 沙盒内或为 None，需要由 `init` 在启动时注入
//! Tauri 解析出的沙盒目录（Documents / Library）。

use std::path::PathBuf;
use std::sync::OnceLock;
use tauri::{AppHandle, Manager};

static USER_DATA: OnceLock<PathBuf> = OnceLock::new();

/// 在 setup 阶段调用一次。桌面端不覆盖默认值。
pub fn init(app: &AppHandle) {
    #[cfg(mobile)]
    if let Ok(dir) = app.path().app_data_dir() {
        let _ = USER_DATA.set(dir);
    }
    #[cfg(desktop)]
    let _ = app;
}

/// 配置 / 用户数据目录。
pub fn user_data_dir() -> PathBuf {
    if let Some(p) = USER_DATA.get() {
        return p.clone();
    }
    dirs::config_dir()
        .map(|p| p.join("AnhePlayer"))
        .unwrap_or_else(|| PathBuf::from(".").join("AnhePlayer"))
}

/// 默认下载目录。iOS 使用 App 的 Documents 目录（可在「文件」App 中看到）。
pub fn download_dir(app: &AppHandle) -> PathBuf {
    #[cfg(mobile)]
    if let Ok(dir) = app.path().document_dir() {
        return dir;
    }
    #[cfg(desktop)]
    let _ = app;
    dirs::download_dir().unwrap_or_else(|| PathBuf::from("."))
}
