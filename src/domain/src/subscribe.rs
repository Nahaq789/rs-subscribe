use rust_decimal::Decimal;
use uuid::Uuid;
use crate::payment_cycle::PaymentCycle;
use crate::subscribe::subscribe_id::SubscribeId;
use crate::subscribe::subscribe_name::SubscribeName;
use crate::user::user_id::UserId;
use crate::value_object::amount::Amount;

pub mod subscribe_id;
mod subscribe_name;
mod subscribe_error;

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
    payment_cycle: PaymentCycle
}

impl Subscribe {
    /// 新しいサブスクを作成する
    ///
    /// # 引数
    /// * `user_id` - [UserId] サブスクを作成するユーザーのID
    /// * `name` - [SubscribeName] サブスク名
    /// * `amount` - [Amount] 金額
    /// * `payment_cycle` - [PaymentCycle] 支払周期
    ///
    /// # 戻り値
    /// - [Subscribe] 作成されたサブスク情報
    pub fn new(user_id: UserId, name: SubscribeName, amount: Amount, payment_cycle: PaymentCycle) -> Self {
        let id = SubscribeId::new();
        let amount = Self::yearly_amount_per_monthly(amount, &payment_cycle);
        Self {
            subscribe_id: id,
            user_id,
            name,
            amount,
            payment_cycle
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
    ///
    /// # 戻り値
    /// - [Subscribe] 作成されたサブスク情報
    pub fn from(subscribe_id: Uuid, user_id: UserId, name: SubscribeName, amount: Amount, payment_cycle: PaymentCycle) -> Self {
        let amount = Self::yearly_amount_per_monthly(amount, &payment_cycle);
        Self {
            subscribe_id: SubscribeId::from(subscribe_id),
            user_id,
            name,
            amount,
            payment_cycle
        }
    }

    fn yearly_amount_per_monthly(amount: Amount, cycle: &PaymentCycle) -> Amount {
        match cycle {
            PaymentCycle::Yearly => {
                let value = amount.value() / Decimal::from(12);
                Amount::try_from(value).unwrap()
            }
            _ => amount
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
    pub fn user_id(&self) -> &UserId { &self.user_id }

    /// サブスク名を取得する
    ///
    /// # 戻り値
    /// - [SubscribeName] サブスク名への参照
    pub fn name(&self) -> &SubscribeName { &self.name  }

    /// 金額を取得する
    ///
    /// # 戻り値
    /// - [Amount] 金額への参照
    pub fn amount(&self) -> &Amount { &self.amount }

    /// 支払周期を取得する
    ///
    /// # 戻り値
    /// - [PaymentCycle] 支払周期への参照
    pub fn payment_cycle(&self) -> &PaymentCycle { &self.payment_cycle }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use super::*;

    #[test]
    fn test_subscribe_new_success() {
        let user_id = UserId::new();
        let name = SubscribeName::new("hoge").unwrap();
        let amount = Amount::try_from(Decimal::ONE_HUNDRED).unwrap();
        let result = Subscribe::new(user_id, name, amount, PaymentCycle::Monthly);
        assert!(!result.subscribe_id.to_string().is_empty());
    }

    #[test]
    fn test_subscribe_from_success() {
        let id = Uuid::new_v4();
        let user_id = UserId::new();
        let name = SubscribeName::new("hoge").unwrap();
        let amount = Amount::try_from(Decimal::ONE_HUNDRED).unwrap();
        let result = Subscribe::from(id, user_id, name, amount, PaymentCycle::Yearly);
        assert!(!result.subscribe_id.to_string().is_empty());
        assert_eq!(format!("SUBSCRIBE-{}", id.to_string()), result.subscribe_id.to_string())
    }
}