use domain::AggregateId;

use super::Entity;

pub mod payment_mapper;

pub trait Mapper<D, E, K>
where
  E: Entity,
  K: AggregateId,
{
  fn to_entity(m: D, k: K) -> E;
  fn to_domain_model(e: E) -> D;
}
