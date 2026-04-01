pub mod feature_status;
pub mod config;

use crate::application::services::website::website_services::WebsiteServices;
use crate::application::use_cases::application::feature_status::FeatureStatusUseCases;
use crate::application::use_cases::application::config::AppConfigUseCases;

#[derive(Clone)]
pub struct WebsiteUseCases {
    pub feature_status: FeatureStatusUseCases,
    pub config: AppConfigUseCases,
}

impl WebsiteUseCases {
    pub fn new(services: WebsiteServices) -> Self {
        Self {
            feature_status: FeatureStatusUseCases::new(services.feature_status),
            config: AppConfigUseCases::new(services.config),
        }
    }
}
