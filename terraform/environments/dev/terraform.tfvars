dynamodb_tables = {
  payment = {
    hash_key       = "payment_id"
    range_key      = "user_id"
    read_capacity  = 1
    write_capacity = 1
    attributes     = {
      payment_id = "S"
      user_id    = "S"
    }
  }
}