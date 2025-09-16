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

            while counter <= 160 {
                interval.tick().await;
                yield counter;
                counter += 1;
            }
        }
    }
}
