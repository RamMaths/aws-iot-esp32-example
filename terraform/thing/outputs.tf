# IoT Thing outputs
output "thing_name" {
  description = "Name of the created IoT Thing"
  value       = aws_iot_thing.thing.name
}

output "thing_arn" {
  description = "ARN of the created IoT Thing"
  value       = aws_iot_thing.thing.arn
}

# Certificate outputs
output "certificate_id" {
  description = "ID of the created certificate"
  value       = aws_iot_certificate.cert.id
}

output "certificate_arn" {
  description = "ARN of the created certificate"
  value       = aws_iot_certificate.cert.arn
}

output "certificate_pem" {
  description = "Certificate PEM content"
  value       = aws_iot_certificate.cert.certificate_pem
  sensitive   = true
}

output "private_key" {
  description = "Private key content"
  value       = aws_iot_certificate.cert.private_key
  sensitive   = true
}

output "public_key" {
  description = "Public key content"
  value       = aws_iot_certificate.cert.public_key
  sensitive   = true
}

# Policy outputs
output "policy_name" {
  description = "Name of the created IoT policy"
  value       = aws_iot_policy.policy.name
}

output "policy_arn" {
  description = "ARN of the created IoT policy"
  value       = aws_iot_policy.policy.arn
}

output "policy_document" {
  description = "Policy document JSON"
  value       = local.policy_document
}

# Certificate file paths (for reference)
output "certificate_files" {
  description = "Paths to downloaded certificate files"
  value = {
    certificate = "certs/${aws_iot_certificate.cert.id}-certificate.pem.crt"
    private_key = "certs/${aws_iot_certificate.cert.id}-private.pem.key"
    public_key  = "certs/${aws_iot_certificate.cert.id}-public.pem.key"
    root_ca_1   = "certs/AmazonRootCA1.pem"
    root_ca_3   = "certs/AmazonRootCA3.pem"
  }
}

# IoT endpoint for MQTT connection
data "aws_iot_endpoint" "endpoint" {
  endpoint_type = "iot:Data-ATS"
}

output "iot_endpoint" {
  description = "AWS IoT Core endpoint for MQTT connections"
  value       = data.aws_iot_endpoint.endpoint.endpoint_address
}

output "mqtt_url" {
  description = "Complete MQTT URL for connections"
  value       = "mqtts://${data.aws_iot_endpoint.endpoint.endpoint_address}"
}

# Configuration template for cfg.toml
output "cfg_toml_template" {
  description = "Template configuration for cfg.toml file"
  value = <<-EOT
[example]
wifi_ssid = "YOUR_WIFI_SSID"
wifi_pass = "YOUR_WIFI_PASSWORD"
mqtt_url = "mqtts://${data.aws_iot_endpoint.endpoint.endpoint_address}"
mqtt_client_id = "${aws_iot_thing.thing.name}"
mqtt_topic_pub = "${var.topic_prefix}/pub"
mqtt_topic_sub = "${var.topic_prefix}/sub"
cert_ca = "certs/AmazonRootCA1.pem"
cert_crt = "certs/${aws_iot_certificate.cert.id}-certificate.pem.crt"
cert_key = "certs/${aws_iot_certificate.cert.id}-private.pem.key"
EOT
}