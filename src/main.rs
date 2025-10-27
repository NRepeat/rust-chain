mod api;
mod blockchain;
mod domain;
mod infrastructure;
use crate::api::server;

#[tokio::main]
async fn main() {
    server::app().await;
}
