use init_workshop::server;

#[tokio::main]
async fn main() {
    server::server().await;
}
