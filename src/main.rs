mod api;
mod blockchain;
mod domain;
mod infrastructure;
use crate::api::server;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    server::app().await;
}
