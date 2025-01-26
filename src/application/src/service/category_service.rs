use std::str::FromStr;

use domain::{
    category::category_id::CategoryId, repository::category_repository::CategoryRepository, subscribe::subscribe_id,
    user::user_id::UserId,
};

use crate::dtos::{category_dto::CategoryDto, DTO};

use super::CategoryService;

pub struct CategoryServiceImpl<T: CategoryRepository> {
    repository: T,
}

impl<T: CategoryRepository> CategoryServiceImpl<T> {
    pub fn new(repository: T) -> CategoryServiceImpl<T> {
        Self { repository }
    }
}

impl<T: CategoryRepository> CategoryService for CategoryServiceImpl<T> {
    fn create_category<'a>(
        &'a self,
        category: crate::dtos::category_dto::CategoryDto,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), crate::error::ApplicationError>> + Send + '_>>
    {
        let result = Box::pin(async move {
            let category = CategoryDto::map_to_domain_model(category)?;
            let result = self.repository.create(&category).await?;
            Ok(result)
        });
        result
    }

    fn find_category_all<'a>(
        &'a self,
        user_id: &'a str,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<Vec<crate::dtos::category_dto::CategoryDto>, crate::error::ApplicationError>,
                > + Send
                + '_,
        >,
    > {
        let result = Box::pin(async move {
            let user_id = UserId::from_str(user_id)?;
            let v = self.repository.find_all(&user_id).await?;
            let result = v.into_iter().map(|item| CategoryDto::map_to_dto(&item)).collect();
            Ok(result)
        });
        result
    }

    fn find_category_by_id<'a>(
        &'a self,
        user_id: &'a str,
        category_id: &'a str,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<crate::dtos::category_dto::CategoryDto, crate::error::ApplicationError>,
                > + Send
                + '_,
        >,
    > {
        let result = Box::pin(async move {
            let user_id = UserId::from_str(user_id)?;
            let category_id = CategoryId::from_str(category_id)?;
            let v = self.repository.find_by_id(&category_id, &user_id).await?;
            let result = CategoryDto::map_to_dto(&v);
            Ok(result)
        });
        result
    }

    fn update_category<'a>(
        &'a self,
        category: crate::dtos::category_dto::CategoryDto,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), crate::error::ApplicationError>> + Send + '_>>
    {
        let result = Box::pin(async move {
            let category = CategoryDto::map_to_domain_model(category)?;
            let result = self.repository.update(&category).await?;
            Ok(result)
        });
        result
    }

    fn delete_category<'a>(
        &'a self,
        user_id: &'a str,
        category_id: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), crate::error::ApplicationError>> + Send + '_>>
    {
        let result = Box::pin(async move {
            let user_id = UserId::from_str(user_id)?;
            let category_id = CategoryId::from_str(category_id)?;
            let result = self.repository.delete(&category_id, &user_id).await?;
            Ok(result)
        });
        result
    }
}
