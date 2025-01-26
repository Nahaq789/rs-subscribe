use std::str::FromStr;

use aws_sdk_dynamodb::error::ProvideErrorMetadata;
use aws_sdk_dynamodb::types::AttributeValue;
use domain::{
    category::{category_error::CategoryError, category_id::CategoryId, category_name::CategoryName, Category},
    repository::category_repository::CategoryRepository,
    user::user_id::UserId,
};
use tracing::{error, info};

use crate::mapper::{as_string, Mapper};

const CATEGORY_KEY: &str = "category_id";
const USER_ID: &str = "user_id";
const CATEGORY_NAME: &str = "category_name";

const USER_ID_CONDITION: &str = "#user_id = :user_id";
const USER_ID_ATTR: &str = "#user_id";
const USER_ID_VALUE: &str = ":user_id";

const CATEGORY_NAME_CONDITION: &str = "#category_name = :category_name";
const CATEGORY_NAME_ATTR: &str = "#category_name";
const CATEGORY_NAME_VALUE: &str = ":category_name";

const CATEGORY_ID_CONDITION: &str = "#category_id = :category_id";
const CATEGORY_ID_ATTR: &str = "#category_id";
const CATEGORY_ID_VALUE: &str = ":category_id";

const UPDATE_EXPRESSION: &str = "SET #user_id = :user_id, #category_name = :category_name";

#[derive(Debug)]
pub struct CategoryRepositoryImpl {
    client: aws_sdk_dynamodb::Client,
    table: String,
}

impl CategoryRepositoryImpl {
    pub fn new(client: aws_sdk_dynamodb::Client, table: &str) -> Self {
        Self { client, table: table.to_owned() }
    }
}

impl CategoryRepository for CategoryRepositoryImpl {
    fn create<'a>(
        &'a self,
        category: &'a domain::category::Category,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), domain::category::category_error::CategoryError>> + Send + '_>,
    > {
        let result = Box::pin(async move {
            let request = self
                .client
                .put_item()
                .table_name(&self.table)
                .table_name(&self.table)
                .item(CATEGORY_KEY, AttributeValue::S(category.category_id().to_string()))
                .item(USER_ID, AttributeValue::S(category.user_id().to_string()))
                .item(CATEGORY_NAME, AttributeValue::S(category.category_name().to_string()));

            match request.send().await {
                Ok(p) => {
                    info!("{:?}", p);
                    Ok(())
                }
                Err(e) => {
                    error!("{:?}", e);
                    Err(CategoryError::CreateCategoryFailed(e.to_string()))
                }
            }
        });
        result
    }

    fn find_all<'a>(
        &'a self,
        user_id: &'a domain::user::user_id::UserId,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<Vec<domain::category::Category>, domain::category::category_error::CategoryError>,
                > + Send
                + '_,
        >,
    > {
        let result = Box::pin(async move {
            let result = self
                .client
                .query()
                .table_name(&self.table)
                .key_condition_expression(USER_ID_CONDITION.to_string())
                .expression_attribute_names(USER_ID_ATTR, USER_ID)
                .expression_attribute_values(USER_ID_VALUE, AttributeValue::S(user_id.to_string()))
                .send()
                .await
                .map_err(|e| {
                    let msg = match e.message() {
                        Some(s) => s.to_string(),
                        None => e.to_string(),
                    };
                    CategoryError::QueryError(msg)
                })?;
            match result.items {
                Some(v) => {
                    let result = v.into_iter().map(|item| CategoryRepositoryImpl::map_to_domain_model(item)).collect();
                    result
                }
                None => Ok(vec![]),
            }
        });
        result
    }

    fn find_by_id<'a>(
        &'a self,
        category_id: &'a domain::category::category_id::CategoryId,
        user_id: &'a domain::user::user_id::UserId,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<domain::category::Category, domain::category::category_error::CategoryError>,
                > + Send
                + '_,
        >,
    > {
        let result = Box::pin(async move {
            let result = self
                .client
                .get_item()
                .table_name(&self.table)
                .key(USER_ID, AttributeValue::S(user_id.to_string()))
                .key(CATEGORY_KEY, AttributeValue::S(category_id.to_string()))
                .send()
                .await
                .map_err(|e| {
                    let msg = match e.message() {
                        Some(s) => s.to_string(),
                        None => e.to_string(),
                    };
                    CategoryError::FindByIdError(msg)
                })?;

            match result.item {
                Some(item) => {
                    info!("{:?}", item);
                    CategoryRepositoryImpl::map_to_domain_model(item)
                }
                None => {
                    let error =
                        CategoryError::FindByIdError(format!("category_id: {}, user_id: {}", category_id, user_id));
                    error!("{:?}", error);
                    Err(error)
                }
            }
        });
        result
    }

