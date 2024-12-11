use fullstack::cmd;

#[tokio::main]
async fn main() {
    let api = cmd::Api::new([127, 0, 0, 1], 3000);
    let html = cmd::Html::new([127, 0, 0, 1], 8080);
    let frontend = html.start();
    let backend = api.start();
    tokio::join!(frontend, backend);
    println!("Hello, world!");
}
