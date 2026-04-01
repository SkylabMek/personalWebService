pub mod service;

use service::service::GetAppConfigService;
use crate::infrastructure::repositories::Repositories;

pub struct WebsiteConfigServices {
    pub get_app_config: GetAppConfigService,
}

impl WebsiteConfigServices {
    pub fn new(repos: &Repositories) -> Self {
        Self {
            get_app_config: GetAppConfigService::new(repos.website.repository.clone()),
        }
    }
}
