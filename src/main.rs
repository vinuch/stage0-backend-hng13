use axum::extract::State;
use axum::http::Request;
use axum::middleware::Next;
use axum::{
    body::Body, extract::ConnectInfo, response::Json, response::Response, routing::get, Router,
};
use chrono::Utc;
use dashmap::DashMap;
use governor::clock::DefaultClock;
use governor::{Quota, RateLimiter};
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::num::NonZero;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};
use tracing_subscriber;

// Type alias for keyed rate limiter using DashMap
type Limiter = RateLimiter<String, DashMap<String, governor::state::InMemoryState>, DefaultClock>;

#[derive(Clone)]
struct AppState {
    limiter: Arc<Limiter>,
}

#[derive(Deserialize, Debug)]
struct CatFact {
    fact: String,
    length: usize,
}

async fn rate_limit(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let ip = addr.ip().to_string();

    // Check if the client exceeded the quota
    if state.limiter.check_key(&ip).is_err() {
        return Response::builder()
            .status(429)
            .body("Too many requests".into())
            .unwrap();
    }

    next.run(req).await
}

async fn log_request(req: Request<Body>, next: Next) -> axum::response::Response {
    info!("Incoming request: {} {}", req.method(), req.uri());
    let resp = next.run(req).await;
    info!("Response generated");
    resp
}

async fn get_foo() -> &'static str {
    "Foooooooo"
}

async fn get_profile() -> Json<Value> {
    let client = Client::new();

    let url = "https://catfact.ninja/fact";

    let response = client.get(url).send().await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                match res.json::<CatFact>().await {
                    Ok(cat_fact) => Json(json!({
                       "status": "success",
                       "user": {
                           "email": "vincentedeh42@gmail.com",
                           "name": "vincent edeh",
                           "stack": "🦀 Rust/Axum"
                       },
                       "timestamp": Utc::now().to_rfc3339(),
                       "fact": cat_fact.fact

                    })),
                    Err(_) => Json(json!({
                        "error": "Failed to parse Cat Facts API response.",
                        "timestamp": Utc::now().to_rfc3339(),
                    })),
                }
            } else {
                Json(json!({
                    "error": format!("Cat Facts API returned status {}", res.status()),
                    "timestamp": Utc::now().to_rfc3339(),
                }))
            }
        }

        Err(err) => {
            eprintln!("Error fetching cat fact: {}", err);
            Json(json!({
                "fact": "Cats are mysterious creatures = even when APIs fail 😼",
                "error": "Failed to reach Cat Facts API",
                "timestamp": Utc::now().to_rfc3339()
            }))
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();

    // Create keyed rate limiter: 5 requests per minute per IP
    let limiter: Arc<Limiter> = Arc::new(RateLimiter::dashmap(Quota::per_minute(
        NonZero::new(5u32).unwrap(),
    )));

    let state = AppState { limiter };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/foo", get(get_foo))
        .route("/me", get(get_profile))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            rate_limit,
        ))
        .layer(axum::middleware::from_fn(log_request))
        .with_state(state);

    // info!("Server running at http://127.0.0.1:3000");
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
