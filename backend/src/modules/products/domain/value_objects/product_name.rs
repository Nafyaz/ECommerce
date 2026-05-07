use crate::modules::products::errors::ProductDomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductName(String);

impl ProductName {
    const NAME_MIN_LENGTH: usize = 3;
    const NAME_MAX_LENGTH: usize = 64;
    pub fn new(name: impl Into<String>) -> Result<Self, ProductDomainError> {
        let name = name.into();
        if name.len() < Self::NAME_MIN_LENGTH || name.len() > Self::NAME_MAX_LENGTH {
            return Err(ProductDomainError::InvalidProductName(name));
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
