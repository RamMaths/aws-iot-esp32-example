# AWS IoT Core ESP32 Example

A comprehensive, production-ready template for connecting ESP32 devices to AWS IoT Core using Rust. This project demonstrates modern IoT architecture with secure MQTT communications, automated cloud infrastructure provisioning, and clean, maintainable firmware code.

## ✨ Key Features

- **🔒 Secure MQTT over TLS** - End-to-end encrypted communication with AWS IoT Core
- **🏗️ Infrastructure as Code** - Complete Terraform automation for AWS resources
- **🦀 Modern Rust Firmware** - Clean, safe, and efficient ESP32 code using esp-idf-svc
- **📡 JSON Message Protocol** - Structured messaging with automatic parsing and type safety
- **📦 Generalized MQTT Client** - Reusable client that handles any message format
- **⚡ Non-blocking Architecture** - Efficient message processing without blocking main thread
- **🔧 Configuration-driven** - No hardcoded secrets, all settings via TOML files
- **🔄 Backward Compatibility** - Fallback support for plain text messages
- **📊 Comprehensive Logging** - Detailed debugging and monitoring capabilities
- **🔄 Automatic Certificate Management** - Seamless certificate provisioning and loading

## 🚀 Quick Start Guide

### Prerequisites

1. **Development Environment**
   ```bash
   # Install Rust toolchain for ESP32
   cargo install espup
   espup install
   source ~/export-esp.sh

   # Install flashing tools
   cargo install cargo-espflash
   ```

2. **AWS Environment**
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

Edit `terraform.tfvars` with your settings:
```hcl
thing_name = "my-esp32-device-001"     # Your unique device name
topic_prefix = "sensors"               # MQTT topic prefix
region = "us-east-1"                   # Your AWS region
tags = {
  Project = "My-IoT-Project"
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

### Step 3: Configure Firmware

```bash
cd ../firmware/example

# Copy certificates from Terraform output
cp -r ../../terraform/certs/ ./

# Create configuration from Terraform template
# Terraform will display the exact cfg.toml content needed
cp cfg.toml.example cfg.toml
```

Edit `cfg.toml` with the values from Terraform output:
```toml
[example]
# Wi-Fi Configuration
wifi_ssid = "YOUR_WIFI_NETWORK"
wifi_pass = "YOUR_WIFI_PASSWORD"

# AWS IoT Configuration (from Terraform output)
mqtt_url = "mqtts://your-endpoint.iot.region.amazonaws.com"
mqtt_client_id = "my-esp32-device-001"
mqtt_topic_pub = "sensors/data"
mqtt_topic_sub = "sensors/commands"

# Certificate paths (automatically configured by Terraform)
cert_ca = "certs/AmazonRootCA1.pem"
cert_crt = "certs/device-certificate.pem.crt"
cert_key = "certs/private-key.pem.key"
```

### Step 4: Build and Flash

```bash
# Build firmware
cargo build --release

# Flash to device (ensure ESP32 is connected via USB)
cargo run --release

# Or flash manually
espflash flash --monitor target/xtensa-esp32s3-espidf/release/example
```

## 🏛️ Architecture Overview

### Latest Improvements (v2.1)

Our latest updates include structured JSON messaging capabilities:

#### ✅ **JSON Message Protocol**
- **Structured messaging** - JSON format for reliable command/response patterns
- **Type-safe parsing** - Serde-based serialization with compile-time validation
- **Backward compatibility** - Fallback to plain text for legacy support
- **Extensible format** - Easy to add new message types and fields

### Previous Improvements (v2.0)

Our architecture refactoring brought significant improvements:

#### ✅ **Dependency Optimization**
- **Removed `anyhow` dependency** - Reduced binary size by using standard `Result<T, Box<dyn std::error::Error>>`
- **Lightweight error handling** - Clean error propagation without external dependencies

#### ✅ **Improved Separation of Concerns**
- **Generalized MQTT Client** - Client struct handles raw data without message format assumptions
- **Application-layer parsing** - Business logic separated from transport layer
- **Modular design** - Easy to extend and maintain

#### ✅ **Non-blocking Message Processing**
- **Background message listener** - MQTT messages processed in dedicated thread
- **Channel-based communication** - Non-blocking message passing between threads
- **Responsive main loop** - Main application never blocks on MQTT operations

### Code Structure

```
firmware/example/src/
├── main.rs         # Application entry point and message processing
├── client.rs       # Generalized MQTT client (transport layer)
├── startup.rs      # WiFi and application initialization
└── lib.rs          # Library exports
```

#### Client Architecture

The `Client` struct is now completely generalized:

```rust
// Returns raw bytes - no assumptions about message format
pub fn start_message_listener(&mut self) -> Result<Receiver<Vec<u8>>, Error>

