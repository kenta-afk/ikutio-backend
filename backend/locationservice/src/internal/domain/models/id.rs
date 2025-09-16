use uuid::Uuid;

pub struct UserId(Uuid);
impl UserId {
    pub fn from_uuid(uuid: Uuid) -> Self {
        UserId(uuid)
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
