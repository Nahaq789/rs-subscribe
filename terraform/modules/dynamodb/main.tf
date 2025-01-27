resource "aws_dynamodb_table" "tables" {
  for_each = var.tables

  name           = "${var.environment}-saddy-${each.key}"
  hash_key       = each.value.hash_key
  range_key      = each.value.range_key
  read_capacity  = each.value.read_capacity
  write_capacity = each.value.write_capacity

  dynamic "attribute" {
    for_each = each.value.attributes
    content {
      name = attribute.key
      type = attribute.value
    }
  }

  dynamic "global_secondary_index" {
    for_each = coalesce(each.value.gsis, {})

    content {
      hash_key        = global_secondary_index.value.hash_key
      range_key       = global_secondary_index.value.range_key
      name            = global_secondary_index.value.name
      projection_type = global_secondary_index.value.projection_type
      read_capacity   = 1
      write_capacity  = 1
    }
  }

  tags = {
    Environment = var.environment
  }
}