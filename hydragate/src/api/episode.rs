use std::sync::{Arc, atomic::Ordering};

use axum::{
    Extension, Json, Router,
    extract::Path,
    response::IntoResponse,
    routing::{get, patch, post},
};
use chrono::Local;
use common::state::{BizType, TcpState};
use serde::Deserialize;
use tokio::{fs::File, io::AsyncWriteExt, sync::Mutex};

use crate::{
    Result,
    db::DbState,
    error::{AppError, Error},
    model::episode::{CreateEpisode, Episode, ResultEnum},
};

use super::{ApiId, ApiResult};

pub fn router() -> Router {
    Router::new()
        .route("/", get(index).post(create))
        .route("/{id}", patch(update).delete(destroy))
        .route("/set-result", post(set_result))
        .route("/result", get(result))
}

async fn index(Extension(app_state): Extension<Arc<DbState>>) -> impl IntoResponse {
    // Json(DataList {
    //     data: Episode::list(&app_state.database).await,
    // })
    ""
}

#[derive(Debug, Deserialize)]
struct EpisodePost {
    robot_id: u32,
    scene_id: u32,
    task_id: u32,
    episode_id: u32,
}
async fn create(
    Extension(app_state): Extension<Arc<DbState>>,
    body: String,
) -> Result<impl IntoResponse> {
    let Ok(post_data) = serde_json::from_str::<EpisodePost>(&body) else {
        return Err(Error::App(AppError::JSON_PARSE_FAILED));
    };

    let id = Episode::create(
        &app_state.database,
        CreateEpisode {
            robot_id: post_data.robot_id,
            scene_id: post_data.scene_id,
            task_id: post_data.task_id,
            episode_id: post_data.episode_id,
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

#[derive(Debug, Deserialize)]
struct SetResult {
    result: bool,
}
async fn set_result(
    Extension(app_state): Extension<Arc<DbState>>,
    Extension(tcp_state): Extension<Arc<Mutex<TcpState>>>,
    body: String,
) -> Result<impl IntoResponse> {
    let Ok(post_data) = serde_json::from_str::<SetResult>(&body) else {
        return Err(Error::App(AppError::JSON_PARSE_FAILED));
    };

    let id = app_state.data_id.load(Ordering::Relaxed);
    if id > 0 {
        Episode::set_result(
            &app_state.database,
            if post_data.result {
                ResultEnum::Success
            } else {
                ResultEnum::Fail
            },
            id,
        )
        .await;

        let mut mut_tcp_state = tcp_state.lock().await;
        if let Ok(mut file) = File::create(format!("{}/result.txt", mut_tcp_state.path)).await {
            file.write_all(
                format!(
                    "{} {}",
                    Local::now().timestamp_millis(),
                    if post_data.result { 1 } else { 0 }
                )
                .as_bytes(),
            )
            .await
            .ok();
        }
        mut_tcp_state.biz_type = BizType::None;
        app_state.show_result.store(false, Ordering::Release);
    }

    Ok(Json(ApiResult { result: true }))
}

async fn result(Extension(app_state): Extension<Arc<DbState>>) -> impl IntoResponse {
    Json(ApiResult {
        result: app_state.show_result.load(Ordering::Relaxed),
    })
}
