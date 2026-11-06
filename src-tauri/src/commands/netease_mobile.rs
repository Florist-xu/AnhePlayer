// 移动端空实现：网易云登录依赖弹出独立 WebView 窗口并读取其 Cookie，iOS 不支持。
// 前端会看到「未登录」，登录入口调用将返回错误。后续可改为应用内 WebView 或手动粘贴 Cookie。

use serde::{Deserialize, Serialize};

const UNSUPPORTED: &str = "netease login is not supported on mobile yet";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NeteaseSession {
    logged_in: bool,
    csrf: String,
    login_window_open: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum NeteaseMethod {
    Get,
    Post,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NeteaseResponse {
    status_code: u16,
    body: String,
}

#[tauri::command]
pub async fn netease_login_open() -> Result<(), String> {
    Err(UNSUPPORTED.to_string())
}

#[tauri::command]
pub async fn netease_login_wait() -> Result<bool, String> {
    Ok(false)
}

#[tauri::command]
pub fn netease_login_close() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn netease_session() -> Result<NeteaseSession, String> {
    Ok(NeteaseSession {
        logged_in: false,
        csrf: String::new(),
        login_window_open: false,
    })
}

#[tauri::command]
pub async fn netease_request(
    _method: NeteaseMethod,
    _path: String,
    _body: Option<String>,
) -> Result<NeteaseResponse, String> {
    Err(UNSUPPORTED.to_string())
}

#[tauri::command]
pub async fn netease_logout() -> Result<(), String> {
    Ok(())
}
