use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FindAllParam {
    pub user_id: String,
}

pub struct FindByIdParams {
    pub user_id: String,
    pub subscribe_id: String,
}
