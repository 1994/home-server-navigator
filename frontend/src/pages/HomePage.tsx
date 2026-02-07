import { useCallback, useEffect, useMemo, useRef, useState } from 'react';

import { fetchDiscoveryStatus, fetchServices, runDiscovery, updateService } from '../api/client';
import { EditServiceModal } from '../components/EditServiceModal';
import { LanguageSwitcher } from '../components/LanguageSwitcher';
import { ServiceCard } from '../components/ServiceCard';
import { ToastContainer } from '../components/Toast';
import { useTranslation } from '../i18n';
import { useDebounce } from '../hooks/useDebounce';
import { useToast } from '../hooks/useToast';
import type { DiscoveryStatusInfo, ServiceEntry, ServiceStatus } from '../types';

// Stat icon SVGs
const statIcons = {
  total: (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" aria-hidden="true">
      <rect x="2" y="3" width="20" height="14" rx="2" />
      <line x1="8" y1="21" x2="16" y2="21" />
      <line x1="12" y1="17" x2="12" y2="21" />
    </svg>
  ),
  running: (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" aria-hidden="true">
      <polygon points="5 3 19 12 5 21 5 3" />
    </svg>
  ),
  stopped: (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" aria-hidden="true">
      <rect x="6" y="6" width="12" height="12" rx="2" />
    </svg>
  ),
  unknown: (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" aria-hidden="true">
      <circle cx="12" cy="12" r="10" />
      <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" />
      <line x1="12" y1="17" x2="12.01" y2="17" />
    </svg>
  ),
};

// Predefined groups
const PREDEFINED_GROUPS = ['Media', 'Download', 'Sync', 'Photo', 'Monitor', 'Other'];

