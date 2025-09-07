use uuid::Uuid;

pub trait UuidGenerator: Send + Sync + 'static {
    fn new_v7(&self) -> Uuid;
}

pub struct UuidGeneratorImpl;

impl UuidGenerator for UuidGeneratorImpl {
    fn new_v7(&self) -> Uuid {
        Uuid::now_v7()
    }
}