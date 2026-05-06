use crate::modules::users::domain::entities::User;
use crate::modules::users::domain::value_objects::phone::Phone;
use crate::modules::users::domain::value_objects::user_name::UserName;
use crate::modules::users::domain::value_objects::{AccountId, UserId};
use crate::modules::users::errors::UserDomainError;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct UserRecord {
    id: Uuid,
    account_id: Uuid,
    name: String,
    phone: Option<String>,
    phone_verified_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UserRecord {
    pub fn from_entity(user: &User) -> Self {
        Self {
            id: user.id().as_uuid().to_owned(),
            account_id: user.account_id().as_uuid().to_owned(),
            name: user.name().as_str().to_owned(),
            phone: user.phone().as_ref().map(|p| p.as_str().to_owned()),
            phone_verified_at: user.phone_verified_at(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn account_id(&self) -> &Uuid {
        &self.account_id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn phone(&self) -> &Option<String> {
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

impl TryFrom<UserRecord> for User {
    type Error = UserDomainError;

    fn try_from(user_row: UserRecord) -> Result<Self, Self::Error> {
        User::reconstitute(
            UserId::from_uuid(user_row.id),
            AccountId::from_uuid(user_row.account_id),
            UserName::new(user_row.name)?,
            user_row.phone.map(Phone::new).transpose()?,
            user_row.phone_verified_at,
            user_row.created_at,
            user_row.updated_at,
        )
    }
}
