///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the service.
//
// CREATED:         05/23/2022
//
// LAST EDITED:     06/02/2022
////

use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::Query, http::StatusCode, response::Redirect, routing::get, Router,
};
use axum_database_sessions::{
    AxumSession, AxumSessionConfig, AxumSessionStore, AxumSessionLayer,
};

use oauth2::{
    AuthUrl, AuthorizationCode, basic::BasicClient, ClientId, ClientSecret,
    CsrfToken, RedirectUrl, reqwest::{async_http_client}, Scope, TokenUrl,
};

// The encrypted source file.
mod filter;

// Log the user into the application
async fn login(session: AxumSession, client: Arc<BasicClient>) -> Redirect {
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user".to_string()))
        .url();
    session.set("csrf_token", csrf_token).await;

    // Redirect the user to the auth url in order to trigger the
    // authorization process.
    Redirect::temporary(auth_url.as_str())
}

// Redirect callback invoked by the API server.
async fn redirect_callback(
    Query(params): Query<HashMap<String, String>>, session: AxumSession,
    client: Arc<BasicClient>
) -> Result<Redirect, StatusCode> {
    // Once the user has been redirected to the redirect URL, we have access to
    // the authorization code. For security reasons, we verify that the `state`
    // parameter returned by the server matches `csrf_state`.
    let state = params.get("state").unwrap();
    let csrf_token: String = session.get("csrf_token").await
        .unwrap_or(String::new());
    if state != &csrf_token {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Now you can trade it for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new("auth code".to_string()))
        .request_async(async_http_client)
        .await
        .unwrap();
    session.set("token", token_result).await;

    // Route "/app" serves the wasm frontend application.
    Ok(Redirect::temporary("/app"))
}

async fn index() -> String {
    "Hello, World!".to_string()
}

#[tokio::main]
async fn main() {
    let session_config = AxumSessionConfig::default()
        .with_table_name("volatile");
    let session_store = AxumSessionStore::new(None, session_config);

    let client = Arc::new(BasicClient::new(
        ClientId::new("client_id".to_string()),
        Some(ClientSecret::new("client_secret".to_string())),
        AuthUrl::new("http://authorize".to_string()).unwrap(),
        Some(TokenUrl::new("http://token".to_string()).unwrap())
    ).set_redirect_uri(RedirectUrl::new("http://redirect".to_string())
                       .unwrap()));

    // build our application with a single route
    let app = Router::new()
        .route("/login", get({
            let client = client.clone();
            move |session| { login(session, client) }
        }))
        .route("/callback", get({
            let client = client.clone();
            move |params, session| {
                redirect_callback(params, session, client)
            }
        }))
        .route("/", get(index))
        .layer(AxumSessionLayer::new(session_store))
        // .route("/callback", get(redirect_callback))
        ;

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

///////////////////////////////////////////////////////////////////////////////
