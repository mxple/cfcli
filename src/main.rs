mod auth;
mod util;

#[tokio::main]
async fn main() {
    let _ = auth::login().await;
}
