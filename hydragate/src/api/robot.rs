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
    model::robot::{CreateRobot, Robot},
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
        data: Robot::list(&app_state.database).await,
    })
}

#[derive(Debug, Deserialize)]
struct RobotPost {
    robot_id: u32,              // 机器人id
    name: String,               // 机器人名称
    info: Option<String>,       // 机器人描述
    perception: Option<String>, // 机器人感知
}
async fn create(
    Extension(app_state): Extension<Arc<DbState>>,
    body: String,
) -> Result<impl IntoResponse> {
    let Ok(post_data) = serde_json::from_str::<RobotPost>(&body) else {
        return Err(Error::App(AppError::JSON_PARSE_FAILED));
    };

    // 数据校验
    {
        // 查询机器人id是否已经存在
        if Robot::exist(&app_state.database, post_data.robot_id).await {
            return Err(Error::App(AppError::ROBOT_ID_EXIST));
        }
    }

    let id = Robot::create(
        &app_state.database,
        CreateRobot {
            robot_id: post_data.robot_id,
            name: post_data.name,
            info: post_data.info.unwrap_or_default(),
            perception: post_data.perception.unwrap_or_default(),
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
