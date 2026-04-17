use crate::modules::users::domain::entities::User;
use crate::modules::users::domain::value_objects::{Email, PasswordHash, Phone, UserId};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub phone: Option<String>,
    pub phone_verified_at: Option<DateTime<Utc>>,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserRow {
    pub fn from_entity(user: &User) -> Self {
        Self {
            id: user.id().as_uuid().to_owned(),
            name: user.name().to_owned(),
            email: user.email().map(|email| email.as_str().to_owned()),
            email_verified_at: user.email_verified_at(),
            phone: user.phone().map(|phone| phone.as_str().to_owned()),
            phone_verified_at: user.phone_verified_at(),
            password_hash: user.password_hash().as_str().to_owned(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }

    pub fn into_entity(self) -> User {
        User::reconstitute(
            UserId::from_uuid(self.id),
            self.name,
            self.email.map(Email::from_trusted),
            self.email_verified_at,
            self.phone.map(Phone::from_trusted),
            self.phone_verified_at,
            PasswordHash::from_hash(self.password_hash.into()),
            self.created_at,
            self.updated_at,
        )
    }
}
