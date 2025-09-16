use uuid::Uuid;

use crate::internal::infrastructure::uuid_generator::UuidGenerator;

pub struct UserId(Uuid);
impl UserId {
    pub fn new(uuid_generator: &impl UuidGenerator) -> Self {
        UserId(uuid_generator.new_v7())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        UserId(uuid)
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub struct GameId(Uuid);
impl GameId {
    pub fn new(uuid_generator: &impl UuidGenerator) -> Self {
        GameId(uuid_generator.new_v7())
    }
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
