use std::str::FromStr;

use domain::{
    category::{category_error::CategoryError, category_id::CategoryId, category_name::CategoryName, Category},
    user::user_id::UserId,
};

use crate::error::{to_aggregate_id_error, to_category_error, ApplicationError};

use super::DTO;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategoryDto {
    category_id: String,
    user_id: String,
    category_name: String,
}

impl CategoryDto {
    pub fn new(category_id: String, user_id: String, category_name: String) -> Self {
        Self { category_id, user_id, category_name }
    }

    pub fn builder() -> CategoryDtoBuilder {
        CategoryDtoBuilder::default()
    }
}

#[derive(Default)]
pub struct CategoryDtoBuilder {
    category_id: Option<String>,
    user_id: Option<String>,
    category_name: Option<String>,
}

impl CategoryDtoBuilder {
    pub fn category_id(mut self, category_id: String) -> Self {
        self.category_id = Some(category_id);
        self
    }

    pub fn user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn category_name(mut self, category_name: String) -> Self {
        self.category_name = Some(category_name);
        self
    }

    pub fn build(self) -> Result<CategoryDto, CategoryError> {
        Ok(CategoryDto {
            category_id: self.category_id.ok_or_else(|| CategoryError::MissingField("category_id".to_string()))?,
            user_id: self.user_id.ok_or_else(|| CategoryError::MissingField("user_id".to_string()))?,
            category_name: self
                .category_name
                .ok_or_else(|| CategoryError::MissingField("category_name".to_string()))?,
        })
    }
}

impl DTO<CategoryDto, Category, ApplicationError> for CategoryDto {
    fn map_to_domain_model(v: CategoryDto) -> Result<Category, ApplicationError> {
        let category_id = match v.category_id {
            s if s.is_empty() => CategoryId::new(),
            _ => CategoryId::from_str(v.category_id.as_str()).map_err(|e| to_aggregate_id_error(e))?,
        };

        let user_id = UserId::from_str(v.user_id.as_str()).map_err(|e| to_aggregate_id_error(e))?;

        let category_name = CategoryName::from_str(v.category_name.as_str()).map_err(|e| to_category_error(e))?;

        Ok(Category::from(category_id, user_id, category_name))
    }

    fn map_to_dto(v: &Category) -> CategoryDto {
        let builder = CategoryDto::builder()
            .category_id(v.category_id().to_string())
            .user_id(v.user_id().to_string())
            .category_name(v.category_name().to_string())
            .build();
        builder.unwrap()
    }
}
