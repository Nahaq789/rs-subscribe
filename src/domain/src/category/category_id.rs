use crate::{generate_id, AggregateId, AggregateIdError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CategoryId {
    value: String,
}

const CATEGORY_PREFIX: &str = "ctg";

impl CategoryId {
    pub fn new() -> Self {
        let value = generate_id(CATEGORY_PREFIX, None);
        Self { value }
    }
}

impl AggregateId for CategoryId {
    fn type_name(&self) -> String {
        CATEGORY_PREFIX.to_string()
    }

    fn value(&self) -> &String {
        &self.value
    }
}

impl From<uuid::Uuid> for CategoryId {
    fn from(value: uuid::Uuid) -> Self {
        Self { value: generate_id(CATEGORY_PREFIX, Some(value)) }
    }
}

impl std::fmt::Display for CategoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::str::FromStr for CategoryId {
    type Err = AggregateIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: Vec<&str> = s.split("_").collect();
        if value.len() != 2 {
            return Err(AggregateIdError::InvalidFormat);
        }
        if value[0] != CATEGORY_PREFIX {
            return Err(AggregateIdError::InvalidFormat);
        }
        let uuid = uuid::Uuid::parse_str(value[1]).map_err(|_| AggregateIdError::InvalidUuid)?;
        Ok(Self::from(uuid))
    }
}
