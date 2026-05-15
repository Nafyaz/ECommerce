use crate::modules::product::domain::ProductImageDomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileSize(i64);

impl FileSize {
    pub const MAX_BYTES: i64 = 10 * 1024 * 1024;
    pub const MIN_BYTES: i64 = 1;

    pub fn new(size: i64) -> Result<Self, ProductImageDomainError> {
        if size < Self::MIN_BYTES {
            return Err(ProductImageDomainError::InvalidSize {
                size,
                min: Self::MIN_BYTES,
                max: Self::MAX_BYTES,
            });
        }
        if size > Self::MAX_BYTES {
            return Err(ProductImageDomainError::InvalidSize {
                size,
                min: Self::MIN_BYTES,
                max: Self::MAX_BYTES,
            });
        }
        Ok(Self(size))
    }

    pub fn from_i64(size: i64) -> Result<Self, ProductImageDomainError> {
        Self::new(size)
    }

    pub fn as_i64(&self) -> i64 {
        self.0
    }
}
