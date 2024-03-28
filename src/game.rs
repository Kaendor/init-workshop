use serde::Serialize;

pub fn create_player() -> Player {}

#[derive(Debug, Serialize)]
pub struct Player {
    pub pseudo: String,
}
