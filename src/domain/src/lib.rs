use uuid::Uuid;

pub mod subscribe;

/// 集約ID用のトレイトです
///
/// 各集約IDはAggregateIdを実装しなければなりません
///
/// - type_name: 集約の型を返します
/// - value: 値を返します
/// - generate_id: Uuidを生成します
pub trait AggregateId {
  fn type_name(&self) -> String;
  fn value(&self) -> String;
  fn generate_id() -> Uuid;
}