use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FindAllParam {
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FindByIdParams {
    pub user_id: String,
    pub payment_method_id: String,
}
