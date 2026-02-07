import type { TranslationSchema } from '../types';

export const zh: TranslationSchema = {
  app: {
    title: 'Home Server Navigator',
    subtitle: '快速找到你的服务，告别记忆端口号',
  },
  
  actions: {
    save: '保存',
    cancel: '取消',
    edit: '编辑',
    open: '打开',
    close: '关闭',
    refresh: '刷新',
    clear: '清除',
    discover: '运行 Discovery',
    discovering: '正在发现...',
  },
  
  nav: {
    home: '首页',
  },
  
  // 专有名词保留英文
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
    total: '总计',
    totalServices: '服务总数',
    running: '运行中',
    runningServices: '运行中服务',
    stopped: '已停止',
    stoppedServices: '已停止服务',
    unknown: '未知',
  },
  
  filters: {
    search: '搜索',
    searchPlaceholder: '搜索名称、标签、分组、端口...',
    allGroups: '全部分组',
    allStatus: '全部状态',
    filterByGroup: '按分组筛选',
    filterByStatus: '按状态筛选',
  },
  
  sections: {
    favorites: '收藏夹',
    services: '服务列表',
    systemServices: '系统服务',
    noSystemServices: '暂无系统服务',
  },
  
  discovery: {
    title: '服务发现',
    lastDiscovery: '上次发现',
    never: '从未',
    scanned: '扫描了',
    units: '个 unit',
    matched: '匹配到',
    ports: '个端口',
    added: '新增',
    updated: '更新',
  },
  
  empty: {
    noServices: '暂无服务',
    noServicesDescription: '运行 Discovery 来自动发现并添加你的服务',
    noResults: '未找到服务',
    noResultsDescription: '尝试调整搜索条件或筛选器',
    noUrl: '未配置 URL',
  },
  
  modal: {
    editService: '编辑服务',
    displayName: '显示名称',
    description: '描述',
    host: '主机',
    port: '端口',
    protocol: '协议',
    path: '路径',
    urlOverride: 'URL（覆盖）',
    group: '分组',
    tags: '标签',
    tagsPlaceholder: 'dashboard, internal（用逗号分隔）',
    icon: '图标',
    iconPlaceholder: 'Emoji 图标',
    favorite: '收藏',
    addToFavorites: '添加到收藏夹',
    hidden: '隐藏',
    hideFromView: '从主视图隐藏',
    previewUrl: '预览 URL',
    lockedFields: '锁定字段',
    lockedFieldsDescription: '防止 Discovery 自动覆盖这些字段',
    saving: '保存中...',
  },
  
  toast: {
    discoveryCompleted: 'Discovery 完成',
    discoveryCompletedMsg: '服务列表已更新',
    discoveryFailed: 'Discovery 失败',
    loadFailed: '加载失败',
    loadServicesFailed: '加载服务列表失败',
    loadSystemFailed: '加载系统服务失败',
    serviceUpdated: '服务已更新',
    saveFailed: '保存失败',
  },
  
  a11y: {
    closeDialog: '关闭对话框',
    clearSearch: '清除搜索',
    refreshServices: '刷新服务列表',
    serviceStatus: '状态',
    favoriteBadge: '已收藏',
  },
};
