use std::sync::Arc;

use axum::{routing::get, Router};
use tokio::net::TcpListener;

use crate::db::DbState;

pub async fn run(addr: String, db_state: Arc<DbState>) -> anyhow::Result<()> {
    let app = Router::new().route("/", get(index)).with_state(db_state);

    let listener = TcpListener::bind(&addr).await?;
    eprintln!("HTTP listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn index() -> &'static str {
    "Hello, World!"
}
