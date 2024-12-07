use crate::user::user_error::UserError;
use crate::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: &User) -> Result<User, UserError>;
}
