use std::net::SocketAddr;
use std::sync::Arc;
use axum::middleware::AddExtension;
use axum::{Extension, Router};
use axum::routing::{get, post};
use crate::driver::module::Modules;
use crate::driver::routes::user_routes::{register_user, user_login};


pub async fn start() {
    let password = std::env::args().nth(1).expect("missing password");
    let modules = Arc::new(Modules::new(&password).await);

    let user_route = Router::new()
        .route("/",get(register_user))
        .route("/register",post(register_user))
        .route("/login",post(user_login));


    let main_route = Router::new()
        .nest("/user",user_route)
        .layer(Extension(Arc::clone(&modules)));


    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    print!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(main_route.into_make_service())
        .await
        .unwrap();
}
