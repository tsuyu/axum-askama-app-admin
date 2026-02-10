use askama::Template;

use super::User;

// Index page template
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub title: String,
    pub message: String,
    pub user: Option<String>,
    pub flash_success: Option<String>,
}

// Error page template
#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub error_code: u16,
    pub error_message: String,
}
