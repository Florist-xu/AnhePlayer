// 移动端空实现：iOS 没有独立的桌面歌词窗口。
// 保留同名命令与 State，使 lib.rs 的 manage / generate_handler 在两端无需分叉。

#[derive(Default)]
pub struct LyricWindowState;

#[tauri::command]
pub fn sync_lyric_window(
    _enabled: bool,
    _always_on_top: bool,
    _locked: bool,
) -> Result<(), String> {
    Ok(())
}
