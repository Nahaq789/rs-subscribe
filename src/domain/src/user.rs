use crate::user::user_id::UserId;
use chrono::{DateTime, Utc};

pub mod user_id;

/// ユーザー情報を表す構造体
#[derive(Debug, Clone)]
pub struct User {
  /// ユーザーを一意に識別するID
  user_id: UserId,

  /// ユーザーが所属する国のID
  country_id: i32,

  /// ユーザーの作成日時（UTC）
  created_at: DateTime<Utc>,

  /// ユーザー情報の最終更新日時（UTC）
  updated_at: DateTime<Utc>,
}

impl User {
  /// 新しいユーザーを作成する
  ///
  /// # 引数
  /// * `country_id` - [i32] 国ID
  ///
  /// # 戻り値
  /// - [User] 作成されたユーザー
  pub fn new(country_id: i32) -> Self {
    let now = Utc::now();
    Self { user_id: UserId::new(), country_id, created_at: now, updated_at: now }
  }

  /// 既存のデータからユーザーを生成する
  ///
  /// # 引数
  /// * `user_id` - [UserId] ユーザーID
  /// * `country_id` - [i32] 国ID
  /// * `created_at` - [DateTime<Utc>] 作成日時
  /// * `updated_at` - [DateTime<Utc>] 更新日時
  ///
  /// # 戻り値
  /// - [User] 生成されたユーザー
  pub fn from(user_id: UserId, country_id: i32, created_at: DateTime<Utc>, updated_at: DateTime<Utc>) -> Self {
    Self { user_id, country_id, created_at, updated_at }
  }

  /// ユーザーIDを取得する
  ///
  /// # 戻り値
  /// - [&UserId] ユーザーIDへの参照
  pub fn user_id(&self) -> &UserId {
    &self.user_id
  }

  /// 国IDを取得する
  ///
  /// # 戻り値
  /// - [i32] 国ID
  pub fn country_id(&self) -> i32 {
    self.country_id
  }

  /// 作成日時を取得する
  ///
  /// # 戻り値
  /// - [&DateTime<Utc>] 作成日時への参照
  pub fn created_at(&self) -> &DateTime<Utc> {
    &self.created_at
  }

  /// 更新日時を取得する
  ///
  /// # 戻り値
  /// - [&DateTime<Utc>] 更新日時への参照
  pub fn updated_at(&self) -> &DateTime<Utc> {
    &self.updated_at
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::TimeZone;

  #[test]
  fn test_user_new_success() {
    let country_id = 1;
    let result = User::new(country_id);

    assert_eq!(country_id, result.country_id());
    assert!(result.created_at() <= &Utc::now());
    assert_eq!(result.created_at(), result.updated_at());
  }

  #[test]
  fn test_user_from_success() {
    let user_id = UserId::new();
    let country_id = 1;
    let created_at = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let updated_at = Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap();

    let result = User::from(user_id.clone(), country_id, created_at, updated_at);

    assert_eq!(&user_id, result.user_id());
    assert_eq!(country_id, result.country_id());
    assert_eq!(&created_at, result.created_at());
    assert_eq!(&updated_at, result.updated_at());
  }

  #[test]
  fn test_user_getters_success() {
    let user_id = UserId::new();
    let country_id = 1;
    let created_at = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let updated_at = Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap();

    let user = User::from(user_id.clone(), country_id, created_at, updated_at);

    assert_eq!(&user_id, user.user_id());
    assert_eq!(country_id, user.country_id());
    assert_eq!(&created_at, user.created_at());
    assert_eq!(&updated_at, user.updated_at());
  }
}
