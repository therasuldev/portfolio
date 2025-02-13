use crate::models::project::Project;
use axum::{extract::State, Json};
use postgrest::Postgrest;
use serde_json::Value;
use std::sync::Arc;

pub async fn get_projects(State(db): State<Arc<Postgrest>>) -> Json<Vec<Project>> {
    let response = db
        .from("projects")
        .select("*")
        .execute()
        .await
        .expect("Failed to fetch projects");

    let body: Value = serde_json::from_str(&response.text().await.expect("Invalid response text"))
        .expect("Invalid JSON response");

    let projects: Vec<Project> = serde_json::from_value(body).expect("Failed to parse projects");

    Json(projects)
}
