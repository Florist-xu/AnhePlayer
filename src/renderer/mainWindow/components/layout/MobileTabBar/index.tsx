// ============================================================================
// MobileTabBar — 移动端底部导航（仅 iOS 渲染，替代桌面 Sidebar）
// ============================================================================

import { useTranslation } from 'react-i18next';
import { useLocation, useNavigate } from 'react-router';
import { Trophy, Flame, HardDrive, Download, Settings } from 'lucide-react';
import type { LucideIcon } from 'lucide-react';
import { cn } from '@common/cn';
import { RoutePaths } from '../../../routes';
import type { RoutePath } from '../../../routes';
import './index.scss';

const TABS: { key: RoutePath; icon: LucideIcon; titleKey: string }[] = [
    { key: RoutePaths.Toplist, icon: Trophy, titleKey: 'toplist.title' },
    { key: RoutePaths.RecommendSheets, icon: Flame, titleKey: 'playlist.recommend' },
    { key: RoutePaths.LocalMusic, icon: HardDrive, titleKey: 'local_music.title' },
    { key: RoutePaths.Download, icon: Download, titleKey: 'download.title' },
    { key: RoutePaths.Setting, icon: Settings, titleKey: 'settings.title' },
];

export default function MobileTabBar() {
    const { t } = useTranslation();
    const navigate = useNavigate();
    const location = useLocation();
    const activeSegment = location.pathname.split('/')[1] ?? '';

    return (
        <nav className="l-mobile-tabbar" aria-label={t('app.nav_main')}>
            {TABS.map(({ key, icon: Icon, titleKey }) => (
                <button
                    key={key}
                    type="button"
                    className={cn('l-mobile-tabbar__item', activeSegment === key && 'is-active')}
                    aria-current={activeSegment === key ? 'page' : undefined}
                    onClick={() => navigate(`/${key}`)}
                >
                    <Icon size={22} />
                    <span className="l-mobile-tabbar__label">{t(titleKey)}</span>
                </button>
            ))}
        </nav>
    );
}
