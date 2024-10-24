use crate::subscribe::subscribe_id::SubscribeId;

pub mod subscribe_id;

#[derive(Debug)]
pub struct Subscribe {
    subscribe_id: SubscribeId,
}

impl Subscribe {
    fn new(subscribe_id: SubscribeId) -> Self {
        Self { subscribe_id }
    }
}
