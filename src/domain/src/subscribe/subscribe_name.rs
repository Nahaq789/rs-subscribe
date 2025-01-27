use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

/// サブスクリプションの名前を表す値オブジェクト
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SubscribeName(String);

/// サブスクリプション名に関するエラー
#[derive(Error, Debug, Clone)]
pub enum SubscribeNameError {
    /// 名前が空の場合のエラー
    #[error("the subscribe name is empty")]
    Empty,
    /// 名前が長すぎる場合のエラー
    #[error("the subscribe name is too long")]
    TooLong,
}

impl SubscribeName {
    /// 新しいサブスクリプション名を生成する
    ///
    /// # 引数
    /// * `value` - [&str] サブスクリプション名
    ///
    /// # 戻り値
    /// - [Result<SubscribeName, SubscribeNameError>] 生成結果
    ///
    /// # エラー
    /// - [SubscribeNameError::Empty] 名前が空の場合
    /// - [SubscribeNameError::TooLong] 名前が20文字を超える場合
    pub fn new(value: &str) -> Result<Self, SubscribeNameError> {
        if value.is_empty() {
            Err(SubscribeNameError::Empty)
        } else if value.chars().count() > 20 {
            Err(SubscribeNameError::TooLong)
        } else {
            Ok(Self(value.to_string()))
        }
    }
}

impl FromStr for SubscribeName {
    type Err = SubscribeNameError;

    /// 文字列からサブスクリプション名を生成する
    ///
    /// # 引数
    /// * `s` - [&str] 変換元の文字列
    ///
    /// # 戻り値
    /// - Ok [SubscribeName]
    /// - Err [SubscribeNameError]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Display for SubscribeName {
    /// 文字列表現を取得する
    ///
    /// # 引数
    /// * `f` - [Formatter] フォーマッター
    ///
    /// # 戻り値
    /// - [std::fmt::Result] フォーマット結果
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscribe_name_new_success() {
        let name = "あいうえおかきくけこさしすせそたちつてと";
        let result = SubscribeName::new(name);
        assert!(result.is_ok());
        assert_eq!(name, result.unwrap().to_string());
    }

    #[test]
    fn test_subscribe_name_new_empty_error() {
        let name = "";
        let result = SubscribeName::new(name);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SubscribeNameError::Empty));
    }

    #[test]
    fn test_subscribe_name_new_too_long_error() {
        let name = "このサブスクリプション名は長すぎて20文字を超えています";
        let result = SubscribeName::new(name);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SubscribeNameError::TooLong));
    }

    #[test]
    fn test_subscribe_name_from_str_success() {
        let name = "テストサブスクリプション";
        let result = SubscribeName::from_str(name);
        assert!(result.is_ok());
        assert_eq!(name, result.unwrap().to_string());
    }

    #[test]
    fn test_subscribe_name_from_str_empty_error() {
        let name = "";
        let result = SubscribeName::from_str(name);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SubscribeNameError::Empty));
    }

    #[test]
    fn test_subscribe_name_from_str_too_long_error() {
        let name = "このサブスクリプション名は長すぎて20文字を超えています";
        let result = SubscribeName::from_str(name);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SubscribeNameError::TooLong));
    }

    #[test]
    fn test_subscribe_name_display() {
        let name = "テストサブスクリプション";
        let subscribe_name = SubscribeName::new(name).unwrap();
        assert_eq!(name, subscribe_name.to_string());
    }
}
