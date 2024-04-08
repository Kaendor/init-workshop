use std::sync::{Arc, Mutex};

use poem::{
    handler,
    http::StatusCode,
    listener::TcpListener,
    middleware::AddData,
    post,
    web::{Data, Json},
    EndpointExt, IntoEndpoint, Result, Route, Server,
};

use crate::game::{self, player::Player, PlayerRepository};

use self::payloads::CreatePlayer;

mod payloads;

#[derive(Clone, Default)]
struct AppState {
    player_store: Arc<Mutex<Vec<Player>>>,
}

impl PlayerRepository for AppState {
    async fn store(&self, player: Player) {
        // Ce match oblige a traiter le cas où on est dans l'incapacité de lock le Mutex
        match self.player_store.lock() {
            Ok(mut store) => store.push(player.clone()),
            Err(_) => return,
        }

        // Cette expression permet de faire la même chose que le match ci-dessus mais de manière plus concise et sans avoir de nesting qui se met en place
        let Ok(mut store) = self.player_store.lock() else {
            return;
        };

        store.push(player);
    }
}

pub fn routes() -> impl IntoEndpoint {
    let state = AppState::default();

    Route::new()
        .at("/player", post(create_player))
        .with(AddData::new(state))
}

pub async fn server() -> Result<(), std::io::Error> {
    let app = routes();

    let listener = TcpListener::bind("0.0.0.0:3000");

    println!("Server started on 0.0.0.0:3000");
    Server::new(listener).run(app).await
}

#[handler]
async fn create_player(
    Json(payload): Json<CreatePlayer>,
    state: Data<&AppState>,
) -> Result<(StatusCode, Json<Player>)> {
    let player = game::create_player_service(payload.pseudo, *state).await;

    Ok((StatusCode::CREATED, Json(player)))
}
