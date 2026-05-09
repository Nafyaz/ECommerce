use crate::modules::product::ports::inbound::{ProductCommandPort, ProductImageCommandPort};
use std::sync::Arc;

#[derive(Clone)]
pub struct ProductHttpState {
    pub product_command_service: Arc<dyn ProductCommandPort>,
    pub product_image_command_service: Arc<dyn ProductImageCommandPort>,
}

impl ProductHttpState {
    pub fn new(
        product_command_service: Arc<dyn ProductCommandPort>,
        product_image_command_service: Arc<dyn ProductImageCommandPort>,
    ) -> Self {
        Self {
            product_command_service,
            product_image_command_service,
        }
    }
}
