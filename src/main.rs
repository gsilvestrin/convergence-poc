mod poc;

use tokio;
use convergence::server::{self, BindOptions};
use std::sync::Arc;
use std::thread::sleep_ms;
use convergence::connection::Connection;
use poc::POCEngine;

async fn start_server() {
    let duck_conn = Arc::new(Connection::open_in_memory().unwrap());

    let port = server::run_background(
        BindOptions::new().with_port(0),
        Arc::new(|| Box::pin(async { POCEngine { duck_conn: duck_conn.clone() } })),
    )
        .await
        .unwrap();

    println!("PG Started on port {}", port);
}

#[tokio::main]
async fn main() {
    start_server().await;
    sleep_ms(60 * 1000);
}
