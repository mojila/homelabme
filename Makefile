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
	@echo "  install    - Install production binary to system"
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
	@echo "📦 Installing to system..."
	$(CARGO) install --path .

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