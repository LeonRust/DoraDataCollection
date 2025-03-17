use sqlx::SqlitePool;

pub const TABLE_ADMIN: &str = "admin";

#[derive(Debug, sqlx::FromRow)]
pub struct Admin {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub nick_name: String,
    pub role_id: u32,
    pub created_at: i64,
}

impl Admin {
    pub(crate) async fn admin(pool: &SqlitePool, username: &str) -> Option<Admin> {
        sqlx::query_as::<_, Admin>(&format!(
            "SELECT * FROM `{TABLE_ADMIN}` WHERE `username` = ?"
        ))
        .bind(username)
        .fetch_optional(pool)
        .await
        .unwrap_or_default()
    }
}
