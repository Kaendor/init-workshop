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

    pub fn fight(&self, other: &Player) -> Option<()> {
        // Ici on créer un tuple (PlayerClass,PlayerClass) afin de determiner le gagnant
        match (self.class, other.class) {
            (PlayerClass::Mage, PlayerClass::Mage) => todo!(),
            (PlayerClass::Mage, PlayerClass::Archer) => todo!(),
            (PlayerClass::Mage, PlayerClass::Priest) => todo!(),
            (PlayerClass::Mage, PlayerClass::DeathKnight) => todo!(),
            (PlayerClass::Archer, PlayerClass::Mage) => todo!(),
            (PlayerClass::Archer, PlayerClass::Archer) => todo!(),
            (PlayerClass::Archer, PlayerClass::Priest) => todo!(),
            (PlayerClass::Archer, PlayerClass::DeathKnight) => todo!(),
            (PlayerClass::Priest, PlayerClass::Mage) => todo!(),
            (PlayerClass::Priest, PlayerClass::Archer) => todo!(),
            (PlayerClass::Priest, PlayerClass::Priest) => todo!(),
            (PlayerClass::Priest, PlayerClass::DeathKnight) => todo!(),
            (PlayerClass::DeathKnight, PlayerClass::Mage) => todo!(),
            (PlayerClass::DeathKnight, PlayerClass::Archer) => todo!(),
            (PlayerClass::DeathKnight, PlayerClass::Priest) => todo!(),
            (PlayerClass::DeathKnight, PlayerClass::DeathKnight) => todo!(),
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
