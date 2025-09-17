use uuid::Uuid;

use crate::internal::infrastructure::uuid_generator::UuidGenerator;

pub struct UserId(Uuid);
impl UserId {
    pub fn from_uuid(uuid: Uuid) -> Self {
        UserId(uuid)
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub struct LocationId(Uuid);
impl LocationId {
    pub fn new(uuid_generator: &impl UuidGenerator) -> Self {
        LocationId(uuid_generator.new_v7())
    }
    pub fn from_uuid(uuid: Uuid) -> Self {
        LocationId(uuid)
    }
    pub fn from_string(uuid_str: String) -> Result<Self, uuid::Error> {
        Uuid::parse_str(&uuid_str).map(LocationId)
    }
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
