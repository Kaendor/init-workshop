use serde::{Deserialize, Serialize};

pub fn create_player_service(pseudo: String) -> Player {
    Player { pseudo, level: 0 }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub pseudo: String,
    pub level: u32,
    // Ajouter une classe en utilisant une enum
}
