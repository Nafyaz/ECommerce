use uuid::Uuid;

pub struct CreateProductResult {
    id: Uuid,
}

impl CreateProductResult {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}
