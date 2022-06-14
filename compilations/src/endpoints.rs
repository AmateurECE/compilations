///////////////////////////////////////////////////////////////////////////////
// NAME:            endpoints.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Service endpoints.
//
// CREATED:         06/03/2022
//
// LAST EDITED:     06/14/2022
////

use std::collections::HashMap;
use std::sync::Arc;

use axum::{extract::Query, http::StatusCode, response::Redirect};
use axum_database_sessions::AxumSession;
use oauth2::{
    AccessToken, AuthorizationCode, basic::BasicClient, CsrfToken,
    reqwest::{async_http_client}, Scope, TokenResponse,
};
use reqwest_middleware::ClientWithMiddleware;
use tracing::{event, Level};

use crate::resolver::Resolver;
use crate::{CSRF_TOKEN_KEY, REDDIT_BASE, USER_AGENT};

// Log the user into the application
pub async fn login(session: AxumSession, client: Arc<BasicClient>) ->
    Redirect
{
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("history".to_string()))
        .add_scope(Scope::new("identity".to_string()))
        .add_scope(Scope::new("save".to_string()))
        .url();
    session.set(CSRF_TOKEN_KEY, csrf_token).await;

    // Redirect the user to the auth url in order to trigger the
    // authorization process.
    Redirect::temporary(auth_url.as_str())
}

// Redirect callback invoked by the API server.
pub async fn redirect_callback(
    Query(params): Query<HashMap<String, String>>, session: AxumSession,
    client: Arc<BasicClient>, resolver: Arc<Resolver>
) -> Result<Redirect, StatusCode> {
    // Once the user has been redirected to the redirect URL, we have access to
    // the authorization code. For security reasons, we verify that the `state`
    // parameter returned by the server matches `csrf_state`.
    // TODO: Check for "error" query parameter here.
    let state = params.get("state").unwrap();
    let csrf_token: String = session.get(CSRF_TOKEN_KEY).await
        .unwrap_or(String::new());
    if state != &csrf_token {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Now you can trade it for an access token.
    let code = params.get("code").unwrap().to_string();
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await
        .unwrap();
    session.set("token", token_result.access_token().clone()).await;

    // Route "/app" serves the wasm frontend application.
    Ok(Redirect::temporary(&resolver.get("app").unwrap()))
}

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

pub async fn api_v1_me(reddit_endpoint: &'static str, session: AxumSession) ->
    Result<String, StatusCode>
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
