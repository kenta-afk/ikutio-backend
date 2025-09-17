use serde::{Deserialize, Serialize};

use crate::internal::domain::models::id::{LocationId, UserId};

pub struct Locations {
    pub location_id: LocationId,
    pub user_id: UserId,
    pub locations: Vec<Location>,
    pub is_finished: bool,
}

impl Locations {
    pub fn new(
        location_id: LocationId,
        user_id: UserId,
        locations: Vec<Location>,
        is_finished: bool,
    ) -> Self {
        Self { location_id, user_id, locations, is_finished }
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
