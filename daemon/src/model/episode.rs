use chrono::Local;
use sqlx::SqlitePool;

use crate::error::{AppError, Error};

pub const TABLE_EPISODE: &str = "episode";

pub struct Episode {
    pub id: u32,
    pub robot_id: u32,
    pub scene_id: u32,
    pub task_id: u32,
    pub episode_id: u32,
    pub result: i32, // 执行结果: ResultEnum
    pub created_at: i64,
    pub ended_at: i64,
}

pub struct CreateEpisode {
    pub robot_id: u32,
    pub scene_id: u32,
    pub task_id: u32,
    pub episode_id: u32,
}

impl Episode {
    pub(crate) async fn create(pool: &SqlitePool, data: CreateEpisode) -> anyhow::Result<i64> {
        Ok(sqlx::query(&format!(
            "INSERT INTO `{TABLE_EPISODE}` (`robot_id`, `scene_id`, `task_id`, `episode_id`, `result`, `created_at`, `ended_at`) VALUES (?, ?, ?, ?, ?, ?, ?)"
        ))
        .bind(data.robot_id)
        .bind(data.scene_id)
        .bind(data.task_id)
        .bind(data.episode_id)
        .bind(i32::from(ResultEnum::None))
        .bind(Local::now().timestamp())
        .bind(0)
        .execute(pool)
        .await
        .or(Err(Error::App(AppError::DB_ERROR)))?
        .last_insert_rowid())
    }

    pub(crate) async fn get_last_episode_id(pool: &SqlitePool, data: CreateEpisode) -> (i64, u32) {
        #[derive(sqlx::FromRow)]
        struct EpisodeId {
            id: i64,
            episode_id: i32,
        }
        let data = sqlx::query_as::<_, EpisodeId>(&format!("SELECT `id`, `episode_id` FROM `{TABLE_EPISODE}` WHERE `robot_id` = ? AND `scene_id` = ? AND `task_id` = ? ORDER BY `episode_id` DESC LIMIT 1;"))
            .bind(data.robot_id)
            .bind(data.scene_id)
            .bind(data.task_id)
            .fetch_optional(pool)
            .await.unwrap_or_default();

        match data {
            Some(data) => (data.id, data.episode_id as u32),
            None => (0, 0),
        }
    }

    pub(crate) async fn set_result(pool: &SqlitePool, result: ResultEnum, id: i64) {
        sqlx::query(&format!(
            "UPDATE `{TABLE_EPISODE}` SET `result` = ? WHERE `id` = ?"
        ))
        .bind(i32::from(result))
        .bind(id)
        .execute(pool)
        .await
        .ok();
    }

    pub(crate) async fn stop_all(pool: &SqlitePool) {
        sqlx::query(&format!(
            "UPDATE `{TABLE_EPISODE}` SET `ended_at` = ? WHERE `ended_at` = 0"
        ))
        .bind(Local::now().timestamp())
        .execute(pool)
        .await
        .ok();
    }

    pub(crate) async fn stop_id(pool: &SqlitePool, id: i64) {
        sqlx::query(&format!(
            "UPDATE `{TABLE_EPISODE}` SET `ended_at` = ? WHERE `id` = ?"
        ))
        .bind(Local::now().timestamp())
        .bind(id)
        .execute(pool)
        .await
        .ok();
    }
}

pub enum ResultEnum {
    None,      // 默认-1
    Fail,      // 失败 0
    Success,   // 成功 1
    Time(u32), // 失败多少次成功, 记录失败点
}

impl From<ResultEnum> for i32 {
    fn from(value: ResultEnum) -> Self {
        match value {
            ResultEnum::None => -1,
            ResultEnum::Fail => 0,
            ResultEnum::Success => 1,
            ResultEnum::Time(time) => time as i32,
        }
    }
}

impl From<i32> for ResultEnum {
    fn from(value: i32) -> Self {
        match value {
            ..=-1 => ResultEnum::None,
            0 => ResultEnum::Fail,
            1 => ResultEnum::Success,
            _ => ResultEnum::Time(value as u32),
        }
    }
}
