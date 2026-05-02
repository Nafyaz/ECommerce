use uuid::Uuid;

pub struct CreateUserResult {
    id: Uuid,
}

impl CreateUserResult {
    pub fn new(id: Uuid) -> Self {
        CreateUserResult { id }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}
