use core::fmt;
use std::fmt::{write, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaymentError {
  hoge,
}

impl fmt::Display for PaymentError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "hoge")
  }
}
