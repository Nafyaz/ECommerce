use crate::modules::users::domain::value_objects::phone::Phone;
use crate::modules::users::domain::value_objects::user_name::UserName;
use crate::modules::users::domain::value_objects::{IdentityId, UserId};
use crate::modules::users::errors::UserDomainError;
use chrono::{DateTime, Utc};
use uuid::Uuid;

//TODO: Manually implement slugs and use them
pub struct User {
    id: UserId,
    identity_id: IdentityId,
    name: UserName,
    phone: Option<Phone>,
    phone_verified_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        identity_id: IdentityId,
        name: UserName,
        phone: Option<Phone>,
        phone_verified_at: Option<DateTime<Utc>>,
    ) -> Result<Self, UserDomainError> {
        let now = Utc::now();

        Ok(Self {
            id: UserId::new(),
            identity_id,
            name,
            phone,
            phone_verified_at,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn reconstitute(
        id: Uuid,
        identity_id: Uuid,
        name: String,
        phone: Option<String>,
        phone_verified_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: UserId::from_uuid(id),
            identity_id: IdentityId::from_uuid(identity_id),
            name: UserName::from_str(name),
            phone: phone.map(|p| Phone::from_str(p)),
            phone_verified_at,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn identity_id(&self) -> &IdentityId {
        &self.identity_id
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
