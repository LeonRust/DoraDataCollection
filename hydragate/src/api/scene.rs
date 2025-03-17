use std::sync::Arc;

use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, patch},
    Extension, Json, Router,
};
use serde::Deserialize;

use crate::{
    db::DbState,
    error::{AppError, Error},
    model::scene::{CreateScene, Scene},
    Result,
};

use super::{ApiId, DataList};

pub fn router() -> Router {
    Router::new()
        .route("/", get(index).post(create))
        .route("/{id}", patch(update).delete(destroy))
}

async fn index(Extension(app_state): Extension<Arc<DbState>>) -> impl IntoResponse {
    Json(DataList {
        data: Scene::list(&app_state.database).await,
    })
}

#[derive(Debug, Deserialize)]
struct ScenePost {
    name: String,
    info: Option<String>,
}
async fn create(
    Extension(app_state): Extension<Arc<DbState>>,
    body: String,
) -> Result<impl IntoResponse> {
    let Ok(post_data) = serde_json::from_str::<ScenePost>(&body) else {
        return Err(Error::App(AppError::JSON_PARSE_FAILED));
    };

    let id = Scene::create(
        &app_state.database,
        CreateScene {
            name: post_data.name,
            info: post_data.info.unwrap_or_default(),
        },
    )
    .await?;

    Ok(Json(ApiId { id }))
}

async fn update(
    Path(id): Path<String>,
    Extension(app_state): Extension<Arc<DbState>>,
) -> &'static str {
    "Hello, World!"
}

async fn destroy(
    Path(id): Path<String>,
    Extension(app_state): Extension<Arc<DbState>>,
) -> &'static str {
    "Hello, World!"
}
