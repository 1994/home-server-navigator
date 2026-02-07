use crate::models::{
    humanize_service_name, service_id, DiscoveryStatusInfo, ServiceEntry, ServiceProtocol,
    ServiceSource, ServiceStatus,
};
use anyhow::Result;
use chrono::Utc;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use tokio::process::Command;

const GROUP_SYSTEM: &str = "系统";
const GROUP_MEDIA: &str = "影音";
const GROUP_DOWNLOADS: &str = "下载";
const GROUP_SYNC: &str = "同步";
const GROUP_PHOTOS: &str = "照片";
const GROUP_MONITORING: &str = "监控";
const GROUP_OTHER: &str = "其他";

const HTTP_DETECTION_TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Debug, Clone)]
pub struct DiscoveryEngine {
    default_host: String,
    http_client: reqwest::Client,
}

impl DiscoveryEngine {
    pub fn new(default_host: impl Into<String>) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(HTTP_DETECTION_TIMEOUT)
            .danger_accept_invalid_certs(true) // Self-signed certs are common in home servers
            .redirect(reqwest::redirect::Policy::limited(3))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            default_host: default_host.into(),
            http_client,
        }
    }

    pub async fn discover(&self) -> Result<(Vec<ServiceEntry>, DiscoveryStatusInfo)> {
        let mut summary = DiscoveryStatusInfo {
            last_started_at: Some(Utc::now()),
            ..Default::default()
        };

        let units = list_systemd_services().await?;
        summary.scanned_units = units.len();
        summary.active_units = units
            .values()
            .filter(|value| **value == ServiceStatus::Running)
            .count();

        let listen_map = collect_listen_ports().await?;
        summary.matched_ports = listen_map.values().map(std::vec::Vec::len).sum();

        // Build service candidates and detect protocols concurrently
        let mut detection_tasks = Vec::new();

        for (unit, status) in units {
            let cleaned_name = unit.trim().to_string();
            let key = cleaned_name.trim_end_matches(".service").to_lowercase();
            let ports = listen_map
                .iter()
                .find_map(|(process, ports)| {
                    if process.contains(&key) || key.contains(process) {
                        Some(ports.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or_default();

            let primary_port = select_primary_port(&ports);
            let host = self.default_host.clone();
            let client = self.http_client.clone();

            // Create detection task for each service
            let detection = async move {
                let protocol = if let Some(port) = primary_port {
                    detect_protocol(&client, &host, port).await
                } else {
                    ServiceProtocol::Other
                };
                (cleaned_name, status, primary_port, protocol)
            };
            detection_tasks.push(detection);
        }

        // Run all detections concurrently
        let detection_results = futures::future::join_all(detection_tasks).await;

        // Build service entries from results
        let mut discovered = Vec::new();
        for (unit, status, primary_port, protocol) in detection_results {
            discovered.push(ServiceEntry {
                id: service_id(&unit),
                service_name: unit.clone(),
                display_name: humanize_service_name(&unit),
                description: None,
                host: self.default_host.clone(),
                port: primary_port,
                protocol,
                path: None,
                url: None,
                status,
                group: None,
                tags: Vec::new(),
                icon: None,
                hidden: false,
                favorite: false,
                source: ServiceSource::Auto,
                locked_fields: Vec::new(),
                last_seen_at: Some(Utc::now()),
                updated_at: Utc::now(),
            });
        }

        for entry in &mut discovered {
            classify_service(entry);
        }

        summary.discovered_services = discovered.len();
        summary.last_finished_at = Some(Utc::now());
        Ok((discovered, summary))
    }
}

/// Detect whether a port serves HTTP, HTTPS, or TCP by making actual requests
async fn detect_protocol(client: &reqwest::Client, host: &str, port: u16) -> ServiceProtocol {
    // First try HTTPS (common for home servers with self-signed certs)
    let https_url = format!("https://{}:{}", host, port);
    if let Ok(true) = is_http_response(client, &https_url).await {
        return ServiceProtocol::Https;
    }

    // Then try HTTP
    let http_url = format!("http://{}:{}", host, port);
    if let Ok(true) = is_http_response(client, &http_url).await {
        return ServiceProtocol::Http;
    }

    // Not HTTP/HTTPS, treat as TCP
    ServiceProtocol::Tcp
}

/// Check if the URL returns a valid HTTP response
async fn is_http_response(client: &reqwest::Client, url: &str) -> Result<bool> {
    match client.get(url).send().await {
        Ok(response) => {
            // Check if it looks like an HTTP response
            let status = response.status();
            // Any valid HTTP status code (2xx, 3xx, 4xx, 5xx) indicates it's HTTP
            Ok(status.as_u16() > 0 && status.as_u16() < 600)
        }
        Err(e) => {
            // Check if it's an HTTP protocol error (not connection refused)
            // Connection refused means nothing is listening
            // Other errors might indicate HTTP is working but with issues
            let err_str = e.to_string().to_lowercase();
            if err_str.contains("connection refused")
                || err_str.contains("timeout")
                || err_str.contains("unable to connect")
            {
                Ok(false)
            } else {
                // Likely an HTTP protocol error (cert issue, etc.)
                Ok(true)
            }
        }
    }
}

pub fn merge_services(
    current: &[ServiceEntry],
    discovered: &[ServiceEntry],
    mut summary: DiscoveryStatusInfo,
) -> (Vec<ServiceEntry>, DiscoveryStatusInfo) {
    let mut current_map: HashMap<String, ServiceEntry> = current
        .iter()
        .map(|value| (value.id.clone(), value.clone()))
        .collect();

    let mut merged_ids = HashSet::new();

    for auto in discovered {
        merged_ids.insert(auto.id.clone());
        if let Some(existing) = current_map.get_mut(&auto.id) {
            let before = existing.clone();
            merge_single(existing, auto);
            if *existing == before {
                summary.unchanged += 1;
            } else {
                summary.updated += 1;
            }
        } else {
            current_map.insert(auto.id.clone(), auto.clone());
            summary.added += 1;
        }
    }

    for service in current_map.values_mut() {
        if !merged_ids.contains(&service.id) {
            service.status = ServiceStatus::Unknown;
            service.updated_at = Utc::now();
        }
    }

    let mut merged: Vec<ServiceEntry> = current_map.into_values().collect();
    merged.sort_by(|left, right| left.display_name.cmp(&right.display_name));
    (merged, summary)
}

fn merge_single(existing: &mut ServiceEntry, discovered: &ServiceEntry) {
    if !existing.is_locked("service_name") {
        existing.service_name = discovered.service_name.clone();
    }
    if !existing.is_locked("display_name") {
        existing.display_name = discovered.display_name.clone();
    }
    if !existing.is_locked("host") {
        existing.host = discovered.host.clone();
    }
    if !existing.is_locked("port") {
        existing.port = discovered.port;
    }
    if !existing.is_locked("protocol") {
        existing.protocol = discovered.protocol.clone();
    }
    if !existing.is_locked("path") {
        existing.path = discovered.path.clone();
    }
    if !existing.is_locked("url") {
        existing.url = discovered.url.clone();
    }
    if !existing.is_locked("status") {
        existing.status = discovered.status.clone();
    }
    if !existing.is_locked("description") {
        existing.description = discovered.description.clone();
    }
    if !existing.is_locked("hidden") {
        existing.hidden = discovered.hidden;
    }
    if !existing.is_locked("favorite") {
        existing.favorite = discovered.favorite;
    }
    existing.last_seen_at = discovered.last_seen_at;
    existing.source = ServiceSource::Merged;
    existing.updated_at = Utc::now();
}

fn classify_service(entry: &mut ServiceEntry) {
    let unit = entry
        .service_name
        .trim_end_matches(".service")
        .to_lowercase();

    if entry.port.is_none() {
        if entry.group.is_none() {
            entry.group = Some(GROUP_SYSTEM.to_string());
        }
        entry.hidden = true;
        return;
    }

    if entry.group.is_some() {
        entry.hidden = false;
        return;
    }

    if contains_any(&unit, &["syncthing"]) {
        entry.group = Some(GROUP_SYNC.to_string());
        entry.icon = entry.icon.clone().or_else(|| Some("🔄".to_string()));
    } else if contains_any(&unit, &["immich"]) {
        entry.group = Some(GROUP_PHOTOS.to_string());
        entry.icon = entry.icon.clone().or_else(|| Some("📷".to_string()));
    } else if contains_any(&unit, &["aria2", "ariang", "qbittorrent", "transmission"]) {
        entry.group = Some(GROUP_DOWNLOADS.to_string());
        entry.icon = entry.icon.clone().or_else(|| Some("⬇️".to_string()));
    } else if contains_any(&unit, &["jellyfin", "plex", "emby"]) {
        entry.group = Some(GROUP_MEDIA.to_string());
        entry.icon = entry.icon.clone().or_else(|| Some("🎬".to_string()));
    } else if contains_any(&unit, &["grafana", "prometheus", "loki"]) {
        entry.group = Some(GROUP_MONITORING.to_string());
        entry.icon = entry.icon.clone().or_else(|| Some("📈".to_string()));
    } else if contains_any(&unit, &["nginx", "caddy", "traefik"]) {
        entry.group = Some(GROUP_SYSTEM.to_string());
        entry.icon = entry.icon.clone().or_else(|| Some("🌐".to_string()));
    } else {
        entry.group = Some(GROUP_OTHER.to_string());
    }

    entry.hidden = entry.group.as_deref() == Some(GROUP_SYSTEM);
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

fn select_primary_port(ports: &[u16]) -> Option<u16> {
    if ports.is_empty() {
        return None;
    }

    let preferred = [443, 80, 8443, 8080, 3000, 8096, 9000];
    for value in preferred {
        if ports.contains(&value) {
            return Some(value);
        }
    }
    ports.iter().copied().min()
}

async fn list_systemd_services() -> Result<HashMap<String, ServiceStatus>> {
    let output = Command::new("systemctl")
        .args([
            "list-units",
            "--type=service",
            "--all",
            "--no-legend",
            "--no-pager",
        ])
        .output()
        .await?;

    if !output.status.success() {
        return Ok(HashMap::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut units = HashMap::new();
    for line in stdout.lines() {
        let mut fields = line.split_whitespace();
        let Some(unit_name) = fields.next() else {
            continue;
        };
        if !unit_name.ends_with(".service") {
            continue;
        }
        let status = if line.contains(" running ") {
            ServiceStatus::Running
        } else if line.contains(" exited ") || line.contains(" dead ") {
            ServiceStatus::Stopped
        } else {
            ServiceStatus::Unknown
        };
        units.insert(unit_name.to_string(), status);
    }

    Ok(units)
}

async fn collect_listen_ports() -> Result<HashMap<String, Vec<u16>>> {
    let output = Command::new("ss").args(["-ltnp"]).output().await?;

    if !output.status.success() {
        return Ok(HashMap::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let process_regex = Regex::new(r#"users:\(\("([^"]+)""#).ok();

    let mut map: HashMap<String, Vec<u16>> = HashMap::new();
    for line in stdout.lines().skip(1) {
        let Some(port) = parse_port(line) else {
            continue;
        };
        let process = process_regex
            .as_ref()
            .and_then(|regex| regex.captures(line))
            .and_then(|capture| capture.get(1).map(|value| value.as_str().to_lowercase()))
            .unwrap_or_else(|| "unknown".to_string());
        map.entry(process).or_default().push(port);
    }

    for ports in map.values_mut() {
        ports.sort_unstable();
        ports.dedup();
    }

    Ok(map)
}

fn parse_port(line: &str) -> Option<u16> {
    let fields = line.split_whitespace().collect::<Vec<_>>();
    if fields.len() < 5 {
        return None;
    }
    let local_addr = fields[3];
    local_addr
        .rsplit(':')
        .next()
        .and_then(|value| value.parse::<u16>().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ServiceProtocol, ServiceSource, ServiceStatus};

    fn base_service() -> ServiceEntry {
        ServiceEntry {
            id: "nginx-service".to_string(),
            service_name: "nginx.service".to_string(),
            display_name: "Nginx".to_string(),
            description: None,
            host: "server.local".to_string(),
            port: Some(80),
            protocol: ServiceProtocol::Http,
            path: None,
            url: None,
            status: ServiceStatus::Running,
            group: Some("proxy".to_string()),
            tags: vec!["gateway".to_string()],
            icon: Some("🌐".to_string()),
            hidden: false,
            favorite: false,
            source: ServiceSource::Manual,
            locked_fields: vec!["display_name".to_string(), "port".to_string()],
            last_seen_at: None,
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn merge_respects_locked_fields() {
        let existing = base_service();
        let mut discovered = base_service();
        discovered.display_name = "Auto Nginx".to_string();
        discovered.port = Some(8080);
        discovered.status = ServiceStatus::Stopped;
        discovered.source = ServiceSource::Auto;

        let (merged, summary) =
            merge_services(&[existing], &[discovered], DiscoveryStatusInfo::default());
        let item = merged.first().expect("service should exist");
        assert_eq!(item.display_name, "Nginx");
        assert_eq!(item.port, Some(80));
        assert_eq!(item.status, ServiceStatus::Stopped);
        assert_eq!(summary.updated, 1);
    }

    #[test]
    fn merge_respects_locked_hidden_and_favorite() {
        let mut existing = base_service();
        existing.hidden = false;
        existing.favorite = true;
        existing.locked_fields.push("hidden".to_string());
        existing.locked_fields.push("favorite".to_string());

        let mut discovered = base_service();
        discovered.hidden = true;
        discovered.favorite = false;
        discovered.source = ServiceSource::Auto;

        let (merged, _) =
            merge_services(&[existing], &[discovered], DiscoveryStatusInfo::default());
        let item = merged.first().expect("service should exist");
        assert!(!item.hidden);
        assert!(item.favorite);
    }

    #[test]
    fn select_primary_prefers_web_ports() {
        assert_eq!(select_primary_port(&[10000, 8080, 9999]), Some(8080));
        assert_eq!(select_primary_port(&[5432, 2222]), Some(2222));
    }

    #[test]
    fn parse_port_handles_listen_lines() {
        let line = "LISTEN 0      4096         0.0.0.0:8080      0.0.0.0:*    users:((\"node\",pid=1,fd=18))";
        assert_eq!(parse_port(line), Some(8080));
    }

    #[test]
    fn classify_system_service_hidden_when_no_port() {
        let mut entry = ServiceEntry {
            id: "systemd-timesyncd".to_string(),
            service_name: "systemd-timesyncd.service".to_string(),
            display_name: "Timesync".to_string(),
            description: None,
            host: "server.local".to_string(),
            port: None,
            protocol: ServiceProtocol::Other,
            path: None,
            url: None,
            status: ServiceStatus::Running,
            group: None,
            tags: Vec::new(),
            icon: None,
            hidden: false,
            favorite: false,
            source: ServiceSource::Auto,
            locked_fields: Vec::new(),
            last_seen_at: None,
            updated_at: Utc::now(),
        };

        classify_service(&mut entry);
        assert_eq!(entry.group.as_deref(), Some(GROUP_SYSTEM));
        assert!(entry.hidden);
    }

    #[test]
    fn classify_syncthing_group() {
        let mut entry = base_service();
        entry.service_name = "syncthing.service".to_string();
        entry.group = None;
        entry.icon = None;
        entry.source = ServiceSource::Auto;

        classify_service(&mut entry);
        assert_eq!(entry.group.as_deref(), Some(GROUP_SYNC));
        assert_eq!(entry.icon.as_deref(), Some("🔄"));
        assert!(!entry.hidden);
    }
}
