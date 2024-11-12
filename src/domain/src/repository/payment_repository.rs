use crate::payment::payment_errpr::PaymentError;
use crate::payment::payment_method_id::PaymentMethodId;
use crate::payment::PaymentMethod;
use async_trait::async_trait;

/// 支払い方法のリポジトリインターフェース
#[async_trait]
pub trait PaymentRepository {
  /// 新しい支払い方法を作成する
  ///
  /// # 引数
  /// * `payment` - [PaymentMethod] 作成する支払い方法情報
  ///
  /// # 戻り値
  /// - [PaymentMethod] 作成された支払い方法情報
  async fn create(&self, payment: &PaymentMethod) -> Result<PaymentMethod, PaymentError>;

  /// すべての支払い方法を取得する
  ///
  /// # 戻り値
  /// - Vec<[PaymentMethod]> 支払い方法のリスト
  async fn find_all(&self) -> Result<Vec<PaymentMethod>, PaymentError>;

  /// IDによって支払い方法を検索する
  ///
  /// # 引数
  /// * `payment_id` - [PaymentMethodId] 検索する支払い方法のID
  ///
  /// # 戻り値
  /// - [PaymentMethod] 検索された支払い方法情報
  async fn find_by_id(&self, payment_id: &PaymentMethodId) -> Result<PaymentMethod, PaymentError>;

  /// 支払い方法を更新する
  ///
  /// # 引数
  /// * `payment` - [PaymentMethod] 更新する支払い方法情報
  ///
  /// # 戻り値
  /// - [PaymentMethod] 更新された支払い方法情報
  async fn update(&self, payment: &PaymentMethod) -> Result<PaymentMethod, PaymentError>;

  /// 支払い方法を削除する
  ///
  /// # 引数
  /// * `payment_id` - [PaymentMethodId] 削除する支払い方法のID
  ///
  /// # 戻り値
  /// - () 削除が成功した場合
  async fn delete(&self, payment_id: &PaymentMethodId) -> Result<(), PaymentError>;
}