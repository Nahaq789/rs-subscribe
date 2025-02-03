use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CategoryName(String);

/// カテゴリー名に関するエラー
#[derive(Error, Debug, Clone)]
pub enum CategoryNameError {
    /// 名前が空の場合のエラー
    #[error("the category name is empty")]
    Empty,
    /// 名前が長すぎる場合のエラー
    #[error("the category name is too long")]
    TooLong,
}

impl CategoryName {
    pub fn new(value: &str) -> Result<Self, CategoryNameError> {
        if value.is_empty() {
            Err(CategoryNameError::Empty)
        } else if value.chars().count() > 20 {
            Err(CategoryNameError::TooLong)
        } else {
            Ok(Self(value.to_string()))
        }
    }
}

impl FromStr for CategoryName {
    type Err = CategoryNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Display for CategoryName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_name_new_success() {
        let name = "あいうえおかきくけこさしすせそたちつてと";
        let result = CategoryName::new(name);
        assert!(result.is_ok());
        assert_eq!(name, result.unwrap().to_string())
    }

    #[test]
    fn test_category_name_new_empty_error() {
        let name = "";
        let result = CategoryName::new(name);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CategoryNameError::Empty))
    }

    #[test]
    fn test_category_name_new_too_long_error() {
        let name = "このカテゴリー名は長すぎて20文字を超えています";
        let result = CategoryName::new(name);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CategoryNameError::TooLong));
    }

    #[test]
    fn test_category_name_from_str_success() {
        let name = "テストカテゴリー";
        let result = CategoryName::from_str(name);
        assert!(result.is_ok());
        assert_eq!(name, result.unwrap().to_string());
    }

    #[test]
    fn test_category_name_from_str_empty_error() {
        let name = "";
        let result = CategoryName::from_str(name);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CategoryNameError::Empty));
    }

    #[test]
    fn test_category_name_from_str_too_long_error() {
        let name = "このカテゴリー名は長すぎて20文字を超えています";
        let result = CategoryName::from_str(name);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CategoryNameError::TooLong));
    }

    #[test]
    fn test_category_name_display() {
        let name = "テストカテゴリー";
        let category_name = CategoryName::new(name).unwrap();
        assert_eq!(name, category_name.to_string());
    }
}
