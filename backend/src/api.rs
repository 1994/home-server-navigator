use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::Serialize;

use crate::{
    models::{
        CreateServiceRequest, DiscoveryRunResponse, DiscoveryStatusInfo, ServiceEntry,
        ServiceQuery, UpdateServiceRequest,
    },
    state::AppState,
};

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: &'static str,
    service_count: usize,
    now: chrono::DateTime<Utc>,
    version: &'static str,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/services", get(list_services).post(create_service))
        .route("/api/services/{id}", get(get_service).patch(update_service))
        .route("/api/discovery/run", post(run_discovery))
        .route("/api/discovery/status", get(get_discovery_status))
        .with_state(state)
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    let service_count = state.services.read().await.len();
    Json(HealthResponse {
        status: "ok",
        service_count,
        now: Utc::now(),
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn list_services(
    State(state): State<AppState>,
    Query(query): Query<ServiceQuery>,
) -> Json<Vec<ServiceEntry>> {
    Json(state.list_services(query).await)
}

async fn get_service(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ServiceEntry>, StatusCode> {
    match state.get_service(&id).await {
        Some(service) => Ok(Json(service)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_service(
    State(state): State<AppState>,
    Json(request): Json<CreateServiceRequest>,
) -> Result<Json<ServiceEntry>, ApiError> {
    if request.service_name.trim().is_empty() {
        return Err(ApiError {
            message: "service_name is required".to_string(),
        });
    }

    state
        .create_service(request)
        .await
        .map(Json)
        .map_err(|error| ApiError {
            message: format!("failed to create service: {error}"),
        })
}

async fn update_service(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<UpdateServiceRequest>,
) -> Result<Json<ServiceEntry>, StatusCode> {
    match state.update_service(&id, request).await {
        Ok(Some(service)) => Ok(Json(service)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn run_discovery(
    State(state): State<AppState>,
) -> Result<Json<DiscoveryRunResponse>, ApiError> {
    state
        .run_discovery()
        .await
        .map(|summary| Json(DiscoveryRunResponse { summary }))
        .map_err(|error| ApiError {
            message: format!("failed to run discovery: {error}"),
        })
}

async fn get_discovery_status(State(state): State<AppState>) -> Json<DiscoveryStatusInfo> {
    Json(state.discovery_status().await)
}
