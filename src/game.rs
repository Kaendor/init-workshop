use self::player::Player;

pub mod player;

pub trait PlayerRepository {
    fn store(&self, player: Player) -> impl std::future::Future<Output = ()> + Send;
}

pub async fn create_player_service<R: PlayerRepository>(
    pseudo: String,
    player_repository: &R,
) -> Player {
    let player = Player::new(pseudo);

    player_repository.store(player.clone()).await;

    player
}
