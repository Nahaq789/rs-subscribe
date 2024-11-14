use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PaymentCycle {
  Monthly,
  Yearly,
}

impl PaymentCycle {
  /// 支払いサイクルを文字列に変換する
  ///
  /// # 戻り値
  /// - [&str] 支払いサイクルを表す文字列
  pub fn as_str(&self) -> &str {
    match self {
      PaymentCycle::Monthly => "monthly",
      PaymentCycle::Yearly => "yearly",
    }
  }
}

impl FromStr for PaymentCycle {
  type Err = ();

  /// 文字列から支払いサイクルを生成する
  ///
  /// # 引数
  /// * `s` - [&str] "monthly" または "yearly" の文字列
  ///
  /// # 戻り値
  /// - [Option<PaymentCycle>] 生成された支払いサイクル。該当しない文字列の場合は [PaymentCycle::Monthly]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "monthly" => Ok(Self::Monthly),
      "yearly" => Ok(Self::Yearly),
      _ => Ok(Self::Monthly),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_payment_cycle_from_str_monthly() {
    let result = PaymentCycle::from_str("monthly");
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), PaymentCycle::Monthly));
  }

  #[test]
  fn test_payment_cycle_from_str_yearly() {
    let result = PaymentCycle::from_str("yearly");
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), PaymentCycle::Yearly));
  }

  #[test]
  fn test_payment_cycle_from_str_invalid() {
    let result = PaymentCycle::from_str("invalid");
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), PaymentCycle::Monthly))
  }

  #[test]
  fn test_payment_cycle_from_str_case_insensitive() {
    let result = PaymentCycle::from_str("MONTHLY");
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), PaymentCycle::Monthly));
  }

  #[test]
  fn test_payment_cycle_as_str_monthly() {
    let cycle = PaymentCycle::Monthly;
    assert_eq!("monthly", cycle.as_str());
  }

  #[test]
  fn test_payment_cycle_as_str_yearly() {
    let cycle = PaymentCycle::Yearly;
    assert_eq!("yearly", cycle.as_str());
  }
}
