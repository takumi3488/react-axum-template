use anyhow::Result;
use axum::{routing::get, Router};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::debug;

fn app() -> Router {
    let static_dir = ServeDir::new("dist");
    let api_routes = Router::new().route("/health", get(health));
    Router::new()
        .nest_service("/", static_dir)
        .nest("/api", api_routes)
        .layer(TraceLayer::new_for_http())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let app = app();
    axum::serve(listener, app).await?;
    Ok(())
}

#[tracing::instrument]
async fn health() -> String {
    debug!("health check");
    "OK".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_health() {
        let server = TestServer::new(app()).unwrap();
        let response = server.get("/api/health").await;
        response.assert_status(StatusCode::OK);
        response.assert_text("OK");
    }
}
