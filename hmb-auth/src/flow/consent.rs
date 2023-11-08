use axum::extract::Query;
use axum::response::{IntoResponse, Redirect};
use axum::headers::{HeaderMap};
use axum::http::header::COOKIE;
use ory_client::apis::configuration::Configuration;
use ory_client::apis::o_auth2_api::{accept_o_auth2_consent_request, get_o_auth2_consent_request};
use ory_client::apis::frontend_api::to_session;
use ory_client::models::{AcceptOAuth2ConsentRequest, AcceptOAuth2ConsentRequestSession};
use serde::{Deserialize, Serialize};
use tokio::join;

#[derive(Serialize, Deserialize)]
pub struct QueryParam{
    pub consent_challenge: String
}

#[derive(Serialize, Deserialize)]
pub struct IdToken{
    email:String
}

pub async fn handler(header: HeaderMap, query:Query<QueryParam>) -> impl IntoResponse {
    let hydra_config = Configuration{
        base_path: "http://admin.hydra.ory.egoavara.net".to_owned(),
        ..Default::default()
    };
    let kratos_config = Configuration{
        base_path: "https://public.kratos.ory.egoavara.net".to_owned(),
        ..Default::default()
    };
    let cookie = header.get(COOKIE).unwrap().to_str().unwrap();
    let consent_challenge = query.consent_challenge.as_str();
    let (req_consent, req_session ) = join!(
        get_o_auth2_consent_request(&hydra_config, consent_challenge),
        to_session(&kratos_config, None, Some(cookie), None)
    );
    let consent = req_consent.unwrap();
    let session = req_session.unwrap();
    //
    let data = AcceptOAuth2ConsentRequest{
        grant_scope:consent.requested_scope.clone(),
        remember:Some(true),
        remember_for:Some(3600),
        session:Some(Box::new(AcceptOAuth2ConsentRequestSession{
            id_token: session.identity.clone().unwrap().traits,
            access_token:None,
        })),
        ..Default::default()
    };

    let accept = accept_o_auth2_consent_request(&hydra_config, consent_challenge, Some(&data)).await.unwrap();

    (Redirect::temporary(accept.redirect_to.as_str()))
}