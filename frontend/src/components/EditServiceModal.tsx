import { type FormEvent, useEffect, useMemo, useState } from 'react';
import { useTranslation } from '../i18n';
import type { ServiceEntry, ServiceProtocol, UpdateServiceRequest } from '../types';

interface EditServiceModalProps {
  service: ServiceEntry | null;
  onClose: () => void;
  onSave: (id: string, payload: UpdateServiceRequest) => Promise<void>;
}

const protocolOptions: ServiceProtocol[] = ['http', 'https', 'tcp', 'other'];

// Get lockable fields with translated labels
const useLockableFields = () => {
  const { t } = useTranslation();
  return [
    { key: 'display_name', label: t('modal.displayName') },
    { key: 'description', label: t('modal.description') },
    { key: 'host', label: t('modal.host') },
    { key: 'port', label: t('modal.port') },
    { key: 'protocol', label: t('modal.protocol') },
    { key: 'path', label: t('modal.path') },
    { key: 'url', label: 'URL' },
    { key: 'group', label: t('modal.group') },
    { key: 'tags', label: t('modal.tags') },
    { key: 'icon', label: t('modal.icon') },
    { key: 'hidden', label: t('modal.hidden') },
    { key: 'favorite', label: t('modal.favorite') },
  ] as const;
};

