use std::fmt::{Display, Formatter};
use crate::AggregateId;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SubscribeId {
  value: Uuid
}

const SUBSCRIBE_PREFIX: &str = "SUBSCRIBE";

impl SubscribeId {
  /// コンストラクタ IDなし
  pub fn new() -> Self {
    let value = Self::generate_id();
    Self { value }
  }
}

impl AggregateId for SubscribeId {
  fn type_name(&self) -> String {
    SUBSCRIBE_PREFIX.to_string()
  }
  fn value(&self) -> String {
    self.value.to_string()
  }
  fn generate_id() -> Uuid {
    Uuid::new_v4()
  }
}

impl From<Uuid> for SubscribeId {
  /// 引数ありコンストラクタ
  ///
  /// # Argument
  /// - `value`: Uuid
  ///
  /// # Return
  /// Self
  fn from(value: Uuid) -> Self {
    Self { value }
  }
}

impl Display for SubscribeId {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}-{}", self.type_name(), self.value)
  }
}
