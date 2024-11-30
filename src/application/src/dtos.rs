pub mod payment_method_dto;

pub trait DTO<T, D, E>
where
  T: Sized,
{
  fn map_to_domain_model(v: T) -> Result<D, E>;
  fn map_to_dto(v: D) -> Result<T, E>;
}
