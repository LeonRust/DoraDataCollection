use axum::{
    http::{HeaderValue, StatusCode, header},
    response::IntoResponse,
};
use serde::{Serialize, ser::SerializeStruct};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("DB error: {0}")]
    Db(String),
    #[error("Server error: {0}")]
    Server(String),
    #[error("App error: {0:?}")]
    App(AppError),
}

#[derive(Debug)]
pub struct AppError {
    /// 错误码
    pub error_code: &'static str,
    /// 错误信息
    pub error_message: &'static str,
    /// http 状态码
    pub status_code: StatusCode,
    /// 用户提示
    pub user_tip: &'static str,
}

macro_rules! app_error {
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $error_code:expr, $error_message: expr, $status_code:expr, $user_tip:expr);
        )*
    ) => {
        impl AppError {
            $(
                $(#[$docs])*
                pub const $name: AppError = AppError {
                    error_code: $error_code,
                    error_message: $error_message,
                    status_code: $status_code,
                    user_tip: $user_tip,
                };
            )+
        }
    };
}

app_error! {
    // ------------------------------------------------------------------------
    // 客户端错误，以 A 开头
    // ------------------------------------------------------------------------

    /// 404 不存在
    (NOT_FOUND, "A0001", "Not Found", StatusCode::NOT_FOUND, "服务器无法取得所请求的数据，请求资源不存在");
    /// 422 验证失败
    (UNPROCESSABLE_ENTITY, "A0003", "Validation Failed", StatusCode::UNPROCESSABLE_ENTITY, "参数验证失败");
    /// 转换json错误
    (JSON_PARSE_FAILED, "A0004", "Body should be a JSON object", StatusCode::BAD_REQUEST, "JSON数据不正确");
    /// 机器人id已经存在
    (ROBOT_ID_EXIST, "A0200", "robot id exist", StatusCode::UNAUTHORIZED, "机器人id已经存在");
    /// 帐号登录失败
    (ACCOUNT_LOGIN_FAIL, "A0201", "login fail", StatusCode::UNAUTHORIZED, "帐号登录失败");
    /// 启动系统失败
    (RUN_SYSTEM_FAIL, "A0202", "run system fail", StatusCode::UNAUTHORIZED, "启动系统失败");


    /// 系统执行出错
    (INTERNAL_SERVER_ERROR, "B0001", "Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR, "系统执行出错, 我们正在努力解决中, 请稍后重试.");
    /// 系统执行超时
    (DB_ERROR, "B0002", "Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR, "系统执行出错, 我们正在努力解决中, 请稍后重试.");
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("AppError", 4)?;
        state.serialize_field("errorCode", &self.error_code)?;
        state.serialize_field("errorMessage", &self.error_message)?;
        state.serialize_field("statusCode", &self.status_code.as_u16())?;
        state.serialize_field("userTip", &self.user_tip)?;
        state.end()
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Server(value.to_string())
    }
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Error::Server(value.to_string())
    }
}

impl From<sqlx::error::Error> for Error {
    fn from(value: sqlx::error::Error) -> Self {
        Error::Db(value.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let app_response = match self {
            Error::App(app_error) => app_error,
            Error::Server(err) => {
                // tracing::error!("Server Error: {err}");
                AppError::INTERNAL_SERVER_ERROR
            }
            Error::Db(err) => {
                // tracing::error!("数据库Error: {err}");
                AppError::DB_ERROR
            }
        };

        (
            app_response.status_code,
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json; charset=utf-8"),
            )],
            serde_json::to_string(&app_response).unwrap(),
        )
            .into_response()
    }
}
