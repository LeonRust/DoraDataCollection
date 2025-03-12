use chrono::Local;
use serde::Serialize;
use sqlx::SqlitePool;

use crate::error::{AppError, Error};

use super::Count;

pub const TABLE_ROBOT: &str = "robot";

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Robot {
    pub id: u32,            // 主键id
    pub robot_id: u32,      // 机器人id
    pub name: String,       // 机器人名称
    pub info: String,       // 机器人描述
    pub perception: String, // 机器人感知
    pub created_at: i64,    // 创建时间
}

pub struct CreateRobot {
    pub robot_id: u32,      // 机器人id
    pub name: String,       // 机器人名称
    pub info: String,       // 机器人描述
    pub perception: String, // 机器人感知
}

impl Robot {
    pub(crate) async fn exist(pool: &SqlitePool, robot_id: u32) -> bool {
        let count = sqlx::query_as::<_, Count>(&format!(
            "SELECT COUNT(*) as `count` FROM `{TABLE_ROBOT}` WHERE `robot_id` = ?"
        ))
        .bind(robot_id)
        .fetch_one(pool)
        .await
        .unwrap_or_default();

        count.count > 0
    }

    pub(crate) async fn create(pool: &SqlitePool, data: CreateRobot) -> anyhow::Result<i64> {
        Ok(sqlx::query(
            &format!("INSERT INTO `{TABLE_ROBOT}` (`robot_id`, `name`, `info`, `perception`, `created_at`) VALUES (?, ?, ?, ?, ?)")
        )
        .bind(data.robot_id)
        .bind(data.name)
        .bind(data.info)
        .bind(data.perception)
        .bind(Local::now().timestamp())
        .execute(pool).await.or(Err(Error::App(AppError::DB_ERROR)))?.last_insert_rowid())
    }

    pub(crate) async fn list(pool: &SqlitePool) -> Vec<Robot> {
        sqlx::query_as::<_, Robot>(&format!("SELECT * FROM `{TABLE_ROBOT}` ORDER BY `id` DESC"))
            .fetch_all(pool)
            .await
            .unwrap_or_default()
    }
}
