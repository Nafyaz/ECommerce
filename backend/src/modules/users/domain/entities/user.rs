use crate::modules::users::domain::value_objects::{Email, PasswordHash, Phone, UserId};
use chrono::{DateTime, Utc};

//TODO: Manually implement slugs and use them
// TODO: Use name VO for empty / size validation
pub struct User {
    id: UserId,
    name: String,
    email: Option<Email>,
    email_verified_at: Option<DateTime<Utc>>,
    phone: Option<Phone>,
    phone_verified_at: Option<DateTime<Utc>>,
    password_hash: PasswordHash,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    fn new(name: String, email: Option<Email>, phone: Option<Phone>, password_hash: PasswordHash) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            name,
            email,
            email_verified_at: None,
            phone,
            phone_verified_at: None,
            password_hash,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn new_by_email(name: String, email: Email, password_hash: PasswordHash) -> Self {
        Self::new(name, Some(email), None, password_hash)
    }

    pub fn new_by_phone(name: String, phone: Phone, password_hash: PasswordHash) -> Self {
        Self::new(name, None, Some(phone), password_hash)
    }

    pub fn reconstitute(
        id: UserId,
        name: String,
        email: Option<Email>,
        email_verified_at: Option<DateTime<Utc>>,
        phone: Option<Phone>,
        phone_verified_at: Option<DateTime<Utc>>,
        password_hash: PasswordHash,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            email,
            email_verified_at,
            phone,
            phone_verified_at,
            password_hash,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> Option<&Email> {
        self.email.as_ref()
    }

    pub fn email_verified_at(&self) -> Option<DateTime<Utc>> {
        self.email_verified_at
    }

    pub fn phone(&self) -> Option<&Phone> {
        self.phone.as_ref()
    }

    pub fn phone_verified_at(&self) -> Option<DateTime<Utc>> {
        self.phone_verified_at
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
