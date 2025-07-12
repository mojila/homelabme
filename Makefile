# Homelabme Makefile
# Build and deployment automation for Rust homelab server

# Variables
APP_NAME = homelabme
CARGO = cargo
TARGET_DIR = target
RELEASE_DIR = $(TARGET_DIR)/release
DEBUG_DIR = $(TARGET_DIR)/debug

# Default target
.PHONY: help
help:
	@echo "Homelabme Build System"
	@echo "====================="
	@echo "Available targets:"
	@echo "  dev        - Run development server (port 3000)"
	@echo "  build      - Build debug version"
	@echo "  build-prod - Build production version (optimized)"
	@echo "  run-prod   - Run production server (port 80)"
	@echo "  test       - Run all tests"
	@echo "  clean      - Clean build artifacts"
	@echo "  check      - Check code without building"
	@echo "  fmt        - Format code"
	@echo "  clippy     - Run clippy linter"
	@echo "  install    - Install as systemd service with auto-startup"
	@echo "  uninstall  - Remove systemd service and binary"
	@echo "  docker     - Build Docker image"
	@echo "  help       - Show this help message"

# Development targets
.PHONY: dev
dev:
	@echo "🚀 Starting development server on port 3000..."
	PORT=3000 $(CARGO) run

.PHONY: build
build:
	@echo "🔨 Building debug version..."
	$(CARGO) build

# Production targets
.PHONY: build-prod
build-prod:
	@echo "🏭 Building production version..."
	$(CARGO) build --release
	@echo "✅ Production build complete: $(RELEASE_DIR)/$(APP_NAME)"

.PHONY: run-prod
run-prod: build-prod
	@echo "🚀 Starting production server on port 80..."
	@echo "⚠️  Note: Port 80 requires sudo privileges"
	PORT=80 $(RELEASE_DIR)/$(APP_NAME)

# Quality assurance targets
.PHONY: test
test:
	@echo "🧪 Running tests..."
	$(CARGO) test

.PHONY: check
check:
	@echo "🔍 Checking code..."
	$(CARGO) check

.PHONY: fmt
fmt:
	@echo "🎨 Formatting code..."
	$(CARGO) fmt

.PHONY: clippy
clippy:
	@echo "📎 Running clippy..."
	$(CARGO) clippy -- -D warnings

# Utility targets
.PHONY: clean
clean:
	@echo "🧹 Cleaning build artifacts..."
	$(CARGO) clean

.PHONY: install
install: build-prod
	@echo "📦 Installing $(APP_NAME) to system..."
	@echo "Creating systemd service..."
	@sudo mkdir -p /opt/$(APP_NAME)
	@sudo cp $(RELEASE_DIR)/$(APP_NAME) /opt/$(APP_NAME)/
	@sudo chmod +x /opt/$(APP_NAME)/$(APP_NAME)
	@echo "[Unit]" | sudo tee /etc/systemd/system/$(APP_NAME).service > /dev/null
	@echo "Description=HomeLab Me - Network Management Server" | sudo tee -a /etc/systemd/system/$(APP_NAME).service > /dev/null
	@echo "After=network.target" | sudo tee -a /etc/systemd/system/$(APP_NAME).service > /dev/null
	@echo "" | sudo tee -a /etc/systemd/system/$(APP_NAME).service > /dev/null
	@echo "[Service]" | sudo tee -a /etc/systemd/system/$(APP_NAME).service > /dev/null
	@echo "Type=simple" | sudo tee -a /etc/systemd/system/$(APP_NAME).service > /dev/null
	@echo "User=root" | sudo tee -a /etc/systemd/system/$(APP_NAME).service > /dev/null
	@echo "WorkingDirectory=/opt/$(APP_NAME)" | sudo tee -a /etc/systemd/system/$(APP_NAME).service > /dev/null
	@echo "ExecStart=/opt/$(APP_NAME)/$(APP_NAME)" | sudo tee -a /etc/systemd/system/$(APP_NAME).service > /dev/null
	@echo "Environment=PORT=80" | sudo tee -a /etc/systemd/system/$(APP_NAME).service > /dev/null
	@echo "Restart=always" | sudo tee -a /etc/systemd/system/$(APP_NAME).service > /dev/null
	@echo "RestartSec=10" | sudo tee -a /etc/systemd/system/$(APP_NAME).service > /dev/null
	@echo "" | sudo tee -a /etc/systemd/system/$(APP_NAME).service > /dev/null
	@echo "[Install]" | sudo tee -a /etc/systemd/system/$(APP_NAME).service > /dev/null
	@echo "WantedBy=multi-user.target" | sudo tee -a /etc/systemd/system/$(APP_NAME).service > /dev/null
	@sudo systemctl daemon-reload
	@sudo systemctl enable $(APP_NAME).service
	@echo "✅ $(APP_NAME) installed and enabled for auto-startup"
	@echo "To start now: sudo systemctl start $(APP_NAME)"
	@echo "To check status: sudo systemctl status $(APP_NAME)"
	@echo "To view logs: sudo journalctl -u $(APP_NAME) -f"

.PHONY: uninstall
uninstall:
	@echo "🗑️  Uninstalling $(APP_NAME)..."
	@sudo systemctl stop $(APP_NAME).service 2>/dev/null || true
	@sudo systemctl disable $(APP_NAME).service 2>/dev/null || true
	@sudo rm -f /etc/systemd/system/$(APP_NAME).service
	@sudo systemctl daemon-reload
	@sudo rm -rf /opt/$(APP_NAME)
	@echo "✅ $(APP_NAME) uninstalled successfully"

# Docker targets
.PHONY: docker
docker:
	@echo "🐳 Building Docker image..."
	docker build -t $(APP_NAME):latest .

# CI/CD targets
.PHONY: ci
ci: fmt check clippy test build-prod
	@echo "✅ CI pipeline completed successfully"

# Quick development workflow
.PHONY: quick
quick: fmt check
	@echo "⚡ Quick checks completed"

# Production deployment preparation
.PHONY: deploy-prep
deploy-prep: clean ci
	@echo "🚀 Ready for deployment"
	@echo "Production binary: $(RELEASE_DIR)/$(APP_NAME)"
	@echo "To run on port 80: make run-prod"

# Show build information
.PHONY: info
info:
	@echo "Build Information:"
	@echo "================="
	@echo "App Name: $(APP_NAME)"
	@echo "Cargo Version: $$($(CARGO) --version)"
	@echo "Rust Version: $$(rustc --version)"
	@echo "Target Directory: $(TARGET_DIR)"
	@echo "Debug Binary: $(DEBUG_DIR)/$(APP_NAME)"
	@echo "Release Binary: $(RELEASE_DIR)/$(APP_NAME)"