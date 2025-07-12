# HomeLab Me 🏠🔧

A modern, web-based network configuration management tool built with Rust and clean architecture principles. HomeLab Me simplifies network setup and management for home lab enthusiasts and system administrators.

## 🚀 Features

### Current Features
- **WiFi Network Management**: Scan, configure, and connect to WiFi networks
- **Static IP Configuration**: Set up static IP addresses for network interfaces
- **Network Interface Discovery**: Automatically detect available network interfaces
- **Real-time Network Scanning**: Live WiFi network discovery with signal strength and security information
- **Web-based Interface**: Clean, responsive UI for easy network management
- **RESTful API**: Complete API for programmatic network configuration

### 🔥 Upcoming Features
- **WiFi Hotspot Creation**: Transform your device into a WiFi access point
  - Custom SSID and password configuration
  - Multiple security protocols support
  - Bandwidth and connection limits
  - Guest network isolation

## 🏗️ Architecture

Built using **Clean Architecture** principles with clear separation of concerns:

```
src/
├── domain/          # Business logic and entities
├── application/      # Use cases and DTOs
├── infrastructure/   # External interfaces (web, repositories)
└── main.rs          # Application entry point
```

### Tech Stack
- **Backend**: Rust with Tokio async runtime
- **Web Framework**: Axum for HTTP server
- **Network Scanning**: wifiscanner crate for WiFi discovery
- **Frontend**: Vanilla JavaScript with modern CSS
- **Architecture**: Clean Architecture with dependency injection

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ installed
- Network interface access permissions
- macOS/Linux (Windows support coming soon)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/homelabme.git
cd homelabme
```

2. Build and run:
```bash
cargo run
```

3. Open your browser and navigate to:
```
http://localhost
```

### Linux Installation (Systemd Service)

For production deployment on Linux systems with systemd:

1. Install as a system service:
```bash
sudo make install
```

This will:
- Build the production binary
- Install it to `/opt/homelabme/`
- Create a systemd service file
- Enable auto-startup on boot
- Configure the service to run on port 80

2. Start the service:
```bash
sudo systemctl start homelabme
```

3. Check service status:
```bash
sudo systemctl status homelabme
```

4. View logs:
```bash
sudo journalctl -u homelabme -f
```

5. To uninstall:
```bash
sudo make uninstall
```

### Using Docker

```bash
# Build the image
make build

# Run the container
make run
```

## 📖 API Documentation

### Network Endpoints

- `GET /` - Network settings web interface
- `GET /api/network/settings` - Get current network configuration
- `POST /api/network/wifi` - Configure WiFi connection
- `POST /api/network/static-ip` - Configure static IP
- `GET /api/network/scan` - Scan for available WiFi networks

### Example WiFi Configuration

```bash
curl -X POST http://localhost/api/network/wifi \
  -H "Content-Type: application/json" \
  -d '{
    "ssid": "MyNetwork",
    "password": "mypassword",
    "interface_name": "wlan0"
  }'
```

## 🔧 Configuration

### Environment Variables

- `PORT` - Server port (default: 80)
- `RUST_LOG` - Logging level (default: info)

### Network Permissions

On macOS/Linux, you may need to run with elevated privileges for network interface access:

```bash
sudo cargo run
```

## 🛠️ Development

### Project Structure

- **Domain Layer**: Core business logic, entities, and service traits
- **Application Layer**: Use cases, DTOs, and application services
- **Infrastructure Layer**: Web handlers, repositories, and external integrations

### Adding New Features

1. Define entities in `src/domain/`
2. Create use cases in `src/application/`
3. Implement infrastructure in `src/infrastructure/`
4. Add web handlers and routes

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for security issues
cargo audit
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Guidelines

1. Follow Rust best practices and idioms
2. Maintain clean architecture principles
3. Add tests for new functionality
4. Update documentation as needed
5. Ensure code passes all CI checks

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Web framework: [Axum](https://github.com/tokio-rs/axum)
- WiFi scanning: [wifiscanner](https://crates.io/crates/wifiscanner)
- Async runtime: [Tokio](https://tokio.rs/)

## 📞 Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/yourusername/homelabme/issues) page
2. Create a new issue with detailed information
3. Join our community discussions

---

**HomeLab Me** - Making network configuration simple and accessible for everyone! 🌐✨