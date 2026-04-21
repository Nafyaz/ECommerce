use crate::modules::shared::AppError;
use crate::modules::vendors::application::commands::CreateVendorCommand;

pub async fn handle() -> Result<axum::response::Response, AppError> {
    let command = CreateVendorCommand::new()
}
