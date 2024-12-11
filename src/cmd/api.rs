use crate::presentation;

pub struct Api {
    host: [u8; 4],
    port: u16,
}

impl Api {
    pub fn new(host: [u8; 4], port: u16) -> Self {
        Self { host, port }
    }
    pub async fn start(&self) {
        let api = presentation::Api::new(self.host, self.port);
        api.start().await;
    }
}
