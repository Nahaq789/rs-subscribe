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
    ///
    /// # Returns
    /// * `Ok(())` - void
    /// * `Err(PaymentError)` - 作成処理が失敗した場合のエラー
    async fn create(&self, payment: &PaymentMethod) -> Result<(), PaymentError>;

    /// ユーザーIDに紐づく全ての支払い方法を取得する
    ///
    /// # Arguments
    /// * `user_id` - 取得対象のユーザーID
    ///
    /// # Returns
    /// * `Ok(Vec<PaymentMethod>)` - 取得された支払い方法のリスト
    /// * `Err(PaymentError)` - 取得処理が失敗した場合のエラー
    async fn find_all(&self, user_id: &UserId) -> Result<Vec<PaymentMethod>, PaymentError>;

    /// 指定されたIDの支払い方法を取得する
    ///
    /// # Arguments
    /// * `payment_id` - 取得する支払い方法のID
    /// * `user_id` - ユーザーID
    ///
    /// # Returns
    /// * `Ok(PaymentMethod)` - 取得された支払い方法の情報
    /// * `Err(PaymentError)` - 取得処理が失敗した場合のエラー
    async fn find_by_id(&self, payment_id: &PaymentMethodId, user_id: &UserId) -> Result<PaymentMethod, PaymentError>;

    /// 支払い方法の情報を更新する
    ///
    /// # Arguments
    /// * `payment` - 更新する支払い方法の情報
    ///
    /// # Returns
    /// * `Ok(())` - void
    /// * `Err(PaymentError)` - 更新処理が失敗した場合のエラー
    async fn update(&self, payment: &PaymentMethod) -> Result<(), PaymentError>;

    /// 支払い方法を削除する
    ///
    /// # Arguments
    /// * `payment_id` - 削除する支払い方法のID
    /// * `user_id` - ユーザーID
    ///
    /// # Returns
    /// * `Ok(())` - 削除処理が成功した場合
    /// * `Err(PaymentError)` - 削除処理が失敗した場合のエラー
    async fn delete(&self, payment_id: &PaymentMethodId, user_id: &UserId) -> Result<(), PaymentError>;

    /// データが存在するかチェック
    ///
    /// # Arguments
    /// * `payment_id` - 削除する支払い方法のID
    /// * `user_id` - ユーザーID
    ///
    /// # Returns
    /// * `Ok(true)` -
    /// * `Err(PaymentError)` - データを取得できなかった場合のエラー
    async fn exists(&self, payment_id: &PaymentMethodId, user_id: &UserId) -> Result<bool, PaymentError>;
}
