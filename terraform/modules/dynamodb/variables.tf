variable "environment" {
  type = string
}

variable "tables" {
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
}