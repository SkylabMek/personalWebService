use axum::response::IntoResponse;
use serde::Serialize;
use crate::interface_adapters::http::v1::presenters::common::api_response::ApiResponse;
use crate::interface_adapters::http::v1::presenters::common::presenter_output::PresenterOutput;

#[derive(Debug, Clone, Serialize)]
pub struct AppConfigResult {
    pub icon_base_url: String,
}

impl PresenterOutput for AppConfigResult {
    fn into_response(self) -> impl IntoResponse {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
    }
}
