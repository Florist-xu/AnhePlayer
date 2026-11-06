pub mod app_config;
pub mod audio_tag;
pub mod dialog;
pub mod download;
pub mod forwarder;
pub mod fs_util;
pub mod global_context;
pub mod http_proxy;
pub mod local_meta;
#[cfg(desktop)]
pub mod lyric_window;
#[cfg(mobile)]
#[path = "lyric_window_mobile.rs"]
pub mod lyric_window;
#[cfg(desktop)]
pub mod netease;
#[cfg(mobile)]
#[path = "netease_mobile.rs"]
pub mod netease;
pub mod paths;
pub mod system_util;
pub mod webdav;
