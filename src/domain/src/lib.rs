use std::str;

use thiserror::Error;
use uuid::Uuid;

mod payment;
pub mod payment_cycle;
pub mod subscribe;
pub mod user;
mod value_object;
pub mod repository;

/// 集約ID用のトレイトです
///
/// 各集約IDはAggregateIdを実装しなければなりません
///
/// - type_name: 集約の型を返します
/// - value: 値を返します
/// - generate_id: Uuidを生成します
pub trait AggregateId {
    fn type_name(&self) -> String;
    fn value(&self) -> &String;
}

#[derive(Debug, Error)]
pub enum AggregateIdError {
    #[error("It is not in the prefix + UUID format.")]
    InvalidFormat,

    #[error("Invalid UUID format")]
    InvalidUuid,
}

pub fn generate_id(p: &str, u: Option<Uuid>) -> String {
    match u {
        Some(u) => {
            format!("{}_{}", p, u)
        }
        None => {
            let value = Uuid::new_v4();
            format!("{}_{}", p, value)
        }
    }
}
