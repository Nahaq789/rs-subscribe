variable "lambda_role_arn" {
  description = "ARN of the IAM role for Lambda"
  type = string
}

variable "ecr_repository_url" {
  description = "URL of ECR repository"
  type = string
}

resource "aws_lambda_function" "rs-subscribe" {
  function_name = "rs-subscribe-saddy"
  role          = var.lambda_role_arn
  package_type = "Image"
  image_uri = "${var.ecr_repository_url}:latest"

  environment {
    variables = {
      RUST_BACKTRACE = "1"
      RUST_LOG       = "info"
      HOST           = "0.0.0.0"
      PORT           = "8080"
    }
  }
}