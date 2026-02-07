use crate::{
    discovery::DiscoveryEngine,
    models::{
        default_locked_fields, normalize_locked_fields, CreateServiceRequest, DiscoveryStatusInfo,
        ServiceEntry, ServiceQuery, UpdateServiceRequest,
    },
    store::ServiceStore,
};
use anyhow::Result;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub default_host: String,
    pub store: Arc<ServiceStore>,
    pub discovery: Arc<DiscoveryEngine>,
    pub services: Arc<RwLock<Vec<ServiceEntry>>>,
    pub discovery_status: Arc<RwLock<DiscoveryStatusInfo>>,
}

impl AppState {
    pub async fn new(default_host: String, data_file: String) -> Result<Self> {
        let store = Arc::new(ServiceStore::new(data_file));
        let mut services = store.load_services().await?;
        services.sort_by(|left, right| left.display_name.cmp(&right.display_name));

        Ok(Self {
            default_host: default_host.clone(),
            discovery: Arc::new(DiscoveryEngine::new(default_host)),
            store,
            services: Arc::new(RwLock::new(services)),
            discovery_status: Arc::new(RwLock::new(DiscoveryStatusInfo::default())),
        })
    }

    pub async fn list_services(&self, query: ServiceQuery) -> Vec<ServiceEntry> {
        let include_hidden = query.include_hidden.unwrap_or(false);
        let services = self.services.read().await;
        services
            .iter()
            .filter(|entry| if include_hidden { true } else { !entry.hidden })
            .filter(|entry| matches_query(entry, &query))
            .cloned()
            .collect()
    }

    pub async fn get_service(&self, id: &str) -> Option<ServiceEntry> {
        let services = self.services.read().await;
        services.iter().find(|entry| entry.id == id).cloned()
    }

    pub async fn create_service(&self, request: CreateServiceRequest) -> Result<ServiceEntry> {
        let mut entry = request.into_entry(&self.default_host);
        let mut services = self.services.write().await;

        if services.iter().any(|value| value.id == entry.id) {
            entry.id = format!("{}-{}", entry.id, Utc::now().timestamp());
        }

        services.push(entry.clone());
        services.sort_by(|left, right| left.display_name.cmp(&right.display_name));
        self.store.save_services(&services).await?;
        Ok(entry)
    }

    pub async fn update_service(
        &self,
        id: &str,
        patch: UpdateServiceRequest,
    ) -> Result<Option<ServiceEntry>> {
        let UpdateServiceRequest {
            display_name,
            description,
            host,
            port,
            protocol,
            path,
            url,
            status,
            group,
            tags,
            icon,
            hidden,
            favorite,
            locked_fields,
            auto_lock,
        } = patch;
        let auto_lock_enabled = auto_lock.unwrap_or(true);

        let mut services = self.services.write().await;
        let Some(existing) = services.iter_mut().find(|entry| entry.id == id) else {
            return Ok(None);
        };

        let mut touched_locked = false;
        if let Some(display_name) = display_name {
            if !display_name.trim().is_empty() {
                existing.display_name = display_name;
                touched_locked = true;
                if auto_lock_enabled {
                    existing.lock_field("display_name");
                }
            }
        }
        if let Some(description) = description {
            existing.description = description;
            touched_locked = true;
            if auto_lock_enabled {
                existing.lock_field("description");
            }
        }
        if let Some(host) = host {
            if !host.trim().is_empty() {
                existing.host = host;
                touched_locked = true;
                if auto_lock_enabled {
                    existing.lock_field("host");
                }
            }
        }
        if let Some(port) = port {
            existing.port = port;
            touched_locked = true;
            if auto_lock_enabled {
                existing.lock_field("port");
            }
        }
        if let Some(protocol) = protocol {
            existing.protocol = protocol;
            touched_locked = true;
            if auto_lock_enabled {
                existing.lock_field("protocol");
            }
        }
        if let Some(path) = path {
            existing.path = path;
            touched_locked = true;
            if auto_lock_enabled {
                existing.lock_field("path");
            }
        }
        if let Some(url) = url {
            existing.url = url;
            touched_locked = true;
            if auto_lock_enabled {
                existing.lock_field("url");
            }
        }
        if let Some(status) = status {
            existing.status = status;
        }
        if let Some(group) = group {
            existing.group = group;
            touched_locked = true;
            if auto_lock_enabled {
                existing.lock_field("group");
            }
        }
        if let Some(tags) = tags {
            existing.tags = tags;
            touched_locked = true;
            if auto_lock_enabled {
                existing.lock_field("tags");
            }
        }
        if let Some(icon) = icon {
            existing.icon = icon;
            touched_locked = true;
            if auto_lock_enabled {
                existing.lock_field("icon");
            }
        }
        if let Some(hidden) = hidden {
            existing.hidden = hidden;
            touched_locked = true;
            if auto_lock_enabled {
                existing.lock_field("hidden");
            }
        }
        if let Some(favorite) = favorite {
            existing.favorite = favorite;
            touched_locked = true;
            if auto_lock_enabled {
                existing.lock_field("favorite");
            }
        }
        if let Some(mut locked_fields) = locked_fields {
            normalize_locked_fields(&mut locked_fields);
            existing.locked_fields = locked_fields;
        } else if touched_locked && existing.locked_fields.is_empty() {
            existing.locked_fields = default_locked_fields();
        }

        existing.updated_at = Utc::now();
        let updated = existing.clone();
        services.sort_by(|left, right| left.display_name.cmp(&right.display_name));
        self.store.save_services(&services).await?;
        Ok(Some(updated))
    }

