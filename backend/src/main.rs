// Home Server Navigator - Main entry point
// https://github.com/yourusername/home-server-navigator
// Licensed under MIT

mod api;
#[cfg(test)]
mod api_tests;
mod discovery;
mod models;
mod state;
mod store;

use anyhow::{anyhow, bail, Context};
use axum::{
    body::Body,
    extract::Path,
    http::{
        header::{HeaderValue, CONTENT_TYPE},
        StatusCode,
    },
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use clap::{Args, Parser, Subcommand};
use std::{
    fs,
    net::SocketAddr,
    path::{Path as StdPath, PathBuf},
};
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::api::create_router;
use crate::state::AppState;

include!(concat!(env!("OUT_DIR"), "/embedded_assets.rs"));

const FALLBACK_INDEX_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Home Server Navigator</title>
  <style>
    :root { color-scheme: dark; font-family: Inter, ui-sans-serif, system-ui; }
    body { margin: 0; background: #0f1220; color: #eef2ff; }
    .wrap { max-width: 920px; margin: 0 auto; padding: 24px; }
    h1 { margin: 0 0 8px; }
    p { opacity: 0.9; line-height: 1.5; }
    .card { background: #171d35; border: 1px solid #2a3357; border-radius: 12px; padding: 16px; margin-top: 16px; }
    code { background: #212a48; padding: 2px 8px; border-radius: 6px; }
    a { color: #89b4ff; }
  </style>
</head>
<body>
  <main class="wrap">
    <h1>Home Server Navigator</h1>
    <p>Single binary is running. Frontend static assets are not embedded in this build yet.</p>
    <section class="card">
      <p>Build frontend first, then rebuild backend binary:</p>
      <p><code>cd frontend && npm install && npm run build</code></p>
      <p><code>cd ../backend && cargo build --release</code></p>
      <p>API: <a href="/api/health">/api/health</a>, <a href="/api/services">/api/services</a></p>
    </section>
  </main>
</body>
</html>
"#;

const APP_NAME: &str = "home-server-navigator";
const DEFAULT_INSTALL_PATH: &str = "/usr/local/bin/home-server-navigator";
const DEFAULT_ENV_PATH: &str = "/etc/default/home-server-navigator";
const DEFAULT_UNIT_PATH: &str = "/etc/systemd/system/home-server-navigator.service";
const DEFAULT_DATA_DIR: &str = "/var/lib/home-server-navigator";

#[derive(Debug, Clone, Parser)]
#[command(name = "home-server-navigator")]
#[command(about = "All-in-one home server service navigator")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    #[arg(long, env = "HOST", default_value = "0.0.0.0")]
    host: String,
    #[arg(long, env = "PORT", default_value_t = 8080)]
    port: u16,
    #[arg(long, env = "DEFAULT_HOST", default_value = "localhost")]
    default_host: String,
    #[arg(long, env = "DATA_FILE", default_value = "data/services.json")]
    data_file: String,
}

#[derive(Debug, Clone, Subcommand)]
enum Command {
    #[command(subcommand)]
    Systemd(SystemdCommand),
}

#[derive(Debug, Clone, Subcommand)]
enum SystemdCommand {
    /// Install systemd unit/env and optionally enable/start.
    Install(SystemdInstallArgs),
    /// Uninstall systemd unit/env (data is kept by default).
    Uninstall(SystemdUninstallArgs),
}

#[derive(Debug, Clone, Args)]
struct SystemdInstallArgs {
    /// Where to install the binary.
    #[arg(long, default_value = DEFAULT_INSTALL_PATH)]
    install_path: String,
    /// systemd unit file path.
    #[arg(long, default_value = DEFAULT_UNIT_PATH)]
    unit_path: String,
    /// Environment file path.
    #[arg(long, default_value = DEFAULT_ENV_PATH)]
    env_path: String,
    /// Data directory (used to default DATA_FILE).
    #[arg(long, default_value = DEFAULT_DATA_DIR)]
    data_dir: String,
    /// Override data file path.
    #[arg(long)]
    data_file: Option<String>,
    /// Listen host to write into env file.
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    /// Listen port to write into env file.
    #[arg(long, default_value_t = 8080)]
    port: u16,
    /// Used to compose clickable URLs in UI.
    #[arg(long, default_value = "localhost")]
    default_host: String,
    /// Do not enable/start the service.
    #[arg(long)]
    no_enable: bool,
}

#[derive(Debug, Clone, Args)]
struct SystemdUninstallArgs {
    /// systemd unit file path.
    #[arg(long, default_value = DEFAULT_UNIT_PATH)]
    unit_path: String,
    /// Environment file path.
    #[arg(long, default_value = DEFAULT_ENV_PATH)]
    env_path: String,
    /// Where the binary was installed.
    #[arg(long, default_value = DEFAULT_INSTALL_PATH)]
    install_path: String,
    /// Remove installed binary.
    #[arg(long)]
    remove_binary: bool,
    /// Remove data directory (DANGEROUS).
    #[arg(long)]
    remove_data: bool,
    /// Data directory to remove when --remove-data is set.
    #[arg(long, default_value = DEFAULT_DATA_DIR)]
    data_dir: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,backend=debug".into()),
        )
        .init();

    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Command::Systemd(systemd) => {
                return handle_systemd(systemd).await;
            }
        }
    }

    let state = AppState::new(cli.default_host, cli.data_file)
        .await
        .context("failed to initialize app state")?;

    let _ = state.run_discovery().await;

    let app = Router::new()
        .merge(create_router(state))
        .route("/", get(index_handler))
        .route("/{*path}", get(asset_or_index_handler))
        .layer(CorsLayer::permissive());

    let bind_addr = format!("{}:{}", cli.host, cli.port);
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .with_context(|| format!("failed binding on {bind_addr}"))?;
    let local_addr: SocketAddr = listener.local_addr()?;

    info!(address = %local_addr, "home server navigator started");
    axum::serve(listener, app)
        .await
        .context("server exited with error")?;

    Ok(())
}

async fn handle_systemd(command: SystemdCommand) -> anyhow::Result<()> {
    if !cfg!(target_os = "linux") {
        bail!("systemd install is only supported on Linux");
    }

    match command {
        SystemdCommand::Install(args) => systemd_install(args).await,
        SystemdCommand::Uninstall(args) => systemd_uninstall(args).await,
    }
}

async fn systemd_install(args: SystemdInstallArgs) -> anyhow::Result<()> {
    let unit_name = unit_name_from_path(&args.unit_path)?;
    let install_path = PathBuf::from(args.install_path);
    let unit_path = PathBuf::from(args.unit_path);
    let env_path = PathBuf::from(args.env_path);
    let data_dir = PathBuf::from(args.data_dir);
    let data_file = args
        .data_file
        .unwrap_or_else(|| data_dir.join("services.json").to_string_lossy().to_string());

    install_self_binary(&install_path)?;
    ensure_dir(&data_dir)?;
    write_file(
        &env_path,
        &render_env(&args.host, args.port, &args.default_host, &data_file),
    )?;
    write_file(
        &unit_path,
        &render_unit(&install_path.to_string_lossy(), &env_path.to_string_lossy()),
    )?;

    run_systemctl(["daemon-reload"]).await?;

    if !args.no_enable {
        run_systemctl(["enable", "--now", unit_name.as_str()]).await?;
        println!("Installed and started: {unit_name}");
    } else {
        println!("Installed: {unit_name}");
        println!("Enable/start: sudo systemctl enable --now {unit_name}");
    }

    Ok(())
}

async fn systemd_uninstall(args: SystemdUninstallArgs) -> anyhow::Result<()> {
    let unit_name = unit_name_from_path(&args.unit_path)?;

    let _ = run_systemctl_allow_failure(["disable", "--now", unit_name.as_str()]).await;

    remove_file_if_exists(&PathBuf::from(args.unit_path))?;
    remove_file_if_exists(&PathBuf::from(args.env_path))?;

    run_systemctl(["daemon-reload"]).await?;

    if args.remove_binary {
        remove_file_if_exists(&PathBuf::from(args.install_path))?;
    }

    if args.remove_data {
        let data_dir = PathBuf::from(args.data_dir);
        if data_dir.exists() {
            fs::remove_dir_all(&data_dir)
                .with_context(|| format!("failed removing data dir {}", data_dir.display()))?;
        }
    }

    println!("Uninstalled: {unit_name}");
    Ok(())
}

fn unit_name_from_path(path: &str) -> anyhow::Result<String> {
    let file_name = StdPath::new(path)
        .file_name()
        .ok_or_else(|| anyhow!("invalid unit path: {path}"))?
        .to_string_lossy()
        .to_string();
    Ok(file_name)
}

fn install_self_binary(install_path: &PathBuf) -> anyhow::Result<()> {
    let exe_path = std::env::current_exe().context("failed to locate current executable")?;

    if let Some(parent) = install_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed creating {}", parent.display()))?;
    }

    if exe_path != *install_path {
        fs::copy(&exe_path, install_path).with_context(|| {
            format!(
                "failed copying {} to {}",
                exe_path.display(),
                install_path.display()
            )
        })?;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = fs::Permissions::from_mode(0o755);
        fs::set_permissions(install_path, permissions).with_context(|| {
            format!(
                "failed setting executable permission on {}",
                install_path.display()
            )
        })?;
    }

    Ok(())
}

