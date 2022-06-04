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

mod resolver;
use resolver::{Resolver, ResolverBuilder};

const REDIRECT_URL: &'static str = "/callback";
const AUTH_URL: &'static str = "https://www.reddit.com/api/v1/authorize";
const TOKEN_URL: &'static str = "https://www.reddit.com/api/v1/access_token";
const USER_AGENT: &'static str =
    "edtwardy-savedapi/1.0;Ethan D. Twardy <ethan.twardy@gmail.com>";
const REDDIT_BASE: &'static str = "https://oauth.reddit.com";
const CSRF_TOKEN_KEY: &'static str = "csrf_token";
const APP_ROUTE_KEY: &'static str = "app";

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
    pub listen_address: String,
    pub script_name: Option<String>,
    pub hostname: String,
}

#[derive(Serialize, Deserialize)]
struct Secret {
    pub id: String,
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
    client: Arc<BasicClient>, resolver: Arc<Resolver>
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
    Ok(Redirect::temporary(&resolver.get(APP_ROUTE_KEY).unwrap()))
}

async fn index() -> String {
    "Hello, World!".to_string()
}

// Load the client secret from the filesystem.
async fn load_secret() -> Secret {
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

    let secret = load_secret().await;
    let configuration = load_configuration().await;
    let resolver = Arc::new(
        ResolverBuilder::default()
            .hostname(configuration.hostname)
            .script_name(configuration.script_name.clone())
            .route(APP_ROUTE_KEY.to_string(), REDIRECT_URL.to_string())
            .build()
            .unwrap()
    );

    let client = Arc::new(BasicClient::new(
        ClientId::new(secret.id.to_string()),
        Some(ClientSecret::new(secret.secret)),
        AuthUrl::new(AUTH_URL.to_string()).unwrap(),
        Some(TokenUrl::new(TOKEN_URL.to_string()).unwrap())
    ).set_redirect_uri(RedirectUrl::new(
        resolver.get(APP_ROUTE_KEY).unwrap()).unwrap())
    );

    let app = Router::new()
        .route("/login", get({
            let client = client.clone();
            move |session| { login(session, client) }
        }))
        .route("/callback", get({
            let client = client.clone();
            let resolver = resolver.clone();
            move |params, session| {
                redirect_callback(params, session, client, resolver)
            }
        }))
        .route("/app", get(index))
        .layer(AxumSessionLayer::new(session_store))
        ;

    let address = configuration.listen_address.parse().unwrap();
    if let Some(script_name) = configuration.script_name {
        let app = Router::new().nest(&script_name, app);
        axum::Server::bind(&address)
            .serve(app.into_make_service())
            .await
            .unwrap();
    } else {
        axum::Server::bind(&address)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

///////////////////////////////////////////////////////////////////////////////
