mod enums;
mod iterators;
mod structs;

fn main() {
    variables_and_control_flow();
}

fn variables_and_control_flow() {
    // Les variables en Rust sont par défaut immutables
    let url = "https://indy.fr";

    // Rust favorise les approches immutable mais permet de muter au besoin avec le mot clé mut
    let counter = 0;
    // On peut voir que le compilateur indique clairement qu'on ne peut pas muter une variable
    // immutable.

    // Il propose une solution si on survol l'erreur avec le LSP.
    // A l'inverse de beaucoup de langage, un effort est fait sur les erreurs de compilation afin
    // qu'elles aident plus qu'elles ne paralysent.
    counter += 1;

    // Pas besoin de parenthèse autour de la condition en Rust
    if counter >= 0 {
        println!("Je suis l'équivalent d'un console.log() (mais en mieux), parce que j'ai de l'interpolation {url}");
    }

    // Une particularité de Rust qui peut être un peu étrange au début, c'est que beaucoup
    // d'expressions peuvent être attribuées à une variable.
    // Par exemple
    let iteration_count = if counter == 1 { 30 } else { 0 };

    // Les boucles en rust sont similaire à celles qu'on retrouve dans d'autre langage
    // A la difference près qu'elles prennent des Iterators en entrée
    // Ici on voit un Range, qui est un Iterator sur des chiffres entiers.
    for c in 0..iteration_count {
        println!("{c}");
    }
}
