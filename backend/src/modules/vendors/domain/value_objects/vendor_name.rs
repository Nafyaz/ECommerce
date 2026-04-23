use crate::modules::vendors::errors::VendorDomainError;

const MAX_NAME_LENGTH: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VendorName(String);

impl VendorName {
    pub fn new(name: impl Into<String>) -> Result<Self, VendorDomainError> {
        let name = name.into();

        if name.is_empty() {
            return Err(VendorDomainError::InvalidName("Name cannot be empty".into()));
        }

        if name.len() > MAX_NAME_LENGTH {
            return Err(VendorDomainError::InvalidName(format!(
                "Name cannot be longer than {MAX_NAME_LENGTH} characters"
            )));
        }

        Ok(Self(name))
    }

    pub fn from_str(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
