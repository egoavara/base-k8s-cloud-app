use axum::extract::Query;
use axum::response::{IntoResponse, Redirect};
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueryParam{
    pub login_challenge: String
}

pub async fn handler(Query(query): Query<QueryParam>) -> impl IntoResponse {
    let mut url = Url::parse("https://auth.egoavara.net/self-service/login/browser").unwrap();
    url.query_pairs_mut()
        .append_pair("login_challenge", query.login_challenge.as_str());
    Redirect::temporary(url.to_string().as_str())
}