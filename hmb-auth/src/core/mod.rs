use openidconnect::{ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, PkceCodeChallenge, RedirectUrl, Scope};
use openidconnect::core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata};
use openidconnect::reqwest::async_http_client;

pub async fn test(){
    let gitlab_client_id = ClientId::new("65b8310b-5c4f-4813-8c77-a9c59c2d9287".to_string());
    let gitlab_client_secret = ClientSecret::new("secret".to_string());
    let issuer_url = IssuerUrl::new("https://public.hydra.ory.egoavara.net".to_string()).expect("Invalid issuer URL");
    let provider_metadata = CoreProviderMetadata::discover_async(issuer_url, async_http_client)
        .await
        .unwrap_or_else(|x|{
            panic!("{:?}", x.to_string())
        });
    // Set up the config for the GitLab OAuth2 process.
    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        gitlab_client_id,
        Some(gitlab_client_secret),
    )
        // This example will be running its own server at localhost:8080.
        // See below for the server implementation.
        .set_redirect_uri(
            RedirectUrl::new("http://auth.egoavara.com".to_string()).expect("Invalid redirect URL"),
        );// Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token, nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        // Set the desired scopes.
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("offline".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();


    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    println!("Browse to: {}", auth_url);
}