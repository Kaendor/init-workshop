use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub pseudo: String,
    pub level: u32,
    class: PlayerClass,
}

impl Player {
    pub fn new(pseudo: String) -> Self {
        // Ici sans le mut, on ne pourrait pas incrémenter level
        let mut level = 0;

        // Le for en Rust prend un iterateur, ici on utilise un range
        // Le _ seul ou devant un nom de variable indique qu'on ne l'utilise pas
        for _ in 0..pseudo.len() {
            level += 1;
        }

        let class_id = pseudo.len() % 13;
        let class = PlayerClass::from(class_id);

        Self {
            pseudo,
            level,
            class,
        }
    }

    /// Cette fonction permet de montrer l'usage des enums via les match
    /// En plus, ce commentaire sera compilé en documentation fonctionnelle via cargo doc --open
    pub fn fight(&self, other: &Player) -> Option<()> {
        // Ici on créer un tuple (PlayerClass,PlayerClass) afin de determiner le gagnant
        match (self.class, other.class) {
            (PlayerClass::Mage, PlayerClass::Mage) => None,
            (PlayerClass::Mage, PlayerClass::Archer) => Some(()),
            (PlayerClass::Mage, PlayerClass::Priest) => Some(()),
            (PlayerClass::Mage, PlayerClass::DeathKnight) => None,
            (PlayerClass::Archer, PlayerClass::Mage) => None,
            (PlayerClass::Archer, PlayerClass::Archer) => Some(()),
            (PlayerClass::Archer, PlayerClass::Priest) => Some(()),
            (PlayerClass::Archer, PlayerClass::DeathKnight) => None,
            (PlayerClass::Priest, PlayerClass::Mage) => None,
            (PlayerClass::Priest, PlayerClass::Archer) => None,
            (PlayerClass::Priest, PlayerClass::Priest) => None,
            (PlayerClass::Priest, PlayerClass::DeathKnight) => Some(()),
            (PlayerClass::DeathKnight, PlayerClass::Mage) => Some(()),
            (PlayerClass::DeathKnight, PlayerClass::Archer) => Some(()),
            (PlayerClass::DeathKnight, PlayerClass::Priest) => None,
            (PlayerClass::DeathKnight, PlayerClass::DeathKnight) => None,
        }
    }
}

#[derive(Debug, Copy, Serialize, Deserialize, Clone)]
enum PlayerClass {
    Mage,
    Archer,
    Priest,
    DeathKnight,
}

impl From<usize> for PlayerClass {
    fn from(value: usize) -> Self {
        match value {
            1 => PlayerClass::Mage,
            2 => PlayerClass::Archer,
            3 => PlayerClass::Priest,
            10 => PlayerClass::DeathKnight,
            _ => PlayerClass::Mage,
        }
    }
}
