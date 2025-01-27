use crate::category::category_id::CategoryId;
use crate::category::category_name::CategoryName;
use crate::user::user_id::UserId;

pub mod category_error;
pub mod category_id;
pub mod category_name;

#[derive(Debug, Clone)]
pub struct Category {
    category_id: CategoryId,
    user_id: UserId,
    category_name: CategoryName,
}

impl Category {
    /// idなしコンストラクタ
    pub fn new(user_id: UserId, category_name: CategoryName) -> Self {
        let category_id = CategoryId::new();
        Self { category_id, user_id, category_name }
    }

    pub fn from(category_id: CategoryId, user_id: UserId, category_name: CategoryName) -> Self {
        Self { category_id, user_id, category_name }
    }

    pub fn category_id(&self) -> &CategoryId {
        &self.category_id
    }
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
    pub fn category_name(&self) -> &CategoryName {
        &self.category_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AggregateId;
    use std::str::FromStr;

    #[test]
    fn test_category_new_success() {
        let user_id = UserId::new();
        let category_name = CategoryName::from_str("hoge").unwrap();

        let result = Category::new(user_id, category_name);
        assert!(!result.category_id.to_string().is_empty())
    }

    #[test]
    fn test_category_from_success() {
        let category_id = CategoryId::new();
        let user_id = UserId::new();
        let category_name = CategoryName::from_str("hoge").unwrap();

        let result = Category::from(category_id.clone(), user_id, category_name);

        assert_eq!(category_id.value(), result.category_id.value())
    }
}
