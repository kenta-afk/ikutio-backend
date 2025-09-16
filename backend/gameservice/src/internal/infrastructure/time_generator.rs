use std::time::Duration;

use tokio::time::Instant;

pub trait TimeGenerator: Send + Sync + 'static {
    fn now(&self) -> Instant;
    fn elapsed_since(&self, start: Instant) -> Duration;
    fn create_duration(&self, seconds: u64) -> Duration;
}

pub struct TimeGeneratorImpl;

impl TimeGenerator for TimeGeneratorImpl {
    fn now(&self) -> Instant {
        Instant::now()
    }

    fn elapsed_since(&self, start: Instant) -> Duration {
        start.elapsed()
    }

    fn create_duration(&self, seconds: u64) -> Duration {
        Duration::from_secs(seconds)
    }
}
