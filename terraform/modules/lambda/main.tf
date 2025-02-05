resource "aws_lambda_function" "function" {
  function_name = "${var.environment}-${var.function_name}"
  role          = var.lambda_role_arn
  package_type  = "Image"
  image_uri     = "${var.ecr_repository_url}:${var.environment}"

  environment {
    variables = var.environment_variables
  }

  tags = {
    Environment = var.environment
  }

  lifecycle {
    prevent_destroy = false
    ignore_changes = all
  }
}