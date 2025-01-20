use std::str::FromStr;

use domain::subscribe::Subscribe;

use crate::error::{self, ApplicationError};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscribeDto {
    subscribe_id: String,
    user_id: String,
    name: String,
    payment_method_id: String,
    amount: String,
    payment_cycle: String,
    category_id: String,
    icon_local_path: String,
    notification: bool,
    first_payment_date: chrono::DateTime<chrono::Utc>,
    next_payment_date: chrono::DateTime<chrono::Utc>,
    auto_renewal: bool,
    status: String,
    memo: Option<String>,
}

impl SubscribeDto {
    pub fn new(
        subscribe_id: String,
        user_id: String,
        name: String,
        payment_method_id: String,
        amount: String,
        payment_cycle: String,
        category_id: String,
        icon_local_path: String,
        notification: bool,
        first_payment_date: chrono::DateTime<chrono::Utc>,
        next_payment_date: chrono::DateTime<chrono::Utc>,
        auto_renewal: bool,
        status: String,
        memo: Option<String>,
    ) -> Self {
        Self {
            subscribe_id,
            user_id,
            name,
            payment_method_id,
            amount,
            payment_cycle,
            category_id,
            icon_local_path,
            notification,
            first_payment_date,
            next_payment_date,
            auto_renewal,
            status,
            memo,
        }
    }

    pub fn builder() -> SubscribeDtoBuilder {
        SubscribeDtoBuilder::default()
    }
}

#[derive(Default)]
pub struct SubscribeDtoBuilder {
    subscribe_id: Option<String>,
    user_id: Option<String>,
    name: Option<String>,
    payment_method_id: Option<String>,
    amount: Option<String>,
    payment_cycle: Option<String>,
    category_id: Option<String>,
    icon_local_path: Option<String>,
    notification: Option<bool>,
    first_payment_date: Option<chrono::DateTime<chrono::Utc>>,
    next_payment_date: Option<chrono::DateTime<chrono::Utc>>,
    auto_renewal: Option<bool>,
    status: Option<String>,
    memo: Option<String>,
}

impl SubscribeDtoBuilder {
    pub fn subscribe_id(mut self, subscribe_id: String) -> Self {
        self.subscribe_id = Some(subscribe_id);
        self
    }

    pub fn user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn payment_method_id(mut self, payment_method_id: String) -> Self {
        self.payment_method_id = Some(payment_method_id);
        self
    }

    pub fn amount(mut self, amount: String) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn payment_cycle(mut self, payment_cycle: String) -> Self {
        self.payment_cycle = Some(payment_cycle);
        self
    }

    pub fn category_id(mut self, category_id: String) -> Self {
        self.category_id = Some(category_id);
        self
    }

    pub fn icon_local_path(mut self, icon_local_path: String) -> Self {
        self.icon_local_path = Some(icon_local_path);
        self
    }

    pub fn notification(mut self, notification: bool) -> Self {
        self.notification = Some(notification);
        self
    }

    pub fn first_payment_date(mut self, first_payment_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.first_payment_date = Some(first_payment_date);
        self
    }

    pub fn next_payment_date(mut self, next_payment_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.next_payment_date = Some(next_payment_date);
        self
    }

    pub fn auto_renewal(mut self, auto_renewal: bool) -> Self {
        self.auto_renewal = Some(auto_renewal);
        self
    }

    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    pub fn memo(mut self, memo: Option<String>) -> Self {
        self.memo = memo;
        self
    }

