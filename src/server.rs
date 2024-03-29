use axum::{http::StatusCode, routing::get, Json, Router};

use crate::game::{self, Player};

use self::payloads::CreatePlayer;

mod payloads;

pub fn routes() -> Router {
    Router::new().route("/player", get(create_player))
}

pub async fn server() {
    let app = routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("unable to bind the listener on local port 3000");

    axum::serve(listener, app)
        .await
        .expect("unable to start server");
}

async fn create_player(Json(payload): Json<CreatePlayer>) -> (StatusCode, Json<Player>) {
    let player = game::create_player();

    (StatusCode::CREATED, Json(player))
}
