//! iOS 音频会话。
//!
//! WKWebView 里的 `<audio>` 要在锁屏 / 切后台后继续播放，需要两个条件：
//! 1. Info.plist 声明 `UIBackgroundModes = [audio]`（见 `Info.ios.plist`）；
//! 2. 进程内把 AVAudioSession 的 category 设为 `playback` 并激活。
//!
//! 这里负责第 2 点，在 app 启动与每次回到前台时调用 `activate`（被电话等打断后需重新激活）。

#[cfg(target_os = "ios")]
pub fn activate() {
    use objc2_avf_audio::{AVAudioSession, AVAudioSessionCategoryPlayback};

    // SAFETY: 仅在主线程之外也安全；AVAudioSession 是线程安全的单例，
    // AVAudioSessionCategoryPlayback 是系统提供的常量。
    unsafe {
        let session = AVAudioSession::sharedInstance();
        let Some(category) = AVAudioSessionCategoryPlayback else {
            log::warn!("[audio-session] AVAudioSessionCategoryPlayback unavailable");
            return;
        };
        if let Err(err) = session.setCategory_error(category) {
            log::warn!("[audio-session] setCategory failed: {err:?}");
            return;
        }
        if let Err(err) = session.setActive_error(true) {
            log::warn!("[audio-session] setActive failed: {err:?}");
        }
    }
}

/// 非 iOS 平台无需处理。
#[cfg(not(target_os = "ios"))]
pub fn activate() {}
