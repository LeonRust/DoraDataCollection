use std::sync::{atomic::Ordering, Arc};

use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, patch, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    db::DbState,
    error::{AppError, Error},
    model::{
        episode::{CreateEpisode, Episode},
        task::{CreateTask, Task},
    },
    Result,
};

use super::{ApiId, DataList};

pub fn router() -> Router {
    Router::new()
        .route("/", get(index).post(create))
        .route("/{id}", patch(update).delete(destroy))
        .route("/current-task", get(current_task))
        .route("/start", post(start))
        .route("/stop", post(stop))
}

async fn index(Extension(app_state): Extension<Arc<DbState>>) -> impl IntoResponse {
    Json(DataList {
        data: Task::list(&app_state.database).await,
    })
}

#[derive(Debug, Deserialize)]
struct TaskPost {
    name: String,
    info: Option<String>,
    tag: Option<String>,
}
async fn create(
    Extension(app_state): Extension<Arc<DbState>>,
    body: String,
) -> Result<impl IntoResponse> {
    let Ok(post_data) = serde_json::from_str::<TaskPost>(&body) else {
        return Err(Error::App(AppError::JSON_PARSE_FAILED));
    };

    let id = Task::create(
        &app_state.database,
        CreateTask {
            name: post_data.name,
            info: post_data.info.unwrap_or_default(),
            tag: post_data.tag.unwrap_or_default(),
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

async fn current_task(Extension(app_state): Extension<Arc<DbState>>) -> impl IntoResponse {
    #[derive(Debug, Serialize)]
    struct CurrentTask {
        is_running: bool,
        task_name: String,
        result: bool, // 是否显示弹窗
        robot_id: u32,
        scene_id: u32,
        task_id: u32,
    }

    let robot_id = app_state.robot_id.load(Ordering::Relaxed);
    let scene_id = app_state.scene_id.load(Ordering::Relaxed);
    let task_id = app_state.task_id.load(Ordering::Relaxed);
    let episode_id = app_state.episode_id.load(Ordering::Relaxed);

    let is_running = robot_id > 0 && scene_id > 0 && task_id > 0;

    Json(CurrentTask {
        is_running,
        task_name: if is_running {
            format!(
                "robot-{}/scene-{}/task-{}/episode-{}",
                robot_id, scene_id, task_id, episode_id
            )
        } else {
            "".to_string()
        },
        result: app_state
            .show_result
            .load(std::sync::atomic::Ordering::Relaxed),
        robot_id,
        scene_id,
        task_id,
    })
}

#[derive(Debug, Deserialize)]
struct StartTask {
    robot_id: u32,
    scene_id: u32,
    task_id: u32,
}
async fn start(
    Extension(app_state): Extension<Arc<DbState>>,
    body: String,
) -> Result<impl IntoResponse> {
    let Ok(post_data) = serde_json::from_str::<StartTask>(&body) else {
        return Err(Error::App(AppError::JSON_PARSE_FAILED));
    };

    // 获取最后的数据
    let (id, episode_id) = Episode::get_last_episode_id(
        &app_state.database,
        CreateEpisode {
            robot_id: post_data.robot_id,
            scene_id: post_data.scene_id,
            task_id: post_data.task_id,
            episode_id: 0,
        },
    )
    .await;
    eprintln!("last id: {}, episode_id: {}", id, episode_id);

    app_state.data_id.store(id, Ordering::Release);
    app_state
        .robot_id
        .store(post_data.robot_id, Ordering::Release);
    app_state
        .scene_id
        .store(post_data.scene_id, Ordering::Release);
    app_state
        .task_id
        .store(post_data.task_id, Ordering::Release);
    app_state.episode_id.store(episode_id, Ordering::Release);

    Ok(Json(()))
}

async fn stop(Extension(app_state): Extension<Arc<DbState>>) -> Result<impl IntoResponse> {
    Episode::stop_all(&app_state.database).await;

    app_state.data_id.store(0, Ordering::Release);
    app_state.robot_id.store(0, Ordering::Release);
    app_state.scene_id.store(0, Ordering::Release);
    app_state.task_id.store(0, Ordering::Release);
    app_state.episode_id.store(0, Ordering::Release);

    Ok(Json(()))
}
