# 🏠 Home Server Navigator

<p align="center">
  <b>An elegant home server service navigator</b><br>
  <i>Auto-discover, manage, and quickly access all your services</i>
</p>

<p align="center">
  <img alt="Build" src="https://img.shields.io/github/actions/workflow/status/yourusername/home-server-navigator/release.yml?branch=main&style=flat-square">
  <img alt="Release" src="https://img.shields.io/github/v/release/yourusername/home-server-navigator?style=flat-square">
  <img alt="License" src="https://img.shields.io/github/license/yourusername/home-server-navigator?style=flat-square">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-1.70+-orange?style=flat-square&logo=rust">
  <img alt="React" src="https://img.shields.io/badge/React-18+-blue?style=flat-square&logo=react">
</p>

<p align="center">
  <a href="#-features">Features</a> •
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-installation">Installation</a> •
  <a href="#-building">Building</a> •
  <a href="#-api">API</a> •
  <a href="#-screenshots">Screenshots</a>
</p>

---

## 📸 Screenshots

<p align="center">
  <img src=".github/images/screenshot-home.png" alt="Home Screen" width="80%">
</p>

<p align="center">
  <i>Service cards with real-time status, search, and quick access</i>
</p>

> 🎨 **Screenshots coming soon!** Place your screenshots in `.github/images/` directory.
>
> Recommended:
> - Main dashboard (`.github/images/screenshot-home.png`)
> - Mobile view (`.github/images/screenshot-mobile.png`)

## ✨ Features

- 🔍 **自动发现** - 自动扫描 systemd 服务和监听端口（通过 `systemctl + ss`）
- 🎨 **精美界面** - React + TypeScript 构建的现代化 UI
- 📱 **响应式设计** - 完美适配桌面和移动设备
- 🏷️ **智能分类** - 自动分组和标签管理
- ⭐ **收藏置顶** - 常用服务一键置顶
- 🔒 **字段锁定** - 防止自动发现覆盖手动编辑的内容
- 🔇 **智能降噪** - 默认隐藏无端口系统服务，界面更清爽
- 📦 **单二进制部署** - 前端资源嵌入，单个可执行文件即可运行
- ⚙️ **systemd 集成** - 内置 systemd 服务安装/卸载命令
- 🌐 **灵活配置** - 支持 CLI 参数和环境变量

## 🚀 Quick Start

### Option 1: Download Binary (Recommended)

