// ============================================================================
// MobileMinePage — 「我的」（仅移动端）
// ============================================================================
//
// 桌面侧边栏里的内容在手机上没有位置，这里集中提供：
//   - 快捷入口：最近播放 / 下载管理 / 电台 / 插件管理 / 主题
//   - 我创建的歌单（含「我喜欢」，可新建 / 导入）
//   - 收藏的歌单
// 歌单行长按 = 桌面端右键菜单（重命名 / 删除 等）。

import { useCallback, useMemo, useRef, type TouchEvent } from 'react';
import { useTranslation } from 'react-i18next';
import { useNavigate } from 'react-router';
import {
    Blocks,
    ChevronRight,
    Clock,
    Download,
    Heart,
    Import as ImportIcon,
    ListMusic,
    Palette,
    Plus,
    Podcast,
} from 'lucide-react';
import type { LucideIcon } from 'lucide-react';
import { safeParse } from '@common/safeSerialize';
import { useMusicSheetList, useStarredSheets } from '@infra/musicSheet/renderer';
import { DEFAULT_FAVORITE_SHEET_ID } from '@infra/musicSheet/common/constant';
import type { IStarredSheetItem } from '@appTypes/infra/musicSheet';
import { showModal } from '../../components/ui/Modal/modalManager';
import { showContextMenu } from '../../components/ui/ContextMenu/contextMenuManager';
import { RoutePaths, localSheetRoute, musicSheetRoute } from '../../routes';
import './index.scss';

const LONG_PRESS_MS = 500;

/** 长按手势：触发后吞掉紧随其后的 click，避免长按松手时又进入歌单。 */
function useLongPress(onLongPress: (pos: { x: number; y: number }) => void) {
    const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
    const firedRef = useRef(false);

    const cancel = useCallback(() => {
        if (timerRef.current) {
            clearTimeout(timerRef.current);
            timerRef.current = null;
        }
    }, []);

    const onTouchStart = useCallback(
        (e: TouchEvent) => {
            cancel();
            firedRef.current = false;
            const touch = e.touches[0];
            if (!touch) return;
            const pos = { x: touch.clientX, y: touch.clientY };
            timerRef.current = setTimeout(() => {
                timerRef.current = null;
                firedRef.current = true;
                onLongPress(pos);
            }, LONG_PRESS_MS);
        },
        [cancel, onLongPress],
    );

    /** 返回 true 表示这次 click 属于长按，应忽略 */
    const consumeClick = useCallback(() => {
        if (firedRef.current) {
            firedRef.current = false;
            return true;
        }
        return false;
    }, []);

    return {
        onTouchStart,
        onTouchMove: cancel,
        onTouchEnd: cancel,
        onTouchCancel: cancel,
        consumeClick,
    };
}

function QuickLink({
    icon: Icon,
    text,
    onClick,
}: {
    icon: LucideIcon;
    text: string;
    onClick: () => void;
}) {
    return (
        <button type="button" className="p-mine__row" onClick={onClick}>
            <Icon size={20} className="p-mine__row-icon" />
            <span className="p-mine__row-label">{text}</span>
            <ChevronRight size={16} className="p-mine__row-arrow" />
        </button>
    );
}

function LocalSheetRow({ sheet }: { sheet: { id: string; title: string } }) {
    const { t } = useTranslation();
    const navigate = useNavigate();
    const isFavorite = sheet.id === DEFAULT_FAVORITE_SHEET_ID;
    const Icon = isFavorite ? Heart : ListMusic;

    const longPress = useLongPress(
        useCallback(
            (pos) => {
                if (isFavorite) return; // 「我喜欢」不可重命名 / 删除
                showContextMenu('LocalSheetMenu', pos, {
                    sheetId: sheet.id,
                    sheetTitle: sheet.title,
                });
            },
            [isFavorite, sheet.id, sheet.title],
        ),
    );

    return (
        <button
            type="button"
            className="p-mine__row"
            onClick={() => {
                if (longPress.consumeClick()) return;
                navigate(localSheetRoute(sheet.id));
            }}
            onTouchStart={longPress.onTouchStart}
            onTouchMove={longPress.onTouchMove}
            onTouchEnd={longPress.onTouchEnd}
            onTouchCancel={longPress.onTouchCancel}
        >
            <Icon size={20} className="p-mine__row-icon" />
            <span className="p-mine__row-label">
                {isFavorite ? t('media.default_favorite_sheet_name') : sheet.title}
            </span>
            <ChevronRight size={16} className="p-mine__row-arrow" />
        </button>
    );
}

