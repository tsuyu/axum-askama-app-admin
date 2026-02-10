use axum::{
    Form,
    extract::{Extension, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use tower_sessions::Session;
use validator::Validate;

use crate::controllers::auth_controller::{
    AdminUser, AuthUser, OptionalAdminUser, OptionalAuthUser,
};
use crate::models;
use crate::repository;
use crate::state::AppState;
use crate::utils;
use crate::views::templates::{
    AdminLoginTemplate, ErrorTemplate, IndexTemplate,
};

use super::shared::{ensure_csrf_token, validate_csrf, LoginForm};

// Index handler
pub async fn index(
    OptionalAuthUser(user): OptionalAuthUser,
    Extension(session): Extension<Session>,
) -> impl IntoResponse {
    let flash_success = match session.get::<String>("flash_success").await {
        Ok(Some(msg)) => {
            let _ = session.remove::<String>("flash_success").await;
            Some(msg)
        }
        _ => None,
    };

    let template = IndexTemplate {
        title: "Welcome".to_string(),
        message: "Hello from Axum + Askama!".to_string(),
        user: user.map(|u| u.username),
        flash_success,
    };

    template
}



// Admin login page (GET)
pub async fn admin_login_page(
    OptionalAdminUser(admin_user): OptionalAdminUser,
    Extension(session): Extension<Session>,
) -> impl IntoResponse {
    if admin_user.is_some() {
        return Redirect::to("/admin/dashboard").into_response();
    }

    AdminLoginTemplate {
        error: None,
        csrf_token: ensure_csrf_token(&session).await,
    }
    .into_response()
}

// Admin login submission (POST)
pub async fn admin_login_submit(
    State(state): State<AppState>,
    Extension(session): Extension<Session>,
    Form(credentials): Form<LoginForm>,
) -> impl IntoResponse {
    tracing::debug!("Admin login attempt for user: {}", credentials.username);

    if !validate_csrf(&session, &credentials.csrf_token).await {
        tracing::warn!("Admin login failed: Invalid CSRF token");
        return AdminLoginTemplate {
            error: Some("Invalid CSRF token".to_string()),
            csrf_token: ensure_csrf_token(&session).await,
        }
        .into_response();
    }

    if credentials.validate().is_err() {
        tracing::warn!("Admin login failed: Invalid login data");
        return AdminLoginTemplate {
            error: Some("Invalid login data".to_string()),
            csrf_token: ensure_csrf_token(&session).await,
        }
        .into_response();
    }

    match repository::find_admin_by_username(&state.db, &credentials.username).await {
        Ok(Some(admin)) => {
            tracing::debug!("Admin user found: {}", admin.username);
            if utils::verify_password_hash(&admin.password_hash, &credentials.password).await {
                tracing::info!("Admin login successful: {}", admin.username);
                let admin_user = AdminUser::new(admin.id, admin.username.clone());
                if let Err(e) = admin_user.login(&session).await {
                    tracing::error!("Failed to set admin session: {:?}", e);
                    return AdminLoginTemplate {
                        error: Some("Session error. Please try again.".to_string()),
                        csrf_token: ensure_csrf_token(&session).await,
                    }
                    .into_response();
                }

                // Cycle session ID for security (prevent session fixation)
                if let Err(e) = session.cycle_id().await {
                    tracing::error!("Failed to cycle session ID: {:?}", e);
                    return AdminLoginTemplate {
                        error: Some("Session error. Please try again.".to_string()),
                        csrf_token: ensure_csrf_token(&session).await,
                    }
                    .into_response();
                }

                tracing::info!("Admin session created, redirecting to /admin/dashboard");
                Redirect::to("/admin/dashboard").into_response()
            } else {
                tracing::warn!("Admin login failed: Invalid password for {}", credentials.username);
                AdminLoginTemplate {
                    error: Some("Invalid username or password".to_string()),
                    csrf_token: ensure_csrf_token(&session).await,
                }
                .into_response()
            }
        }
        Ok(None) => {
            tracing::warn!("Admin login failed: User not found - {}", credentials.username);
            AdminLoginTemplate {
                error: Some("Invalid username or password".to_string()),
                csrf_token: ensure_csrf_token(&session).await,
            }
            .into_response()
        }
        Err(e) => {
            tracing::error!("Admin login database error: {:?}", e);
            AdminLoginTemplate {
                error: Some("Database error. Please try again.".to_string()),
                csrf_token: ensure_csrf_token(&session).await,
            }
            .into_response()
        }
    }
}

// Logout handler
pub async fn logout(Extension(session): Extension<Session>) -> impl IntoResponse {
    let _ = AuthUser::logout(&session).await;
    Redirect::to("/index")
}

// Error handler
pub async fn handle_404() -> impl IntoResponse {
    let template = ErrorTemplate {
        error_code: 404,
        error_message: "Page not found".to_string(),
    };

    (StatusCode::NOT_FOUND, template)
}
