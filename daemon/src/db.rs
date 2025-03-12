use std::{
    path::Path,
    sync::atomic::{AtomicBool, AtomicI64, AtomicU32},
};

use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};

#[derive(Debug)]
pub struct DbState {
    pub database: SqlitePool,    // db pool
    pub data_id: AtomicI64,      // the last data id, current
    pub robot_id: AtomicU32,     // robot id
    pub scene_id: AtomicU32,     // scene id
    pub task_id: AtomicU32,      // task id
    pub episode_id: AtomicU32,   // episode id
    pub show_result: AtomicBool, // 是否显示弹窗
}

pub async fn db_connect() -> anyhow::Result<SqlitePool> {
    let mut init_table = false;
    let db_url = "sqlite://db/database.db";
    // 是否存在数据库
    if !Path::new("db/database.db").exists() {
        Sqlite::create_database(db_url).await?;
        init_table = true;
    }

    // 创建数据库连接
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    if init_table {
        sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS "admin" (
                    "id"    INTEGER NOT NULL UNIQUE,
                    "username"  TEXT NOT NULL,
                    "password"  TEXT NOT NULL,
                    "nick_name" TEXT,
                    "role_id"   INTEGER,
                    "created_at"    INTEGER NOT NULL,
                    PRIMARY KEY("id" AUTOINCREMENT)
                );
                INSERT INTO "admin" VALUES (1,'admin','$2b$12$vD7Fb6aNSAREe/ivAzAQ1OQaskvbr1ficl0zIJzQiNIHDEbqi2IhG','总管理员',0,0);
                CREATE TABLE IF NOT EXISTS "robot" (
                    "id"    INTEGER NOT NULL UNIQUE,
                    "robot_id"    INTEGER NOT NULL,
                    "name"  TEXT NOT NULL,
                    "info"  TEXT NOT NULL,
                    "perception"    TEXT NOT NULL,
                    "created_at"    INTEGER NOT NULL,
                    PRIMARY KEY("id" AUTOINCREMENT)
                );
                CREATE TABLE IF NOT EXISTS "scene" (
                    "id"    INTEGER NOT NULL UNIQUE,
                    "name"  TEXT NOT NULL,
                    "info"  TEXT NOT NULL,
                    "created_at"    INTEGER NOT NULL,
                    PRIMARY KEY("id" AUTOINCREMENT)
                );
                CREATE TABLE IF NOT EXISTS "task" (
                    "id"    INTEGER NOT NULL UNIQUE,
                    "name"  TEXT NOT NULL,
                    "info"  TEXT NOT NULL,
                    "tag"   TEXT NOT NULL,
                    "created_at"    INTEGER NOT NULL,
                    PRIMARY KEY("id" AUTOINCREMENT)
                );
                CREATE TABLE IF NOT EXISTS "episode" (
                    "id"          INTEGER NOT NULL UNIQUE,
                    "robot_id"    INTEGER NOT NULL,
                    "scene_id"    INTEGER NOT NULL,
                    "task_id"     INTEGER NOT NULL,
                    "episode_id"  INTEGER NOT NULL,
                    "result"      INTEGER NOT NULL DEFAULT -1,
                    "created_at"  INTEGER NOT NULL,
                    "ended_at"    INTEGER NOT NULL DEFAULT 0,
                    PRIMARY KEY("id" AUTOINCREMENT)
                );
                CREATE INDEX "all_index" ON "episode" (
                    "robot_id"  ASC,
                    "scene_id"  ASC,
                    "task_id"   ASC,
                    "episode_id"    ASC
                );
                "#,
        )
        .execute(&pool)
        .await?;
    }

    Ok(pool)
}
