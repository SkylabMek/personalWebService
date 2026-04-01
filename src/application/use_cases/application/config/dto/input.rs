pub struct GetAppConfigInput {
    pub app_id: String,
}

impl GetAppConfigInput {
    pub fn new(app_id: String) -> Self {
        Self { app_id }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.app_id.trim().is_empty() {
            return Err("app_id cannot be empty".to_string());
        }
        Ok(())
    }
}
