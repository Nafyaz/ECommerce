use crate::modules::product::errors::ImageError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProductImageStatus {
    PendingUpload,
    Uploaded,
    Processing,
    Ready,
    Failed,
    Deleted,
}

impl ProductImageStatus {
    pub fn from_str(product_image_status: impl Into<String>) -> Result<Self, ImageError> {
        match product_image_status.into().as_str() {
            "PENDING_UPLOAD" => Ok(Self::PendingUpload),
            "UPLOADED" => Ok(Self::Uploaded),
            "PROCESSING" => Ok(Self::Processing),
            "READY" => Ok(Self::Ready),
            "FAILED" => Ok(Self::Failed),
            "DELETED" => Ok(Self::Deleted),
            pis => Err(ImageError::InvalidProductImageStatus(pis.to_string())),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::PendingUpload => "PENDING_UPLOAD",
            Self::Uploaded => "UPLOADED",
            Self::Processing => "PROCESSING",
            Self::Ready => "READY",
            Self::Failed => "FAILED",
            Self::Deleted => "DELETED",
        }
    }
}
