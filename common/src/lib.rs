use async_trait::async_trait;

#[async_trait]
pub trait Helloer {
    async fn say_hello(&self) -> String;
}
