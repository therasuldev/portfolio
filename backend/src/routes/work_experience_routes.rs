use crate::models::work_experience::WorkExperience;
use axum::{extract::State, Json};
use postgrest::Postgrest;
use std::sync::Arc;

pub async fn get_work_experience(
    State(db): State<Arc<Postgrest>>,
) -> Result<Json<Vec<WorkExperience>>, String> {
    let response = db
        .from("work_experience")
        .select("id, company, position, period, description")
        .order("id.desc")
        .execute()
        .await
        .map_err(|_| "Failed to fetch work experience data")?;

    let body_text = response
        .text()
        .await
        .map_err(|_| "Failed to read response body")?;

    let experiences: Vec<WorkExperience> =
        serde_json::from_str(&body_text).map_err(|_| "Failed to parse JSON response")?;

    Ok(Json(experiences))
}
