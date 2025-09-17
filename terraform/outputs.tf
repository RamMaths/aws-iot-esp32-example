# Pass through outputs from the thing module
output "thing_name" {
  description = "Name of the created IoT Thing"
  value       = module.iot_thing.thing_name
}

output "thing_arn" {
  description = "ARN of the created IoT Thing"
  value       = module.iot_thing.thing_arn
}

output "certificate_id" {
  description = "ID of the created certificate"
  value       = module.iot_thing.certificate_id
}

output "certificate_arn" {
  description = "ARN of the created certificate"
  value       = module.iot_thing.certificate_arn
}

output "policy_name" {
  description = "Name of the created IoT policy"
  value       = module.iot_thing.policy_name
}

output "policy_arn" {
  description = "ARN of the created IoT policy"
  value       = module.iot_thing.policy_arn
}

output "iot_endpoint" {
  description = "AWS IoT Core endpoint for MQTT connections"
  value       = module.iot_thing.iot_endpoint
}

output "mqtt_url" {
  description = "Complete MQTT URL for connections"
  value       = module.iot_thing.mqtt_url
}

output "certificate_files" {
  description = "Paths to downloaded certificate files"
  value       = module.iot_thing.certificate_files
}

output "cfg_toml_template" {
  description = "Template configuration for cfg.toml file"
  value       = module.iot_thing.cfg_toml_template
}

# Instructions for next steps
output "next_steps" {
  description = "Instructions for using the created resources"
  value       = <<-EOT

🎉 AWS IoT resources created successfully!

📁 Certificate files downloaded to:
  - ${module.iot_thing.certificate_files.certificate}
  - ${module.iot_thing.certificate_files.private_key}
  - ${module.iot_thing.certificate_files.public_key}
  - ${module.iot_thing.certificate_files.root_ca_1}
  - ${module.iot_thing.certificate_files.root_ca_3}

📋 Next steps:
1. Copy certificate files to your firmware project:
   cp -r certs/ ../firmware/example/

2. Create/update your cfg.toml file with:
${module.iot_thing.cfg_toml_template}

3. Build and flash your ESP32 firmware:
   cd ../firmware/example
   cargo build --release
   cargo run --release

🌐 MQTT Connection Details:
  - Endpoint: ${module.iot_thing.iot_endpoint}
  - Thing Name: ${module.iot_thing.thing_name}
  - Topics: ${var.topic_prefix}/*
  
EOT
}