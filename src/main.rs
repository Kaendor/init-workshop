use init_workshop::server;

#[tokio::main]
async fn main() {
    let _ = server::server().await;
}
