use thiserror::Error;

use crate::AggregateIdError;

use super::category_name::CategoryNameError;

#[derive(Debug, Error)]
pub enum CategoryError {
    #[error("Failed to create category: {0}")]
    CreateCategoryFailed(String),

    #[error("Failed to find by id category: {0}")]
    FindByIdError(String),

    #[error("Failed to query category: {0}")]
    QueryError(String),

    #[error("Failed to update category: {0}")]
    UpdateCategoryFailed(String),

    #[error("Failed to delete category: {0}")]
    DeleteCategoryFailed(String),

    #[error("Failed to parse category: {0}")]
    ParseCategoryFailed(String),

    #[error("Category not exist")]
    NotExist,

    #[error("Required category field '{0}' was missing")]
    MissingField(String),

    #[error("{0}")]
    CategoryIdFailed(String),

    #[error("{0}")]
    CategoryNameFailed(String),
}

impl From<AggregateIdError> for CategoryError {
    fn from(value: AggregateIdError) -> Self {
        CategoryError::CategoryIdFailed(value.to_string())
    }
}

impl From<CategoryNameError> for CategoryError {
    fn from(value: CategoryNameError) -> Self {
        CategoryError::CategoryNameFailed(value.to_string())
    }
}
