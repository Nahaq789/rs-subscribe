use crate::category::category_id;
use crate::subscribe::subscribe_id::SubscribeId;
use crate::subscribe::subscribe_name::SubscribeName;
use crate::subscribe::subscribe_status::SubscribeStatus;
use crate::user::user_id::UserId;
use crate::value_object::amount::Amount;
use crate::{payment::payment_method_id::PaymentMethodId, payment_cycle::PaymentCycle};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub mod subscribe_error;
pub mod subscribe_id;
pub mod subscribe_name;
pub mod subscribe_status;

/// サブスク情報を管理する構造体
#[derive(Debug)]
pub struct Subscribe {
  /// サブスクID
  subscribe_id: SubscribeId,

  /// ユーザーID
  user_id: UserId,

  /// サブスク名
  name: SubscribeName,

  /// 支払方法ID
  payment_method_id: PaymentMethodId,

  /// 金額
  amount: Amount,

  /// 支払周期
  payment_cycle: PaymentCycle,

  /// カテゴリID
  category_id: category_id::CategoryId,

  /// アイコンのローカルパス
  icon_local_path: String,

  /// 通知設定
  notification: bool,

  /// 初回支払日
  first_payment_date: DateTime<Utc>,

  /// 次回支払予定日
  next_payment_date: DateTime<Utc>,

  /// 自動更新フラグ
  auto_renewal: bool,

  /// ステータス
  status: SubscribeStatus,

  /// メモ欄
  memo: String,
}

impl Subscribe {
  /// 新しいサブスクを作成する
  ///
  /// # 引数
  /// * `user_id` - [UserId] サブスクを作成するユーザーのID
  /// * `name` - [SubscribeName] サブスク名
  /// * `payment_method_id` - [PaymentMethodId] 支払方法ID
  /// * `amount` - [Amount] 金額
  /// * `payment_cycle` - [PaymentCycle] 支払周期
  /// * `category_id` - カテゴリID
  /// * `icon_local_path` - アイコンのローカルパス
  /// * `notification` - 通知設定
  /// * `first_payment_date` - 初回支払日
  /// * `next_payment_date` - 次回支払予定日
  /// * `auto_renewal` - 自動更新フラグ
  /// * `status` - [SubscribeStatus] ステータス
  /// * `memo` - メモ欄
  ///
  /// # 戻り値
  /// - [Subscribe] 作成されたサブスク情報
  pub fn new(
    user_id: UserId,
    name: SubscribeName,
    payment_method_id: PaymentMethodId,
    amount: Amount,
    payment_cycle: PaymentCycle,
    category_id: category_id::CategoryId,
    icon_local_path: String,
    notification: bool,
    first_payment_date: DateTime<Utc>,
    next_payment_date: DateTime<Utc>,
    auto_renewal: bool,
    status: SubscribeStatus,
    memo: &str,
  ) -> Self {
    let id = SubscribeId::new();
    let amount = Self::yearly_amount_per_monthly(amount, &payment_cycle);
    Self {
      subscribe_id: id,
      user_id,
      name,
      payment_method_id,
      amount,
      payment_cycle,
      category_id,
      icon_local_path,
      notification,
      first_payment_date,
      next_payment_date,
      auto_renewal,
      status,
      memo: memo.to_owned(),
    }
  }

  /// 既存のIDからサブスクを作成する
  ///
  /// # 引数
  /// * `subscribe_id` - [SubscribeId] 既存のサブスクID
  /// * `user_id` - [UserId] サブスクに紐づくユーザーID
  /// * `payment_method_id` - [PaymentMethodId] 支払方法ID
  /// * `name` - [SubscribeName] サブスク名
  /// * `amount` - [Amount] 金額
  /// * `payment_cycle` - [PaymentCycle] 支払周期
  /// * `category_id` - カテゴリID
  /// * `icon_local_path` - アイコンのローカルパス
  /// * `notification` - 通知設定
  /// * `first_payment_date` - 初回支払日
  /// * `next_payment_date` - 次回支払予定日
  /// * `auto_renewal` - 自動更新フラグ
  /// * `status` - [SubscribeStatus] ステータス
  /// * `memo` - メモ欄
  ///
  /// # 戻り値
  /// - [Subscribe] 作成されたサブスク情報
  pub fn from(
    subscribe_id: SubscribeId,
    user_id: UserId,
    name: SubscribeName,
    payment_method_id: PaymentMethodId,
    amount: Amount,
    payment_cycle: PaymentCycle,
    category_id: category_id::CategoryId,
    icon_local_path: String,
    notification: bool,
    first_payment_date: DateTime<Utc>,
    next_payment_date: DateTime<Utc>,
    auto_renewal: bool,
    status: SubscribeStatus,
    memo: &str,
  ) -> Self {
    let amount = Self::yearly_amount_per_monthly(amount, &payment_cycle);
    Self {
      subscribe_id,
      user_id,
      name,
      payment_method_id,
      amount,
      payment_cycle,
      category_id,
      icon_local_path,
      notification,
      first_payment_date,
      next_payment_date,
      auto_renewal,
      status,
      memo: memo.to_owned(),
    }
  }

