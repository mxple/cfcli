mod auth;
mod utils;

#[tokio::main]
async fn main() {
    let _ = auth::login().await;
}
