use async_trait::async_trait;
use crate::application::errors::ApplicationError;
use crate::application::use_cases::use_case::UseCase;
use crate::application::use_cases::application::config::dto::input::GetAppConfigInput;
use crate::application::use_cases::application::config::dto::result::AppConfigResult;
use crate::infrastructure::repository_impl::application::feature_status::repository::AppRepositoryImpl;

pub struct GetAppConfigService {
    _repository: AppRepositoryImpl,
}

impl GetAppConfigService {
    pub fn new(repository: AppRepositoryImpl) -> Self {
        Self { _repository: repository }
    }
}

#[async_trait]
impl UseCase for GetAppConfigService {
    type Input = GetAppConfigInput;
    type Output = AppConfigResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        // For now, return the static response as requested
        Ok(AppConfigResult {
            icon_base_url: "https://cdn.brandfetch.io/domain".to_string(),
        })
    }
}
