#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProductImageStatus {
    PendingUpload,
    Uploaded,
    Processing,
    Ready,
    Failed,
    Deleted,
}
