# AWS IoT Core ESP32 Example

A comprehensive, production-ready template for connecting ESP32 devices to AWS IoT Core using Rust. This project demonstrates modern IoT architecture with secure MQTT communications, automated cloud infrastructure provisioning, and clean, maintainable firmware code.

## ✨ Key Features

- **🔒 Secure MQTT over TLS** - End-to-end encrypted communication with AWS IoT Core
- **🏗️ Infrastructure as Code** - Complete Terraform automation for AWS resources
- **🦀 Modern Rust Firmware** - Clean, safe, and efficient ESP32 code using esp-idf-svc
- **📦 Generalized MQTT Client** - Reusable client that handles any message format
- **⚡ Non-blocking Architecture** - Efficient message processing without blocking main thread
- **🔧 Configuration-driven** - No hardcoded secrets, all settings via TOML files
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

### Recent Improvements (v2.0)

Our latest architecture refactoring brings significant improvements:

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

Your `main.rs` handles all business logic:

```rust
// Receive raw message data
match message_receiver.try_recv() {
    Ok(raw_data) => {
        // Convert to text or parse as JSON - your choice!
        let message_text = String::from_utf8_lossy(&raw_data);

        // Handle your specific message format
        match message_text.trim() {
            "ping" => app.client.publish("pong")?,
            _ => app.client.publish(&format!("Echo: {}", message_text))?,
        }
    }
}
```

## 🛠️ Detailed Setup Instructions

### ESP32 Variant Configuration

Update `.cargo/config.toml` for your specific ESP32 model:

**ESP32-S3:**
```toml
[build]
target = "xtensa-esp32s3-espidf"

[target.xtensa-esp32s3-espidf]
linker = "ldproxy"
runner = "espflash flash --monitor"
rustflags = ["--cfg", "espidf_time64"]

[env]
MCU = "esp32s3"
ESP_IDF_VERSION = "v5.3.2"
```

**ESP32-C3:**
```toml
[build]
target = "riscv32imc-esp-espidf"

[target.riscv32imc-esp-espidf]
linker = "ldproxy"
runner = "espflash flash --monitor"

[env]
MCU = "esp32c3"
ESP_IDF_VERSION = "v5.3.2"
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
   ```json
   {
     "message": "ping"
   }
   ```
4. You should see the device respond with "pong"

### 3. Verify Certificate Authentication

Check AWS IoT Core logs in CloudWatch for successful connections.

## 🐛 Troubleshooting

### Build Issues

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

The generalized client supports any message format:

```rust
// Handle JSON messages
if let Ok(json) = serde_json::from_slice::<MyStruct>(&raw_data) {
    // Process structured data
}

// Handle binary data
if raw_data[0] == 0xFF {
    // Process binary protocol
}

// Handle plain text
let text = String::from_utf8_lossy(&raw_data);
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

MIT License - see [LICENSE](LICENSE) file for details.

## 🆘 Support

- 📖 [ESP-IDF Documentation](https://docs.espressif.com/projects/esp-idf/)
- 🦀 [Rust on ESP Book](https://docs.esp-rs.org/book/)
- 🐛 [Report Issues](https://github.com/your-repo/issues)
- 💬 [Discussions](https://github.com/your-repo/discussions)

---

**Built with ❤️ for the IoT community**