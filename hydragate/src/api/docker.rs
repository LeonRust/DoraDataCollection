use std::process::Command;

use axum::{Router, response::IntoResponse, routing::post};

use crate::Result;

pub fn router() -> Router {
    Router::new().route("/container", post(create_container))
}

async fn create_container() -> Result<impl IntoResponse> {
    Command::new("sudo")
        .args(["docker", "run", "--rm", "hello-world"])
        .output()
        .ok()
        .map(|output| {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        })
        .expect("Failed to execute command");

    Ok(())
}
