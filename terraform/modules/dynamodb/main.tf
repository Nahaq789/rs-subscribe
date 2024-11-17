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

  tags = {
    Environment = var.environment
  }
}