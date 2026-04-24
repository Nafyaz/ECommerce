use uuid::Uuid;

pub struct CreateProductResult {
    product_id: Uuid,
}

impl CreateProductResult {
    pub fn new(product_id: Uuid) -> Self {
        Self { product_id }
    }

    pub fn product_id(&self) -> &Uuid {
        &self.product_id
    }
}
