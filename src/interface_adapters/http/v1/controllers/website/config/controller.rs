use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use crate::application::use_cases::application::config::dto::input::GetAppConfigInput;
use crate::application::use_cases::use_case::UseCase;
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::v1::presenters::website::feature_status::presenter::WebsitePresenter;

pub async fn get_app_config_ctrl(
    State(state): State<AppState>,
    Path(app_id): Path<String>,
) -> Response {
    let input = GetAppConfigInput::new(app_id);

    match state
        .website
        .config
        .get_app_config
        .execute(input)
        .await
    {
        Ok(result) => WebsitePresenter::success(result).into_response(),
        Err(error) => WebsitePresenter::error(error).into_response(),
    }
}
