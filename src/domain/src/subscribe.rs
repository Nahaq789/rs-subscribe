use chrono::{DateTime, Utc};
use crate::payment_cycle::PaymentCycle;
use crate::subscribe::subscribe_id::SubscribeId;
use crate::subscribe::subscribe_name::SubscribeName;
use crate::user::user_id::UserId;
use crate::value_object::amount::Amount;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use uuid::Uuid;

mod subscribe_error;
pub mod subscribe_id;
mod subscribe_name;

/// サブスク情報を管理する構造体
#[derive(Debug)]
pub struct Subscribe {
    /// サブスクID
    subscribe_id: SubscribeId,

    /// ユーザーID
    user_id: UserId,

    /// サブスク名
    name: SubscribeName,

    /// 金額
    amount: Amount,

    /// 支払周期
    payment_cycle: PaymentCycle,

    /// カテゴリID
    category_id: i32,

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
    status: u8,

    /// メモ欄
    memo: String
}

impl Subscribe {
    /// 新しいサブスクを作成する
    ///
    /// # 引数
    /// * `user_id` - [UserId] サブスクを作成するユーザーのID
    /// * `name` - [SubscribeName] サブスク名
    /// * `amount` - [Amount] 金額
    /// * `payment_cycle` - [PaymentCycle] 支払周期
    /// * `category_id` - カテゴリID
    /// * `icon_local_path` - アイコンのローカルパス
    /// * `notification` - 通知設定
    /// * `first_payment_date` - 初回支払日
    /// * `next_payment_date` - 次回支払予定日
    /// * `auto_renewal` - 自動更新フラグ
    /// * `status` - ステータス
    /// * `memo` - メモ欄
    ///
    /// # 戻り値
    /// - [Subscribe] 作成されたサブスク情報
    pub fn new(
        user_id: UserId,
        name: SubscribeName,
        amount: Amount,
        payment_cycle: PaymentCycle,
        category_id: i32,
        icon_local_path: String,
        notification: bool,
        first_payment_date: DateTime<Utc>,
        next_payment_date: DateTime<Utc>,
        auto_renewal: bool,
        status: u8,
        memo: String,
    ) -> Self {
        let id = SubscribeId::new();
        let amount = Self::yearly_amount_per_monthly(amount, &payment_cycle);
        let status = if status.is_zero() { 1 } else { status };
        Self {
            subscribe_id: id,
            user_id,
            name,
            amount,
            payment_cycle,
            category_id,
            icon_local_path,
            notification,
            first_payment_date,
            next_payment_date,
            auto_renewal,
            status,
            memo,
        }
    }

