use mongodb::{error::Error, options::{ClientOptions, ServerApi, ServerApiVersion}, Client, Database};


#[derive(Clone,Debug)]
pub struct DatabaseConfig {
    pub connect_id: String,
    pub database_id: String
}


impl DatabaseConfig {
    pub fn new(connect_id: impl Into<String>, database_id: impl Into<String>) -> DatabaseConfig {
        DatabaseConfig { connect_id: connect_id.into(), database_id: database_id.into() }
    }
}

pub struct DatabaseDriver {
    config: DatabaseConfig,
    client: Client,
}

impl DatabaseDriver {
    pub async fn new(config: DatabaseConfig) -> Result<DatabaseDriver, Error> {
        let mut client_options = ClientOptions::parse(&config.connect_id).await?;

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();

        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options)?;

        Ok(DatabaseDriver { config, client })
    }

    pub fn get_database(&self) -> Database {
        let client = self.client.clone();
        return client.database(self.config.database_id.as_str());
    }

    pub fn get_client(&self) -> Client{
        return self.client.clone();
    }

    pub fn get_config(&self)-> DatabaseConfig{
        return self.config.clone();
    }

}
