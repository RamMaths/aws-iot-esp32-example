# AWS IoT Core ESP32 Example

A comprehensive, production-ready template for connecting ESP32 devices to AWS IoT Core using Rust. This project demonstrates modern IoT architecture with secure MQTT communications, automated cloud infrastructure provisioning, and clean, maintainable firmware code.

## ✨ Key Features

- **🔒 Secure MQTT over TLS** - End-to-end encrypted communication with AWS IoT Core
- **🏗️ Infrastructure as Code** - Complete Terraform automation for AWS resources
- **🦀 Modern Rust Firmware** - Clean, safe, and efficient ESP32 code using esp-idf-svc
- **⚡ Non-blocking Architecture** - Efficient message processing without blocking main thread
- **🔧 Configuration-driven** - No hardcoded secrets, all settings via TOML files
- **🔄 Automatic Certificate Management** - Seamless certificate provisioning and loading

## 🚀 Quick Start Guide

### Prerequisites

1. **Development Environment**
   - Follow the comprehensive installation guide from the official [ESP Rust Book](https://docs.espressif.com/projects/rust/book/installation/riscv-and-xtensa.html)
   - Install `espflash` using this [book guide](https://docs.espressif.com/projects/rust/book/tooling/espflash.html#espflash-1)
   - Install `espmonitor` using the official [repository](https://github.com/esp-rs/espmonitor)

3. **AWS Environment**
   ```bash
   # Install and configure AWS CLI
   aws configure

   # Install Terraform
   # Visit: https://www.terraform.io/downloads.html
   ```

### Step 1: Clone and Setup

```bash
git clone <repository-url>
cd aws-iot-esp32-example
```

### Step 2: Provision AWS Infrastructure

```bash
cd terraform

# Copy and configure variables
cp terraform.tfvars.example terraform.tfvars
```

Edit `terraform.tfvars` to create multiple ESP32 devices:
```hcl
# Create multiple ESP32 things (esp32s3 and esp32c3)
things = [
  {
    name         = "esp32s3"
    topic_prefix = "esp32"
  },
  {
    name         = "esp32c3"
    topic_prefix = "esp32"
  }
]

region = "us-east-1"                   # Your AWS region
tags = {
  Project = "ESP32-IoT-Multi-Device"
  Environment = "development"
  Owner = "your-name"
}
```

Deploy infrastructure:
```bash
terraform init
terraform plan
terraform apply  # Type 'yes' to confirm
```

This will create:
- **Two IoT Things**: `esp32s3` and `esp32c3`
- **Separate certificate directories**: `certs/esp32s3/` and `certs/esp32c3/`
- **Shared MQTT topics**: Both devices use `esp32/*` topic prefix
- **Individual policies**: Each device gets its own policy and certificates

### Step 3: Configure Firmware

#### For ESP32-S3 Device:

```bash
cd ../firmware/example

# Copy ESP32-S3 certificates
cp -r ../../terraform/certs/esp32s3/ ./certs/

# Create ESP32-S3 configuration
cp cfg.toml.example cfg.toml
```

Edit `cfg.toml` for ESP32-S3:
```toml
[example]
# Wi-Fi Configuration
wifi_ssid = "YOUR_WIFI_NETWORK"
wifi_pass = "YOUR_WIFI_PASSWORD"

# AWS IoT Configuration (from Terraform output)
mqtt_url = "mqtts://your-endpoint.iot.region.amazonaws.com"
mqtt_client_id = "esp32s3"
mqtt_topic_pub = "esp32/pub"
mqtt_topic_sub = "esp32/sub"

# Certificate paths (ESP32-S3 specific)
cert_ca = "certs/AmazonRootCA1.pem"
cert_crt = "certs/[certificate-id]-certificate.pem.crt"
cert_key = "certs/[certificate-id]-private.pem.key"
```

### Step 4: Build and Flash

#### For ESP32-S3 Device:

```bash
cd firmware/example

# Configure for ESP32-S3 (Xtensa architecture)
# Make sure your .cargo/config.toml has:
# target = "xtensa-esp32s3-espidf"

# Build firmware
cargo build --release

# Flash to ESP32-S3 device (ensure device is connected via USB)
cargo run --release

# Or flash manually
espflash flash --monitor target/xtensa-esp32s3-espidf/release/example
```

#### For ESP32-C3 Device:

```bash
cd ../exampleC3

# Configure for ESP32-C3 (RISC-V architecture)
# Update .cargo/config.toml with:
# target = "riscv32imc-esp-espidf"

# Clean previous builds when switching architectures
cargo clean

# Build firmware
cargo build --release

# Flash to ESP32-C3 device (ensure device is connected via USB)
cargo run --release

# Or flash manually
espflash flash --monitor target/riscv32imc-esp-espidf/release/example
```

## 🛠️ Detailed Setup Instructions

### ESP32 Variant Configuration

The project supports multiple ESP32 variants with different architectures. Choose the configuration that matches your hardware.

> **📋 Reference:** See all supported targets at [esp-idf-svc examples](https://github.com/esp-rs/esp-idf-svc#examples)

#### Switching Between ESP32 Variants

**Step 1: Install Required Toolchain**

ESP32 variants use different architectures requiring specific toolchains:

```bash
# For Xtensa-based chips (ESP32, ESP32-S2, ESP32-S3)
espup install --targets esp32,esp32s2,esp32s3

# For RISC-V-based chips (ESP32-C3, ESP32-C6, ESP32-H2)
espup install --targets esp32c3,esp32c6,esp32h2

# Or install all targets at once
espup install --targets all

# Source environment (required every terminal session)
source ~/export-esp.sh
```

**Step 2: Update `.cargo/config.toml`**

Choose the appropriate configuration for your target:

**ESP32-S3 (Xtensa Architecture):**
```toml
[build]
target = "xtensa-esp32s3-espidf"

[target.xtensa-esp32s3-espidf]
linker = "ldproxy"
runner = "espflash flash --monitor"
rustflags = ["--cfg", "espidf_time64"]

[unstable]
build-std = ["std", "panic_abort"]

[env]
MCU = "esp32s3"
ESP_IDF_VERSION = "v5.3.2"
```

**ESP32-C3 (RISC-V Architecture):**
```toml
[build]
target = "riscv32imc-esp-espidf"

[target.riscv32imc-esp-espidf]
linker = "ldproxy"
runner = "espflash flash --monitor"

[unstable]
build-std = ["std", "panic_abort"]

[env]
MCU = "esp32c3"
ESP_IDF_VERSION = "v5.3.2"
```

**ESP32 (Original - Xtensa Architecture):**
```toml
[build]
target = "xtensa-esp32-espidf"

[target.xtensa-esp32-espidf]
linker = "ldproxy"
runner = "espflash flash --monitor"
rustflags = ["--cfg", "espidf_time64"]

[unstable]
build-std = ["std", "panic_abort"]

[env]
MCU = "esp32"
ESP_IDF_VERSION = "v5.3.2"
```

**Step 3: Clean and Rebuild**

When switching targets, always clean previous builds:

```bash
cargo clean
cargo build --release
```

### Terraform Configuration Details

The Terraform module creates:

- **IoT Thing** - Device representation in AWS IoT Core
- **Device Certificate** - X.509 certificate for secure authentication
- **IoT Policy** - Permissions for MQTT operations
- **Certificate Downloads** - Automatic retrieval of all required certificates

Output includes:
```bash
# Example Terraform output
mqtt_endpoint = "a1b2c3d4e5f6g7.iot.us-east-1.amazonaws.com"
thing_name = "my-esp32-device-001"
certificate_arn = "arn:aws:iot:us-east-1:123456789012:cert/a1b2c3..."

# Ready-to-use cfg.toml template displayed
```

### Certificate Management

Certificates are automatically organized:
```
certs/
├── AmazonRootCA1.pem           # AWS Root CA
├── device-certificate.pem.crt  # Your device certificate
└── private-key.pem.key         # Device private key
```

## 📡 JSON Message Protocol

### Message Format

The ESP32 device now communicates using structured JSON messages:

#### Incoming Message Structure
```json
{
  "message": "command_name"
}
```

#### Outgoing Response Structure
```json
{
  "message": "response_content"
}
```

### Supported Commands

| Command | Description | Example Request | Example Response |
|---------|-------------|-----------------|------------------|
| `ping` | Connectivity test | `{"message": "ping"}` | `{"message": "pong"}` |
| Any other | Unknown command | `{"message": "test"}` | `{"message": "Unknown action: test"}` |
| Plain text | Fallback for non-JSON | `Hello World` | `{"message": "Plain text: Hello World"}` |

### Example Communication Flow

**1. Send ping command:**
```bash
# Publish to: sensors/commands
{"message": "ping"}
```

**2. Receive response:**
```bash
# Received on: sensors/data
{"message": "pong"}
```

**3. Send unknown command:**
```bash
# Publish to: sensors/commands
{"message": "status"}
```

**4. Receive error response:**
```bash
# Received on: sensors/data
{"message": "Unknown action: status"}
```

### Extending the Protocol

Add new commands by modifying the match statement in `main.rs`:

```rust
let response = match msg.message.as_str() {
    "ping" => JsonMessage { message: "pong".to_string() },
    "get_status" => JsonMessage {
        message: "Device online".to_string()
    },
    "restart" => {
        // Handle restart command
        JsonMessage { message: "Restarting...".to_string() }
    },
    _ => JsonMessage {
        message: format!("Unknown action: {}", msg.message)
    },
};
```

## 📋 Configuration Reference

### Required Settings

| Setting | Description | Example |
|---------|-------------|---------|
| `wifi_ssid` | WiFi network name | `"MyNetwork"` |
| `wifi_pass` | WiFi password | `"SecurePassword123"` |
| `mqtt_url` | AWS IoT endpoint | `"mqtts://abc123.iot.us-east-1.amazonaws.com"` |
| `mqtt_client_id` | Unique device ID | `"sensor-001"` |
| `mqtt_topic_pub` | Publish topic | `"sensors/temperature"` |
| `mqtt_topic_sub` | Subscribe topic | `"commands/led"` |

### Certificate Paths

| Setting | Description | Default |
|---------|-------------|---------|
| `cert_ca` | Root CA certificate | `"certs/AmazonRootCA1.pem"` |
| `cert_crt` | Device certificate | `"certs/device-certificate.pem.crt"` |
| `cert_key` | Private key | `"certs/private-key.pem.key"` |

## 🧪 Testing Your Setup

### 1. Monitor Device Output

```bash
espflash monitor
```

Look for these success indicators:
```
I (1234) example: WiFi connected successfully
I (1235) example: MQTT client created successfully
I (1236) example: Subscribed to topic "sensors/commands"
I (1237) example: Starting main application loop
```

### 2. Test MQTT Communication

Using AWS IoT Core Test Console:

1. Go to AWS IoT Core → Test → MQTT test client
2. **Subscribe** to your device's publish topic: `sensors/data`
3. **Publish** to your device's subscribe topic: `sensors/commands`

**Send JSON commands:**
```json
{
  "message": "ping"
}
```

**Expected JSON response:**
```json
{
  "message": "pong from {your mqtt thing id}"
}
```

### 3. Verify Certificate Authentication

Check AWS IoT Core logs in CloudWatch for successful connections.

## 📈 Performance Considerations

- **Binary Size**: ~2.5MB (reduced from 3MB after removing anyhow)
- **Memory Usage**: ~200KB RAM for MQTT operations
- **CPU Usage**: Non-blocking architecture minimizes CPU overhead
- **Network**: Efficient MQTT keep-alive and message batching

## 🤝 Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Test on real hardware
4. Update documentation
5. Submit pull request

## 📄 License

GPL-3.0 License - see [LICENSE](LICENSE) file for details.

## 🆘 Support

- 📖 [ESP-IDF Documentation](https://docs.espressif.com/projects/esp-idf/)
- 🦀 [Rust on ESP Book](https://docs.esp-rs.org/book/)
- 🐛 [Report Issues](https://github.com/your-repo/issues)
- 💬 [Discussions](https://github.com/your-repo/discussions)

---

**Built with ❤️ for the IoT community**
