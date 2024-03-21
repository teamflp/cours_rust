
/*fn main() {
    println!("Hello, world!");
}
*/

/*fn main() {
    let x: String = String::from("Hello, world!"); // Une chaîne de caractères dynamique
    println!("La valeur de x est : {}", x); // La valeur de x est : Hello, world!
}*/

/*fn main() {
    let vecteur = vec!["un", "deux", "trois", "quatre", "cinq"]; // Création d'un vecteur avec des valeurs.
    let chaine = vecteur.join(" - "); // Concaténation des éléments du vecteur avec une chaîne séparatrice.

    println!("Le vecteur concaténé est : {}", chaine); // Affichage du vecteur concaténé.
}*/

/*fn main() {
    let vecteur = vec![1, 2, 3, 4, 5, 6, 7, 8]; // Création d'un vecteur avec des valeurs.
    let nombre_pairs = vecteur.iter().filter(|&x| x % 2 == 0).count(); // Comptage des éléments pairs.

    println!("Le nombre d'éléments pairs est : {}", nombre_pairs); // Affichage du nombre d'éléments pairs.
}*/

/*fn main() {
    let vecteur = vec![1, 2, 3, 4, 5, 6, 7, 8]; // Création d'un vecteur avec des valeurs.
    let (gauche, droite) = vecteur.split_at(4); // Division du vecteur en deux à l'index 3.

    println!("{:?}", gauche); // Affichage de la partie gauche du vecteur.
    println!("{:?}", droite); // Affichage de la partie droite du vecteur.
}*/



/*fn main() {
    let mut noms = Vec::new(); // Création d'un vecteur pour stocker les noms.
    let mut ages = Vec::new(); // Création d'un vecteur pour stocker les âges.

    loop {
        println!("Entrez le nom de l'utilisateur (ou 'fin' pour terminer) :");
        let mut nom = String::new(); // Création d'une nouvelle chaîne pour stocker le nom.
        io::stdin().read_line(&mut nom).expect("Échec de la lecture de l'entrée"); // Lecture du nom de l'utilisateur.

        if nom.trim().eq_ignore_ascii_case("fin") { // Vérification si l'utilisateur a entré "fin".
            break; // Sortie de la boucle si l'utilisateur a entré "fin".
        }

        noms.push(nom.trim().to_string()); // Ajout du nom à la liste des noms.

        println!("Entrez l'âge de l'utilisateur :");
        let mut age_utilisateur = String::new(); // Création d'une nouvelle chaîne pour stocker l'âge.
        io::stdin().read_line(&mut age_utilisateur).expect("Échec de la lecture de l'entrée"); // Lecture de l'âge de l'utilisateur.

        let age_utilisateur: u32 = match age_utilisateur.trim().parse() {
            Ok(valeur) => valeur, // Récupération de la valeur si la conversion réussit.
            Err(_) => {
                println!("Veuillez entrer un nombre valide pour l'âge.");
                noms.pop(); // Supprime le dernier nom saisi puisque l'âge correspondant est invalide
                continue; // Retour au début de la boucle si l'âge n'est pas valide.
            }
        };

        ages.push(age_utilisateur); // Ajout de l'âge à la liste des âges.

        // Affichage des noms et âges après chaque entrée
        println!("Nom de l'utilisateur ajouté : {}", noms.last().unwrap());
        println!("Âge de l'utilisateur ajouté : {}", ages.last().unwrap());
    }

    // Affichage final des listes complètes
    println!("Les noms des utilisateurs sont : {:?}", noms);
    println!("Les âges des utilisateurs sont : {:?}", ages);
}*/

//use rand::Rng;

