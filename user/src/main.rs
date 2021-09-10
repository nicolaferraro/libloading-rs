use common::Helloer;
use plugin::Actor;

#[tokio::main]
async fn main() {
    let helloer: Box<dyn Helloer> = Box::new(Actor{});
    let resp = helloer.say_hello().await;
    println!("{}", resp);
}
