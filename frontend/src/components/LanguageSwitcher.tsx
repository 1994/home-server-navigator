import { useTranslation } from '../i18n';

export function LanguageSwitcher() {
  const { locale, setLocale } = useTranslation();

  return (
    <div className="language-switcher">
      <button
        type="button"
        className={locale === 'en' ? 'active' : ''}
        onClick={() => setLocale('en')}
        aria-label="Switch to English"
        title="English"
      >
        EN
      </button>
      <span className="divider">|</span>
      <button
        type="button"
        className={locale === 'zh' ? 'active' : ''}
        onClick={() => setLocale('zh')}
        aria-label="切换到中文"
        title="中文"
      >
        中
      </button>
    </div>
  );
}
