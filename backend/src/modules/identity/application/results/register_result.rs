use uuid::Uuid;

pub struct RegisterResult {
    id: Uuid,
}

impl RegisterResult {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}
