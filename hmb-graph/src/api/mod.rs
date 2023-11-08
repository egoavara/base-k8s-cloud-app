mod admin;
mod oauth2;
mod search;

use std::thread::Scope;
use async_graphql::ErrorExtensions;
use axum::{Router};
use axum::handler::Handler;
use axum::response::IntoResponse;
use openidconnect::{AuthUrl, ClientId, ClientSecret, CsrfToken, IssuerUrl, JsonWebKeySet, JsonWebKeySetUrl, Nonce, PkceCodeChallenge, RedirectUrl};
use openidconnect::core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata};
use openidconnect::reqwest::http_client;


async fn unauth()->impl IntoResponse{
    return "unauthorized"
}

pub fn router() -> Router{
    Router::new()
        .nest("/admin", admin::router())
        .nest("/oauth2", oauth2::router())
        .nest("/search", search::router())

}