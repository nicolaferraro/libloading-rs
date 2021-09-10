use std::convert::TryFrom;

use async_trait::async_trait;
use hyper::{Client, Uri};
use hyper::body::HttpBody;
use tokio::io::AsyncWriteExt;

use common::Helloer;

pub struct Actor {}

#[async_trait]
impl Helloer for Actor {
    async fn say_hello(&self) -> String {
        let client = Client::new();
        let uri = Uri::try_from("http://eu.httpbin.org/").unwrap();
        let mut response = client.get(uri).await.unwrap();

        let mut data = Vec::<u8>::new();
        while let Some(chunk) = response.body_mut().data().await {
            data.write_all(&chunk.unwrap()).await.unwrap();
        }
        let str = String::from_utf8_lossy(data.as_slice());
        str.to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check_working() {
        let a = Actor {};
        let res = a.say_hello().await;
        assert!(res.contains("<html"));
    }
}
