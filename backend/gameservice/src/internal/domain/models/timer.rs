use std::pin::Pin;
use std::task::{Context, Poll};

use tokio_stream::Stream;

pub struct Timer(u32);

impl Stream for Timer {
    type Item = u32;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.0 == 0 {
            Poll::Ready(None)
        } else {
            // 実際のタイマー実装では、内部状態を使って遅延を管理する
            let current_value = self.0;
            self.0 -= 1;
            Poll::Ready(Some(current_value))
        }
    }
}
