mod poc;

use tokio;
use convergence::server::{self, BindOptions};
use std::sync::Arc;
use std::thread::sleep_ms;
use convergence::connection::Connection;
use poc::POCEngine;
use duckdb::{DuckdbConnectionManager, params};

async fn start_server() {
    let manager = DuckdbConnectionManager::memory().unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();


    let port = server::run_background(
        BindOptions::new().with_port(0),
        Arc::new(|| Box::pin(async { POCEngine { pool: pool.clone() } })),
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
