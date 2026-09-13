#[derive(Clone)]
pub struct AppContext {
    pub app_name: String,
}

impl AppContext {
    pub fn new(app_name: impl Into<String>) -> Self {
        Self {
            app_name: app_name.into(),
        }
    }
}
