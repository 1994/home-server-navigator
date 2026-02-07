import { useCallback } from 'react';
import { useI18n } from './context';
import type { TranslationSchema } from './types';

type NestedKeyOf<T extends object> = {
  [K in keyof T & (string | number)]: T[K] extends object
    ? `${K}` | `${K}.${NestedKeyOf<T[K]>}`
    : `${K}`;
}[keyof T & (string | number)];

type TranslationPath = NestedKeyOf<TranslationSchema>;

function getNestedValue<T>(obj: T, path: string): string {
  const keys = path.split('.');
  let value: unknown = obj;
  
  for (const key of keys) {
    if (value === null || value === undefined) {
      return path;
    }
    value = (value as Record<string, unknown>)[key];
  }
  
  return typeof value === 'string' ? value : path;
}

export function useTranslation() {
  const { t, locale, setLocale } = useI18n();

  // Use useCallback to ensure t function reference is stable
  const tFn = useCallback((key: TranslationPath): string => {
    return getNestedValue(t, key);
  }, [t]);

  return {
    t: tFn,
    locale,
    setLocale,
  };
}
