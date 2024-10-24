use uuid::Uuid;
use crate::subscribe::subscribe_id::SubscribeId;
use crate::subscribe::subscribe_name::SubscribeName;
use crate::user::user_id::UserId;

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
    name: SubscribeName
}

impl Subscribe {
    /// 新しいサブスクを作成する
    ///
    /// # 引数
    /// * `user_id` - [UserId] サブスクを作成するユーザーのID
    /// * `name` - [SubscribeName] サブスク名
    ///
    /// # 戻り値
    /// - [Subscribe] 作成されたサブスク情報
    pub fn new(user_id: UserId, name: SubscribeName) -> Self {
        let id = SubscribeId::new();
        Self {
            subscribe_id: id,
            user_id,
            name
        }
    }

    /// 既存のIDからサブスクを作成する
    ///
    /// # 引数
    /// * `subscribe_id` - [SubscribeId] 既存のサブスクID
    /// * `user_id` - [UserId] サブスクに紐づくユーザーID
    /// * `name` - [SubscribeName] サブスク名
    ///
    /// # 戻り値
    /// - [Subscribe] 作成されたサブスク情報
    pub fn from(subscribe_id: Uuid, user_id: UserId, name: SubscribeName) -> Self {
        Self {
            subscribe_id: SubscribeId::from(subscribe_id),
            user_id,
            name
        }
    }

    /// サブスクIDを取得する
    ///
    /// # 戻り値
    /// - [SubscribeId] サブスクIDへの参照
    pub fn subscribe_id(&self) -> &SubscribeId {
        &self.subscribe_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscribe_new_success() {
        let user_id = UserId::new();
        let name = SubscribeName::new("hoge").unwrap();
        let result = Subscribe::new(user_id, name);
        assert!(!result.subscribe_id.to_string().is_empty());
    }

    #[test]
    fn test_subscribe_from_success() {
        let id = Uuid::new_v4();
        let user_id = UserId::new();
        let name = SubscribeName::new("hoge").unwrap();
        let result = Subscribe::from(id, user_id, name);
        assert!(!result.subscribe_id.to_string().is_empty());
        assert_eq!(format!("SUBSCRIBE-{}", id.to_string()), result.subscribe_id.to_string())
    }
}