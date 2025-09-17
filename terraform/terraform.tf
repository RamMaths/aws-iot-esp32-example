terraform {
  required_version = ">=1.0.0"

  backend "s3" {
    bucket  = "esp32-aws-iot-core-example"
    key     = "aws_infra"
    region  = "us-east-1"
    profile = "ramses"
    encrypt = true
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 6.0.0"
    }
  }
}
