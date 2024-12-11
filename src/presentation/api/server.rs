use serde_json::json;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct Api {
    host: [u8; 4],
    port: u16,
}
impl Api {
    pub fn new(host: [u8; 4], port: u16) -> Self {
        Self { host, port }
    }
    pub async fn start(&self) {
        let filter_layer =
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            });
        tracing_subscriber::registry().with(filter_layer).init();
        let cors = tower_http::cors::CorsLayer::new().allow_origin(tower_http::cors::Any);
        let app = axum::Router::new()
            .fallback(|| async {
                (
                    axum::http::StatusCode::OK,
                    axum::Json(json!({"code":"message"})),
                )
            })
            .layer(cors);
        let addr = std::net::SocketAddr::from((self.host, self.port));
        let listenner = tokio::net::TcpListener::bind(addr).await.unwrap();
        tracing::debug!("listening on {:?}:{}", self.host, self.port);
        axum::serve(listenner, app).await.unwrap();
    }
}
