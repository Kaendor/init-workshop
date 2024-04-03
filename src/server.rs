use std::sync::{Arc, Mutex};

use poem::{
    handler, listener::TcpListener, middleware::AddData, post, web::Json, EndpointExt, Result,
    Route, Server,
};

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

pub fn routes() -> Route {}

pub async fn server() -> Result<(), std::io::Error> {
    let state = AppState::default();

    let routes = Route::new()
        .at("/player", post(create_player))
        .with(AddData::new(state));

    let listener = TcpListener::bind("0.0.0.0:3000");

    println!("Server started on 0.0.0.0:3000");
    Server::new(listener).run(app).await
}

#[handler]
async fn create_player(Json(payload): Json<CreatePlayer>) -> Result<Json<Player>> {
    let player = game::create_player_service(payload.pseudo);

    state.store_player(player.clone());

    (StatusCode::CREATED, Json(player))
}
