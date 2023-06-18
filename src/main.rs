
use api::server;
mod models;
mod api;
mod db;

#[tokio::main]
async fn main() {

match server::start().await {
    Ok(_) => println!("Server started successfully"),
    Err(e) => println!("Error starting server: {:?}", e),
};

}
