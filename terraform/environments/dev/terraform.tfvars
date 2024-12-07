dynamodb_tables = {
  payment = {
    hash_key       = "user_id"
    range_key      = "payment_method_id"
    read_capacity  = 1
    write_capacity = 1
    attributes     = {
      payment_method_id = "S"
      user_id    = "S"
    }
  }
}