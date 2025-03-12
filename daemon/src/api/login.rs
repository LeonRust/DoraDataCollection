use std::sync::Arc;

use axum::{response::IntoResponse, routing::post, Extension, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{
    db::DbState,
    error::{AppError, Error},
    model::admin::Admin,
    Result,
};

pub fn router() -> Router {
    Router::new().route("/", post(login))
}

#[derive(Debug, Deserialize)]
struct LoginParams {
    username: String,
    password: String,
}
async fn login(
    Extension(app_state): Extension<Arc<DbState>>,
    body: String,
) -> Result<impl IntoResponse> {
    let Ok(post_data) = serde_json::from_str::<LoginParams>(&body) else {
        return Err(Error::App(AppError::JSON_PARSE_FAILED));
    };

    let admin = Admin::admin(&app_state.database, &post_data.username).await;

    if let Some(ref admin) = admin {
        let valid = bcrypt::verify(post_data.password.as_str(), &admin.password)
            .or(Err(Error::App(AppError::ACCOUNT_LOGIN_FAIL)))?;
        if !valid {
            return Err(Error::App(AppError::ACCOUNT_LOGIN_FAIL));
        }
    }

    #[derive(Debug, Serialize)]
    struct LoginResponse {
        token: String,
        timestamp: u64,
    }

    Ok(Json(LoginResponse {
        token: "".to_string(),
        timestamp: 0,
    }))
}
