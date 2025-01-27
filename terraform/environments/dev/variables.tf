variable "environment" {
  type    = string
  default = "dev"
}

variable "aws_region" {
  type    = string
  default = "ap-northeast-1"
}

variable "dynamodb_tables" {
  type = map(object({
    hash_key       = string
    range_key = optional(string)
    read_capacity  = number
    write_capacity = number
    attributes = map(string)
    gsis = optional(map(object({
      hash_key        = string
      range_key       = string
      name            = string
      projection_type = string
    })))
  }))
  description = "DynamoDB tables configuration"
}