function StarredSheetRow({ item }: { item: IStarredSheetItem }) {
    const navigate = useNavigate();
    const fullItem = useMemo(
        () =>
            safeParse<IMusic.IMusicSheetItem>(item.raw) ?? {
                platform: item.platform,
                id: item.id,
                title: item.title ?? '',
            },
        [item.raw, item.platform, item.id, item.title],
    );

    const longPress = useLongPress(
        useCallback(
            (pos) => {
                showContextMenu('StarredSheetMenu', pos, {
                    sheetItem: { platform: item.platform, id: item.id },
                });
            },
            [item.platform, item.id],
        ),
    );

    return (
        <button
            type="button"
            className="p-mine__row"
            onClick={() => {
                if (longPress.consumeClick()) return;
                navigate(musicSheetRoute(item.platform, item.id), {
                    state: { sheetItem: fullItem },
                });
            }}
            onTouchStart={longPress.onTouchStart}
            onTouchMove={longPress.onTouchMove}
            onTouchEnd={longPress.onTouchEnd}
            onTouchCancel={longPress.onTouchCancel}
        >
            <ListMusic size={20} className="p-mine__row-icon" />
            <span className="p-mine__row-label">{item.title ?? fullItem.title}</span>
            <ChevronRight size={16} className="p-mine__row-arrow" />
        </button>
    );
}

export default function MobileMinePage() {
    const { t } = useTranslation();
    const navigate = useNavigate();
    const sheets = useMusicSheetList();
    const starredSheets = useStarredSheets();

    return (
        <div className="p-mine">
            <h2 className="p-mine__title">{t('app.nav_mine')}</h2>

            <section className="p-mine__group">
                <QuickLink
                    icon={Clock}
                    text={t('history.title')}
                    onClick={() => navigate(`/${RoutePaths.RecentlyPlayed}`)}
                />
                <QuickLink
                    icon={Download}
                    text={t('download.title')}
                    onClick={() => navigate(`/${RoutePaths.Download}`)}
                />
                <QuickLink
                    icon={Podcast}
                    text={t('radio.title')}
                    onClick={() => navigate(`/${RoutePaths.Radio}`)}
                />
                <QuickLink
                    icon={Blocks}
                    text={t('plugin.plugin_management')}
                    onClick={() => navigate(`/${RoutePaths.PluginManager}`)}
                />
                <QuickLink
                    icon={Palette}
                    text={t('theme.title')}
                    onClick={() => navigate(`/${RoutePaths.Theme}`)}
                />
            </section>

            <section className="p-mine__group">
                <div className="p-mine__group-header">
                    <h3 className="p-mine__group-title">{t('playlist.created_sheets')}</h3>
                    <div className="p-mine__group-actions">
                        <button
                            type="button"
                            className="p-mine__icon-btn"
                            aria-label={t('plugin.method_import_music_sheet')}
                            onClick={() =>
                                showModal('PluginImportModal', { type: 'importMusicSheet' })
                            }
                        >
                            <ImportIcon size={18} />
                        </button>
                        <button
                            type="button"
                            className="p-mine__icon-btn"
                            aria-label={t('playlist.create_sheet')}
                            onClick={() => showModal('CreateSheetModal')}
                        >
                            <Plus size={18} />
                        </button>
                    </div>
                </div>
                {sheets.length > 0 ? (
                    sheets.map((sheet) => <LocalSheetRow key={sheet.id} sheet={sheet} />)
                ) : (
                    <div className="p-mine__placeholder">{t('playlist.no_sheets')}</div>
                )}
            </section>

            <section className="p-mine__group">
                <div className="p-mine__group-header">
                    <h3 className="p-mine__group-title">{t('playlist.starred_sheets')}</h3>
                </div>
                {starredSheets.length > 0 ? (
                    starredSheets.map((item) => (
                        <StarredSheetRow key={`${item.platform}\0${item.id}`} item={item} />
                    ))
                ) : (
                    <div className="p-mine__placeholder">{t('playlist.no_starred')}</div>
                )}
            </section>
        </div>
    );
}
