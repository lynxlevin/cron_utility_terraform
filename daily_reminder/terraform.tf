terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 6.11"
    }
  }

  required_version = ">= 1.13"
}

provider "aws" {
  region = "ap-northeast-1"
}