    pub async fn run_discovery(&self) -> Result<DiscoveryStatusInfo> {
        let (discovered, summary) = self.discovery.discover().await?;

        let merged = {
            let services = self.services.read().await;
            crate::discovery::merge_services(&services, &discovered, summary)
        };

        {
            let mut services = self.services.write().await;
            *services = merged.0.clone();
            self.store.save_services(&services).await?;
        }

        {
            let mut status = self.discovery_status.write().await;
            *status = merged.1.clone();
        }

        Ok(merged.1)
    }

    pub async fn discovery_status(&self) -> DiscoveryStatusInfo {
        self.discovery_status.read().await.clone()
    }
}

fn matches_query(entry: &ServiceEntry, query: &ServiceQuery) -> bool {
    if let Some(group) = &query.group {
        if entry.group.as_deref().unwrap_or_default() != group {
            return false;
        }
    }

    if let Some(status) = &query.status {
        if &entry.status != status {
            return false;
        }
    }

    if let Some(q) = &query.q {
        let needle = q.to_lowercase();
        if !entry.display_name.to_lowercase().contains(&needle)
            && !entry.service_name.to_lowercase().contains(&needle)
            && !entry
                .tags
                .iter()
                .any(|tag| tag.to_lowercase().contains(&needle))
            && !entry
                .group
                .as_ref()
                .map(|group| group.to_lowercase().contains(&needle))
                .unwrap_or(false)
            && !entry
                .port
                .map(|port| port.to_string().contains(&needle))
                .unwrap_or(false)
        {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ServiceProtocol, ServiceSource};

    #[test]
    fn query_filters_by_text() {
        let entry = ServiceEntry {
            id: "grafana-service".to_string(),
            service_name: "grafana.service".to_string(),
            display_name: "Grafana".to_string(),
            description: None,
            host: "server.local".to_string(),
            port: Some(3000),
            protocol: ServiceProtocol::Http,
            path: None,
            url: None,
            status: ServiceStatus::Running,
            group: Some("monitoring".to_string()),
            tags: vec!["dashboard".to_string()],
            icon: None,
            hidden: false,
            favorite: false,
            source: ServiceSource::Manual,
            locked_fields: Vec::new(),
            last_seen_at: None,
            updated_at: Utc::now(),
        };

        assert!(matches_query(
            &entry,
            &ServiceQuery {
                q: Some("graf".to_string()),
                ..Default::default()
            }
        ));
        assert!(!matches_query(
            &entry,
            &ServiceQuery {
                q: Some("jellyfin".to_string()),
                ..Default::default()
            }
        ));
    }
}