fn ensure_dir(path: &PathBuf) -> anyhow::Result<()> {
    fs::create_dir_all(path)
        .with_context(|| format!("failed creating directory {}", path.display()))?;
    Ok(())
}

fn write_file(path: &PathBuf, content: &str) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed creating directory {}", parent.display()))?;
    }
    fs::write(path, content).with_context(|| format!("failed writing {}", path.display()))?;
    Ok(())
}

fn remove_file_if_exists(path: &PathBuf) -> anyhow::Result<()> {
    if path.exists() {
        fs::remove_file(path).with_context(|| format!("failed removing {}", path.display()))?;
    }
    Ok(())
}

fn render_env(host: &str, port: u16, default_host: &str, data_file: &str) -> String {
    format!(
        "# {APP_NAME} env\n\
HOST={host}\n\
PORT={port}\n\
DEFAULT_HOST={default_host}\n\
DATA_FILE={data_file}\n"
    )
}

fn render_unit(install_path: &str, env_path: &str) -> String {
    format!(
        "[Unit]\n\
Description=Home Server Navigator\n\
After=network-online.target\n\
Wants=network-online.target\n\
\n\
[Service]\n\
Type=simple\n\
EnvironmentFile={env_path}\n\
ExecStart={install_path} --host ${{HOST}} --port ${{PORT}} --default-host ${{DEFAULT_HOST}} --data-file ${{DATA_FILE}}\n\
Restart=on-failure\n\
RestartSec=2\n\
\n\
NoNewPrivileges=true\n\
PrivateTmp=true\n\
ProtectSystem=full\n\
ProtectHome=true\n\
\n\
[Install]\n\
WantedBy=multi-user.target\n"
    )
}

