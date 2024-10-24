use crate::AggregateId;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

/// サブスクの一意識別子を表す構造体
#[derive(Debug, Clone)]
pub struct SubscribeId {
    /// UUIDの値
    value: Uuid,
}

const SUBSCRIBE_PREFIX: &str = "SUBSCRIBE";

impl SubscribeId {
    /// 新しいサブスクIDを生成する
    ///
    /// # 戻り値
    /// - [SubscribeId] 生成されたサブスクID
    pub fn new() -> Self {
        let value = Self::generate_id();
        Self { value }
    }
}

impl AggregateId for SubscribeId {
    /// プレフィックスを取得する
    ///
    /// # 戻り値
    /// - [String] "SUBSCRIBE"という文字列
    fn type_name(&self) -> String {
        SUBSCRIBE_PREFIX.to_string()
    }

    /// IDの値を取得する
    ///
    /// # 戻り値
    /// - [String] UUID文字列
    fn value(&self) -> String {
        self.value.to_string()
    }

    /// 新しいUUIDを生成する
    ///
    /// # 戻り値
    /// - [Uuid] 生成されたUUID
    fn generate_id() -> Uuid {
        Uuid::new_v4()
    }
}

impl From<Uuid> for SubscribeId {
    /// UUIDからサブスクIDを生成する
    ///
    /// # 引数
    /// * `value` - [Uuid] 変換元のUUID
    ///
    /// # 戻り値
    /// - [SubscribeId] 生成されたサブスクID
    fn from(value: Uuid) -> Self {
        Self { value }
    }
}

impl Display for SubscribeId {
    /// 文字列表現を取得する
    ///
    /// # 引数
    /// * `f` - [Formatter] フォーマッター
    ///
    /// # 戻り値
    /// - [std::fmt::Result] フォーマット結果
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.type_name(), self.value)
    }
}