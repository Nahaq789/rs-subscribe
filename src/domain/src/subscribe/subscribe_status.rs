#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SubscribeStatus {
    ACTIVE,
    PAUSED,
    CANCELLED,
}
