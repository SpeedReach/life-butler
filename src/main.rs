use std::net::SocketAddr;

use axum::Router;

mod database;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let main_route = Router::new();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("->> LISTENING on {addr}\n");

    axum::Server::bind(&addr)
        .serve(main_route.into_make_service())
        .await
        .unwrap();
}
