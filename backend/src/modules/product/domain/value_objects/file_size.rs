use crate::modules::product::errors::ImageError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileSize(u64);

impl FileSize {
    pub const MAX_BYTES: u64 = 10 * 1024 * 1024;

    pub fn new(size: u64) -> Result<Self, ImageError> {
        if size > Self::MAX_BYTES {
            return Err(ImageError::InvalidSize {
                size,
                max: Self::MAX_BYTES,
            });
        }
        Ok(Self(size))
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}