export function HomePage() {
  // i18n - use ref to avoid re-renders
  const { t } = useTranslation();
  const tRef = useRef(t);
  tRef.current = t;

  // Data state
  const [services, setServices] = useState<ServiceEntry[]>([]);
  const [systemServices, setSystemServices] = useState<ServiceEntry[]>([]);
  const [discoveryStatus, setDiscoveryStatus] = useState<DiscoveryStatusInfo | null>(null);
  
  // UI state
  const [systemExpanded, setSystemExpanded] = useState(false);
  const [query, setQuery] = useState('');
  const [groupFilter, setGroupFilter] = useState('');
  const [statusFilter, setStatusFilter] = useState<ServiceStatus | ''>('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [runningDiscovery, setRunningDiscovery] = useState(false);
  const [editingService, setEditingService] = useState<ServiceEntry | null>(null);
  
  // Hooks
  const debouncedQuery = useDebounce(query, 300);
  const { toasts, showSuccess, showError, removeToast } = useToast();
  const showErrorRef = useRef(showError);
  showErrorRef.current = showError;
  const showSuccessRef = useRef(showSuccess);
  showSuccessRef.current = showSuccess;

  // Load main services data - stable callback
  const loadData = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [serviceData, statusData] = await Promise.all([
        fetchServices({
          q: debouncedQuery || undefined,
          group: groupFilter || undefined,
          status: statusFilter || undefined,
          include_hidden: false,
        }),
        fetchDiscoveryStatus(),
      ]);
      setServices(serviceData);
      setDiscoveryStatus(statusData);
    } catch (loadError) {
      const message = loadError instanceof Error ? loadError.message : 'Failed to load services';
      setError(message);
      showErrorRef.current(tRef.current('toast.loadServicesFailed'), message);
    } finally {
      setLoading(false);
    }
  }, [debouncedQuery, groupFilter, statusFilter]); // Only depend on filter states

  // Load system services - stable callback
  const loadSystemServices = useCallback(async () => {
    try {
      const data = await fetchServices({
        q: debouncedQuery || undefined,
        group: '系统',
        status: statusFilter || undefined,
        include_hidden: true,
      });
      setSystemServices(data);
    } catch (loadError) {
      const message = loadError instanceof Error ? loadError.message : 'Failed to load system services';
      showErrorRef.current(tRef.current('toast.loadSystemFailed'), message);
    }
  }, [debouncedQuery, statusFilter]); // Only depend on filter states

  // Initial load
  useEffect(() => {
    void loadData();
  }, [loadData]);

  // Computed values
  const uniqueGroups = useMemo(() => {
    const set = new Set<string>();
    for (const service of services) {
      if (service.group && !PREDEFINED_GROUPS.includes(service.group) && service.group !== '系统') {
        set.add(service.group);
      }
    }
    return Array.from(set).sort();
  }, [services]);

  const favoriteServices = useMemo(() => 
    services.filter((service) => service.favorite),
    [services]
  );

  const nonFavoriteServices = useMemo(() => 
    services.filter((service) => !service.favorite),
    [services]
  );

  const statusCounts = useMemo(() => 
    services.reduce(
      (acc, service) => {
        acc[service.status] += 1;
        return acc;
      },
      { running: 0, stopped: 0, unknown: 0 }
    ),
    [services]
  );

  // Handlers
  const handleRunDiscovery = async () => {
    setRunningDiscovery(true);
    try {
      await runDiscovery();
      showSuccessRef.current(tRef.current('toast.discoveryCompleted'), tRef.current('toast.discoveryCompletedMsg'));
      await loadData();
      if (systemExpanded) {
        await loadSystemServices();
      }
    } catch (runError) {
      const message = runError instanceof Error ? runError.message : 'Failed to run discovery';
      showErrorRef.current(tRef.current('toast.discoveryFailed'), message);
    } finally {
      setRunningDiscovery(false);
    }
  };

  const toggleSystem = async () => {
    const next = !systemExpanded;
    setSystemExpanded(next);
    if (next) {
      await loadSystemServices();
    }
  };

  const handleSaveService = async (id: string, payload: Parameters<typeof updateService>[1]) => {
    try {
      const updated = await updateService(id, payload);
      setServices((current) =>
        current
          .map((service) => (service.id === id ? updated : service))
          .sort((left, right) => left.display_name.localeCompare(right.display_name))
      );
      setSystemServices((current) =>
        current
          .map((service) => (service.id === id ? updated : service))
          .sort((left, right) => left.display_name.localeCompare(right.display_name))
      );
      showSuccessRef.current(tRef.current('toast.serviceUpdated'), updated.display_name);
    } catch (saveError) {
      const message = saveError instanceof Error ? saveError.message : 'Failed to save service';
      showErrorRef.current(tRef.current('toast.saveFailed'), message);
      throw saveError;
    }
  };

  const clearFilters = () => {
    setQuery('');
    setGroupFilter('');
    setStatusFilter('');
  };

  const hasActiveFilters = debouncedQuery || groupFilter || statusFilter;

  // Render helpers
  const renderEmptyState = () => {
    if (hasActiveFilters) {
      return (
        <div className="empty-state">
          <div className="empty-icon" aria-hidden="true">🔍</div>
          <h3>{t('empty.noResults')}</h3>
          <p>{t('empty.noResultsDescription')}</p>
          <button type="button" onClick={clearFilters}>
            {t('actions.clear')} {t('filters.allGroups')}
          </button>
        </div>
      );
    }
    return (
      <div className="empty-state">
        <div className="empty-icon" aria-hidden="true">🧭</div>
        <h3>{t('empty.noServices')}</h3>
        <p>{t('empty.noServicesDescription')}</p>
        <button type="button" onClick={handleRunDiscovery} disabled={runningDiscovery}>
          {runningDiscovery ? t('actions.discovering') : t('actions.discover')}
        </button>
      </div>
    );
  };

  return (
    <main className="page">
      {/* Header */}
      <header className="topbar">
        <div className="topbar-brand">
          <div className="brand-icon" aria-hidden="true">🏠</div>
          <div>
            <h1>{t('app.title')}</h1>
            <p>{t('app.subtitle')}</p>
          </div>
        </div>
        <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
          <LanguageSwitcher />
          <button
            type="button"
            className="discovery-btn"
            onClick={handleRunDiscovery}
            disabled={runningDiscovery}
          >
            {runningDiscovery ? (
              <>
                <span className="spinner" aria-hidden="true" />
                {t('actions.discovering')}
              </>
            ) : (
              <>
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" aria-hidden="true">
                  <circle cx="11" cy="11" r="8" />
                  <path d="m21 21-4.35-4.35" />
                </svg>
                {t('actions.discover')}
              </>
            )}
          </button>
        </div>
      </header>

      {/* Stats */}
      <section className="stats-row" aria-label="Service statistics">
        <article className="stat-card stat-total">
          <div className="stat-icon-wrapper">
            {statIcons.total}
          </div>
          <div className="stat-content">
            <strong>{services.length}</strong>
            <span>{t('stats.total')}</span>
          </div>
        </article>
        <article className="stat-card stat-running">
          <div className="stat-icon-wrapper">
            {statIcons.running}
          </div>
          <div className="stat-content">
            <strong>{statusCounts.running}</strong>
            <span>{t('stats.running')}</span>
          </div>
        </article>
        <article className="stat-card stat-stopped">
          <div className="stat-icon-wrapper">
            {statIcons.stopped}
          </div>
          <div className="stat-content">
            <strong>{statusCounts.stopped}</strong>
            <span>{t('stats.stopped')}</span>
          </div>
        </article>
        <article className="stat-card stat-unknown">
          <div className="stat-icon-wrapper">
            {statIcons.unknown}
          </div>
          <div className="stat-content">
            <strong>{statusCounts.unknown}</strong>
            <span>{t('stats.unknown')}</span>
          </div>
        </article>
      </section>

      {/* Filters */}
      <section className="filter-bar" aria-label="Filter services">
        <div className="search-box">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" aria-hidden="true">
            <circle cx="11" cy="11" r="8" />
            <path d="m21 21-4.35-4.35" />
          </svg>
          <input
            value={query}
            placeholder={t('filters.searchPlaceholder')}
            onChange={(e) => setQuery(e.target.value)}
            aria-label={t('filters.search')}
          />
          {query && (
            <button type="button" className="clear-btn" onClick={() => setQuery('')} aria-label={t('a11y.clearSearch')}>
              ✕
            </button>
          )}
        </div>

        <select 
          value={groupFilter} 
          onChange={(e) => setGroupFilter(e.target.value)}
          aria-label={t('filters.filterByGroup')}
        >
          <option value="">{t('filters.allGroups')}</option>
          {PREDEFINED_GROUPS.map((group) => (
            <option key={group} value={group}>{group}</option>
          ))}
          {uniqueGroups.map((group) => (
            <option key={group} value={group}>{group}</option>
          ))}
        </select>

        <select
          value={statusFilter}
          onChange={(e) => setStatusFilter(e.target.value as ServiceStatus | '')}
          aria-label={t('filters.filterByStatus')}
        >
          <option value="">{t('filters.allStatus')}</option>
          <option value="running">{t('status.running')}</option>
          <option value="stopped">{t('status.stopped')}</option>
          <option value="unknown">{t('status.unknown')}</option>
        </select>

        <button 
          type="button" 
          className="refresh-btn" 
          onClick={() => void loadData()} 
          title={t('actions.refresh')}
          aria-label={t('a11y.refreshServices')}
        >
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" aria-hidden="true">
            <path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
            <path d="M3 3v5h5" />
            <path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" />
            <path d="M16 21h5v-5" />
          </svg>
        </button>
      </section>

      {/* Discovery Status */}
      {discoveryStatus && (
        <section className="discovery-note">
          <span>
            {t('discovery.lastDiscovery')}: {' '}
            {discoveryStatus.last_finished_at
              ? new Date(discoveryStatus.last_finished_at).toLocaleString()
              : t('discovery.never')}
          </span>
          <span>
            {t('discovery.scanned')}: {discoveryStatus.scanned_units} {t('discovery.units')}, {' '}
            {t('discovery.matched')}: {discoveryStatus.matched_ports} {t('discovery.ports')}, {' '}
            {t('discovery.added')}: {discoveryStatus.added}, {t('discovery.updated')}: {discoveryStatus.updated}
          </span>
        </section>
      )}

      {/* Error */}
      {error && (
        <p className="error-note" role="alert">
          {error}
        </p>
      )}

      {/* Loading */}
      {loading ? (
        <div className="loading-state">
          <div className="spinner-large" aria-hidden="true" />
          <p>Loading services...</p>
        </div>
      ) : (
        <>
          {/* Favorites */}
          {favoriteServices.length > 0 && (
            <section className="service-section" aria-label="Favorite services">
              <h2 className="section-title">
                <span aria-hidden="true">⭐</span>
                {t('sections.favorites')}
                <span className="sr-only">({favoriteServices.length} services)</span>
              </h2>
              <div className="card-grid">
                {favoriteServices.map((service) => (
                  <ServiceCard key={service.id} service={service} onEdit={setEditingService} />
                ))}
              </div>
            </section>
          )}

          {/* Regular Services */}
          {nonFavoriteServices.length > 0 && (
            <section className="service-section" aria-label="All services">
              {favoriteServices.length > 0 && (
                <h2 className="section-title">{t('sections.services')}</h2>
              )}
              <div className="card-grid">
                {nonFavoriteServices.map((service) => (
                  <ServiceCard key={service.id} service={service} onEdit={setEditingService} />
                ))}
              </div>
            </section>
          )}

          {/* Empty State */}
          {services.length === 0 && renderEmptyState()}

          {/* System Services Toggle */}
          <section className="system-services-section">
            <button 
              type="button" 
              className="system-toggle" 
              onClick={() => void toggleSystem()}
              aria-expanded={systemExpanded}
              aria-controls="system-services-grid"
            >
              <span aria-hidden="true">▶</span>
              <span>{t('sections.systemServices')}</span>
              <span className="system-count">{systemServices.length}</span>
            </button>
          </section>

          {/* System Services Grid */}
          {systemExpanded && (
            <section 
              id="system-services-grid" 
              className="card-grid"
              aria-label="System services"
            >
              {systemServices.map((service) => (
                <ServiceCard key={service.id} service={service} onEdit={setEditingService} />
              ))}
              {systemServices.length === 0 && (
                <p className="empty-note">{t('sections.noSystemServices')}</p>
              )}
            </section>
          )}
        </>
      )}

      {/* Edit Modal */}
      <EditServiceModal
        service={editingService}
        onClose={() => setEditingService(null)}
        onSave={handleSaveService}
      />

      {/* Toast Notifications */}
      <ToastContainer toasts={toasts} onRemove={removeToast} />
    </main>
  );
}
