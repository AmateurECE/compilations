///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the service.
//
// CREATED:         05/23/2022
//
// LAST EDITED:     06/06/2022
////

use std::sync::Arc;

use axum::{routing::get, Router,};
use axum_database_sessions::{
    AxumSessionConfig, AxumSessionStore, AxumSessionLayer,
};
use clap::Parser;
use oauth2::{
    AuthUrl, basic::BasicClient, ClientId, ClientSecret, RedirectUrl, TokenUrl,
};

mod configuration;
mod endpoints;
mod resolver;

use configuration::{load_secret, load_configuration};
use endpoints::{login, redirect_callback, index};
use resolver::ResolverBuilder;

const REDIRECT_URL: &'static str = "/callback";
const AUTH_URL: &'static str = "https://www.reddit.com/api/v1/authorize";
const TOKEN_URL: &'static str = "https://www.reddit.com/api/v1/access_token";
const USER_AGENT: &'static str =
    "edtwardy-savedapi/1.0;Ethan D. Twardy <ethan.twardy@gmail.com>";
const REDDIT_BASE: &'static str = "https://oauth.reddit.com";
pub(crate) const CSRF_TOKEN_KEY: &'static str = "csrf_token";
pub(crate) const APP_ROUTE_KEY: &'static str = "app";

#[derive(Parser, Debug)]
#[clap(author, version, about = None, long_about = None)]
struct Args {
    #[clap(short, long)]
    secret_file: String,

    #[clap(short, long)]
    conf_file: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let secret = load_secret(&args.secret_file).await.unwrap();
    let configuration = load_configuration(&args.conf_file).await.unwrap();

    let session_config = AxumSessionConfig::default()
        .with_table_name("volatile");
    let session_store = AxumSessionStore::new(None, session_config);

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
