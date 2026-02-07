export type ServiceProtocol = 'http' | 'https' | 'tcp' | 'other';
export type ServiceStatus = 'running' | 'stopped' | 'unknown';
export type ServiceSource = 'auto' | 'manual' | 'merged';

export interface ServiceEntry {
  id: string;
  service_name: string;
  display_name: string;
  description?: string;
  host: string;
  port?: number;
  protocol: ServiceProtocol;
  path?: string;
  url?: string;
  status: ServiceStatus;
  group?: string;
  tags: string[];
  icon?: string;
  hidden: boolean;
  favorite: boolean;
  source: ServiceSource;
  locked_fields: string[];
  last_seen_at?: string;
  updated_at: string;
}

export interface UpdateServiceRequest {
  display_name?: string;
  description?: string | null;
  host?: string;
  port?: number | null;
  protocol?: ServiceProtocol;
  path?: string | null;
  url?: string | null;
  status?: ServiceStatus;
  group?: string | null;
  tags?: string[];
  icon?: string | null;
  hidden?: boolean;
  favorite?: boolean;
  locked_fields?: string[];
  auto_lock?: boolean;
}

export interface DiscoveryStatusInfo {
  last_started_at?: string;
  last_finished_at?: string;
  last_error?: string;
  scanned_units: number;
  active_units: number;
  matched_ports: number;
  discovered_services: number;
  added: number;
  updated: number;
  unchanged: number;
}

export interface DiscoveryRunResponse {
  summary: DiscoveryStatusInfo;
}