async fn run_systemctl<const N: usize>(args: [&str; N]) -> anyhow::Result<()> {
    let output = tokio::process::Command::new("systemctl")
        .args(args)
        .output()
        .await
        .context("failed spawning systemctl")?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    Err(anyhow!(
        "systemctl {:?} failed (code: {:?})\nstdout: {}\nstderr: {}",
        args,
        output.status.code(),
        stdout,
        stderr
    ))
}

async fn run_systemctl_allow_failure<const N: usize>(args: [&str; N]) -> anyhow::Result<()> {
    let output = tokio::process::Command::new("systemctl")
        .args(args)
        .output()
        .await;
    match output {
        Ok(output) if output.status.success() => Ok(()),
        Ok(_) => Ok(()),
        Err(_) => Ok(()),
    }
}

async fn index_handler() -> Response {
    serve_index_html()
}

async fn asset_or_index_handler(Path(path): Path<String>) -> Response {
    if let Some(response) = serve_static_asset(&path) {
        return response;
    }
    serve_index_html()
}

fn serve_index_html() -> Response {
    if let Some((_, body, content_type)) = find_asset("/index.html") {
        build_binary_response(StatusCode::OK, body, content_type)
    } else {
        Html(FALLBACK_INDEX_HTML).into_response()
    }
}

fn serve_static_asset(path: &str) -> Option<Response> {
    let normalized = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    };
    let (_, bytes, content_type) = find_asset(&normalized)?;
    Some(build_binary_response(StatusCode::OK, bytes, content_type))
}

fn find_asset(path: &str) -> Option<&'static (&'static str, &'static [u8], &'static str)> {
    EMBEDDED_ASSETS
        .iter()
        .find(|(asset_path, _, _)| *asset_path == path)
}

fn build_binary_response(status: StatusCode, body: &'static [u8], content_type: &str) -> Response {
    let mut response = Response::new(Body::from(body));
    *response.status_mut() = status;
    if let Ok(value) = HeaderValue::from_str(content_type) {
        response.headers_mut().insert(CONTENT_TYPE, value);
    }
    response
}
