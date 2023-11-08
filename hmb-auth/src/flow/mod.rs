use axum::Router;
use axum::routing::get;

mod login_start;
mod login_ui;
mod login_complete;
mod consent;

pub fn router() -> Router{
    Router::new()
        .route("/login-start", get(login_start::handler))
        .route("/login-ui", get(login_ui::handler))
        .route("/login-complete", get(login_complete::handler))
        .route("/consent", get(consent::handler))
}
