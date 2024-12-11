pub struct Html {
    host: [u8; 4],
    port: u16,
}
impl Html {
    pub fn new(host: [u8; 4], port: u16) -> Self {
        Self { host, port }
    }
    pub async fn start(&self) {
        let app =
            axum::Router::new().nest_service("/", tower_http::services::ServeDir::new("public"));
        let addr = std::net::SocketAddr::from((self.host, self.port));
        let listenner = tokio::net::TcpListener::bind(addr).await.unwrap();
        tracing::debug!("listening on {:?}:{}", self.host, self.port);
        axum::serve(listenner, app).await.unwrap();
    }
}
