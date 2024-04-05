use serde::{Deserialize, Serialize};

/// La struct Player représente un personnage dans le jeu
///
/// Au dessus de sa déclaration, on peut voir l'implementation automatique de plusieurs traits.
///
/// Un Trait en Rust est la déclaration d'un comportement, similaire à une interface en Java.
///
/// Afin de pouvoir implémenter automatiquement des traits, les membres de notre struct doivent les
/// implémenter aussi.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    /// Chaque commentaire avec triple /// sera compilé en documentation via cargo doc --open
    pub pseudo: String,
    pub level: u32,
    /// Un membre de struct peut contenir un type maison au lieu d'un type primitif
    class: PlayerClass,
}

/// Ce bloc déclare l'implémentation de Player
impl Player {
    /// Le mot clé pub permet de rendre la fonction accessible depuis l'extérieur du module
    pub fn new(pseudo: String) -> Self {
        // Par défaut, les variables en Rust sont immutables
        // Ici sans le mut, on ne pourrait pas incrémenter level
        let level = 0;

        // Le for en Rust prend un iterateur, ici on utilise un range
        // On peut noter l'absence de parenthèse autour de la condition
        // Le _ seul ou devant un nom de variable indique qu'on ne l'utilise pas
        for _ in 0..pseudo.len() {
            level += 1;
        }

        // Les méthodes statiques sont appelées avec le nom de la struct suivi de ::
        // pseudo.len() retourn un usize. Ce type représente sémantiquement une taille de tableau
        // et est le type par référence a utiliser lors de l'accès par index a un tableau car il
        // évite naturelle les out of bound.
        let class = PlayerClass::from(pseudo.len());

        // On peut voir que le mot clé return est optionnel en Rust et que le mot clé Self permet
        // de faire référence au type implémenté dans le bloc Impl courant.
        Self {
            pseudo,
            level,
            class,
        }
    }

    /// On peut noter le premier argument qui est self, il s'agit d'une référence à l'instance de la struct.
    /// Cette fonction permet de montrer l'usage des enums via les match
    pub fn fight(&self, other: &Player) -> Option<()> {
        // Ici on crée un tuple (PlayerClass,PlayerClass) afin de determiner le gagnant d'un duel
        match (self.class, other.class) {
            (PlayerClass::Mage, PlayerClass::Mage) => None,
            (PlayerClass::Mage, PlayerClass::Archer) => Some(()),
            (PlayerClass::Mage, PlayerClass::Priest) => Some(()),
            // Ces accolades permettent de déstructurer le variant DeathKnight
            // Les .. permettent d'ignorer les autres champs
            (PlayerClass::Mage, PlayerClass::DeathKnight { .. }) => None,
            (PlayerClass::Archer, PlayerClass::Mage) => None,
            (PlayerClass::Archer, PlayerClass::Archer) => Some(()),
            (PlayerClass::Archer, PlayerClass::Priest) => Some(()),
            (PlayerClass::Archer, PlayerClass::DeathKnight { .. }) => None,
            (PlayerClass::Priest, PlayerClass::Mage) => None,
            (PlayerClass::Priest, PlayerClass::Archer) => None,
            (PlayerClass::Priest, PlayerClass::Priest) => None,
            (PlayerClass::Priest, PlayerClass::DeathKnight { .. }) => Some(()),
            (PlayerClass::DeathKnight { .. }, PlayerClass::Mage) => Some(()),
            (PlayerClass::DeathKnight { .. }, PlayerClass::Archer) => Some(()),
            (PlayerClass::DeathKnight { .. }, PlayerClass::Priest) => None,
            (PlayerClass::DeathKnight { .. }, PlayerClass::DeathKnight { .. }) => None,
            // Si un cas n'est pas traité ou ignoré explicitement, Rust va nous avertir
        }
    }
}

/// Une enum Rust en plus d'être un structure qu'on retrouve ailleurs,
/// Elle est aussi capable de contenir de l'information
#[derive(Debug, Copy, Serialize, Deserialize, Clone)]
enum PlayerClass {
    Mage,
    Archer,
    Priest,
    /// Ici le variant contient des données qui lui sont propres
    DeathKnight {
        sword_length: f32,
    },
}

impl From<usize> for PlayerClass {
    fn from(value: usize) -> Self {
        let value = value % 4;
        match value {
            1 => PlayerClass::Mage,
            2 => PlayerClass::Archer,
            3 => PlayerClass::Priest,
            4 => PlayerClass::DeathKnight { sword_length: 1.0 },
            _ => PlayerClass::Mage,
        }
    }
}
