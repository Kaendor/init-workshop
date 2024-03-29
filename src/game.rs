use serde::Serialize;

use crate::chapters;

pub fn create_player() -> Player {
    chapters();
}

#[derive(Debug, Serialize)]
pub struct Player {
    pub pseudo: String,
}
