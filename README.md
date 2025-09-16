# AWS IoT Core Example with ESP32

This monorepo contains a comprehensive example of building an MQTT client for ESP32 devices that connects securely to AWS IoT Core, along with the corresponding cloud infrastructure using AWS CDK, AWS Lambda, and AWS IoT Core. This project serves as a production-ready template for IoT applications.

## 🚀 Features

- **Secure MQTT over TLS** connection to AWS IoT Core
- **Configuration-driven** setup using TOML files (no hardcoded secrets)
- **Automatic certificate loading** from configurable paths
- **Wi-Fi connectivity** with robust error handling and retry logic
- **Build-time validation** for configuration and certificates
- **ESP-IDF integration** with Rust using esp-idf-svc
- **Comprehensive logging** and debugging support

## 📋 Prerequisites

### Development Environment Setup

1. **Install Rust and ESP-IDF toolchain** following [The Rust on ESP Book](https://docs.esp-rs.org/book/installation/index.html)

2. **Install espup** and configure the environment:
   ```bash
   cargo install espup
   espup install
   source ~/export-esp.sh  # Run this in every new terminal session
   ```

3. **Install additional tools**:
   ```bash
   cargo install cargo-espflash
   ```

### AWS IoT Core Setup

You can set up AWS IoT Core resources in two ways: **using our Terraform automation (recommended)** or manually through the AWS Console.

#### Option A: Automated Setup with Terraform (Recommended)

Our Terraform module automatically creates all necessary AWS IoT Core resources and downloads certificates.

**Prerequisites:**
- [AWS CLI](https://aws.amazon.com/cli/) configured with appropriate permissions
- [Terraform](https://www.terraform.io/downloads.html) installed (>= 1.0.0)

**Steps:**

1. **Navigate to terraform directory:**
   ```bash
   cd terraform
   ```

2. **Configure your variables:**
   ```bash
   cp terraform.tfvars.example terraform.tfvars
   ```
   
   Edit `terraform.tfvars` with your specific values:
   ```hcl
   thing_name = "my-esp32-device-001"
   topic_prefix = "sensors"  # Your custom topic prefix
   region = "us-east-1"
   tags = {
     Project = "My-IoT-Project"
     Owner   = "your-name"
   }
   ```

3. **Initialize and deploy:**
   ```bash
   terraform init
   terraform plan    # Review the resources to be created
   terraform apply   # Type 'yes' to confirm
   ```

4. **Terraform will automatically:**
   - Create IoT Thing, Certificate, and Policy in AWS
   - Download all certificate files to `certs/` directory
   - Display configuration template and next steps
   - Generate the exact cfg.toml template you need

5. **Copy certificates to firmware project:**
   ```bash
   cp -r certs/ ../firmware/example/
   ```

6. **Use the generated cfg.toml template** displayed in Terraform output

#### Option B: Manual Setup via AWS Console

If you prefer to set up resources manually:

1. **Create an IoT Thing** in AWS IoT Console
2. **Generate device certificates** and download:
   - Device certificate (`.pem.crt`)
   - Private key (`.pem.key`)
   - Amazon Root CA 1 certificate
3. **Create and attach an IoT policy** to your certificate with appropriate permissions

## 🛠️ Project Setup

### 1. Navigate to the Firmware Directory

```bash
cd firmware/example
```

### 2. Configure Your Target Device

Update `firmware/example/.cargo/config.toml` based on your ESP32 variant. Supported targets can be found [here](https://github.com/esp-rs/esp-idf-svc#examples).

**For ESP32-S3:**
```toml
[build]
target = "xtensa-esp32s3-espidf"

[target.xtensa-esp32s3-espidf]
linker = "ldproxy"
runner = "espflash flash --monitor"
rustflags = [ "--cfg",  "espidf_time64"]

[unstable]
build-std = ["std", "panic_abort"]

[env]
MCU="esp32s3"
ESP_IDF_VERSION = "v5.3.2"
```

**For ESP32-C3:**
```toml
[build]
target = "riscv32imc-esp-espidf"

[target.riscv32imc-esp-espidf]
linker = "ldproxy"
runner = "espflash flash --monitor"

[unstable]
build-std = ["std", "panic_abort"]

[env]
MCU="esp32c3"
ESP_IDF_VERSION = "v5.3.2"
```

### 3. Create Configuration File

```bash
cp cfg.toml.example cfg.toml
```

### 4. Configure Your Settings

Edit `cfg.toml` with your specific values:

```toml
[example]
# Wi-Fi Configuration
wifi_ssid = "YOUR_WIFI_NETWORK"
wifi_pass = "YOUR_WIFI_PASSWORD"

# AWS IoT Core Configuration  
mqtt_url = "mqtts://your-endpoint.iot.region.amazonaws.com"
mqtt_client_id = "your-unique-device-id"
mqtt_topic_pub = "device/data"
mqtt_topic_sub = "device/commands"

# Certificate Paths (relative to project root)
cert_ca = "certs/AmazonRootCA1.pem"
cert_crt = "certs/your-device-certificate.pem.crt"
cert_key = "certs/your-private-key.pem.key"
```

### 5. Add Your Certificates

Create the `certs/` directory and place your AWS IoT certificates:

```bash
mkdir -p certs
# Copy your certificates to the certs/ directory:
# - AmazonRootCA1.pem
# - your-device-certificate.pem.crt  
# - your-private-key.pem.key
```

## 🔨 Building and Flashing

### Build the Project

```bash
cargo build --release
```

The build process will:
- Validate your `cfg.toml` configuration
- Check that all certificate files exist
- Generate certificate loading code from your configuration
- Compile the firmware

### Flash to Device

```bash
cargo run --release
```

Or use espflash directly:

```bash
espflash flash --monitor target/xtensa-esp32s3-espidf/release/example
```

## 📁 Project Structure

```
├── firmware/example/       # ESP32 Rust firmware
│   ├── Cargo.toml         # Rust dependencies and project config
│   ├── build.rs           # Build script for config validation and cert generation
│   ├── cfg.toml.example   # Configuration template
│   ├── cfg.toml           # Your actual configuration (gitignored)
│   ├── certs/             # Certificate directory (gitignored)
│   │   ├── AmazonRootCA1.pem
│   │   ├── device-cert.pem.crt
│   │   └── private-key.pem.key
│   └── src/
│       ├── main.rs        # Application entry point
│       ├── lib.rs         # Library entry point  
│       ├── startup.rs     # Wi-Fi and MQTT setup, configuration management
│       └── client.rs      # MQTT client implementation
├── terraform/              # AWS infrastructure as code
│   ├── terraform.tf       # Terraform and provider configuration
│   ├── main.tf            # Root module configuration
│   ├── variables.tf       # Input variables
│   ├── outputs.tf         # Output values
│   ├── terraform.tfvars.example  # Variables template
│   ├── terraform.tfvars   # Your actual variables (gitignored)
│   └── thing/             # IoT Thing module
│       ├── main.tf        # Thing, certificate, and policy resources
│       ├── variables.tf   # Module input variables
│       └── outputs.tf     # Module outputs
└── README.md              # This file
```

## 🔧 Configuration Reference

### Required Configuration Fields

| Field | Description | Example |
|-------|-------------|---------|
| `wifi_ssid` | Wi-Fi network name | `"MyWiFiNetwork"` |
| `wifi_pass` | Wi-Fi password | `"MySecretPassword"` |
| `mqtt_url` | AWS IoT Core endpoint | `"mqtts://abc123.iot.us-east-1.amazonaws.com"` |
| `mqtt_client_id` | Unique device identifier | `"my-esp32-device-001"` |
| `mqtt_topic_pub` | Topic for publishing data | `"sensors/temperature"` |
| `mqtt_topic_sub` | Topic for receiving commands | `"commands/led"` |

### Optional Certificate Configuration

| Field | Description | Default |
|-------|-------------|---------|
| `cert_ca` | Root CA certificate path | `"certs/AmazonRootCA1.pem"` |
| `cert_crt` | Device certificate path | Auto-detected |
| `cert_key` | Private key path | Auto-detected |

## 🐛 Troubleshooting

### Terraform Issues

**Error: "AccessDeniedException" or permissions errors**
- Ensure your AWS CLI is configured: `aws configure`
- Verify your AWS credentials have the necessary IoT permissions:
  ```json
  {
    "Version": "2012-10-17",
    "Statement": [
      {
        "Effect": "Allow",
        "Action": [
          "iot:CreateThing",
          "iot:CreateKeysAndCertificate",
          "iot:CreatePolicy",
          "iot:AttachPolicy",
          "iot:AttachThingPrincipal",
          "iot:DescribeEndpoint",
          "iot:DeleteThing",
          "iot:DeleteCertificate",
          "iot:DeletePolicy",
          "iot:DetachPolicy",
          "iot:DetachThingPrincipal",
          "iot:UpdateCertificate",
          "sts:GetCallerIdentity"
        ],
        "Resource": "*"
      }
    ]
  }
  ```

**Error: "terraform.tfvars not found"**
```bash
cd terraform
cp terraform.tfvars.example terraform.tfvars
# Edit terraform.tfvars with your values
```

**Error: Certificate files not downloading**
- Check that `curl` is installed on your system
- Verify internet connectivity to download Amazon Root CA certificates
- Check file permissions in the current directory

### Build Issues

**Error: "cfg.toml file not found"**
```bash
cp cfg.toml.example cfg.toml
# Edit cfg.toml with your values
```

**Error: "Certificate file not found"**
- Ensure all certificate files exist in the specified paths
- Check that certificate paths in `cfg.toml` are correct
- Verify certificates are valid and not corrupted

**Error: "WiFi SSID is empty"**
- Check that your `cfg.toml` has all required fields filled out
- Ensure the `[example]` section exists and is correctly formatted

### Runtime Issues

**Wi-Fi Connection Failed**
- Verify SSID and password are correct
- Check Wi-Fi signal strength
- Ensure your network supports the ESP32's Wi-Fi standards

**MQTT Connection Failed**  
- Verify your AWS IoT Core endpoint URL
- Check that your device certificate is active in AWS IoT Console
- Ensure your IoT policy allows the necessary MQTT actions
- Verify certificate files are valid and readable

**Certificate Errors**
- Ensure you're using the correct Amazon Root CA certificate
- Verify device certificate and private key match
- Check that certificates haven't expired

### Debugging

Enable verbose logging by modifying the log level in your code:

```rust
esp_idf_svc::log::EspLogger::initialize_default();
log::set_max_level(log::LevelFilter::Debug);
```

Monitor serial output during execution:
```bash
espflash monitor
```

## 🔐 Security Best Practices

1. **Never commit sensitive files**:
   - `cfg.toml` (contains credentials)
   - Certificate files (`.pem`, `.crt`, `.key`)

2. **Use unique device identifiers** for each device

3. **Implement least-privilege IoT policies** in AWS

4. **Regularly rotate certificates** following AWS IoT best practices

5. **Enable AWS CloudTrail** for API monitoring

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly on actual hardware
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

For issues and questions:
- Check the [troubleshooting section](#-troubleshooting) above
- Review [ESP-IDF documentation](https://docs.espressif.com/projects/esp-idf/)
- Consult [The Rust on ESP Book](https://docs.esp-rs.org/book/)
- Open an issue in this repository