// Simple publish interface
pub fn publish(&mut self, payload: &str) -> Result<(), Error>

// Clean subscription management
pub fn subscribe(&mut self) -> Result<(), Error>
```

#### Application Layer

Your `main.rs` handles all business logic with structured JSON messaging:

```rust
// Define your message structures
#[derive(Serialize, Deserialize, Debug)]
struct JsonMessage {
    message: String,
}

// Receive and parse JSON messages
match message_receiver.try_recv() {
    Ok(raw_data) => {
        // Parse as JSON with fallback to plain text
        match serde_json::from_slice::<JsonMessage>(&raw_data) {
            Ok(msg) => {
                let response = match msg.message.as_str() {
                    "ping" => JsonMessage { message: "pong".to_string() },
                    _ => JsonMessage { message: format!("Unknown: {}", msg.message) },
                };

                // Send structured JSON response
                let json_response = serde_json::to_string(&response)?;
                app.client.publish(&json_response)?;
            }
            Err(_) => {
                // Handle plain text messages as fallback
                let text = String::from_utf8_lossy(&raw_data);
                let response = JsonMessage {
                    message: format!("Plain text: {}", text)
                };
                app.client.publish(&serde_json::to_string(&response)?)?;
            }
        }
    }
}
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

#### Architecture Differences

| Chip Family | Architecture | Rust Target | Key Features |
|-------------|--------------|-------------|--------------|
| ESP32 | Xtensa LX6 | `xtensa-esp32-espidf` | Dual-core, WiFi + Bluetooth |
| ESP32-S2 | Xtensa LX7 | `xtensa-esp32s2-espidf` | Single-core, WiFi only |
| ESP32-S3 | Xtensa LX7 | `xtensa-esp32s3-espidf` | Dual-core, WiFi + Bluetooth 5 |
| ESP32-C3 | RISC-V | `riscv32imc-esp-espidf` | Single-core, WiFi + Bluetooth 5 |
| ESP32-C6 | RISC-V | `riscv32imac-esp-espidf` | WiFi 6 + Bluetooth 5 |
| ESP32-H2 | RISC-V | `riscv32imac-esp-espidf` | Thread/Zigbee focused |

#### Troubleshooting Architecture Changes

**"Linker not found" errors:**
```bash
# Ensure you've installed the correct toolchain
espup install --targets <your-chip>
source ~/export-esp.sh

# Check installed targets
rustup target list --installed | grep esp
```

**"Target not found" errors:**
```bash
# Install the specific Rust target
rustup target add riscv32imc-esp-espidf  # For RISC-V
rustup target add xtensa-esp32s3-espidf  # For Xtensa
```

**Build cache issues:**
```bash
# Clean everything and rebuild
cargo clean
rm -rf target/
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
  "message": "pong"
}
```

**Test plain text (fallback):**
Send: `Hello ESP32`

Receive:
```json
{
  "message": "Plain text: Hello ESP32"
}
```

### 3. Verify Certificate Authentication

Check AWS IoT Core logs in CloudWatch for successful connections.

## 🐛 Troubleshooting

### Build Issues

**ESP32 Architecture/Target Issues:**

