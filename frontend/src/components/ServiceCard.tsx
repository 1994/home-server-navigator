import { memo, useMemo } from 'react';
import { useTranslation } from '../i18n';
import type { ServiceEntry } from '../types';

interface ServiceCardProps {
  service: ServiceEntry;
  onEdit: (service: ServiceEntry) => void;
}

// Status colors mapped to CSS classes
const statusClasses: Record<ServiceEntry['status'], string> = {
  running: 'status-running',
  stopped: 'status-stopped',
  unknown: 'status-unknown',
};

// Protocol badge classes
const protocolClasses: Record<ServiceEntry['protocol'], string> = {
  http: 'http',
  https: 'https',
  tcp: 'tcp',
  other: 'other',
};

// Icon mapping for common services
const serviceIcons: Record<string, string> = {
  ssh: '🔐',
  nginx: '🌐',
  apache: '🪶',
  mysql: '🐬',
  postgres: '🐘',
  redis: '🔴',
  mongodb: '🍃',
  docker: '🐳',
  kubernetes: '☸️',
  grafana: '📊',
  prometheus: '🔥',
  elasticsearch: '🔍',
  kafka: '📨',
  rabbitmq: '🐰',
  gitlab: '🦊',
  jenkins: '👷',
  homeassistant: '🏠',
  plex: '🎬',
  jellyfin: '🎭',
  nextcloud: '☁️',
  pihole: '🕳️',
  adguard: '🛡️',
  vaultwarden: '🔑',
  portainer: '📦',
  traefik: '🚦',
};

function getServiceIcon(service: ServiceEntry): string {
  // Use custom icon if set
  if (service.icon) return service.icon;
  
  // Try to match by service name
  const name = service.service_name.toLowerCase();
  for (const [key, icon] of Object.entries(serviceIcons)) {
    if (name.includes(key)) return icon;
  }
  
  // Default icons based on protocol
  if (service.protocol === 'https') return '🔒';
  if (service.protocol === 'http') return '🌐';
  if (service.protocol === 'tcp') return '📡';
  
  return '📎';
}

function ServiceCardComponent({ service, onEdit }: ServiceCardProps) {
  const { t } = useTranslation();
  
  const resolvedUrl = useMemo(() => {
    if (service.url) {
      return service.url;
    }
    if ((service.protocol === 'http' || service.protocol === 'https') && service.port) {
      const path = service.path ? (service.path.startsWith('/') ? service.path : `/${service.path}`) : '';
      return `${service.protocol}://${service.host}:${service.port}${path}`;
    }
    return undefined;
  }, [service]);

  const portDisplay = service.port ? `:${service.port}` : '';
  const icon = getServiceIcon(service);
  const statusClass = statusClasses[service.status];
  const protocolClass = protocolClasses[service.protocol];
  
  // Get translated status label
  const statusLabel = t(`status.${service.status}`);
  // Get translated protocol label (uppercase for display)
  const protocolLabel = service.protocol.toUpperCase();

  return (
    <article className="service-card">
      <div className="service-card-header">
        <div className="service-card-title-row">
          <span className="service-icon" aria-hidden="true">
            {icon}
          </span>
          <div className="service-title-content">
            <h3 title={service.display_name}>{service.display_name}</h3>
            <p className="service-subtitle" title={service.service_name}>
              {service.service_name}
            </p>
          </div>
        </div>
        <div className="status-wrapper">
          <span
            className={`status-dot ${statusClass}`}
            title={statusLabel}
            aria-label={`${t('a11y.serviceStatus')}: ${statusLabel}`}
          />
        </div>
      </div>

      <div className="service-url-bar">
        {resolvedUrl ? (
          <a
            href={resolvedUrl}
            target="_blank"
            rel="noreferrer"
            className="url-link"
            title={resolvedUrl}
          >
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" aria-hidden="true">
              <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
              <polyline points="15 3 21 3 21 9" />
              <line x1="10" y1="14" x2="21" y2="3" />
            </svg>
            <span className="url-text">
              {service.host}
              {portDisplay}
              {service.path || ''}
            </span>
          </a>
        ) : (
          <span className="url-link disabled">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" aria-hidden="true">
              <circle cx="12" cy="12" r="10" />
              <line x1="15" y1="9" x2="9" y2="15" />
              <line x1="9" y1="9" x2="15" y2="15" />
            </svg>
            <span className="url-text">{t('empty.noUrl')}</span>
          </span>
        )}
      </div>

      <div className="service-meta-compact">
        <span className={`protocol-badge ${protocolClass}`}>
          {protocolLabel}
        </span>
        {service.group && <span className="group-badge">{service.group}</span>}
        {service.favorite && (
          <span className="favorite-badge" title={t('a11y.favoriteBadge')}>
            ⭐
          </span>
        )}
      </div>

      {service.tags.length > 0 ? (
        <div className="tag-list">
          {service.tags.slice(0, 4).map((tag) => (
            <span key={tag} className="tag-item">
              #{tag}
            </span>
          ))}
          {service.tags.length > 4 && (
            <span className="tag-item tag-more">+{service.tags.length - 4}</span>
          )}
        </div>
      ) : null}

      <div className="service-actions">
        <button 
          type="button" 
          className="edit-btn" 
          onClick={() => onEdit(service)} 
          title={t('actions.edit')}
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" aria-hidden="true">
            <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
            <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
          </svg>
          {t('actions.edit')}
        </button>
        <a
          className={`open-link ${resolvedUrl ? '' : 'disabled'}`}
          href={resolvedUrl || '#'}
          target="_blank"
          rel="noreferrer"
          onClick={(event) => {
            if (!resolvedUrl) {
              event.preventDefault();
            }
          }}
          aria-disabled={!resolvedUrl}
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" aria-hidden="true">
            <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
            <polyline points="15 3 21 3 21 9" />
            <line x1="10" y1="14" x2="21" y2="3" />
          </svg>
          {t('actions.open')}
        </a>
      </div>
    </article>
  );
}

// Memoize to prevent unnecessary re-renders
export const ServiceCard = memo(ServiceCardComponent);
