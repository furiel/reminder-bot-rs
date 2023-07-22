use uuid::Uuid;

pub struct ID {
    pub description: String,
    pub uuid: Uuid,
}

impl PartialEq for ID {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
