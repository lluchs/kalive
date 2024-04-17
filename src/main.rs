use std::sync::Arc;

use anyhow::{bail, Context};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use chrono::Utc;
use hyper::StatusCode;
use kalive_api::trias;
use serde_derive::Deserialize;
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tower_http::validate_request::ValidateRequestHeaderLayer;

#[derive(Deserialize, Debug)]
struct Config {
    pub listen_url: String,
    pub trias_url: String,
    pub trias_ref: String,
    pub api_secret: String,
}

struct AppState {
    config: Config,
    client: reqwest::Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(err) = dotenvy::dotenv() {
        if !err.not_found() {
            bail!("dotenvy: {}", err);
        }
    }
    let config: Config = envy::from_env()?;
    let listen_url = config.listen_url.clone();
    let api_secret = config.api_secret.clone();
    let state = Arc::new(AppState {
        config,
        client: reqwest::Client::new(),
    });

    let app = Router::new()
        .route("/stops", get(get_stops))
        .route("/departures", get(get_departures))
        .layer(ValidateRequestHeaderLayer::bearer(&api_secret))
        .with_state(state);

    println!("Listening on {}", &listen_url);

    let listener = TcpListener::bind(listen_url).await.context("bind failed")?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Deserialize)]
struct StopsParams {
    pub latitude: f32,
    pub longitude: f32,
}

async fn get_stops(
    Query(params): Query<StopsParams>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, AppError> {
    let timestamp = Utc::now();
    let req_xml = trias::format_location_information_request(
        timestamp,
        &state.config.trias_ref,
        params.latitude,
        params.longitude,
    );

    let res = state
        .client
        .post(&state.config.trias_url)
        .header("Content-Type", "application/xml")
        .body(req_xml)
        .send()
        .await?;

    let res_xml = res.text().await?;
    let stops = trias::parse_location_information_response(&res_xml)?;
    Ok(Json(json!({ "stops": stops })))
}

#[derive(Deserialize)]
struct DepartureParams {
    stop: String,
}

async fn get_departures(
    Query(params): Query<DepartureParams>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, AppError> {
    let timestamp = Utc::now();
    let req_xml =
        trias::format_stop_event_request(timestamp, &state.config.trias_ref, &params.stop);

    let res = state
        .client
        .post(&state.config.trias_url)
        .header("Content-Type", "application/xml")
        .body(req_xml)
        .send()
        .await?;

    let res_xml = res.text().await?;
    let departures = trias::parse_stop_event_response(&res_xml)?;
    Ok(Json(json!({ "departures": departures })))
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        println!("error: {:#}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("{:#}", self.0) })),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
