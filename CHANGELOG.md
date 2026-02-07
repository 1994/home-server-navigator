# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of Home Server Navigator
- Auto-discovery of systemd services and listening ports
- React-based modern UI with glassmorphism design
- Multi-language support (English/Chinese)
- Service management (CRUD operations)
- Favorite and hidden services
- Field locking to prevent auto-discovery overwrites
- Responsive design for desktop and mobile
- systemd integration with install/uninstall commands
- RESTful API for service management
- Real-time service status display

### Technical
- Rust backend with Axum framework
- React 18 + TypeScript frontend
- Single binary deployment with embedded frontend
- Optimized release build (~2.9MB)
- GitHub Actions CI/CD pipeline
- Multi-platform release builds (Linux x86_64/ARM64, macOS x86_64/ARM64)

## [0.1.0] - 2024-02-07

### Added
- First stable release
- Service auto-discovery via `systemctl` and `ss`
- Web UI for service management
- Basic authentication not included (designed for trusted networks)

---

## Release Template

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security improvements
```