  fn yearly_amount_per_monthly(amount: Amount, cycle: &PaymentCycle) -> Amount {
    match cycle {
      PaymentCycle::Yearly => {
        let value = amount.value() / Decimal::from(12);
        Amount::try_from(value).unwrap()
      }
      _ => amount,
    }
  }

  /// サブスクIDを取得する
  ///
  /// # 戻り値
  /// - [SubscribeId] サブスクIDへの参照
  pub fn subscribe_id(&self) -> &SubscribeId {
    &self.subscribe_id
  }

  /// ユーザーIDを取得する
  ///
  /// # 戻り値
  /// - [UserId] ユーザーIDへの参照
  pub fn user_id(&self) -> &UserId {
    &self.user_id
  }

  /// サブスク名を取得する
  ///
  /// # 戻り値
  /// - [SubscribeName] サブスク名への参照
  pub fn name(&self) -> &SubscribeName {
    &self.name
  }

  /// 支払方法IDを取得する
  ///
  /// # 戻り値
  /// - [PaymentMethodId] 支払方法IDへの参照
  pub fn payment_method_id(&self) -> &PaymentMethodId {
    &self.payment_method_id
  }

  /// 金額を取得する
  ///
  /// # 戻り値
  /// - [Amount] 金額への参照
  pub fn amount(&self) -> &Amount {
    &self.amount
  }

  /// 支払周期を取得する
  ///
  /// # 戻り値
  /// - [PaymentCycle] 支払周期への参照
  pub fn payment_cycle(&self) -> &PaymentCycle {
    &self.payment_cycle
  }

  /// カテゴリIDを取得する
  ///
  /// # 戻り値
  /// - [i32] カテゴリIDへの参照
  pub fn category_id(&self) -> &category_id::CategoryId {
    &self.category_id
  }

  /// アイコンのローカルパスを取得する
  ///
  /// # 戻り値
  /// - [String] アイコンのローカルパスへの参照
  pub fn icon_local_path(&self) -> &String {
    &self.icon_local_path
  }

  /// 通知設定を取得する
  ///
  /// # 戻り値
  /// - [bool] 通知設定
  pub fn notification(&self) -> bool {
    self.notification
  }

  /// 初回支払日を取得する
  ///
  /// # 戻り値
  /// - [DateTime<Utc>] 初回支払日への参照
  pub fn first_payment_date(&self) -> &DateTime<Utc> {
    &self.first_payment_date
  }

  /// 次回支払予定日を取得する
  ///
  /// # 戻り値
  /// - [DateTime<Utc>] 次回支払予定日への参照
  pub fn next_payment_date(&self) -> &DateTime<Utc> {
    &self.next_payment_date
  }

  /// 自動更新フラグを取得する
  ///
  /// # 戻り値
  /// - [bool] 自動更新フラグ
  pub fn auto_renewal(&self) -> bool {
    self.auto_renewal
  }

  /// ステータスを取得する
  ///
  /// # 戻り値
  /// - [SubscribeStatus] ステータス
  pub fn status(&self) -> &SubscribeStatus {
    &self.status
  }

