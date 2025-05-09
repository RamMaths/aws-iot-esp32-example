# AWS IOT CORE EXAMPLE WITH ESP32

This mono repo contains a solid example of how to build an mqtt client for an Esp32 device as well as the correspoding cloud infrastructure and its configuration using AWS CDK, AWS Lambda and AWS Iot Core. Hopefully it can be helpful as a template.

## Getting started with firmware

Make sure you installed the ESP-IDF envirnoment necessary to build and flash firmware in esp32 devices described in [The Rust on ESP Book](https://docs.esp-rs.org/book/installation/index.html). After we install espup a bash script will be generated at `~/export-esp.sh` this script exports the envirnoment variables needed to build any esp-idf project.

Now it's time to build our example project! depending on the board (MCU) we are using we want to update our configuration, `firmware/led-client/.cargo/config.toml` a list of supported MCUs and it's correspoing targets can be found [at](https://github.com/esp-rs/esp-idf-svc#examples).

```toml
[build]
target = "xtensa-esp32s3-espidf" # <----- THIS IS THE TARGET THE BINARY WILL BE COMPILED TO

[target.xtensa-esp32s3-espidf] # <----- CONFIGURATION FOR THAT TARGET
linker = "ldproxy"
runner = "espflash flash --monitor"
rustflags = [ "--cfg",  "espidf_time64"]

[unstable]
build-std = ["std", "panic_abort"]

[env]
MCU="esp32s3" # <----- THE ACTUAL MICRO CONTROLLER UNIT WE ARE USING
# Note: this variable is not used by the pio builder (`cargo build --features pio`)
ESP_IDF_VERSION = "v5.3.2"
```

After configuring our board we want to copy and paste the configuration file.

```sh
cd firmware/led-client
cp cfg-example.toml cfg.toml
```

This is how the file looks so you might want to update the values as your needs.

```toml
[led-client]
wifi_ssid = ""
wifi_pass = ""
mqtt_url = ""
mqtt_client_id = ""
mqtt_topic_pub = ""
mqtt_topic_sub = ""
```

To finally build the project we need to setup this envirnoment variables with the correct paths pointing to your certificates needed to make a secure SSL connection with AWS.

```sh
export SERVER_CERT_PATH="./AmazonRootCA1.pem"
export CLIENT_CERT_PATH="./Device-certificate.pem.crt"
export PRIVATE_KEY_PATH="./private-key-private.pem.key"

cargo build --release
```
