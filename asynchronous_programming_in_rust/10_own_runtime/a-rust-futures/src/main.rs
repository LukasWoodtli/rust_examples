mod http;
mod runtime;


fn main() {
    let mut runtime = runtime::init();
    runtime.block_on(async_main());
}

async fn async_main() {
    println!("Program starting!");
    let txt = http::Http::get("/600/HelloAsyncAwait").await;
    println!("{txt}");
    let txt = http::Http::get("/400/HelloAsyncAwait").await;
    println!("{txt}");
}