*"Linker 'riscv32-esp-elf-gcc' not found" (when switching to ESP32-C3):*
```bash
# Install RISC-V toolchain
espup install --targets esp32c3
source ~/export-esp.sh
cargo clean
cargo build --release
```

*"Linker 'xtensa-esp32s3-elf-gcc' not found" (when switching to ESP32-S3):*
```bash
# Install Xtensa toolchain
espup install --targets esp32s3
source ~/export-esp.sh
cargo clean
cargo build --release
```

*"Target 'riscv32imc-esp-espidf' not found":*
```bash
# The target wasn't installed properly
rustup target add riscv32imc-esp-espidf
```

*Mixed architecture build errors:*
```bash
# Clean all build artifacts when switching architectures
cargo clean
rm -rf target/
cargo build --release
```

**"anyhow not found" errors:**
- ✅ Fixed in v2.0 - We removed the anyhow dependency

**Certificate loading errors:**
```bash
# Verify certificates exist
ls -la certs/
# Verify cfg.toml paths are correct
cat cfg.toml
```

**WiFi connection issues:**
```bash
# Enable debug logging in main.rs
log::set_max_level(log::LevelFilter::Debug);
```

### AWS/Terraform Issues

**Permission denied:**
```bash
# Verify AWS credentials
aws sts get-caller-identity

# Check IoT permissions
aws iot describe-endpoint --endpoint-type iot:Data-ATS
```

**Terraform state issues:**
```bash
# Reset if needed (destroys resources!)
terraform destroy
terraform apply
```

### Runtime Issues

**MQTT connection failures:**
- Verify IoT policy allows `iot:Connect`, `iot:Publish`, `iot:Subscribe`
- Check certificate is ACTIVE in AWS IoT Console
- Ensure device clock is accurate (NTP sync)

**Message not received:**
- Verify topic names match exactly
- Check IoT policy allows operations on your topics
- Monitor AWS IoT logs in CloudWatch

## 🚀 Extending the Project

### Custom Message Formats

The application now supports structured JSON messaging with fallback:

```rust
// Define custom message structures
#[derive(Serialize, Deserialize, Debug)]
struct SensorData {
    sensor_type: String,
    value: f32,
    timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct CommandMessage {
    action: String,
    parameters: Option<serde_json::Value>,
}

// Handle different message types
match serde_json::from_slice::<CommandMessage>(&raw_data) {
    Ok(cmd) => match cmd.action.as_str() {
        "read_sensor" => {
            let sensor_data = SensorData {
                sensor_type: "temperature".to_string(),
                value: 23.5,
                timestamp: get_timestamp(),
            };
            app.client.publish(&serde_json::to_string(&sensor_data)?)?;
        }
        "set_led" => {
            // Handle LED control with parameters
            if let Some(params) = cmd.parameters {
                // Process LED state from parameters
            }
        }
        _ => {
            // Handle unknown commands
        }
    },
    Err(_) => {
        // Fallback to plain text processing
        let text = String::from_utf8_lossy(&raw_data);
        // Handle plain text commands
    }
}
```

### Adding Sensors

1. **Add hardware interfaces** in main.rs
2. **Create data structures** for your sensor data
3. **Publish sensor readings** using `client.publish()`
4. **Handle commands** in the message processing loop

### Multiple Device Management

1. **Deploy multiple Terraform instances** with different `thing_name`
2. **Use topic patterns** like `devices/{device_id}/data`
3. **Implement device discovery** using MQTT topics

## 🔐 Security Best Practices

1. **Certificate Management**
   - Rotate certificates regularly
   - Use unique certificates per device
   - Store private keys securely

2. **Network Security**
   - Use strong WiFi passwords
   - Consider VPN for sensitive deployments
   - Monitor network traffic

3. **Access Control**
   - Implement least-privilege IoT policies
   - Regularly audit permissions
   - Use CloudTrail for monitoring

4. **Code Security**
   - Never commit secrets to version control
   - Validate all input data
   - Use secure coding practices

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
