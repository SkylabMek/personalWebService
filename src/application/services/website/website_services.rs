use crate::infrastructure::repositories::Repositories;
use crate::application::services::website::feature_status::FeatureStatusServices;
use crate::application::services::website::config::WebsiteConfigServices;

pub struct WebsiteServices {
    pub feature_status: FeatureStatusServices,
    pub config: WebsiteConfigServices,
}

impl WebsiteServices {
    pub fn new(repos: &Repositories) -> Self {
        Self {
            feature_status: FeatureStatusServices::new(repos),
            config: WebsiteConfigServices::new(repos),
        }
    }
}
