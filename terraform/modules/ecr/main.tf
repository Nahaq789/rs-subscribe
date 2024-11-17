resource "aws_ecr_repository" "repo" {
  name = "${var.environment}-${var.repository_name}"

  image_scanning_configuration {
    scan_on_push = true
  }

  tags = {
    Environment = var.environment
  }
}