use crate::internal::domain::models::id::{GameId, UserId};
use crate::internal::infrastructure::uuid_generator::UuidGenerator;

pub struct Game {
    pub game_id: GameId,
    pub user_id: UserId,
    pub is_finished: bool,
    pub score: u32,
}

impl Game {
    pub fn new(user_id: UserId, generator: &impl UuidGenerator) -> Self {
        Game { game_id: GameId::new(generator), user_id, is_finished: false, score: 0 }
    }

    pub fn finish(&mut self, score: u32) {
        self.is_finished = true;
        self.score = score;
    }
}
