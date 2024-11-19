use domain::user::user_id::UserId;

use super::Entity;

pub mod payment_mapper;

pub trait Mapper<D, E>
where
  E: Entity,
{
  fn to_entity(m: D, u: UserId) -> E;
  fn to_domain_model(e: E) -> D;
}
