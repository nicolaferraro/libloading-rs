use std::env::current_exe;

use libloading::{Library, library_filename};
use tokio::runtime::Handle;

use common::Helloer;

#[tokio::main]
async fn main() {
    let (_lib, helloer) = load_helloer_from_lib();

    Handle::current().spawn(async {
        println!("1. Spawn on current runtime from main lib");
    }).await.unwrap();

    let resp = helloer.say_hello().await;
    println!("{}", resp);

    println!("5. End of main");
}

fn load_helloer_from_lib() -> (Library, Box<dyn Helloer>) {
    let exe_path = current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let lib_name = library_filename("plugin");
    let lib_file = exe_dir.join(lib_name);
    unsafe {
        let lib = libloading::Library::new(lib_file).unwrap();
        let func: libloading::Symbol<unsafe extern fn(Handle) -> Box<dyn Helloer>> = lib.get(b"load_plugin").unwrap();
        let helloer = func(Handle::current());
        (lib, helloer)
    }
}
