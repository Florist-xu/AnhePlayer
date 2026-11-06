/**
 * 运行平台检测（同步，不依赖 Rust 回填）。
 *
 * iPadOS 13+ 的 UA 伪装成 Mac，因此额外用 maxTouchPoints 判断。
 * 非浏览器环境（测试 / SSR）一律视为桌面。
 */
function detectIOS(): boolean {
    if (typeof navigator === 'undefined') return false;
    const ua = navigator.userAgent || '';
    if (/iPhone|iPad|iPod/i.test(ua)) return true;
    return /Macintosh/i.test(ua) && (navigator.maxTouchPoints ?? 0) > 1;
}

export const isIOS = detectIOS();

/** 当前是否按移动端布局渲染（目前仅 iOS） */
export const isMobile = isIOS;

/** 与 Rust `std::env::consts::OS` 同名的平台字符串，用作 globalContext 兜底值 */
export const fallbackPlatform = isIOS ? 'ios' : 'win32';
