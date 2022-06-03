///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the service.
//
// CREATED:         05/23/2022
//
// LAST EDITED:     06/03/2022
////

use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::Query, http::StatusCode, response::Redirect, routing::get, Router,
};
use axum_database_sessions::{
    AxumSession, AxumSessionConfig, AxumSessionStore, AxumSessionLayer,
};

use clap::Parser;

use oauth2::{
    AuthUrl, AuthorizationCode, basic::BasicClient, ClientId, ClientSecret,
    CsrfToken, RedirectUrl, reqwest::{async_http_client}, Scope, TokenUrl,
};

use serde::{Serialize, Deserialize};

const CLIENT_ID: &'static str = "";
const AUTH_URL: &'static str = "";
const TOKEN_URL: &'static str = "";
const REDIRECT_URL: &'static str = "";
const CSRF_TOKEN_KEY: &'static str = "csrf_token";

#[derive(Parser, Debug)]
#[clap(author, version, about = None, long_about = None)]
struct Args {
    #[clap(short, long)]
    secret_file: String,

    #[clap(short, long)]
    conf_file: String,
}

#[derive(Serialize, Deserialize)]
struct Configuration {
    pub address: String,
}

#[derive(Serialize, Deserialize)]
struct Secret {
    pub secret: String,
}

// Log the user into the application
async fn login(session: AxumSession, client: Arc<BasicClient>) -> Redirect {
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user".to_string()))
        .url();
    session.set(CSRF_TOKEN_KEY, csrf_token).await;

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
    let csrf_token: String = session.get(CSRF_TOKEN_KEY).await
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

// Load the client secret from the filesystem.
async fn load_secret() -> String {
    todo!()
}

async fn load_configuration() -> Configuration {
    todo!()
}

#[tokio::main]
async fn main() {
    let session_config = AxumSessionConfig::default()
        .with_table_name("volatile");
    let session_store = AxumSessionStore::new(None, session_config);

    let client_secret = load_secret().await;
    let configuration = load_configuration().await;
    let client = Arc::new(BasicClient::new(
        ClientId::new(CLIENT_ID.to_string()),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(AUTH_URL.to_string()).unwrap(),
        Some(TokenUrl::new(TOKEN_URL.to_string()).unwrap())
    ).set_redirect_uri(RedirectUrl::new(REDIRECT_URL.to_string()).unwrap()));

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
        .route("/app", get(index))
        .layer(AxumSessionLayer::new(session_store))
        ;

    // Serve it
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

///////////////////////////////////////////////////////////////////////////////
