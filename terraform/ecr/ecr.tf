resource "aws_ecr_repository" "rs-subscribe-saddy-repository" {
  name = "rs-subscribe-saddy-repository"
}

output "repository_arn" {
  value = aws_ecr_repository.rs-subscribe-saddy-repository.arn
}

output "repository_url" {
  value = aws_ecr_repository.rs-subscribe-saddy-repository.repository_url
}