use crate::models::user::User;
use axum::{extract::State, Json};
use postgrest::Postgrest;
use std::sync::Arc;

pub async fn get_profile(State(db): State<Arc<Postgrest>>) -> Result<Json<User>, String> {
    let response = db
        .from("users")
        .select("fullname, description, about")
        .limit(1)
        .execute()
        .await
        .map_err(|_| "Failed to fetch user data")?;

    let body_text = response
        .text()
        .await
        .map_err(|_| "Failed to read response body")?;

    let users: Vec<User> =
        serde_json::from_str(&body_text).map_err(|_| "Failed to parse JSON response")?;

    if let Some(user) = users.into_iter().next() {
        Ok(Json(user))
    } else {
        Err("No user found".to_string())
    }
}
