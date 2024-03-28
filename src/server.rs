use axum::Router;

pub async fn server() {
    let app = Router::new();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("unable to bind the listener on local port 3000");
    axum::serve(listener, app)
        .await
        .expect("unable to start server");
}
