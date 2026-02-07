// i18n types

export type Locale = 'en' | 'zh';

export interface TranslationSchema {
  // Common
  app: {
    title: string;
    subtitle: string;
  };
  
  // Actions
  actions: {
    save: string;
    cancel: string;
    edit: string;
    open: string;
    close: string;
    refresh: string;
    clear: string;
    discover: string;
    discovering: string;
  };
  
  // Navigation
  nav: {
    home: string;
  };
  
  // Service statuses - keep English terms
  status: {
    running: string;
    stopped: string;
    unknown: string;
  };
  
  // Protocols - keep English terms
  protocol: {
    http: string;
    https: string;
    tcp: string;
    other: string;
  };
  
  // Stats
  stats: {
    total: string;
    totalServices: string;
    running: string;
    runningServices: string;
    stopped: string;
    stoppedServices: string;
    unknown: string;
  };
  
  // Filters
  filters: {
    search: string;
    searchPlaceholder: string;
    allGroups: string;
    allStatus: string;
    filterByGroup: string;
    filterByStatus: string;
  };
  
  // Service sections
  sections: {
    favorites: string;
    services: string;
    systemServices: string;
    noSystemServices: string;
  };
  
  // Discovery
  discovery: {
    title: string;
    lastDiscovery: string;
    never: string;
    scanned: string;
    units: string;
    matched: string;
    ports: string;
    added: string;
    updated: string;
  };
  
  // Empty states
  empty: {
    noServices: string;
    noServicesDescription: string;
    noResults: string;
    noResultsDescription: string;
    noUrl: string;
  };
  
  // Modal
  modal: {
    editService: string;
    displayName: string;
    description: string;
    host: string;
    port: string;
    protocol: string;
    path: string;
    urlOverride: string;
    group: string;
    tags: string;
    tagsPlaceholder: string;
    icon: string;
    iconPlaceholder: string;
    favorite: string;
    addToFavorites: string;
    hidden: string;
    hideFromView: string;
    previewUrl: string;
    lockedFields: string;
    lockedFieldsDescription: string;
    saving: string;
  };
  
  // Toast messages
  toast: {
    discoveryCompleted: string;
    discoveryCompletedMsg: string;
    discoveryFailed: string;
    loadFailed: string;
    loadServicesFailed: string;
    loadSystemFailed: string;
    serviceUpdated: string;
    saveFailed: string;
  };
  
  // Accessibility
  a11y: {
    closeDialog: string;
    clearSearch: string;
    refreshServices: string;
    serviceStatus: string;
    favoriteBadge: string;
  };
}

export type TranslationKey = keyof TranslationSchema;