  /// メモを取得する
  ///
  /// # 戻り値
  /// - [String] メモへの参照
  pub fn memo(&self) -> &String {
    &self.memo
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::AggregateId;
  use chrono::Utc;
  use rstest::rstest;
  use rust_decimal::Decimal;

  #[test]
  fn test_subscribe_new_success() {
    let user_id = UserId::new();
    let name = SubscribeName::new("hoge").unwrap();
    let payment_method_id = PaymentMethodId::new();
    let category_id = category_id::CategoryId::new();
    let amount = Amount::try_from(Decimal::ONE_HUNDRED).unwrap();
    let now = Utc::now();

    let result = Subscribe::new(
      user_id,
      name,
      payment_method_id,
      amount,
      PaymentCycle::Monthly,
      category_id,
      String::from("/path/to/icon"),
      true,
      now,
      now,
      true,
      SubscribeStatus::ACTIVE,
      "テストメモ",
    );

    assert!(!result.subscribe_id.to_string().is_empty());
  }

  #[test]
  fn test_subscribe_from_success() {
    let id = SubscribeId::new();
    let user_id = UserId::new();
    let name = SubscribeName::new("hoge").unwrap();
    let payment_method_id = PaymentMethodId::new();
    let category_id = category_id::CategoryId::new();
    let amount = Amount::try_from(Decimal::ONE_HUNDRED).unwrap();
    let now = Utc::now();

    let result = Subscribe::from(
      id.clone(),
      user_id,
      name,
      payment_method_id,
      amount,
      PaymentCycle::Yearly,
      category_id,
      String::from("/path/to/icon"),
      true,
      now,
      now,
      true,
      SubscribeStatus::ACTIVE,
      "テストメモ",
    );

    assert!(!result.subscribe_id.to_string().is_empty());
    assert_eq!(id.to_string(), result.subscribe_id.to_string())
  }

  #[test]
  fn test_getters() {
    let subscribe_id = SubscribeId::new();
    let user_id = UserId::new();
    let name = SubscribeName::new("hoge").unwrap();
    let payment_method_id = PaymentMethodId::new();
    let amount = Amount::try_from(Decimal::ONE_HUNDRED).unwrap();
    let payment_cycle = PaymentCycle::Monthly;
    let now = Utc::now();
    let category_id = category_id::CategoryId::new();
    let icon_path = String::from("/path/to/icon");
    let notification = true;
    let auto_renewal = true;
    let status = SubscribeStatus::ACTIVE;
    let memo = "テストメモ";

    let subscribe = Subscribe::from(
      subscribe_id.clone(),
      user_id.clone(),
      name.clone(),
      payment_method_id.clone(),
      amount.clone(),
      payment_cycle.clone(),
      category_id.clone(),
      icon_path.clone(),
      notification,
      now,
      now,
      auto_renewal,
      status.clone(),
      memo,
    );

    assert_eq!(
      subscribe.subscribe_id().value().as_str(),
      &subscribe_id.to_string()
    );
    assert_eq!(subscribe.user_id(), &user_id);
    assert_eq!(subscribe.name(), &name);
    assert_eq!(subscribe.amount(), &amount);
    assert_eq!(subscribe.payment_method_id(), &payment_method_id);
    assert_eq!(subscribe.payment_cycle(), &payment_cycle);
    assert_eq!(subscribe.category_id(), &category_id);
    assert_eq!(subscribe.icon_local_path(), &icon_path);
    assert_eq!(subscribe.notification(), notification);
    assert_eq!(subscribe.first_payment_date(), &now);
    assert_eq!(subscribe.next_payment_date(), &now);
    assert_eq!(subscribe.auto_renewal(), auto_renewal);
    assert_eq!(subscribe.status(), &status);
    assert_eq!(subscribe.memo(), &memo);
  }

  #[rstest]
  #[case(1000, PaymentCycle::Yearly)]
  #[case(1000, PaymentCycle::Monthly)]
  fn test_yearly_amount_per_monthly(#[case] a: i32, #[case] b: PaymentCycle) {
    let dec = Decimal::from(a);
    let amount = Amount::try_from(dec).unwrap();
    let result = Subscribe::yearly_amount_per_monthly(amount, &b);

    match b {
      PaymentCycle::Monthly => {
        assert_eq!(result.value(), &dec)
      }
      PaymentCycle::Yearly => {
        let u = dec / Decimal::from(12);
        assert_eq!(result.value(), &u)
      }
    }
  }
}
