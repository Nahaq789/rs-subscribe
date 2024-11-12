use crate::subscribe::subscribe_error::SubscribeError;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SubscribeStatus {
    ACTIVE,
    PAUSED,
    CANCELLED,
}

impl Display for SubscribeStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscribeStatus::ACTIVE => write!(f, "ACTIVE"),
            SubscribeStatus::PAUSED => write!(f, "PAUSED"),
            SubscribeStatus::CANCELLED => write!(f, "CANCELLED"),
        }
    }
}

impl FromStr for SubscribeStatus {
    type Err = SubscribeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ACTIVE" => Ok(SubscribeStatus::ACTIVE),
            "PAUSED" => Ok(SubscribeStatus::PAUSED),
            "CANCELLED" => Ok(SubscribeStatus::CANCELLED),
            _ => Err(SubscribeError::InvalidSubscribeStatus(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscribe_status_fmt() {
        let test_case = vec![
            (SubscribeStatus::ACTIVE, "ACTIVE"),
            (SubscribeStatus::PAUSED, "PAUSED"),
            (SubscribeStatus::CANCELLED, "CANCELLED"),
        ];

        for (expected, actual) in test_case {
            assert_eq!(expected.to_string(), actual)
        }
    }

    #[test]
    fn test_subscribe_status_from_str_success() {
        let test_case = vec![
            ("ACTIVE", SubscribeStatus::ACTIVE),
            ("PAUSED", SubscribeStatus::PAUSED),
            ("CANCELLED", SubscribeStatus::CANCELLED),
        ];

        for (expected, actual) in test_case {
            let result = SubscribeStatus::from_str(expected);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), actual)
        }
    }

    #[test]
    fn test_subscribe_status_from_str_failed() {
        let test_case = vec!["ACT IVE", "PAU SED", "CAN CELLED"];

        for input in test_case {
            let result = SubscribeStatus::from_str(input);
            assert!(result.is_err());
            assert_eq!(
                SubscribeError::InvalidSubscribeStatus(input.to_string()).to_string(),
                format!("Not match Subscribe Status: {}", input)
            )
        }
    }
}
