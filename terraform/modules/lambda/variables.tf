variable "environment" {
  type = string
}

variable "function_name" {
  type = string
}

variable "lambda_role_arn" {
  type = string
}

variable "ecr_repository_url" {
  type = string
}

variable "environment_variables" {
  type    = map(string)
  default = {}
}