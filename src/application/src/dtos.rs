pub mod payment_method_dto;
pub mod subscribe_dto;
/// DTOとドメインモデル間の相互変換を行うトレイト
///
/// # 型パラメータ
/// * `T` - DTOの型
/// * `D` - ドメインモデルの型
/// * `E` - エラーの型
///
/// # トレイトの境界
/// * `T: Sized` - DTOの型は固定サイズである必要がある
///
/// # メソッド
/// * `map_to_domain_model` - DTOからドメインモデルへの変換
/// * `map_to_dto` - ドメインモデルからDTOへの変換
pub trait DTO<T, D, E>
where
    T: Sized,
{
    fn map_to_domain_model(v: T) -> Result<D, E>;
    fn map_to_dto(v: &D) -> T;
}
