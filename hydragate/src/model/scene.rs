use chrono::Local;
use serde::Serialize;
use sqlx::SqlitePool;

use crate::error::{AppError, Error};

pub const TABLE_SCENE: &str = "scene";

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Scene {
    pub id: u32,         // 场景id
    pub name: String,    // 场景名称
    pub info: String,    // 场景描述
    pub created_at: i64, // 创建时间
}

pub struct CreateScene {
    pub name: String, // 机器人名称
    pub info: String, // 机器人描述
}

impl Scene {
    pub(crate) async fn create(pool: &SqlitePool, data: CreateScene) -> anyhow::Result<i64> {
        Ok(sqlx::query(&format!(
            "INSERT INTO `{TABLE_SCENE}` (`name`, `info`, `created_at`) VALUES (?, ?, ?)"
        ))
        .bind(data.name)
        .bind(data.info)
        .bind(Local::now().timestamp())
        .execute(pool)
        .await
        .or(Err(Error::App(AppError::DB_ERROR)))?
        .last_insert_rowid())
    }

    pub(crate) async fn list(pool: &SqlitePool) -> Vec<Scene> {
        sqlx::query_as::<_, Scene>(&format!("SELECT * FROM `{TABLE_SCENE}` ORDER BY `id` DESC"))
            .fetch_all(pool)
            .await
            .unwrap_or_default()
    }
}
