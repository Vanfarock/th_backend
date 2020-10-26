use mongodb::sync::{ Client, Database };

pub struct DatabaseConnection;

impl DatabaseConnection {
    pub fn init() -> Database {
        let client_uri = "mongodb://th_backend:!ThBackend!@thbackend-shard-00-00.njpgf.gcp.mongodb.net:27017,thbackend-shard-00-01.njpgf.gcp.mongodb.net:27017,thbackend-shard-00-02.njpgf.gcp.mongodb.net:27017/ThBackend?ssl=true&replicaSet=atlas-dt7ray-shard-0&authSource=admin&retryWrites=true&w=majority";
        Client::with_uri_str(client_uri.as_ref())
                .expect("Could not start connection with mongodb")
                .database("gettingStarted")
    }
}