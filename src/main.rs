use std::net::SocketAddr;

use axum::Router;

use crate::database::data_base_service::*;

mod database;

#[tokio::main]
async fn main() {
    let main_route = Router::new();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let _database = MongoDatabase::new(DatabaseConfig::new("mongodb+srv://brian920128:<password>@cluster0.hek6yds.mongodb.net/?retryWrites=true&w=majority".to_owned()))
        .await
        .unwrap();

    axum::Server::bind(&addr)
        .serve(main_route.into_make_service())
        .await
        .unwrap();
}