export function EditServiceModal({ service, onClose, onSave }: EditServiceModalProps) {
  const { t } = useTranslation();
  const lockableFields = useLockableFields();
  
  // Form state
  const [displayName, setDisplayName] = useState('');
  const [description, setDescription] = useState('');
  const [host, setHost] = useState('');
  const [port, setPort] = useState('');
  const [protocol, setProtocol] = useState<ServiceProtocol>('http');
  const [path, setPath] = useState('');
  const [url, setUrl] = useState('');
  const [group, setGroup] = useState('');
  const [tags, setTags] = useState('');
  const [icon, setIcon] = useState('');
  const [hidden, setHidden] = useState(false);
  const [favorite, setFavorite] = useState(false);
  const [lockedFields, setLockedFields] = useState<string[]>([]);
  const [saving, setSaving] = useState(false);

  // Initialize form when service changes
  useEffect(() => {
    if (!service) return;
    
    setDisplayName(service.display_name);
    setDescription(service.description ?? '');
    setHost(service.host);
    setPort(service.port?.toString() ?? '');
    setProtocol(service.protocol);
    setPath(service.path ?? '');
    setUrl(service.url ?? '');
    setGroup(service.group ?? '');
    setTags(service.tags.join(', '));
    setIcon(service.icon ?? '');
    setHidden(service.hidden);
    setFavorite(service.favorite);
    setLockedFields(service.locked_fields);
  }, [service]);

  // Compute preview URL
  const previewUrl = useMemo(() => {
    if (url.trim()) return url.trim();
    
    const parsedPort = Number(port);
    if ((protocol === 'http' || protocol === 'https') && 
        Number.isFinite(parsedPort) && parsedPort > 0) {
      const normalizedPath = path.trim();
      const pathSuffix = normalizedPath
        ? normalizedPath.startsWith('/') ? normalizedPath : `/${normalizedPath}`
        : '';
      return `${protocol}://${host.trim() || 'localhost'}:${parsedPort}${pathSuffix}`;
    }
    return 'N/A';
  }, [host, path, port, protocol, url]);

  // Close on Escape key
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose();
    };
    
    if (service) {
      document.addEventListener('keydown', handleEscape);
      document.body.style.overflow = 'hidden';
    }
    
    return () => {
      document.removeEventListener('keydown', handleEscape);
      document.body.style.overflow = '';
    };
  }, [service, onClose]);

  if (!service) return null;

  const toggleLockedField = (field: string) => {
    setLockedFields((current) =>
      current.includes(field) 
        ? current.filter((value) => value !== field) 
        : [...current, field]
    );
  };

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setSaving(true);
    
    try {
      const parsedPort = port.trim() === '' ? null : Number(port);
      const payload: UpdateServiceRequest = {
        display_name: displayName.trim(),
        description: description.trim() || null,
        host: host.trim(),
        port: parsedPort === null || Number.isNaN(parsedPort) || parsedPort <= 0
          ? null
          : Math.floor(parsedPort),
        protocol,
        path: path.trim() || null,
        url: url.trim() || null,
        group: group.trim() || null,
        tags: tags
          .split(',')
          .map((value) => value.trim())
          .filter(Boolean),
        icon: icon.trim() || null,
        hidden,
        favorite,
        locked_fields: lockedFields,
        auto_lock: false,
      };
      
      await onSave(service.id, payload);
      onClose();
    } finally {
      setSaving(false);
    }
  };

  const handleBackdropClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) onClose();
  };

  return (
    <div 
      className="modal-backdrop" 
      onClick={handleBackdropClick}
      role="dialog"
      aria-modal="true"
      aria-labelledby="edit-service-title"
    >
      <section className="modal-panel" onClick={(e) => e.stopPropagation()}>
        <header className="modal-header">
          <h3 id="edit-service-title">{t('modal.editService')}</h3>
          <button 
            type="button" 
            className="modal-close-btn"
            onClick={onClose}
            aria-label={t('a11y.closeDialog')}
          >
            ✕
          </button>
        </header>

        <form className="modal-form" onSubmit={handleSubmit}>
          {/* Display Name */}
          <label>
            {t('modal.displayName')}
            <input 
              value={displayName} 
              onChange={(e) => setDisplayName(e.target.value)} 
              required 
              placeholder="My Service"
            />
          </label>

          {/* Description */}
          <label>
            {t('modal.description')}
            <input 
              value={description} 
              onChange={(e) => setDescription(e.target.value)}
              placeholder="Brief description..."
            />
          </label>

          {/* Host */}
          <label>
            {t('modal.host')}
            <input 
              value={host} 
              onChange={(e) => setHost(e.target.value)} 
              required 
              placeholder="localhost"
            />
          </label>

          {/* Port */}
          <label>
            {t('modal.port')}
            <input
              value={port}
              onChange={(e) => setPort(e.target.value)}
              inputMode="numeric"
              placeholder="8080"
            />
          </label>

          {/* Protocol */}
          <label>
            {t('modal.protocol')}
            <select 
              value={protocol} 
              onChange={(e) => setProtocol(e.target.value as ServiceProtocol)}
            >
              {protocolOptions.map((value) => (
                <option key={value} value={value}>
                  {value.toUpperCase()}
                </option>
              ))}
            </select>
          </label>

          {/* Path */}
          <label>
            {t('modal.path')}
            <input 
              value={path} 
              onChange={(e) => setPath(e.target.value)} 
              placeholder="/api"
            />
          </label>

          {/* URL Override */}
          <label>
            {t('modal.urlOverride')}
            <input 
              value={url} 
              onChange={(e) => setUrl(e.target.value)} 
              placeholder="https://..."
            />
          </label>

          {/* Group */}
          <label>
            {t('modal.group')}
            <input 
              value={group} 
              onChange={(e) => setGroup(e.target.value)} 
              placeholder="media"
            />
          </label>

          {/* Tags */}
          <label>
            {t('modal.tags')}
            <input 
              value={tags} 
              onChange={(e) => setTags(e.target.value)} 
              placeholder={t('modal.tagsPlaceholder')}
            />
          </label>

          {/* Icon */}
          <label>
            {t('modal.icon')}
            <input 
              value={icon} 
              onChange={(e) => setIcon(e.target.value)} 
              placeholder={t('modal.iconPlaceholder')}
            />
          </label>

          {/* Favorite Toggle */}
          <label className="toggle-row">
            <input 
              type="checkbox" 
              checked={favorite} 
              onChange={(e) => setFavorite(e.target.checked)} 
            />
            <span>⭐ {t('modal.addToFavorites')}</span>
          </label>

          {/* Hidden Toggle */}
          <label className="toggle-row">
            <input 
              type="checkbox" 
              checked={hidden} 
              onChange={(e) => setHidden(e.target.checked)} 
            />
            <span>🙈 {t('modal.hideFromView')}</span>
          </label>

          {/* Preview */}
          <div className="preview-row">
            <strong>{t('modal.previewUrl')}:</strong>
            <span>{previewUrl}</span>
          </div>

          {/* Locked Fields */}
          <fieldset className="lock-grid">
            <legend>🔒 {t('modal.lockedFields')} - {t('modal.lockedFieldsDescription')}</legend>
            {lockableFields.map(({ key, label }) => (
              <label key={key} className="checkbox-row">
                <input
                  type="checkbox"
                  checked={lockedFields.includes(key)}
                  onChange={() => toggleLockedField(key)}
                />
                <span>{label}</span>
              </label>
            ))}
          </fieldset>

          {/* Actions */}
          <div className="modal-actions">
            <button type="button" onClick={onClose}>
              {t('actions.cancel')}
            </button>
            <button type="submit" disabled={saving}>
              {saving ? t('modal.saving') : t('actions.save')}
            </button>
          </div>
        </form>
      </section>
    </div>
  );
}
