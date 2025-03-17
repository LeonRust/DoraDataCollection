use std::sync::Arc;

use axum::{
    Extension, Router,
    http::{StatusCode, Uri, header},
    response::{Html, IntoResponse, Response},
};
use common::state::{TcpState, UsbState, UsbType};
use tokio::{
    net::TcpListener,
    sync::{Mutex, broadcast::Sender},
};
use tower_http::services::ServeDir;

use crate::{api, asset::Asset, db::DbState};

const INDEX_HTML: &str = "index.html";
pub async fn run(
    addr: String,
    db_state: Arc<DbState>,
    tcp_state: Arc<Mutex<TcpState>>,
    usb_state: Arc<Mutex<UsbState>>,
    datasets_path: String,
) -> anyhow::Result<()> {
    let app = Router::new()
        .nest_service(&format!("/{}", datasets_path), ServeDir::new(datasets_path))
        .nest("/api", api::router())
        .layer(Extension(db_state))
        .layer(Extension(tcp_state))
        .layer(Extension(usb_state))
        .fallback(static_handler);

    let listener = TcpListener::bind(&addr).await?;
    eprintln!("HTTP listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Asset::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => not_found().await,
    }
}

async fn index_html() -> Response {
    match Asset::get(INDEX_HTML) {
        Some(content) => {
            let body = String::from_utf8_lossy(&content.data);
            Html(body.to_string()).into_response()
        }
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404 Not found").into_response()
}
