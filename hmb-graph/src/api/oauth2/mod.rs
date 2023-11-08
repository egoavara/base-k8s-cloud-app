use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{get, post};

pub fn router()->Router{
    Router::new()
        .route("/login", get(login))
        .route("/callback", get(callback))
        .route("/logout", post(logout))
}

pub async fn login() -> impl IntoResponse{

}

pub async fn logout() -> impl IntoResponse{

}

pub async fn callback() -> impl IntoResponse{

}