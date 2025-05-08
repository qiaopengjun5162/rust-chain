use axum::{Extension, Router, response::Html, routing::get};

use dotenv::dotenv;
use ethers::{
    prelude::{Http, Provider},
    types::Address,
};
use eyre::Result;
use std::sync::Arc;
use tracing::info;

mod counter;
mod routes;
use counter::Counter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let rpc_url = std::env::var("RPC_URL").expect("RPC_URL must be set");
    let counter_address = std::env::var("COUNTER_ADDRESS").expect("COUNTER_ADDRESS must be set");

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let counter_address: Address = String::from(counter_address).parse()?;
    let counter = Counter::new(Arc::new(provider), counter_address);

    let app = Router::new()
        .route("/", get(handler))
        .route("/api/number", get(routes::handle_number))
        .route("/api/block_number", get(routes::handle_block_number))
        .layer(Extension(counter));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
