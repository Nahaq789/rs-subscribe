use uuid::Uuid;
use crate::subscribe::subscribe_id::SubscribeId;

pub mod subscribe_id;

#[derive(Debug)]
pub struct Subscribe {
    subscribe_id: SubscribeId,
}

impl Subscribe {
    pub fn new() -> Self {
        let id = SubscribeId::new();
        Self {
            subscribe_id: id
        }
    }

    pub fn from(subscribe_id: Uuid) -> Self {
        Self {
            subscribe_id: SubscribeId::from(subscribe_id)
        }
    }

    pub fn subscribe_id(&self) -> &SubscribeId {
        &self.subscribe_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscribe_new_success() {
        let result = Subscribe::new();
        assert!(!result.subscribe_id.to_string().is_empty());
    }

    #[test]
    fn test_subscribe_from_success() {
        let id = Uuid::new_v4();
        let result = Subscribe::from(id);
        assert!(!result.subscribe_id.to_string().is_empty());
        assert_eq!(format!("SUBSCRIBE-{}", id.to_string()), result.subscribe_id.to_string())
    }
}