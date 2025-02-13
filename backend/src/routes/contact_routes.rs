use crate::models::contact::Contact;
use axum::{extract::State, Json};
use postgrest::Postgrest;
use serde_json::Value;
use std::sync::Arc;

pub async fn get_contacts(State(db): State<Arc<Postgrest>>) -> Json<Vec<Contact>> {
    let response = db
        .from("contacts")
        .select("*")
        .execute()
        .await
        .expect("Failed to fetch contacts");

    let body: Value = serde_json::from_str(&response.text().await.expect("Invalid response text"))
        .expect("Invalid JSON response");

    let contacts: Vec<Contact> = serde_json::from_value(body).expect("Failed to parse contacts");

    Json(contacts)
}
