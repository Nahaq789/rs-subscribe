use crate::subscribe::subscribe_error::SubscribeError;
use crate::subscribe::subscribe_id::SubscribeId;
use crate::subscribe::Subscribe;
use crate::user::user_id::UserId;
use async_trait::async_trait;

#[async_trait]
pub trait SubscribeRepository {
  /// 新しいサブスクを作成する
  ///
  /// # 引数
  /// * `subscribe` - [Subscribe] 作成するサブスク情報
  ///
  /// # 戻り値                                                 
  /// * `Ok(())` - void                                        
  /// * `Err(SubscribeError)` - 更新処理が失敗した場合のエラー
  async fn create(&self, subscribe: &Subscribe) -> Result<(), SubscribeError>;

  /// ユーザーの全てのサブスクを取得する
  ///
  /// # 引数
  /// * `user_id` - [UserId] 取得対象のユーザーID
  ///
  /// # 戻り値
  /// - Vec<[Subscribe]> サブスク情報のリスト
  async fn find_all(&self, user_id: &UserId) -> Result<Vec<Subscribe>, SubscribeError>;

  /// 指定されたサブスクを取得する
  ///
  /// # 引数
  /// * `subscribe_id` - [SubscribeId] 取得対象のサブスクID
  /// * `user_id` - [UserId] サブスクの所有者ID
  ///
  /// # 戻り値
  /// - [Subscribe] サブスク情報（存在しない場合はNone）
  async fn find_by_id(
    &self,
    subscribe_id: &SubscribeId,
    user_id: &UserId,
  ) -> Result<Subscribe, SubscribeError>;

  /// サブスク情報を更新する
  ///
  /// # 引数
  /// * `subscribe` - [Subscribe] 更新するサブスク情報
  ///
  /// # 戻り値
  /// * `Ok(())` - void
  /// * `Err(SubscribeError)` - 更新処理が失敗した場合のエラー
  async fn update(&self, subscribe: &Subscribe) -> Result<(), SubscribeError>;

  /// サブスクを削除する
  ///
  /// # 引数
  /// * `subscribe_id` - [SubscribeId] 削除対象のサブスクID
  /// * `user_id` - [UserId] 削除対象のユーザーID
  ///
  /// # 戻り値                                                 
  /// * `Ok(())` - void                                        
  /// * `Err(SubscribeError)` - 更新処理が失敗した場合のエラー
  async fn delete(
    &self,
    subscribe_id: &SubscribeId,
    user_id: &UserId,
  ) -> Result<(), SubscribeError>;
}