    fn update<'a>(
        &'a self,
        category: &'a domain::category::Category,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), domain::category::category_error::CategoryError>> + Send + '_>,
    > {
        let result = Box::pin(async move {
            let request = self
                .client
                .update_item()
                .table_name(&self.table)
                .key(USER_ID, AttributeValue::S(category.user_id().to_string()))
                .key(CATEGORY_KEY, AttributeValue::S(category.category_id().to_string()))
                .update_expression(UPDATE_EXPRESSION)
                .expression_attribute_names(USER_ID_ATTR, USER_ID)
                .expression_attribute_names(CATEGORY_ID_ATTR, CATEGORY_KEY)
                .expression_attribute_names(CATEGORY_NAME_ATTR, CATEGORY_NAME)
                .expression_attribute_values(USER_ID, AttributeValue::S(category.user_id().to_string()))
                .expression_attribute_values(CATEGORY_KEY, AttributeValue::S(category.category_id().to_string()))
                .expression_attribute_values(CATEGORY_NAME, AttributeValue::S(category.category_name().to_string()));

            match request.send().await {
                Ok(p) => {
                    info!("{:?}", p);
                    Ok(())
                }
                Err(e) => {
                    error!("{:?}", e);
                    Err(CategoryError::UpdateCategoryFailed(e.to_string()))
                }
            }
        });
        result
    }

    fn delete<'a>(
        &'a self,
        category_id: &'a domain::category::category_id::CategoryId,
        user_id: &'a domain::user::user_id::UserId,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), domain::category::category_error::CategoryError>> + Send + '_>,
    > {
        let result = Box::pin(async move {
            let result = self
                .client
                .delete_item()
                .table_name(&self.table)
                .key(USER_ID, AttributeValue::S(user_id.to_string()))
                .key(CATEGORY_KEY, AttributeValue::S(category_id.to_string()))
                .send()
                .await
                .map_err(|e| {
                    let msg = match e.message() {
                        Some(s) => s.to_string(),
                        None => e.to_string(),
                    };
                    CategoryError::DeleteCategoryFailed(msg)
                });

            match result {
                Ok(u) => {
                    info!("{:?}", u);
                    Ok(())
                }
                Err(e) => {
                    error!("{:?}", e);
                    Err(CategoryError::DeleteCategoryFailed(e.to_string()))
                }
            }
        });
        result
    }
}

impl Mapper<Category, CategoryError> for CategoryRepositoryImpl {
    fn map_to_domain_model(v: std::collections::HashMap<String, AttributeValue>) -> Result<Category, CategoryError> {
        let category_id = CategoryId::from_str(&as_string(v.get(CATEGORY_KEY), ""))?;
        let user_id = UserId::from_str(&as_string(v.get(USER_ID), ""))?;
        let category_name = CategoryName::from_str(&as_string(v.get(CATEGORY_NAME), ""))?;

        Ok(Category::from(category_id, user_id, category_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_map_to_domain_model_success() {
        let test_case = vec![
            HashMap::from([
                (CATEGORY_KEY.into(), AttributeValue::S(CategoryId::new().to_string())),
                (USER_ID.into(), AttributeValue::S(UserId::new().to_string())),
                (CATEGORY_NAME.into(), AttributeValue::S("123".to_string())),
            ]),
        ];

        let _test = test_case
            .into_iter()
            .map(|test| match CategoryRepositoryImpl::map_to_domain_model(test.clone()) {
                Ok(v) => {
                    assert_eq!(v.category_id().to_string(), as_string(test.get(CATEGORY_KEY), ""));
                    assert_eq!(v.user_id().to_string(), as_string(test.get(USER_ID), ""));
                    assert_eq!(v.category_name().to_string(), as_string(test.get(CATEGORY_NAME), ""));
                }
                Err(e) => {
                    error!("{:?}", e);
                    assert!(false);
                }
            })
            .collect::<()>();
    }
}