前往 [Releases](https://github.com/yourusername/home-server-navigator/releases) 页面下载适合你系统的预编译二进制文件。

```bash
# 下载后赋予执行权限
chmod +x home-server-navigator

# 运行
./home-server-navigator --host 0.0.0.0 --port 18080
```

### Option 2: Using Docker

```bash
# Docker 支持正在开发中
docker run -d \
  --name home-server-navigator \
  --net=host \
  -v /var/run/dbus:/var/run/dbus \
  -v ./data:/data \
  ghcr.io/yourusername/home-server-navigator:latest
```

### Option 3: Build from Source

```bash
# 克隆仓库
git clone https://github.com/yourusername/home-server-navigator.git
cd home-server-navigator

# 一键构建
make build

# 运行
./dist/home-server-navigator --host 0.0.0.0 --port 18080
```

## 📦 Installation

### Install as systemd Service (Recommended for Production)

```bash
# 先构建二进制
make build

# 安装为 systemd 服务
sudo ./dist/home-server-navigator systemd install \
  --host 0.0.0.0 \
  --port 18080 \
  --default-host server.lan

# 或者使用 Makefile
make systemd-install
```

安装脚本会自动完成：
- ✅ 复制二进制到 `/usr/local/bin/home-server-navigator`
- ✅ 创建数据目录 `/var/lib/home-server-navigator/`
- ✅ 安装环境配置文件 `/etc/default/home-server-navigator`
- ✅ 创建 systemd unit 文件
- ✅ 启动并启用服务

### Service Management

```bash
# 查看状态
sudo systemctl status home-server-navigator

# 重启服务
sudo systemctl restart home-server-navigator

# 修改配置后重载
sudo systemctl daemon-reload
sudo systemctl restart home-server-navigator

# 卸载服务
sudo /usr/local/bin/home-server-navigator systemd uninstall
# 或
make systemd-uninstall
```

### Configuration

配置文件路径：`/etc/default/home-server-navigator`

```bash
# 服务监听地址
HOST=0.0.0.0

# 服务监听端口
PORT=18080

# 用于拼接服务 URL 的默认主机名
DEFAULT_HOST=server.lan

# 数据文件路径
DATA_FILE=/var/lib/home-server-navigator/services.json
```

修改后执行 `sudo systemctl restart home-server-navigator` 生效。

## 🔧 Building

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+ 
- [Node.js](https://nodejs.org/) 18+
- npm 或 yarn

### Build Steps

```bash
# 1. 克隆仓库
git clone https://github.com/yourusername/home-server-navigator.git
cd home-server-navigator

# 2. 构建前端（用于嵌入二进制）
cd frontend
npm install
npm run build
cd ..

# 3. 构建 Rust 后端
cd backend
cargo build --release
cd ..

# 二进制产物：backend/target/release/home-server-navigator
```

### Using Makefile (Recommended)

```bash
# 查看所有可用命令
make help

# 构建完整二进制
make build

# 开发模式运行后端
make run

# 安装到系统
make install

# 安装 systemd 服务
make systemd-install

# 清理构建产物
make clean
```

### CLI Arguments

```
Usage: home-server-navigator [OPTIONS]

Options:
  -h, --host <HOST>              监听地址 [default: 0.0.0.0] [env: HOST=]
  -p, --port <PORT>              监听端口 [default: 8080] [env: PORT=]
      --default-host <HOST>      默认主机名（用于拼接服务 URL）[default: localhost] [env: DEFAULT_HOST=]
      --data-file <PATH>         数据文件路径 [default: data/services.json] [env: DATA_FILE=]
      --systemd install          安装 systemd 服务
      --systemd uninstall        卸载 systemd 服务
  -V, --version                  打印版本
  --help                         打印帮助
```

## 🌐 API

### RESTful Endpoints

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/api/health` | 健康检查 |
| GET | `/api/services` | 获取所有服务列表 |
| POST | `/api/services` | 创建新服务 |
| GET | `/api/services/:id` | 获取单个服务详情 |
| PATCH | `/api/services/:id` | 更新服务信息 |
| DELETE | `/api/services/:id` | 删除服务 |
| POST | `/api/discovery/run` | 手动触发服务发现 |
| GET | `/api/discovery/status` | 获取发现任务状态 |

### API Examples

```bash
# 健康检查
curl http://localhost:8080/api/health

# 获取所有服务
curl http://localhost:8080/api/services | jq

# 手动触发服务发现
curl -X POST http://localhost:8080/api/discovery/run

# 更新服务
curl -X PATCH http://localhost:8080/api/services/ssh \
  -H "Content-Type: application/json" \
  -d '{"name": "SSH Server", "favorite": true}'
```

## 🛠️ Tech Stack

### Backend
- **[Axum](https://github.com/tokio-rs/axum)** - Rust Web 框架
- **[Tokio](https://tokio.rs/)** - 异步运行时
- **[Tower](https://github.com/tower-rs/tower)** - 中间件和服务抽象
- **[Serde](https://serde.rs/)** - 序列化/反序列化
- **[Clap](https://github.com/clap-rs/clap)** - 命令行参数解析

### Frontend
- **[React 18](https://react.dev/)** - UI 框架
- **[TypeScript](https://www.typescriptlang.org/)** - 类型安全
- **[Vite](https://vitejs.dev/)** - 构建工具

### Build & Deploy
- **Cargo** - Rust 构建系统
- **npm** - 前端包管理
- **systemd** - Linux 服务管理

## 📁 Project Structure

```
home-server-navigator/
├── backend/              # Rust 后端
│   ├── src/              # 源代码
│   │   ├── main.rs       # 入口
│   │   ├── api.rs        # API 路由
│   │   ├── discovery.rs  # 服务发现
│   │   ├── models.rs     # 数据模型
│   │   ├── state.rs      # 应用状态
│   │   └── store.rs      # 数据存储
│   ├── build.rs          # 构建脚本（嵌入前端资源）
│   └── Cargo.toml
├── frontend/             # React 前端
│   ├── src/              # 源代码
│   │   ├── App.tsx       # 主组件
│   │   ├── components/   # UI 组件
│   │   ├── pages/        # 页面
│   │   └── api/          # API 客户端
│   ├── index.html
│   └── package.json
├── systemd/              # systemd 配置模板
├── data/                 # 数据文件目录
├── Makefile              # 构建脚本
└── README.md
```

## 🤝 Contributing

欢迎贡献！请阅读 [CONTRIBUTING.md](CONTRIBUTING.md) 了解如何参与项目。

### Development Workflow

1. Fork 本仓库
2. 创建你的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### Submitting Issues

如果你发现 bug 或有新功能建议，请通过 [GitHub Issues](https://github.com/yourusername/home-server-navigator/issues) 提交。

## 📝 Roadmap

- [x] Multi-language support (i18n) ✅
- [ ] Docker image support
- [ ] Light/Dark theme toggle
- [ ] Service health check (HTTP ping)
- [ ] User authentication & access control
- [ ] Import/Export configuration
- [ ] Custom icon upload
- [ ] Advanced service grouping
- [ ] Service dependency visualization
- [ ] Metrics integration (Prometheus/Grafana)

## ⚠️ Important Notes

- 🔒 **Security**: Designed for trusted local networks. Do not expose directly to the public internet without additional protection.
- 🐧 **System Requirements**: Service discovery requires Linux (uses `systemctl` and `ss` commands)
- 📦 **Frontend Embedding**: Frontend must be built before backend, otherwise the binary will fallback to a placeholder page

## 📄 License

This project is licensed under the [MIT](LICENSE) License.

## 🙏 Acknowledgments

- Thanks to all developers who contributed code and feedback
- Special thanks to the [Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/) communities for excellent tools

---

<p align="center">
  Made with ❤️ for home server enthusiasts
</p>
