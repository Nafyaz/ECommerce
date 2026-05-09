use crate::modules::product::errors::ImageError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContentType(String);

impl ContentType {
    const ALLOWED: &'static [&'static str] = &["image/jpeg", "image/png", "image/webp"];
    // TODO: Study and allow AVIF

    pub fn new(content_type: impl Into<String>) -> Result<Self, ImageError> {
        let content_type = content_type.into();
        if !Self::ALLOWED.contains(&content_type.as_str()) {
            return Err(ImageError::InvalidContentType(content_type));
        }
        Ok(Self(content_type))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
