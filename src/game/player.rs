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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum PlayerClass {
    Warrior,
    Mage,
    Archer,
    Priest,
    Paladin,
    Monk,
    Druid,
    Shaman,
    Warlock,
    Rogue,
    DeathKnight,
    Hunter,
    DemonHunter,
}

impl From<usize> for PlayerClass {
    fn from(value: usize) -> Self {
        match value {
            0 => PlayerClass::Warrior,
            1 => PlayerClass::Mage,
            2 => PlayerClass::Archer,
            3 => PlayerClass::Priest,
            4 => PlayerClass::Paladin,
            5 => PlayerClass::Monk,
            6 => PlayerClass::Druid,
            7 => PlayerClass::Shaman,
            8 => PlayerClass::Warlock,
            9 => PlayerClass::Rogue,
            10 => PlayerClass::DeathKnight,
            11 => PlayerClass::Hunter,
            12 => PlayerClass::DemonHunter,
            _ => PlayerClass::Warrior,
        }
    }
}
