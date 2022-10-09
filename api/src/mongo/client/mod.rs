use mongodb::{options::ClientOptions, Client};

use crate::utils;

pub async fn client() -> Client {
    let uri = utils::get_app_vars("MONGO_URI".to_owned());

    let client_options = client_options(uri).await;

    let client = create_client(client_options).await;

    client
}

async fn client_options(uri: String) -> ClientOptions {
    let mut client_options = match ClientOptions::parse(uri).await {
        Ok(client_options) => client_options,
        Err(e) => panic!("Error with uri {}", e),
    };

    // Manually set an option.
    client_options.app_name = Some("Test App".to_owned());

    client_options
}

async fn create_client(client_options: ClientOptions) -> Client {
    let client = match Client::with_options(client_options) {
        Ok(client) => client,
        Err(e) => panic!("Error creating client: {}", e),
    };

    client
}
