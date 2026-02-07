# 🏠 Home Server Navigator

<p align="center">
  <b>Your elegant home server service dashboard</b><br>
  <i>Auto-discover, organize, and access all your self-hosted services in one place</i>
</p>

<p align="center">
  <a href="https://github.com/1994/home-server-navigator/actions/workflows/ci.yml">
    <img alt="CI" src="https://img.shields.io/github/actions/workflow/status/1994/home-server-navigator/ci.yml?branch=main&style=flat-square&logo=github&label=CI">
  </a>
  <a href="https://github.com/1994/home-server-navigator/releases">
    <img alt="Release" src="https://img.shields.io/github/v/release/1994/home-server-navigator?style=flat-square&logo=github">
  </a>
  <a href="https://github.com/1994/home-server-navigator/pkgs/container/home-server-navigator">
    <img alt="Docker" src="https://img.shields.io/badge/Docker-ghcr.io-blue?style=flat-square&logo=docker">
  </a>
  <img alt="License" src="https://img.shields.io/github/license/1994/home-server-navigator?style=flat-square">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-1.70+-orange?style=flat-square&logo=rust">
  <img alt="React" src="https://img.shields.io/badge/React-18+-61DAFB?style=flat-square&logo=react">
</p>

<p align="center">
  <a href="#-features">Features</a> •
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-installation">Installation</a> •
  <a href="#-building">Building</a> •
  <a href="#-api">API</a> •
  <a href="#-contributing">Contributing</a>
</p>

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| 🔍 **Auto Discovery** | Automatically scan systemd services and listening ports via `systemctl` + `ss` |
| 🎨 **Glassmorphism UI** | Modern React + TypeScript interface with elegant glass effects |
| 🌍 **Multi-language** | English & 简体中文 support (i18n) |
| 📱 **Responsive** | Perfect on desktop, tablet, and mobile |
| 🏷️ **Smart Organization** | Auto-grouping, tags, and custom categories |
| ⭐ **Favorites** | Pin your most-used services to the top |
| 🔒 **Field Locking** | Prevent auto-discovery from overwriting your manual edits |
| 🔇 **Smart Filtering** | Hide portless system services by default for a cleaner view |
| 📦 **Single Binary** | Frontend assets embedded, one file deployment (~3MB) |
| ⚙️ **systemd Integration** | Built-in service install/uninstall commands |
| 🐳 **Docker Ready** | Official multi-arch images (amd64/arm64) |
| 🔔 **Toast Notifications** | Real-time feedback for all actions |

---

## 🚀 Quick Start

### Option 1: Download Binary (Recommended)

Download the latest release for your platform:

```bash
# Linux x86_64
wget https://github.com/1994/home-server-navigator/releases/latest/download/home-server-navigator-linux-x86_64.tar.gz
tar -xzf home-server-navigator-linux-x86_64.tar.gz
chmod +x home-server-navigator
./home-server-navigator
```

Available platforms:
- `linux-x86_64` / `linux-aarch64`
- `darwin-x86_64` / `darwin-aarch64` (macOS)

### Option 2: Docker

```bash
docker run -d \
  --name home-server-navigator \
  --net=host \
  -v /var/run/dbus:/var/run/dbus:ro \
  -v /data:/data \
  ghcr.io/1994/home-server-navigator:latest
```

Or use `docker-compose.yml`:

```yaml
version: '3'
services:
  home-server-navigator:
    image: ghcr.io/1994/home-server-navigator:latest
    container_name: home-server-navigator
    network_mode: host
    volumes:
      - /var/run/dbus:/var/run/dbus:ro
      - ./data:/data
    environment:
      - HOST=0.0.0.0
      - PORT=8080
      - DEFAULT_HOST=server.local
    restart: unless-stopped
```

### Option 3: Build from Source

```bash
git clone https://github.com/1994/home-server-navigator.git
cd home-server-navigator
make build
./dist/home-server-navigator
```

---

## 📦 Installation

### systemd Service (Recommended for Production)

```bash
# Build first
make build

# Install as systemd service
sudo make systemd-install HOST=0.0.0.0 PORT=8080 DEFAULT_HOST=server.lan

# Or manually
sudo ./dist/home-server-navigator systemd install \
  --host 0.0.0.0 \
  --port 8080 \
  --default-host server.lan
```

This will:
- ✅ Copy binary to `/usr/local/bin/home-server-navigator`
- ✅ Create data directory `/var/lib/home-server-navigator/`
- ✅ Install config to `/etc/default/home-server-navigator`
- ✅ Create and enable systemd service
- ✅ Start the service

### Service Management

```bash
# Check status
sudo systemctl status home-server-navigator

# Restart
sudo systemctl restart home-server-navigator

# View logs
sudo journalctl -u home-server-navigator -f

# Uninstall
sudo make systemd-uninstall
# or
sudo /usr/local/bin/home-server-navigator systemd uninstall
```

### Configuration

Edit `/etc/default/home-server-navigator`:

```bash
# Server bind address
HOST=0.0.0.0

# Server port
PORT=8080

# Default hostname for service URLs
DEFAULT_HOST=server.lan

# Data file location
DATA_FILE=/var/lib/home-server-navigator/services.json
```

