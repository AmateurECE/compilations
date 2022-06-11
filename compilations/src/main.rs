///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the service.
//
// CREATED:         05/23/2022
//
// LAST EDITED:     06/11/2022
////

use std::error::Error;
use std::sync::Arc;

use axum::{
    body::{self, Empty, Full}, http::{header::{self, HeaderValue}, StatusCode},
    extract::Path, response::{IntoResponse, Response}, routing::get, Router,
};
use axum_database_sessions::{
    AxumSessionConfig, AxumSessionStore, AxumSessionLayer,
};
use clap::Parser;
use include_dir::{include_dir, Dir};
use oauth2::{
    AuthUrl, basic::BasicClient, ClientId, ClientSecret, RedirectUrl, TokenUrl,
};
use tower_http::trace::TraceLayer;
use tracing::{event, Level};

mod configuration;
mod endpoints;
mod resolver;

use configuration::{load_secret, load_configuration};
use endpoints::{login, redirect_callback};
use resolver::ResolverBuilder;

const APP_URL: &'static str = "/app";
const REDIRECT_URL: &'static str = "/callback";
const AUTH_URL: &'static str = "https://www.reddit.com/api/v1/authorize";
const TOKEN_URL: &'static str = "https://www.reddit.com/api/v1/access_token";
const USER_AGENT: &'static str =
    "edtwardy-savedapi/1.0;Ethan D. Twardy <ethan.twardy@gmail.com>";
const REDDIT_BASE: &'static str = "https://oauth.reddit.com";
pub(crate) const CSRF_TOKEN_KEY: &'static str = "csrf_token";

static FRONTEND_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/../frontend/dist");

#[derive(Parser, Debug)]
#[clap(author, version, about = None, long_about = None)]
struct Args {
    #[clap(short, long)]
    secret_file: String,

    #[clap(short, long)]
    conf_file: String,
}

async fn frontend_resource(Path(path): Path<String>) -> impl IntoResponse {
    let path = match path.trim_start_matches('/') {
        "" => "index.html",
        a => a,
    };
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match FRONTEND_DIR.get_file(path) {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let secret = load_secret(&args.secret_file).await?;
    let configuration = load_configuration(&args.conf_file).await?;

    let session_config = AxumSessionConfig::default()
        .with_table_name("volatile");
    let session_store = AxumSessionStore::new(None, session_config);

    let resolver = Arc::new(
        ResolverBuilder::default()
            .hostname(configuration.hostname)
            .script_name(configuration.script_name.clone())
            .route("redirect".to_string(), REDIRECT_URL.to_string())
            .route("app".to_string(), APP_URL.to_string())
            .build()?
    );

    let client = Arc::new(BasicClient::new(
        ClientId::new(secret.id.to_string()),
        Some(ClientSecret::new(secret.secret)),
        AuthUrl::new(AUTH_URL.to_string()).unwrap(),
        Some(TokenUrl::new(TOKEN_URL.to_string()).unwrap())
    ).set_redirect_uri(RedirectUrl::new(
        resolver.get_full("redirect").unwrap())?));

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/login", get({
            let client = client.clone();
            move |session| { login(session, client) }
        }))
        .route("/app/*path", get(frontend_resource))
        .route(REDIRECT_URL, get({
            let client = client.clone();
            let resolver = resolver.clone();
            move |params, session| {
                redirect_callback(params, session, client, resolver)
            }
        }))
        .layer(AxumSessionLayer::new(session_store))
        .layer(TraceLayer::new_for_http())
        ;

    let address = configuration.listen_address.parse().unwrap();
    if let Some(script_name) = configuration.script_name {
        let app = Router::new().nest(&script_name, app);
        axum::Server::bind(&address)
            .serve(app.into_make_service())
            .await?;
    } else {
        axum::Server::bind(&address)
            .serve(app.into_make_service())
            .await?;
    }

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
