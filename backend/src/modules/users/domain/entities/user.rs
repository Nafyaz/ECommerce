use crate::modules::users::domain::value_objects::phone::Phone;
use crate::modules::users::domain::value_objects::user_name::UserName;
use crate::modules::users::domain::value_objects::{AccountId, UserId};
use crate::modules::users::errors::UserDomainError;
use chrono::{DateTime, Utc};

//TODO: Manually implement slugs and use them
pub struct User {
    id: UserId,
    account_id: AccountId,
    name: UserName,
    phone: Option<Phone>,
    phone_verified_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(account_id: AccountId, name: UserName, phone: Option<Phone>) -> Result<Self, UserDomainError> {
        let now = Utc::now();

        Ok(Self {
            id: UserId::new(),
            account_id,
            name,
            phone,
            phone_verified_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn reconstitute(
        id: UserId,
        account_id: AccountId,
        name: UserName,
        phone: Option<Phone>,
        phone_verified_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, UserDomainError> {
        if updated_at < created_at {
            return Err(UserDomainError::InternalError(
                "User updated_at cannot be earlier than created_at".to_owned(),
            ));
        }

        Ok(Self {
            id,
            account_id,
            name,
            phone,
            phone_verified_at,
            created_at,
            updated_at,
        })
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn phone(&self) -> &Option<Phone> {
        &self.phone
    }

    pub fn phone_verified_at(&self) -> Option<DateTime<Utc>> {
        self.phone_verified_at
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
