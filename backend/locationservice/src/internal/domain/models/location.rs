use serde::{Deserialize, Serialize};

use crate::internal::domain::models::id::UserId;

pub struct Locations {
    pub user_id: UserId,
    pub locations: Vec<Location>,
    pub is_finished: bool,
}

impl Locations {
    pub fn new(user_id: UserId, locations: Vec<Location>, is_finished: bool) -> Self {
        Self { user_id, locations, is_finished }
    }

    pub fn finished(&mut self) {
        self.is_finished = true;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: i64,
}
