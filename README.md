# Introduction a Rust

## Objectifs

- Lire le Rust
- Pouvoir comparer simplement ses features avec d'autres langages
- Avoir une API web simple

## Déroulé

- Présentation globale au groupe
- Découverte d'une feature de Rust -> séparation en petit groupe pour l'exercer
- Regroupement pour explications

## Etapes

- Variables & Control flow
- Modules
- Structs
- Iterators
- Enum
- Pattern matching
- Ownership
- Tests
- Docs
- Traits
- Async

## Point d'entrée

Le point d'entrée va permettre une approche progressive du langage avec explications orale en plus de ce qui est écrit.
`src/game/player.rs`


## Commandes

Vérification du code :

`cargo check` ou `cargo clippy` quand on souhaite une analyse plus poussée

Lancement des tests :

`cargo test` on peut scoper les tests en passant une string en plus comme : `cargo test player` qui va lancer tous les tests concernant player

Lancement de l'application :

`cargo run` ou `cargo run --release` pour une version optimisée pour la production
