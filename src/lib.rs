#![allow(dead_code)]
#![allow(unused)]
use std::net::SocketAddr;
use axum::Router;

pub mod application;
pub mod infrastructure;
pub mod driver;
pub mod domain;



pub async fn start() {
    let main_route = Router::new();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    print!("listening on {}", addr);



    axum::Server::bind(&addr)
        .serve(main_route.into_make_service())
        .await
        .unwrap();
}
