use tokio_stream::Stream;

pub struct StartGameDto {
    pub time: Box<dyn Stream<Item = u32> + Send + Unpin>,
}
