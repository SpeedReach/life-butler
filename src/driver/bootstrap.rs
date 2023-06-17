use crate::driver::module::Modules;
use crate::driver::routes::user_routes::{register_user, user_login};
use axum::middleware::AddExtension;
use axum::routing::{get, patch, post, Route};
use axum::{Extension, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::driver::routes::event_routes::{create_event, get_expired_events, get_recent_events};
use crate::driver::routes::routine_routes::{get_routine, user_eat, user_sleep};
use crate::driver::routes::task_routes::{create_task, get_expired_tasks, get_ongoing_tasks, update_task};

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

    let event_route = Router::new()
        .route("/",post(create_event))
        .route("/recent", get(get_recent_events).post(get_recent_events))
        .route("/expired", get(get_expired_events).post(get_expired_tasks));

    let task_route = Router::new()
        .route("/", post(create_task))
        .route("/ongoing", get(get_ongoing_tasks).post(get_ongoing_tasks))
        .route("/expired", get(get_expired_tasks).post(get_expired_tasks))
        .route("/update", post(update_task).patch(update_task));

    let routine_route = Router::new()
        .route("/:user_id", get(get_routine))
        .route("/sleep",post(user_sleep))
        .route("/eat/:user_id",post(user_eat));

    let main_route = Router::new()
        .nest("/user", user_route)
        .nest("/event", event_route)
        .nest("/task", task_route)
        .nest("/routine", routine_route)
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
