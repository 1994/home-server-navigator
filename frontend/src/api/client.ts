import type {
  DiscoveryRunResponse,
  DiscoveryStatusInfo,
  ServiceEntry,
  UpdateServiceRequest,
} from '../types';

const jsonHeaders = {
  'Content-Type': 'application/json',
};

export async function fetchServices(params?: {
  q?: string;
  group?: string;
  status?: string;
  include_hidden?: boolean;
}): Promise<ServiceEntry[]> {
  const query = new URLSearchParams();
  if (params?.q) {
    query.set('q', params.q);
  }
  if (params?.group) {
    query.set('group', params.group);
  }
  if (params?.status) {
    query.set('status', params.status);
  }
  if (params?.include_hidden) {
    query.set('include_hidden', 'true');
  }
  const suffix = query.toString() ? `?${query.toString()}` : '';
  const response = await fetch(`/api/services${suffix}`);
  if (!response.ok) {
    throw new Error('Failed to fetch services');
  }
  return response.json();
}

export async function updateService(id: string, payload: UpdateServiceRequest): Promise<ServiceEntry> {
  const response = await fetch(`/api/services/${id}`, {
    method: 'PATCH',
    headers: jsonHeaders,
    body: JSON.stringify(payload),
  });

  if (!response.ok) {
    throw new Error('Failed to update service');
  }
  return response.json();
}

export async function runDiscovery(): Promise<DiscoveryRunResponse> {
  const response = await fetch('/api/discovery/run', {
    method: 'POST',
  });
  if (!response.ok) {
    throw new Error('Failed to run discovery');
  }
  return response.json();
}

export async function fetchDiscoveryStatus(): Promise<DiscoveryStatusInfo> {
  const response = await fetch('/api/discovery/status');
  if (!response.ok) {
    throw new Error('Failed to fetch discovery status');
  }
  return response.json();
}
