use async_trait::async_trait;
use tokio::runtime::Handle;

use common::Helloer;

#[no_mangle]
pub fn load_plugin(handle: Handle) -> Box<dyn Helloer> {
    Box::new(Actor { handle })
}

pub struct Actor {
    handle: Handle,
}

#[async_trait]
impl Helloer for Actor {
    async fn say_hello(&self) -> String {
        println!("2. Enter say_hello, let's spawn a new task on the handle from the dylib...");
        self.handle.spawn(async {
            println!("3. Print something (will never happen)!");
        }).await.unwrap();
        "4. Return value".to_string()
    }
}
