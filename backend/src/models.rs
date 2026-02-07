use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ServiceProtocol {
    Http,
    Https,
    Tcp,
    #[default]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ServiceStatus {
    Running,
    Stopped,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ServiceSource {
    Auto,
    Manual,
    #[default]
    Merged,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServiceEntry {
    pub id: String,
    pub service_name: String,
    pub display_name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub host: String,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub protocol: ServiceProtocol,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub status: ServiceStatus,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub favorite: bool,
    #[serde(default)]
    pub source: ServiceSource,
    #[serde(default)]
    pub locked_fields: Vec<String>,
    #[serde(default)]
    pub last_seen_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

impl ServiceEntry {
    #[allow(dead_code)]
    pub fn resolved_url(&self) -> Option<String> {
        if let Some(url) = &self.url {
            return Some(url.clone());
        }
        build_service_url(&self.protocol, &self.host, self.port, self.path.as_deref())
    }

    pub fn is_locked(&self, field: &str) -> bool {
        self.locked_fields.iter().any(|value| value == field)
    }

    pub fn lock_field(&mut self, field: &str) {
        if !self.is_locked(field) {
            self.locked_fields.push(field.to_string());
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateServiceRequest {
    pub service_name: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub protocol: Option<ServiceProtocol>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub hidden: Option<bool>,
    #[serde(default)]
    pub favorite: Option<bool>,
    #[serde(default)]
    pub locked_fields: Option<Vec<String>>,
}

impl CreateServiceRequest {
    pub fn into_entry(self, default_host: &str) -> ServiceEntry {
        let now = Utc::now();
        let service_name = self.service_name.trim().to_string();
        let mut entry = ServiceEntry {
            id: service_id(&service_name),
            display_name: self
                .display_name
                .unwrap_or_else(|| humanize_service_name(&service_name)),
            service_name,
            description: self.description,
            host: self
                .host
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| default_host.to_string()),
            port: self.port,
            protocol: self
                .protocol
                .unwrap_or_else(|| infer_protocol_from_port(self.port)),
            path: clean_optional(self.path),
            url: clean_optional(self.url),
            status: ServiceStatus::Unknown,
            group: clean_optional(self.group),
            tags: self.tags.unwrap_or_default(),
            icon: clean_optional(self.icon),
            hidden: self.hidden.unwrap_or(false),
            favorite: self.favorite.unwrap_or(false),
            source: ServiceSource::Manual,
            locked_fields: self.locked_fields.unwrap_or_else(default_locked_fields),
            last_seen_at: None,
            updated_at: now,
        };
        normalize_locked_fields(&mut entry.locked_fields);
        entry
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateServiceRequest {
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub description: Option<Option<String>>,
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub port: Option<Option<u16>>,
    #[serde(default)]
    pub protocol: Option<ServiceProtocol>,
    #[serde(default)]
    pub path: Option<Option<String>>,
    #[serde(default)]
    pub url: Option<Option<String>>,
    #[serde(default)]
    pub status: Option<ServiceStatus>,
    #[serde(default)]
    pub group: Option<Option<String>>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub icon: Option<Option<String>>,
    #[serde(default)]
    pub hidden: Option<bool>,
    #[serde(default)]
    pub favorite: Option<bool>,
    #[serde(default)]
    pub locked_fields: Option<Vec<String>>,
    #[serde(default)]
    pub auto_lock: Option<bool>,
}

impl UpdateServiceRequest {
    #[allow(dead_code)]
    pub fn auto_lock_enabled(&self) -> bool {
        self.auto_lock.unwrap_or(true)
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ServiceQuery {
    #[serde(default)]
    pub q: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub status: Option<ServiceStatus>,
    #[serde(default)]
    pub include_hidden: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscoveryStatusInfo {
    pub last_started_at: Option<DateTime<Utc>>,
    pub last_finished_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub scanned_units: usize,
    pub active_units: usize,
    pub matched_ports: usize,
    pub discovered_services: usize,
    pub added: usize,
    pub updated: usize,
    pub unchanged: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiscoveryRunResponse {
    pub summary: DiscoveryStatusInfo,
}

pub fn service_id(service_name: &str) -> String {
    let mut output = String::with_capacity(service_name.len());
    let mut previous_dash = false;
    for character in service_name.chars().flat_map(|value| value.to_lowercase()) {
        if character.is_ascii_alphanumeric() {
            output.push(character);
            previous_dash = false;
        } else if !previous_dash {
            output.push('-');
            previous_dash = true;
        }
    }
    output.trim_matches('-').to_string()
}

pub fn infer_protocol_from_port(port: Option<u16>) -> ServiceProtocol {
    match port {
        Some(443 | 8443) => ServiceProtocol::Https,
        Some(80 | 3000 | 5000 | 8080 | 8096 | 9000) => ServiceProtocol::Http,
        Some(_) => ServiceProtocol::Tcp,
        None => ServiceProtocol::Other,
    }
}

pub fn build_service_url(
    protocol: &ServiceProtocol,
    host: &str,
    port: Option<u16>,
    path: Option<&str>,
) -> Option<String> {
    let scheme = match protocol {
        ServiceProtocol::Http => "http",
        ServiceProtocol::Https => "https",
        _ => return None,
    };
    let port = port?;
    let mut url = format!("{scheme}://{host}:{port}");
    if let Some(path) = path.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else if trimmed.starts_with('/') {
            Some(trimmed.to_string())
        } else {
            Some(format!("/{trimmed}"))
        }
    }) {
        url.push_str(&path);
    }
    Some(url)
}

pub fn humanize_service_name(service_name: &str) -> String {
    service_name
        .trim_end_matches(".service")
        .split(['-', '_', '.'])
        .filter(|value| !value.is_empty())
        .map(capitalize_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn capitalize_word(value: &str) -> String {
    let mut chars = value.chars();
    let first = chars
        .next()
        .map(|character| character.to_uppercase().collect::<String>())
        .unwrap_or_default();
    format!("{first}{}", chars.as_str())
}

pub fn default_locked_fields() -> Vec<String> {
    vec![
        "display_name".to_string(),
        "host".to_string(),
        "port".to_string(),
        "protocol".to_string(),
        "path".to_string(),
        "url".to_string(),
        "group".to_string(),
        "tags".to_string(),
        "icon".to_string(),
        "description".to_string(),
        "hidden".to_string(),
        "favorite".to_string(),
    ]
}

pub fn normalize_locked_fields(values: &mut Vec<String>) {
    values.retain(|value| !value.trim().is_empty());
    values.sort();
    values.dedup();
}

fn clean_optional(value: Option<String>) -> Option<String> {
    value.and_then(|inner| {
        let trimmed = inner.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}
