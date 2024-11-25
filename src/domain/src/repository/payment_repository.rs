use crate::payment::payment_error::PaymentError;
use crate::payment::payment_method_id::PaymentMethodId;
use crate::payment::PaymentMethod;
use crate::user::user_id::UserId;
use async_trait::async_trait;

/// 支払い方法を管理するリポジトリのトレイト定義
#[async_trait]
pub trait PaymentRepository: Send + Sync {
  /// 支払い方法を新規作成する
  ///
  /// # Arguments
  /// * `payment` - 作成する支払い方法の情報
  /// * `table` - 保存先のテーブル名
  ///
  /// # Returns
  /// * `Ok(())` - void
  /// * `Err(PaymentError)` - 作成処理が失敗した場合のエラー
  async fn create(
    &self,
    payment: &PaymentMethod,
    user_id: &UserId,
    table: &str,
  ) -> Result<(), PaymentError>;

  /// ユーザーIDに紐づく全ての支払い方法を取得する
  ///
  /// # Arguments
  /// * `user_id` - 取得対象のユーザーID
  /// * `table` - 検索対象のテーブル名
  ///
  /// # Returns
  /// * `Ok(Vec<PaymentMethod>)` - 取得された支払い方法のリスト
  /// * `Err(PaymentError)` - 取得処理が失敗した場合のエラー
  async fn find_all(
    &self,
    user_id: &UserId,
    table: &str,
  ) -> Result<Vec<PaymentMethod>, PaymentError>;

  /// 指定されたIDの支払い方法を取得する
  ///
  /// # Arguments
  /// * `payment_id` - 取得する支払い方法のID
  /// * `table` - 検索対象のテーブル名
  ///
  /// # Returns
  /// * `Ok(PaymentMethod)` - 取得された支払い方法の情報
  /// * `Err(PaymentError)` - 取得処理が失敗した場合のエラー
  async fn find_by_id(
    &self,
    payment_id: &PaymentMethodId,
    table: &str,
  ) -> Result<PaymentMethod, PaymentError>;

  /// 支払い方法の情報を更新する
  ///
  /// # Arguments
  /// * `payment` - 更新する支払い方法の情報
  /// * `table` - 更新対象のテーブル名
  ///
  /// # Returns
  /// * `Ok(())` - void
  /// * `Err(PaymentError)` - 更新処理が失敗した場合のエラー
  async fn update(&self, payment: &PaymentMethod, table: &str) -> Result<(), PaymentError>;

  /// 支払い方法を削除する
  ///
  /// # Arguments
  /// * `payment_id` - 削除する支払い方法のID
  /// * `table` - 削除対象のテーブル名
  /// * `key` - 削除に使用する認証キー
  ///
  /// # Returns
  /// * `Ok(())` - 削除処理が成功した場合
  /// * `Err(PaymentError)` - 削除処理が失敗した場合のエラー
  async fn delete(
    &self,
    payment_id: &PaymentMethodId,
    table: &str,
    key: &str,
  ) -> Result<(), PaymentError>;
}