    /// 既存のIDからサブスクを作成する
    ///
    /// # 引数
    /// * `subscribe_id` - [SubscribeId] 既存のサブスクID
    /// * `user_id` - [UserId] サブスクに紐づくユーザーID
    /// * `name` - [SubscribeName] サブスク名
    /// * `amount` - [Amount] 金額
    /// * `payment_cycle` - [PaymentCycle] 支払周期
    /// * `category_id` - カテゴリID
    /// * `icon_local_path` - アイコンのローカルパス
    /// * `notification` - 通知設定
    /// * `first_payment_date` - 初回支払日
    /// * `next_payment_date` - 次回支払予定日
    /// * `auto_renewal` - 自動更新フラグ
    /// * `status` - ステータス
    /// * `memo` - メモ欄
    ///
    /// # 戻り値
    /// - [Subscribe] 作成されたサブスク情報
    pub fn from(
        subscribe_id: Uuid,
        user_id: UserId,
        name: SubscribeName,
        amount: Amount,
        payment_cycle: PaymentCycle,
        category_id: i32,
        icon_local_path: String,
        notification: bool,
        first_payment_date: DateTime<Utc>,
        next_payment_date: DateTime<Utc>,
        auto_renewal: bool,
        status: u8,
        memo: String,
    ) -> Self {
        let amount = Self::yearly_amount_per_monthly(amount, &payment_cycle);
        let status = if status.is_zero() { 1 } else { status };
        Self {
            subscribe_id: SubscribeId::from(subscribe_id),
            user_id,
            name,
            amount,
            payment_cycle,
            category_id,
            icon_local_path,
            notification,
            first_payment_date,
            next_payment_date,
            auto_renewal,
            status,
            memo,
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
    pub fn category_id(&self) -> &i32 { &self.category_id }

    /// アイコンのローカルパスを取得する
    ///
    /// # 戻り値
    /// - [String] アイコンのローカルパスへの参照
    pub fn icon_local_path(&self) -> &String { &self.icon_local_path }

    /// 通知設定を取得する
    ///
    /// # 戻り値
    /// - [bool] 通知設定
    pub fn notification(&self) -> bool { self.notification }

    /// 初回支払日を取得する
    ///
    /// # 戻り値
    /// - [DateTime<Utc>] 初回支払日への参照
    pub fn first_payment_date(&self) -> &DateTime<Utc> { &self.first_payment_date }

    /// 次回支払予定日を取得する
    ///
    /// # 戻り値
    /// - [DateTime<Utc>] 次回支払予定日への参照
    pub fn next_payment_date(&self) -> &DateTime<Utc> { &self.next_payment_date }

    /// 自動更新フラグを取得する
    ///
    /// # 戻り値
    /// - [bool] 自動更新フラグ
    pub fn auto_renewal(&self) -> bool { self.auto_renewal }

    /// ステータスを取得する
    ///
    /// # 戻り値
    /// - [u8] ステータス
    pub fn status(&self) -> u8 { self.status }

    /// メモを取得する
    ///
    /// # 戻り値
    /// - [String] メモへの参照
    pub fn memo(&self) -> &String { &self.memo }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use chrono::Utc;
    use crate::AggregateId;

    #[test]
    fn test_subscribe_new_success() {
        let user_id = UserId::new();
        let name = SubscribeName::new("hoge").unwrap();
        let amount = Amount::try_from(Decimal::ONE_HUNDRED).unwrap();
        let now = Utc::now();

        let result = Subscribe::new(
            user_id,
            name,
            amount,
            PaymentCycle::Monthly,
            1,
            String::from("/path/to/icon"),
            true,
            now,
            now,
            true,
            1,
            String::from("テストメモ"),
        );

        assert!(!result.subscribe_id.to_string().is_empty());
    }

    #[test]
    fn test_subscribe_from_success() {
        let id = Uuid::new_v4();
        let user_id = UserId::new();
        let name = SubscribeName::new("hoge").unwrap();
        let amount = Amount::try_from(Decimal::ONE_HUNDRED).unwrap();
        let now = Utc::now();

        let result = Subscribe::from(
            id,
            user_id,
            name,
            amount,
            PaymentCycle::Yearly,
            1,
            String::from("/path/to/icon"),
            true,
            now,
            now,
            true,
            1,
            String::from("テストメモ"),
        );

        assert!(!result.subscribe_id.to_string().is_empty());
        assert_eq!(
            format!("SUBSCRIBE-{}", id.to_string()),
            result.subscribe_id.to_string()
        )
    }

    #[test]
    fn test_getters() {
        let subscribe_id = Uuid::new_v4();
        let user_id = UserId::new();
        let name = SubscribeName::new("hoge").unwrap();
        let amount = Amount::try_from(Decimal::ONE_HUNDRED).unwrap();
        let now = Utc::now();
        let category_id = 1;
        let icon_path = String::from("/path/to/icon");
        let notification = true;
        let auto_renewal = true;
        let status = 1;
        let memo = String::from("テストメモ");

        let subscribe = Subscribe::from(
            subscribe_id.clone(),
            user_id.clone(),
            name.clone(),
            amount.clone(),
            PaymentCycle::Monthly,
            category_id,
            icon_path.clone(),
            notification,
            now,
            now,
            auto_renewal,
            status,
            memo.clone(),
        );

        assert_eq!(subscribe.subscribe_id().value().as_str(), &subscribe_id.to_string());
        assert_eq!(subscribe.user_id(), &user_id);
        assert_eq!(subscribe.name(), &name);
        assert_eq!(subscribe.amount(), &amount);
        assert_eq!(subscribe.category_id(), &category_id);
        assert_eq!(subscribe.icon_local_path(), &icon_path);
        assert_eq!(subscribe.notification(), notification);
        assert_eq!(subscribe.first_payment_date(), &now);
        assert_eq!(subscribe.next_payment_date(), &now);
        assert_eq!(subscribe.auto_renewal(), auto_renewal);
        assert_eq!(subscribe.status(), status);
        assert_eq!(subscribe.memo(), &memo);
    }
}