use crate::modules::product::errors::ImageError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectKey(String);

impl ObjectKey {
    const MAX_LENGTH: usize = 768;

    pub fn new(key: impl Into<String>) -> Result<Self, ImageError> {
        let key = key.into();

        if key.is_empty() {
            return Err(ImageError::InvalidObjectKey("empty".into()));
        }
        if key.len() > Self::MAX_LENGTH {
            return Err(ImageError::InvalidObjectKey(format!(
                "length {} exceeds {}",
                key.len(),
                Self::MAX_LENGTH
            )));
        }
        if key.contains("..") || key.contains('\0') || key.starts_with('/') {
            return Err(ImageError::InvalidObjectKey("malformed".into()));
        }

        Ok(Self(key))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
