use axum::extract::Query;
use axum::headers::{Header, Referer, UserAgent};
use axum::{Json, TypedHeader};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::CookieJar;
use openidconnect::{AuthorizationCode, AuthType, ClientId, ClientSecret, IssuerUrl, OAuth2TokenResponse, PkceCodeVerifier, RedirectUrl};
use openidconnect::core::{CoreClient, CoreProviderMetadata};
use openidconnect::reqwest::async_http_client;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueryParam{
    pub code: String,
    pub scope: String,
    pub state: String,
}

pub async fn handler(Query(query): Query<QueryParam>, jar: CookieJar, headers: HeaderMap) -> impl IntoResponse {
    let client_id = ClientId::new("65b8310b-5c4f-4813-8c77-a9c59c2d9287".to_string());
    let client_secret = ClientSecret::new("secret".to_string());
    let issuer_url = IssuerUrl::new("https://public.hydra.ory.egoavara.net".to_string()).expect("Invalid issuer URL");
    let provider_metadata = CoreProviderMetadata::discover_async(issuer_url, async_http_client)
        .await
        .unwrap_or_else(|x|{
            panic!("{:?}", x.to_string())
        });
    let referer = headers.get("Referer").unwrap().to_str().unwrap();
    let mut referer_url = Url::parse(referer).unwrap();
    referer_url.query_pairs_mut().clear();
    let redirect_url = referer_url.to_string().replace("?", "");

    // Set up the config for the GitLab OAuth2 process.
    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        client_id,
        Some(client_secret),
    )
        .set_auth_type(AuthType::RequestBody)
        .set_redirect_uri(
            RedirectUrl::new(redirect_url).expect("Invalid redirect URL"),
        );
    let pkce_verifier = PkceCodeVerifier::new(jar.get("pkce_verifier").unwrap().value().to_owned());
    let token = client.exchange_code(AuthorizationCode::new(query.code.clone()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .unwrap();
    Json(token)
}