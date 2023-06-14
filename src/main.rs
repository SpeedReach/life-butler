#![allow(dead_code)]
#![allow(unused)]



use life_butler::driver::bootstrap::start;

#[tokio::main]
async fn main() {

    start().await;
}
