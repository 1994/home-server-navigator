import type { TranslationSchema } from '../types';

export const en: TranslationSchema = {
  app: {
    title: 'Home Server Navigator',
    subtitle: 'Find your services quickly and stop memorizing ports',
  },
  
  actions: {
    save: 'Save',
    cancel: 'Cancel',
    edit: 'Edit',
    open: 'Open',
    close: 'Close',
    refresh: 'Refresh',
    clear: 'Clear',
    discover: 'Run Discovery',
    discovering: 'Discovering...',
  },
  
  nav: {
    home: 'Home',
  },
  
  // Keep technical terms in English
  status: {
    running: 'Running',
    stopped: 'Stopped',
    unknown: 'Unknown',
  },
  
  protocol: {
    http: 'HTTP',
    https: 'HTTPS',
    tcp: 'TCP',
    other: 'Other',
  },
  
  stats: {
    total: 'Total',
    totalServices: 'Total services',
    running: 'Running',
    runningServices: 'Running services',
    stopped: 'Stopped',
    stoppedServices: 'Stopped services',
    unknown: 'Unknown',
  },
  
  filters: {
    search: 'Search',
    searchPlaceholder: 'Search name, tag, group, port...',
    allGroups: 'All Groups',
    allStatus: 'All Status',
    filterByGroup: 'Filter by group',
    filterByStatus: 'Filter by status',
  },
  
  sections: {
    favorites: 'Favorites',
    services: 'Services',
    systemServices: 'System Services',
    noSystemServices: 'No system services found',
  },
  
  discovery: {
    title: 'Discovery',
    lastDiscovery: 'Last discovery',
    never: 'Never',
    scanned: 'Scanned',
    units: 'units',
    matched: 'matched',
    ports: 'ports',
    added: 'added',
    updated: 'updated',
  },
  
  empty: {
    noServices: 'No services yet',
    noServicesDescription: 'Run discovery to automatically find and add your services',
    noResults: 'No services found',
    noResultsDescription: 'Try adjusting your search or filters',
    noUrl: 'No URL configured',
  },
  
  modal: {
    editService: 'Edit Service',
    displayName: 'Display Name',
    description: 'Description',
    host: 'Host',
    port: 'Port',
    protocol: 'Protocol',
    path: 'Path',
    urlOverride: 'URL (override)',
    group: 'Group',
    tags: 'Tags',
    tagsPlaceholder: 'dashboard, internal (comma-separated)',
    icon: 'Icon',
    iconPlaceholder: 'Emoji icon',
    favorite: 'Favorite',
    addToFavorites: 'Add to favorites',
    hidden: 'Hidden',
    hideFromView: 'Hide from main view',
    previewUrl: 'Preview URL',
    lockedFields: 'Locked Fields',
    lockedFieldsDescription: 'Prevent auto-discovery from overwriting these fields',
    saving: 'Saving...',
  },
  
  toast: {
    discoveryCompleted: 'Discovery completed',
    discoveryCompletedMsg: 'Services have been updated',
    discoveryFailed: 'Discovery failed',
    loadFailed: 'Failed to load',
    loadServicesFailed: 'Failed to load services',
    loadSystemFailed: 'Failed to load system services',
    serviceUpdated: 'Service updated',
    saveFailed: 'Save failed',
  },
  
  a11y: {
    closeDialog: 'Close dialog',
    clearSearch: 'Clear search',
    refreshServices: 'Refresh services',
    serviceStatus: 'Status',
    favoriteBadge: 'Favorited',
  },
};
