use axum::{Server};

use std::net::SocketAddr;
use openidconnect::core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata};
use openidconnect::{ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, PkceCodeChallenge, RedirectUrl, Scope};
use openidconnect::reqwest::{async_http_client, http_client};

mod api;
mod middleware;

#[tokio::main]
async fn main() {
    // // initialize tracing
    // tracing_subscriber::fmt::init();
    //
    // let socket:SocketAddr = "0.0.0.0:3000".parse().unwrap();
    // Server::bind(&socket)
    //     .serve(api::router().into_make_service())
    //     .await
    //     .unwrap();

    // {"client_id":"564da403-de33-4930-b949-c85e85a13f41","client_name":"hmb-app-web","client_secret":"7C4G3btaqGce-_vvbJCX1MDTVh"}

    let core =
        CoreProviderMetadata::discover_async(
        IssuerUrl::new("http://public.hydra.ory.egoavara.net/".to_string()).unwrap(),
        async_http_client
    )

        .await
        .unwrap();
    let core_client =
        CoreClient::from_provider_metadata(
        core,
        ClientId::new("564da403-de33-4930-b949-c85e85a13f41".to_string()),
        Some(ClientSecret::new("7C4G3btaqGce-_vvbJCX1MDTVh".to_string())),
    )
        .set_redirect_uri(RedirectUrl::new("http://www.egoavara.net:3000/oauth2/callback".to_string()).unwrap());


// Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token, nonce) = core_client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        // Set the desired scopes.
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();
    println!("Browse to: {}", auth_url);
}