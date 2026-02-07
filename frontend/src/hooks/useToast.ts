import { useCallback, useState } from 'react';
import type { ToastMessage, ToastType } from '../components/Toast';

let toastId = 0;

export interface UseToastReturn {
  toasts: ToastMessage[];
  showToast: (type: ToastType, title: string, message?: string, duration?: number) => void;
  showSuccess: (title: string, message?: string, duration?: number) => void;
  showError: (title: string, message?: string, duration?: number) => void;
  showWarning: (title: string, message?: string, duration?: number) => void;
  showInfo: (title: string, message?: string, duration?: number) => void;
  removeToast: (id: string) => void;
  clearToasts: () => void;
}

export function useToast(): UseToastReturn {
  const [toasts, setToasts] = useState<ToastMessage[]>([]);

  const showToast = useCallback((
    type: ToastType, 
    title: string, 
    message?: string, 
    duration = 5000
  ) => {
    const id = `toast-${++toastId}`;
    setToasts((prev) => [...prev, { id, type, title, message, duration }]);
  }, []);

  const showSuccess = useCallback((
    title: string, 
    message?: string, 
    duration?: number
  ) => {
    showToast('success', title, message, duration);
  }, [showToast]);

  const showError = useCallback((
    title: string, 
    message?: string, 
    duration?: number
  ) => {
    showToast('error', title, message, duration);
  }, [showToast]);

  const showWarning = useCallback((
    title: string, 
    message?: string, 
    duration?: number
  ) => {
    showToast('warning', title, message, duration);
  }, [showToast]);

  const showInfo = useCallback((
    title: string, 
    message?: string, 
    duration?: number
  ) => {
    showToast('info', title, message, duration);
  }, [showToast]);

  const removeToast = useCallback((id: string) => {
    setToasts((prev) => prev.filter((toast) => toast.id !== id));
  }, []);

  const clearToasts = useCallback(() => {
    setToasts([]);
  }, []);

  return {
    toasts,
    showToast,
    showSuccess,
    showError,
    showWarning,
    showInfo,
    removeToast,
    clearToasts,
  };
}
