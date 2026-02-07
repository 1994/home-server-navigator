SHELL := /usr/bin/env bash

APP_NAME := home-server-navigator
BACKEND_DIR := backend
FRONTEND_DIR := frontend
DIST_DIR := dist

PREFIX ?= /usr/local
SYSTEMD_DIR ?= /etc/systemd/system
SERVICE_USER ?= $(shell id -un)
SERVICE_GROUP ?= $(shell id -gn)
DATA_DIR ?= /var/lib/$(APP_NAME)

.PHONY: help
help:
	@echo "Targets:"
	@echo "  make build              Build all-in-one binary into ./dist/"
	@echo "  make run                Run local dev (backend only)"
	@echo "  make install            Install binary into $(PREFIX)/bin"
	@echo "  make systemd-install    Install systemd unit + env + data dir"
	@echo "  make systemd-uninstall  Remove systemd unit + env"
	@echo "  make clean              Remove build artifacts"

.PHONY: build
build:
	@mkdir -p $(DIST_DIR)
	@echo "[1/2] Building frontend..."
	@cd $(FRONTEND_DIR) && npm install && npm run build
	@echo "[2/2] Building backend (release)..."
	@cd $(BACKEND_DIR) && cargo build --release
	@cp -f $(BACKEND_DIR)/target/release/$(APP_NAME) $(DIST_DIR)/$(APP_NAME)
	@echo "Built: $(DIST_DIR)/$(APP_NAME)"

.PHONY: run
run:
	@cd $(BACKEND_DIR) && cargo run -- --host 127.0.0.1 --port 8080

.PHONY: install
install:
	@install -Dm755 $(DIST_DIR)/$(APP_NAME) $(PREFIX)/bin/$(APP_NAME)
	@echo "Installed: $(PREFIX)/bin/$(APP_NAME)"

.PHONY: systemd-install
systemd-install:
	@test -f $(DIST_DIR)/$(APP_NAME) || (echo "Binary not found. Run: make build" && exit 1)
	@echo "Registering service via built-in command..."
	@sudo $(DIST_DIR)/$(APP_NAME) systemd install \
		--install-path $(PREFIX)/bin/$(APP_NAME) \
		--unit-path $(SYSTEMD_DIR)/$(APP_NAME).service \
		--env-path /etc/default/$(APP_NAME) \
		--data-dir $(DATA_DIR)
	@echo "Done. Check: sudo systemctl status $(APP_NAME).service"

.PHONY: systemd-uninstall
systemd-uninstall:
	@echo "Unregistering service via built-in command..."
	@sudo $(DIST_DIR)/$(APP_NAME) systemd uninstall \
		--unit-path $(SYSTEMD_DIR)/$(APP_NAME).service \
		--env-path /etc/default/$(APP_NAME)
	@echo "Uninstalled systemd unit. Data dir kept: $(DATA_DIR)"

.PHONY: clean
clean:
	@rm -rf $(DIST_DIR)
	@cd $(BACKEND_DIR) && cargo clean
	@rm -rf $(FRONTEND_DIR)/dist
	@echo "Cleaned"
