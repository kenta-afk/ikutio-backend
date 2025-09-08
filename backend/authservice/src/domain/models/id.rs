use serde::Serialize;
use uuid::Uuid;

use crate::infrastructure::uuid_generator::UuidGenerator;

#[derive(Serialize, Clone, Copy)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(uuid_generator: &impl UuidGenerator) -> Self {
        UserId(uuid_generator.new_v7())
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
