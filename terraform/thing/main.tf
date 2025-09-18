# Data source to get current AWS account ID if not provided
data "aws_caller_identity" "current" {}

# Local values for computed resources
locals {
  account_id  = var.account_id
  policy_name = var.policy_name != null ? var.policy_name : "${var.thing_name}-policy"
  
  # IoT policy document with dynamic topic prefix
  policy_document = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "iot:Publish",
          "iot:Receive",
          "iot:PublishRetain"
        ]
        Resource = "arn:aws:iot:${var.region}:${local.account_id}:topic/${var.topic_prefix}/*"
      },
      {
        Effect   = "Allow"
        Action   = "iot:Subscribe"
        Resource = "arn:aws:iot:${var.region}:${local.account_id}:topicfilter/${var.topic_prefix}/*"
      },
      {
        Effect   = "Allow"
        Action   = "iot:Connect"
        Resource = "arn:aws:iot:${var.region}:${local.account_id}:client/*"
      }
    ]
  })
}

# Create IoT Thing
resource "aws_iot_thing" "thing" {
  name = var.thing_name

  attributes = {
    created_by = "terraform"
  }
}

# Create IoT Certificate
resource "aws_iot_certificate" "cert" {
  active = var.certificate_active
}

# Create IoT Policy
resource "aws_iot_policy" "policy" {
  name   = local.policy_name
  policy = local.policy_document

  tags = var.tags
}

# Attach policy to certificate
resource "aws_iot_policy_attachment" "policy_attachment" {
  policy = aws_iot_policy.policy.name
  target = aws_iot_certificate.cert.arn
}

# Attach certificate to thing
resource "aws_iot_thing_principal_attachment" "thing_attachment" {
  thing     = aws_iot_thing.thing.name
  principal = aws_iot_certificate.cert.arn
}

# Download all certificate files using local-exec
resource "null_resource" "download_certificates" {
  triggers = {
    certificate_id = aws_iot_certificate.cert.id
  }

  provisioner "local-exec" {
    command = <<-EOT
      # Create thing-specific certs directory
      mkdir -p certs/${var.thing_name}

      # Download device certificate
      echo '${aws_iot_certificate.cert.certificate_pem}' > certs/${var.thing_name}/${aws_iot_certificate.cert.id}-certificate.pem.crt

      # Download public key
      echo '${aws_iot_certificate.cert.public_key}' > certs/${var.thing_name}/${aws_iot_certificate.cert.id}-public.pem.key

      # Download private key
      echo '${aws_iot_certificate.cert.private_key}' > certs/${var.thing_name}/${aws_iot_certificate.cert.id}-private.pem.key

      # Download Amazon Root CA certificates to thing directory (or shared location)
      curl -s https://www.amazontrust.com/repository/AmazonRootCA1.pem -o certs/${var.thing_name}/AmazonRootCA1.pem
      curl -s https://www.amazontrust.com/repository/AmazonRootCA3.pem -o certs/${var.thing_name}/AmazonRootCA3.pem

      echo "Certificates downloaded to certs/${var.thing_name}/ directory:"
      echo "- Device certificate: certs/${var.thing_name}/${aws_iot_certificate.cert.id}-certificate.pem.crt"
      echo "- Public key: certs/${var.thing_name}/${aws_iot_certificate.cert.id}-public.pem.key"
      echo "- Private key: certs/${var.thing_name}/${aws_iot_certificate.cert.id}-private.pem.key"
      echo "- Amazon Root CA 1: certs/${var.thing_name}/AmazonRootCA1.pem"
      echo "- Amazon Root CA 3: certs/${var.thing_name}/AmazonRootCA3.pem"
    EOT
  }

  depends_on = [
    aws_iot_certificate.cert,
    aws_iot_policy_attachment.policy_attachment,
    aws_iot_thing_principal_attachment.thing_attachment
  ]
}

# Cleanup certificates on destroy
resource "null_resource" "cleanup_certificates" {
  provisioner "local-exec" {
    when = destroy
    command = <<-EOT
      # Remove thing-specific certificate directory when destroying
      rm -rf certs/${self.triggers.thing_name}
      echo "Cleaned up certificate directory for thing: ${self.triggers.thing_name}"
    EOT
  }

  triggers = {
    certificate_id = aws_iot_certificate.cert.id
    thing_name     = var.thing_name
  }
}