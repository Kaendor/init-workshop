use std::sync::{Arc, Mutex};

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};

use crate::game::{self, Player};

use self::payloads::CreatePlayer;

mod payloads;

#[derive(Clone, Default)]
struct AppState {
    player_store: Arc<Mutex<Vec<Player>>>,
}

impl AppState {
    pub fn store_player(&self, player: Player) {
        let Ok(mut store) = self.player_store.lock() else {
            return;
        };

        store.push(player);
    }
}

pub fn routes() -> Router {
    Router::new()
        .route("/player", post(create_player))
        .with_state(AppState::default())
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

async fn create_player(
    Json(payload): Json<CreatePlayer>,
    state: State<AppState>,
) -> (StatusCode, Json<Player>) {
    let player = game::create_player(payload.pseudo);

    state.store_player(player.clone());

    (StatusCode::CREATED, Json(player))
}
