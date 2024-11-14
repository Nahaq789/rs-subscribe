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
  /// - [Subscribe] 作成されたサブスク情報
  async fn create(&self, subscribe: &Subscribe) -> Result<Subscribe, SubscribeError>;

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
  /// - Option<[Subscribe]> サブスク情報（存在しない場合はNone）
  async fn find_by_id(
    &self,
    subscribe_id: &SubscribeId,
    user_id: &UserId,
  ) -> Result<Option<Subscribe>, SubscribeError>;

  /// サブスク情報を更新する
  ///
  /// # 引数
  /// * `subscribe` - [Subscribe] 更新するサブスク情報
  ///
  /// # 戻り値
  /// - [Subscribe] 更新後のサブスク情報
  async fn update(&self, subscribe: &Subscribe) -> Result<Subscribe, SubscribeError>;

  /// サブスクを削除する
  ///
  /// # 引数
  /// * `subscribe_id` - [SubscribeId] 削除対象のサブスクID
  ///
  /// # 戻り値
  /// - 成功時は空のタプル、失敗時はエラー
  async fn delete(&self, subscribe_id: &SubscribeId) -> Result<(), SubscribeError>;
}
