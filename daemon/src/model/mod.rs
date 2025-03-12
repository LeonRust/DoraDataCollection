pub mod admin;
pub mod episode;
pub mod robot;
pub mod scene;
pub mod task;

#[derive(Debug, sqlx::FromRow, Default)]
pub struct Count {
    pub count: i64,
}
