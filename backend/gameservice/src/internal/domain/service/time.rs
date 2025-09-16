use tokio::time::{Duration as TokioDuration, Duration, Instant, interval};
use tokio_stream::Stream;

use crate::internal::infrastructure::time_generator::TimeGenerator;

pub struct TimeDomainService;

impl TimeDomainService {
    pub fn get_time_stream(
        _game_start: Instant,
        _game_duration: Duration,
        _generator: &impl TimeGenerator,
    ) -> impl Stream<Item = u32> + Send + 'static {
        async_stream::stream! {
            let mut counter = 1u32;
            let mut interval = interval(TokioDuration::from_secs(1));

            // Set interval to not miss ticks and ensure real-time delivery
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            while counter <= 160 {
                // Wait for the tick first, then yield immediately for real-time streaming
                interval.tick().await;

                tracing::info!("Timer tick: {}", counter);
                yield counter;
                counter += 1;
            }

            tracing::info!("Timer stream completed");
        }
    }
}
