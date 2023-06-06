#![allow(dead_code)]
#![allow(unused)]
#![feature(async_fn_in_trait)]


use life_butler::driver::bootstrap::start;

#[tokio::main]
async fn main() {
    start().await;
}
