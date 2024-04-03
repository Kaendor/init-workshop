use self::player::Player;

pub mod player;

pub trait PlayerRepository {
    async fn store(&self, player: Player);
}

pub async fn create_player_service<R: PlayerRepository>(
    pseudo: String,
    player_repository: &R,
) -> Player {
    let player = Player::new(pseudo);

    player_repository.store(player.clone()).await;

    player
}
