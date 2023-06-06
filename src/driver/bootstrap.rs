use std::net::SocketAddr;
use std::sync::Arc;
use axum::middleware::AddExtension;
use axum::{Extension, Router};
use crate::driver::module::Modules;


pub async fn start() {
    let password = std::env::args().nth(1).expect("missing password");
    let modules = Arc::new(Modules::new(&password).await);
    let main_route = Router::new()
        .layer(Extension(Arc::clone(&modules)));


    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    print!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(main_route.into_make_service())
        .await
        .unwrap();
}
