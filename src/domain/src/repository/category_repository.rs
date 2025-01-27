use crate::category::category_error::CategoryError;
use crate::category::category_id::CategoryId;
use crate::category::Category;
use crate::user::user_id::UserId;

pub trait CategoryRepository: Send + Sync {
    fn create<'a>(
        &'a self,
        category: &'a Category,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), CategoryError>> + Send + '_>>;

    fn find_all<'a>(
        &'a self,
        user_id: &'a UserId,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Category>, CategoryError>> + Send + '_>>;

    fn find_by_id<'a>(
        &'a self,
        category_id: &'a CategoryId,
        user_id: &'a UserId,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Category, CategoryError>> + Send + '_>>;

    fn update<'a>(
        &'a self,
        category: &'a Category,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), CategoryError>> + Send + '_>>;

    fn delete<'a>(
        &'a self,
        category_id: &'a CategoryId,
        user_id: &'a UserId,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), CategoryError>> + Send + '_>>;
}
