use domain::repository::payment_repository::PaymentRepository;

pub struct PaymentMethodService<T: PaymentRepository> {
  repository: T,
}

impl<T: PaymentRepository> PaymentMethodService<T> {
  pub fn new(repository: T) -> PaymentMethodService<T> {
    Self { repository }
  }
}
