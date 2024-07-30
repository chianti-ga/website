use mongodb::Client;

use crate::CONFIG;

#[derive(Clone)]
pub struct DatabaseStruct {
    pub client: Client,
}
impl DatabaseStruct {
    pub async fn init() -> Self {
        let uri: &String = &CONFIG.mongo_db_uri;
        let client: Client = Client::with_uri_str(uri).await.expect("[ERROR] Can't connect to mongodb server!");
        DatabaseStruct {
            client
        }
    }
}