    pub fn build(self) -> Result<SubscribeDto, domain::subscribe::subscribe_error::SubscribeError> {
        use domain::subscribe::subscribe_error::SubscribeError;

        Ok(SubscribeDto {
            subscribe_id: self.subscribe_id.ok_or_else(|| SubscribeError::MissingField("subscribe_id".to_string()))?,
            user_id: self.user_id.ok_or_else(|| SubscribeError::MissingField("user_id".to_string()))?,
            name: self.name.ok_or_else(|| SubscribeError::MissingField("name".to_string()))?,
            payment_method_id: self
                .payment_method_id
                .ok_or_else(|| SubscribeError::MissingField("payment_method_id".to_string()))?,
            amount: self.amount.ok_or_else(|| SubscribeError::MissingField("amount".to_string()))?,
            payment_cycle: self
                .payment_cycle
                .ok_or_else(|| SubscribeError::MissingField("payment_cycle".to_string()))?,
            category_id: self.category_id.ok_or_else(|| SubscribeError::MissingField("category_id".to_string()))?,
            icon_local_path: self
                .icon_local_path
                .ok_or_else(|| SubscribeError::MissingField("icon_local_path".to_string()))?,
            notification: self.notification.unwrap_or(false),
            first_payment_date: self
                .first_payment_date
                .ok_or_else(|| SubscribeError::MissingField("first_payment_date".to_string()))?,
            next_payment_date: self
                .next_payment_date
                .ok_or_else(|| SubscribeError::MissingField("next_payment_date".to_string()))?,
            auto_renewal: self.auto_renewal.unwrap_or(false),
            status: self.status.ok_or_else(|| SubscribeError::MissingField("status".to_string()))?,
            memo: self.memo,
        })
    }
}

impl crate::dtos::DTO<SubscribeDto, domain::subscribe::Subscribe, ApplicationError> for SubscribeDto {
    fn map_to_domain_model(v: SubscribeDto) -> Result<domain::subscribe::Subscribe, ApplicationError> {
        use domain::category::category_id::CategoryId;
        use domain::payment::payment_method_id::PaymentMethodId;
        use domain::payment_cycle::PaymentCycle;
        use domain::subscribe::{
            subscribe_id::SubscribeId, subscribe_name::SubscribeName, subscribe_status::SubscribeStatus,
        };
        use domain::user::user_id::UserId;
        use domain::value_object::amount::Amount;

        let subscribe_id = match v.subscribe_id {
            s if s.is_empty() => SubscribeId::new(),
            _ => SubscribeId::from_str(v.subscribe_id.as_str()).map_err(|e| error::to_aggregate_id_error(e))?,
        };
        let user_id = UserId::from_str(v.user_id.as_str()).map_err(|e| error::to_aggregate_id_error(e))?;
        let name = SubscribeName::from_str(v.name.as_str()).map_err(|e| error::to_subscribe_error(e))?;
        let payment_method_id =
            PaymentMethodId::from_str(v.payment_method_id.as_str()).map_err(|e| error::to_aggregate_id_error(e))?;
        let amount = Amount::from_str(v.amount.as_str()).map_err(|e| error::to_subscribe_error(e))?;

        let payment_cycle =
            PaymentCycle::from_str(v.payment_cycle.as_str()).map_err(|e| error::to_subscribe_error(e))?;

        let category_id = CategoryId::from_str(v.category_id.as_str()).map_err(|e| error::to_subscribe_error(e))?;

        let status = SubscribeStatus::from_str(v.status.as_str()).map_err(|e| error::to_subscribe_error(e))?;

        Ok(Subscribe::from(
            subscribe_id,
            user_id,
            name,
            payment_method_id,
            amount,
            payment_cycle,
            category_id,
            v.icon_local_path,
            v.notification,
            v.first_payment_date,
            v.next_payment_date,
            v.auto_renewal,
            status,
            v.memo,
        ))
    }

    fn map_to_dto(v: &domain::subscribe::Subscribe) -> SubscribeDto {
        let builder = SubscribeDto::builder()
            .subscribe_id(v.subscribe_id().to_string())
            .user_id(v.user_id().to_string())
            .name(v.name().to_string())
            .payment_method_id(v.payment_method_id().to_string())
            .amount(v.amount().to_string())
            .payment_cycle(v.payment_cycle().to_string())
            .category_id(v.category_id().to_string())
            .icon_local_path(v.icon_local_path().to_string())
            .notification(v.notification())
            .first_payment_date(*v.first_payment_date())
            .next_payment_date(*v.next_payment_date())
            .auto_renewal(v.auto_renewal())
            .status(v.status().to_string())
            .memo(v.memo().to_owned())
            .build();
        builder.unwrap()
    }
}
