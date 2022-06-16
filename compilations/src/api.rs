///////////////////////////////////////////////////////////////////////////////
// NAME:            api.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Easy methods for proxying Reddit API endpoints.
//
// CREATED:         06/16/2022
//
// LAST EDITED:     06/16/2022
////

use axum_database_sessions::AxumSession;
use axum::http::StatusCode;
use oauth2::AccessToken;
use reqwest_middleware::ClientWithMiddleware;
use tracing::{event, Level};
use crate::REDDIT_BASE;
use crate::USER_AGENT;

async fn get_user_client(session: &AxumSession) ->
    Result<ClientWithMiddleware, StatusCode>
{
    // Initialize a reqwest client for this session
    let token: AccessToken = session.get("token").await
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut headers = reqwest::header::HeaderMap::new();
    let auth = "bearer ".to_string() + token.secret().as_str();
    let mut auth = reqwest::header::HeaderValue::from_str(&auth).map_err(|e| {
        event!(Level::ERROR, "{:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    auth.set_sensitive(true);
    headers.insert(reqwest::header::AUTHORIZATION, auth);

    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .default_headers(headers)
        .build()
        .map_err(|e| {
            event!(Level::ERROR, "{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let client = reqwest_middleware::ClientBuilder::new(client)
        .with(reqwest_tracing::TracingMiddleware)
        .build();
    Ok(client)
}

///////////////////////////////////////////////////////////////////////////////
// Public API
////

pub async fn proxy_simple(
    reddit_endpoint: &'static str, session: AxumSession
) -> Result<String, StatusCode>
{
    let client = get_user_client(&session).await?;
    let response = client.get(REDDIT_BASE.to_string() + reddit_endpoint)
        .send()
        .await
        .map_err(|e| {
            event!(Level::ERROR, "{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(response.text().await.map_err(|e| {
        event!(Level::ERROR, "{:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?)
}

///////////////////////////////////////////////////////////////////////////////
