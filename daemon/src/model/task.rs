use chrono::Local;
use serde::Serialize;
use sqlx::SqlitePool;

use crate::error::{AppError, Error};

pub const TABLE_TASK: &str = "task";

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Task {
    pub id: u32,         // 任务id
    pub name: String,    // 任务名称
    pub info: String,    // 任务描述
    pub tag: String,     // 任务tag
    pub created_at: i64, // 创建时间
}

pub struct CreateTask {
    pub name: String, // 任务名称
    pub info: String, // 任务描述
    pub tag: String,  // 任务tag
}

impl Task {
    pub(crate) async fn create(pool: &SqlitePool, data: CreateTask) -> anyhow::Result<i64> {
        Ok(sqlx::query(&format!(
            "INSERT INTO `{TABLE_TASK}` (`name`, `info`, `tag`, `created_at`) VALUES (?, ?, ?, ?)"
        ))
        .bind(data.name)
        .bind(data.info)
        .bind(data.tag)
        .bind(Local::now().timestamp())
        .execute(pool)
        .await
        .or(Err(Error::App(AppError::DB_ERROR)))?
        .last_insert_rowid())
    }

    pub(crate) async fn list(pool: &SqlitePool) -> Vec<Task> {
        sqlx::query_as::<_, Task>(&format!("SELECT * FROM `{TABLE_TASK}` ORDER BY `id` DESC"))
            .fetch_all(pool)
            .await
            .unwrap_or_default()
    }
}
