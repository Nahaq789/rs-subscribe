use crate::{generate_id, AggregateId, AggregateIdError};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use uuid::Uuid;

/// ユーザーの一意識別子を表す構造体
///
/// フォーマット: "usr_<uuid>"
/// 例: "usr_550e8400-e29b-41d4-a716-446655440000"
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserId {
    /// UUIDの文字列
    value: String,
}

/// プレフィックス文字列
/// ユーザーIDの先頭に付与される識別子
const USER_PREFIX: &str = "usr";

impl UserId {
    /// 新しいユーザーIDを生成する
    ///
    /// # 戻り値
    /// - [UserId] 生成されたユーザーID
    pub fn new() -> Self {
        let value = generate_id(USER_PREFIX, None);
        Self { value }
    }
}

impl AggregateId for UserId {
    /// プレフィックスを取得する
    ///
    /// # 戻り値
    /// - [String] "pay"という文字列
    fn type_name(&self) -> String {
        USER_PREFIX.to_string()
    }

    /// IDの値を取得する
    ///
    /// # 戻り値
    /// - [String] UUID文字列
    fn value(&self) -> &String {
        &self.value
    }
}

impl From<Uuid> for UserId {
    /// UUIDからユーザーIDを生成する
    ///
    /// # 引数
    /// * `value` - [Uuid] 変換元のUUID
    ///
    /// # 戻り値
    /// - [UserId] 生成されたユーザーID
    fn from(value: Uuid) -> Self {
        Self {
            value: generate_id(USER_PREFIX, Some(value)),
        }
    }
}

impl Display for UserId {
    /// 文字列表現を取得する
    ///
    /// # 引数
    /// * `f` - [Formatter] フォーマッター
    ///
    /// # 戻り値
    /// - [std::fmt::Result] フォーマット結果
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for UserId {
    type Err = AggregateIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: Vec<&str> = s.split("_").collect();
        if value.len() != 2 {
            return Err(AggregateIdError::InvalidFormat);
        }
        if value[0] != USER_PREFIX {
            return Err(AggregateIdError::InvalidFormat);
        }
        let uuid = Uuid::parse_str(value[1]).map_err(|_| AggregateIdError::InvalidUuid)?;
        Ok(Self::from(uuid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_success() {
        let id = UserId::new();
        assert!(!id.value.is_empty());
        assert!(id.value.starts_with("usr_"));
    }

    #[test]
    fn test_type_name_success() {
        let id = UserId::new();
        assert!(!id.type_name().is_empty());
        assert_eq!(USER_PREFIX, id.type_name());
    }

    #[test]
    fn test_from_success() {
        let uuid = Uuid::new_v4();
        let format = format!("usr_{}", uuid);
        let result = UserId::from(uuid);
        assert!(result.value.starts_with("usr_"));
        assert_eq!(format, result.value);
    }

    #[test]
    fn test_from_str_success() {
        let uuid = Uuid::new_v4();
        let format = format!("usr_{}", uuid);
        let result = UserId::from_str(&format);
        assert!(result.is_ok());
        assert_eq!(format, result.unwrap().value);
    }

    #[test]
    fn test_display_format() {
        let id = UserId::new();
        let display_string = id.to_string();
        assert!(display_string.starts_with("usr_"));
    }

    #[test]
    fn test_clone_equality() {
        let id1 = UserId::new();
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_debug_format() {
        let id = UserId::new();
        let debug_string = format!("{:?}", id);
        assert!(!debug_string.is_empty());
    }

    #[test]
    fn test_from_str_failed_invalid_format() {
        let result = UserId::from_str("invalid");
        assert!(matches!(result, Err(AggregateIdError::InvalidFormat)));
    }

    #[test]
    fn test_from_str_failed_invalid_uuid() {
        let format = format!("usr_{}", "invalid-uuid");
        let result = UserId::from_str(&format);
        assert!(matches!(result, Err(AggregateIdError::InvalidUuid)));
    }

    #[test]
    fn test_from_str_failed_wrong_prefix() {
        let uuid = Uuid::new_v4();
        let format = format!("wrong_{}", uuid);
        let result = UserId::from_str(&format);
        assert!(matches!(result, Err(AggregateIdError::InvalidFormat)));
    }

    #[test]
    fn test_value_reference() {
        let id = UserId::new();
        let value_ref = id.value();
        assert_eq!(&id.value, value_ref);
    }

    #[test]
    fn test_multiple_instances_unique() {
        let id1 = UserId::new();
        let id2 = UserId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_from_str_empty_string() {
        let result = UserId::from_str("");
        assert!(matches!(result, Err(AggregateIdError::InvalidFormat)));
    }

    #[test]
    fn test_from_str_too_many_parts() {
        let result = UserId::from_str("usr_uuid_extra");
        assert!(matches!(result, Err(AggregateIdError::InvalidFormat)));
    }

    #[test]
    fn test_display_and_from_str_consistency() {
        let original = UserId::new();
        let display_string = original.to_string();
        assert!(display_string.contains(&original.value));
    }
}
