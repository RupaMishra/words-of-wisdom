mod service;

#[tokio::main]
async fn main() {
    service::get_quotes().await;
}
