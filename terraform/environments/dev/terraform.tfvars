dynamodb_tables = {
  payment = {
    hash_key       = "user_id"
    range_key      = "payment_method_id"
    read_capacity  = 1
    write_capacity = 1
    attributes = {
      payment_method_id = "S"
      user_id           = "S"
    }
  },
  subscribe = {
    hash_key       = "user_id"
    range_key      = "subscribe_id"
    read_capacity  = 1
    write_capacity = 1
    attributes = {
      subscribe_id = "S"
      user_id      = "S"
      category_id  = "S"
    }
    gsis = {
      category = {
        hash_key        = "user_id"
        range_key       = "category_id"
        name            = "gsi-category"
        projection_type = "ALL"
      }
    }
  },
  category = {
    hash_key       = "user_id"
    range_key      = "category_id"
    read_capacity  = 1
    write_capacity = 1
    attributes = {
      category_id = "S"
      user_id     = "S"
    }
  }
}