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
    // Ici on a une approche fonctionnelle pour ceux qui aiment.
    let players: Result<Vec<Player>, String> = state
        .player_store
        // Lock du mutex
        .try_lock()
        // On créer un Iterateur à partir du Result retourné par le try_lock()
        .iter()
        // Si on a réussi a lock le mutex, alors on récupère tous les joueurs à l'intérieur du
        // store en utilisant iter() sur le store sans oublier de flat_map pour ne pas itérer sur
        // des tableaux de tableaux
        .flat_map(|s| s.iter())
        // Clone permet de cloner la valeur à l'intérieur de l'itérateur et non l'itérateur.
        // (permet d'itérer sur T au lieu de &T, au coût d'un clone)
        .cloned()
        // try_level_up retourne un Result<Player, String> contenant le joueur ou l'erreur suite a
        // un level up
        .map(|p| p.try_level_up())
        // Le collect() permet de transformer l'iterator qui est lazy (qui ne se lance pas tant
        // qu'on ne le collect ou qu'on itère pas dessus avec un for) en un type différent qui peut
        // être créé à partir d'un iterator, ici un Result<Vec<Player>, String>.
        //
        // Cette opération indique qu'on a réussi a faire monter en niveau tous les joueurs sans
        // erreur OU qu'on a au moins un joueur en erreur.
        //
        // Il est possible de récupérer tous les joueurs qui ont réussi ainsi qu'un tableau
        // d'erreur mais cela va au de ce workshop. (Je laisse votre google-fu s'exercer si le
        // coeur vous en dit. "vec of result into result of vec")
        .collect();

    let player = game::create_player_service(payload.pseudo, *state).await;

    Ok((StatusCode::CREATED, Json(player)))
}
