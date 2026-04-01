pub mod dto;

use std::sync::Arc;
use crate::application::services::website::config::WebsiteConfigServices;

#[derive(Clone)]
pub struct AppConfigUseCases {
    pub get_app_config: Arc<crate::application::services::website::config::service::service::GetAppConfigService>,
}

impl AppConfigUseCases {
    pub fn new(services: WebsiteConfigServices) -> Self {
        Self {
            get_app_config: Arc::new(services.get_app_config),
        }
    }
}
