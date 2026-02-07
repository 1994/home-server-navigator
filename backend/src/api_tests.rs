use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use chrono::Utc;
use tower::ServiceExt;

use crate::{api::create_router, models::UpdateServiceRequest, state::AppState};

async fn create_state() -> AppState {
    let temp = std::env::temp_dir();
    let data_file = temp.join(format!("navigator-test-{}.json", uuid_like()));
    AppState::new(
        "localhost".to_string(),
        data_file.to_string_lossy().to_string(),
    )
    .await
    .expect("state init should succeed")
}

fn uuid_like() -> String {
    format!(
        "{}-{}",
        std::process::id(),
        Utc::now().timestamp_nanos_opt().unwrap_or(0)
    )
}

#[tokio::test]
async fn health_endpoint_works() {
    let state = create_state().await;
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .method("GET")
                .body(Body::empty())
                .expect("request should be built"),
        )
        .await
        .expect("response should succeed");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn patch_unknown_service_returns_404() {
    let state = create_state().await;
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/services/not-found")
                .method("PATCH")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&UpdateServiceRequest {
                        display_name: Some("x".to_string()),
                        description: None,
                        host: None,
                        port: None,
                        protocol: None,
                        path: None,
                        url: None,
                        status: None,
                        group: None,
                        tags: None,
                        icon: None,
                        hidden: None,
                        favorite: None,
                        locked_fields: None,
                        auto_lock: None,
                    })
                    .expect("json"),
                ))
                .expect("request should be built"),
        )
        .await
        .expect("response should succeed");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
