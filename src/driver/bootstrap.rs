use crate::driver::module::Modules;
use crate::driver::routes::user_routes::{register_user, user_login};
use axum::middleware::AddExtension;
use axum::routing::{get, post};
use axum::{Extension, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::driver::swagger::ApiDoc;

static MONGO_PASSWD: &str = "MONGO_PASSWD";

pub async fn start() {
    let env_passwd = std::env::var(MONGO_PASSWD);
    let mut password: Option<String> = Option::None;
    if let Ok(pwd) = env_passwd {
        password = Some(pwd)
    } else {
        password = std::env::args().nth(1);
    }
    let password = password.expect("database password not set \"cargo run <MONGODB_PASSWORD>\" ");
    let modules = Arc::new(Modules::new(password.as_str()).await);


    let user_route = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(user_login));

    let main_route = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/user", user_route)
        .layer(Extension(Arc::clone(&modules)));

    let tracing_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry().with(tracing_layer).init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(main_route.into_make_service())
        .await
        .unwrap();

}