Then restart: `sudo systemctl restart home-server-navigator`

---

## 🔧 Building

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- [Node.js](https://nodejs.org/) 18+
- npm

### Using Makefile

```bash
# View all commands
make help

# Build optimized release binary (~3MB)
make build

# Development mode (frontend dev server + backend)
make dev

# Run backend only
make run

# Install to /usr/local/bin
sudo make install

# Run tests
make test

# Clean build artifacts
make clean
```

### Manual Build

```bash
# 1. Build frontend
cd frontend
npm ci
npm run build
cd ..

# 2. Build Rust backend
cd backend
cargo build --release
cd ..

# Binary: backend/target/release/home-server-navigator
```

### CLI Usage

```
Usage: home-server-navigator [OPTIONS] [COMMAND]

Commands:
  systemd  Manage systemd service (install/uninstall)
  help     Print this message

Options:
  -h, --host <HOST>          Bind address [default: 0.0.0.0] [env: HOST=]
  -p, --port <PORT>          Listen port [default: 8080] [env: PORT=]
      --default-host <HOST>  Default hostname for URLs [default: localhost] [env: DEFAULT_HOST=]
      --data-file <PATH>     Data file path [default: data/services.json] [env: DATA_FILE=]
  -V, --version              Print version
      --help                 Print help

Examples:
  # Run with defaults
  ./home-server-navigator

  # Custom host and port
  ./home-server-navigator --host 0.0.0.0 --port 8080

  # Install as systemd service
  sudo ./home-server-navigator systemd install --host 0.0.0.0 --port 80
```

---

## 🌐 API

### Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/health` | Health check |
| GET | `/api/services` | List all services |
| POST | `/api/services` | Create service |
| GET | `/api/services/:id` | Get service details |
| PATCH | `/api/services/:id` | Update service |
| DELETE | `/api/services/:id` | Delete service |
| POST | `/api/discovery/run` | Trigger discovery |
| GET | `/api/discovery/status` | Discovery status |

### Examples

```bash
# Health check
curl http://localhost:8080/api/health

# List services
curl http://localhost:8080/api/services | jq

# Trigger discovery
curl -X POST http://localhost:8080/api/discovery/run

# Update service
curl -X PATCH http://localhost:8080/api/services/grafana \
  -H "Content-Type: application/json" \
  -d '{"favorite": true, "tags": ["monitoring"]}'
```

---

## 🛠️ Tech Stack

### Backend
- **[Axum](https://github.com/tokio-rs/axum)** - Rust web framework
- **[Tokio](https://tokio.rs/)** - Async runtime
- **[Tower](https://github.com/tower-rs/tower)** - Middleware layer
- **[Serde](https://serde.rs/)** - Serialization
- **[Clap](https://github.com/clap-rs/clap)** - CLI parsing

### Frontend
- **[React 18](https://react.dev/)** - UI framework
- **[TypeScript](https://www.typescriptlang.org/)** - Type safety
- **[Vite](https://vitejs.dev/)** - Build tool

### Build & Deploy
- **GitHub Actions** - CI/CD with multi-platform releases
- **Docker** - Multi-arch images (amd64/arm64)
- **systemd** - Linux service management

---

## 📁 Project Structure

```
home-server-navigator/
├── backend/              # Rust backend
│   ├── src/
│   │   ├── main.rs       # Entry point
│   │   ├── api.rs        # REST API routes
│   │   ├── discovery.rs  # Service discovery
│   │   ├── models.rs     # Data models
│   │   ├── state.rs      # App state management
│   │   └── store.rs      # JSON persistence
│   ├── build.rs          # Embed frontend assets
│   └── Cargo.toml
├── frontend/             # React frontend
│   ├── src/
│   │   ├── App.tsx
│   │   ├── components/   # UI components
│   │   ├── i18n/         # Translations (EN/ZH)
│   │   └── api/          # API client
│   └── package.json
├── systemd/              # systemd templates
├── Dockerfile            # Multi-stage build
├── Makefile              # Build automation
└── README.md
```

---

## 🤝 Contributing

We welcome contributions! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Quick Start

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/home-server-navigator.git
cd home-server-navigator

# Create branch
git checkout -b feature/your-feature

# Make changes and commit
git commit -m "feat: add your feature"

# Push and create PR
git push origin feature/your-feature
```

### Development Setup

```bash
# Run frontend dev server
cd frontend && npm run dev

# In another terminal, run backend
cd backend && cargo run
```

---

## 📝 Roadmap

- [x] Auto service discovery via systemd
- [x] Multi-language support (EN/ZH)
- [x] Docker support with multi-arch images
- [x] Responsive glassmorphism UI
- [ ] Light/Dark theme toggle
- [ ] HTTP health checks for services
- [ ] User authentication
- [ ] Import/Export configuration
- [ ] Custom icon upload
- [ ] Service dependency graph
- [ ] Prometheus metrics integration

---

## ⚠️ Security Notice

This tool is designed for **trusted local networks**. It does not include authentication and should not be exposed directly to the public internet without additional protection (reverse proxy with auth, VPN, etc.).

---

## 📄 License

[MIT](LICENSE) © 2024 Home Server Navigator Contributors

---

<p align="center">
  Made with ❤️ for the self-hosted community
</p>
