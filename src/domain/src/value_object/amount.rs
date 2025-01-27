use rust_decimal::Decimal;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use thiserror::Error;

/// 金額を表す値オブジェクト
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Amount {
    /// Decimal型で保持する金額値
    value: Decimal,
}

/// 金額に関するエラー
#[derive(Debug, Clone, Error)]
pub enum AmountError {
    #[error("Amount must be greater than 0")]
    ZeroOrLess,

    #[error("Amount can not parse into Decimal")]
    ParseError,
}

impl Display for Amount {
    /// 金額の文字列表現を取得する
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

impl TryFrom<Decimal> for Amount {
    type Error = AmountError;

    /// Decimal型から金額を生成する
    ///
    /// # 引数
    /// * `value` - [Decimal] 金額値
    ///
    /// # 戻り値
    /// - [Result<Amount, AmountError>] 生成結果
    ///
    /// # エラー
    /// - [AmountError] 金額が0以下の場合
    fn try_from(value: Decimal) -> Result<Self, Self::Error> {
        if value <= Decimal::from(0) {
            Err(AmountError::ZeroOrLess)?
        };
        Ok(Self::new(value))
    }
}

impl FromStr for Amount {
    type Err = AmountError;

    /// str型から金額を生成する
    ///
    /// # 引数
    /// * `value` - [&str] 金額値
    ///
    /// # 戻り値
    /// - [Result<Amount, AmountError>] 生成結果
    ///
    /// # エラー
    /// - [AmountError] パースエラー
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<Decimal>().map_err(|_| AmountError::ParseError)?;
        Ok(Self::try_from(value)?)
    }
}

impl Amount {
    /// 新しい金額を生成する（内部用）
    ///
    /// # 引数
    /// * `value` - [Decimal] 金額値
    ///
    /// # 戻り値
    /// - [Amount] 生成された金額
    fn new(value: Decimal) -> Self {
        Self { value }
    }

    /// 金額値を取得する
    ///
    /// # 戻り値
    /// - [&Decimal] 金額値への参照
    pub fn value(&self) -> &Decimal {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1)]
    #[case(100)]
    fn test_amount_new_success(#[case] value: i32) {
        let result = Amount::new(Decimal::from(value));
        assert_eq!(Decimal::from(value), result.value)
    }

    #[test]
    fn test_amount_try_from_success() {
        let value = Decimal::from(1);
        let result = Amount::try_from(value);

        // assert
        assert!(result.is_ok());
        assert_eq!(value, result.unwrap().value)
    }

    #[test]
    fn test_amount_try_from_failed() {
        let value = Decimal::ZERO;
        let result = Amount::try_from(value);

        //assert
        assert!(result.is_err())
    }

    #[rstest]
    #[case(100)]
    #[case(50)]
    fn test_amount_value_success(#[case] value: i32) {
        let result = Amount::try_from(Decimal::from(value));

        // assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), &Decimal::from(value))
    }

    #[rstest]
    #[case("100")]
    #[case("1")]
    fn test_amount_from_str_success(#[case] value: &str) {
        let result = Amount::from_str(value);

        //assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), &Decimal::from(i32::from_str(value).unwrap()))
    }
}