/*fn main() {
    let nombre_secret = rand::thread_rng().gen_range(1..101);
    println!("Devinez le nombre secret entre 1 et 100.");

    loop {
        let mut supposition = String::new();
        std::io::stdin().read_line(&mut supposition).expect("Échec de la lecture de l'entrée");
        let supposition: u32 = match supposition.trim().parse() {
            Ok(valeur) => valeur,
            Err(_) => {
                println!("Veuillez entrer un nombre valide.");
                continue;
            }
        };

        match supposition.cmp(&nombre_secret) {
            std::cmp::Ordering::Less => println!("Trop petit !"),
            std::cmp::Ordering::Greater => println!("Trop grand !"),
            std::cmp::Ordering::Equal => {
                println!("Bravo, vous avez deviné le nombre secret !");
                break;
            }
        }
    }
}*/

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/*fn main() {
    let handle = thread::spawn(|| { // Création d'un nouveau thread.
        // Code exécuté dans un nouveau thread.
        for i in 1..=10 {
            println!("Nouveau thread : {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    for i in 1..=3 {
        println!("Thread principal : {}", i);
        thread::sleep(std::time::Duration::from_millis(1000));
    }

    handle.join().unwrap(); // Attente de la fin du thread.
}*/

/*fn main() {
    let (emetteur, recepteur) = mpsc::channel(); // Création d'un canal pour la communication entre threads.

    thread::spawn(move || { // Création d'un nouveau thread.
        let message = "Bonjour depuis le thread émetteur !".to_string();
        emetteur.send(message).unwrap(); // Envoi d'un message à travers le canal.
    });

    let message_recu = recepteur.recv().unwrap(); // Réception du message à travers le canal.
    println!("Message reçu : {}", message_recu); // Affichage du message reçu.
}*/

/*fn main() {
    let donnees_partagees = Arc::new(Mutex::new(0)); // Création d'une mémoire partagée avec Mutex.

    let mut threads = vec![]; // Création d'un vecteur pour stocker les threads.

    for _ in 0..5 {
        let donnees_partagees = Arc::clone(&donnees_partagees); // Clonage de la mémoire partagée pour chaque thread.
        let thread = thread::spawn(move || { // Création d'un nouveau thread.
            let mut donnees = donnees_partagees.lock().unwrap(); // Verrouillage de la mémoire partagée.
            *donnees += 1; // Incrémentation des données partagées.
        });
        threads.push(thread); // Ajout du thread au vecteur.
    }

    for thread in threads {
        thread.join().unwrap(); // Attente de la fin de chaque thread.
    }

    println!("Données partagées : {}", *donnees_partagees.lock().unwrap()); // Affichage des données partagées.
}*/

fn telecharger_donnees(url: &str, donnees: Arc<Mutex<Vec<u8>>>) {
    // Simulation du téléchargement de données à partir de l'URL.
    thread::sleep(Duration::from_secs(2));
    let donnees_telechargees = b"Exemple de donn\xE9es t\xE9l\xE9charg\xE9es";
    let mut donnees = donnees.lock().unwrap();
    donnees.extend_from_slice(donnees_telechargees);
}

fn main() {
    let donnees = Arc::new(Mutex::new(Vec::new())); // Création d'une mémoire partagée avec Mutex pour stocker les données téléchargées.

    let urls = vec!["http://exemple.com/fichier1", "http://exemple.com/fichier2", "http://exemple.com/fichier3"];

    let mut threads = vec![]; // Création d'un vecteur pour stocker les threads.

    for url in urls {
        let donnees = Arc::clone(&donnees); // Clonage de la mémoire partagée pour chaque thread.
        let thread = thread::spawn(move || { // Création d'un nouveau thread.
            telecharger_donnees(url, donnees); // Téléchargement des données à partir de l'URL.
        });
        threads.push(thread); // Ajout du thread au vecteur.
    }

    for thread in threads {
        thread.join().unwrap(); // Attente de la fin de chaque thread.
    }

    let donnees = donnees.lock().unwrap(); // Verrouillage de la mémoire partagée.
    println!("Données téléchargées : {:?}", *donnees); // Affichage des données téléchargées.
}