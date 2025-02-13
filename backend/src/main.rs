use axum::{routing::get, Router};
use dotenv::dotenv;
use postgrest::Postgrest;
use routes::{
    contact_routes::get_contacts, profile_routes::get_profile, project_routes::get_projects,
    work_experience_routes::get_work_experience,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let db = Arc::new(Postgrest::new(&db_url));

    let app = Router::new()
        .route("/profile", get(get_profile))
        .route("/work-experience", get(get_work_experience))
        .route("/projects", get(get_projects))
        .route("/contacts", get(get_contacts))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    let port = listener.local_addr().unwrap().port();
    println!("Server running on http://localhost:{}", port);

    axum::serve(listener, app).await.unwrap();
}
