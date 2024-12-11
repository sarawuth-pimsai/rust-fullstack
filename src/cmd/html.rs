use crate::presentation;

pub struct Html {
    host: [u8; 4],
    port: u16,
}
impl Html {
    pub fn new(host: [u8; 4], port: u16) -> Self {
        Self { host, port }
    }
    pub async fn start(&self) {
        let html = presentation::Html::new(self.host, self.port);
        html.start().await;
    }
}
