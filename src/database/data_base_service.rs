use mongodb::{
    error::Error,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

pub struct DatabaseConfig {
    connect_id: String,
}

impl DatabaseConfig {
    pub fn new(connect_id: String) -> DatabaseConfig {
        DatabaseConfig { connect_id }
    }
}

pub struct MongoDatabase {
    config: DatabaseConfig,
    client: Client,
}

impl MongoDatabase {
    pub async fn new(config: DatabaseConfig) -> Result<MongoDatabase, Error> {
        let mut client_options = ClientOptions::parse(&config.connect_id).await?;

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();

        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options)?;

        Ok(MongoDatabase { config, client })
    }

    pub fn get_client(&self) -> Client {
        return self.client.clone();
    }
}
