mod config;
mod db;
mod scrapers;
mod service;

use std::sync::Arc;

use axum::{extract::State, response::Html, routing::{get, post}, Form, Json, Router};
use config::AppConfig;
use db::OfferRepository;
use dotenvy::dotenv;
use price_check_shared::SearchRequest;
use serde::Deserialize;
use service::SearchService;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    search: Arc<SearchService>,
}

#[derive(Deserialize)]
struct SearchForm {
    query: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = AppConfig::from_env();
    let repo = OfferRepository::connect(&cfg.mongo_uri, &cfg.mongo_db, &cfg.mongo_collection).await?;
    repo.ensure_indexes().await?;

    let http = scrapers::build_client()?;
    let search = Arc::new(SearchService::new(repo, http));

    let app = Router::new()
        .route("/", get(index))
        .route("/search", post(search_form))
        .route("/api/search", post(search_json))
        .layer(CorsLayer::permissive())
        .with_state(AppState { search });

    let listener = tokio::net::TcpListener::bind(&cfg.bind_addr).await?;
    tracing::info!("backend listening on {}", cfg.bind_addr);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn index() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
<html>
  <head>
    <meta charset='utf-8'/>
    <meta name='viewport' content='width=device-width, initial-scale=1'/>
    <title>PriceChecker Rust</title>
  </head>
  <body>
    <h1>PriceChecker (Rust backend)</h1>
    <form action='/search' method='post'>
      <input type='text' name='query' placeholder='apple watch 10' />
      <button type='submit'>Search</button>
    </form>
  </body>
</html>"#,
    )
}

async fn search_form(State(state): State<AppState>, Form(form): Form<SearchForm>) -> Html<String> {
    match state.search.search(&form.query).await {
        Ok(result) => {
            let mut rows = String::new();
            for o in result.offers {
                rows.push_str(&format!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td><td><a href='{}' target='_blank'>Open</a></td><td>{}</td></tr>",
                    html_escape(&o.product_title),
                    html_escape(&o.shop_name),
                    html_escape(&o.price),
                    html_escape(&o.link),
                    html_escape(&o.timestamp),
                ));
            }

            Html(format!(
                "<!doctype html><html><body><h2>Query: {}</h2><p>Source: {}</p><table border='1'><tr><th>Product Title</th><th>Shop Name</th><th>Price</th><th>Link</th><th>Timestamp</th></tr>{}</table><p><a href='/'>Back</a></p></body></html>",
                html_escape(&result.query),
                if result.from_cache { "DB cache" } else { "Fresh scrape" },
                rows
            ))
        }
        Err(err) => Html(format!("<p>Search failed: {}</p><p><a href='/'>Back</a></p>", html_escape(&err.to_string()))),
    }
}

async fn search_json(State(state): State<AppState>, Json(req): Json<SearchRequest>) -> Json<serde_json::Value> {
    match state.search.search(&req.query).await {
        Ok(result) => Json(serde_json::json!({ "ok": true, "data": result })),
        Err(err) => Json(serde_json::json!({ "ok": false, "error": err.to_string() })),
    }
}

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}
