use std::io::{self, Write};

// Structure d'une tâche
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

fn main() {
    // Vecteur pour stocker les tâches
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("Que voulez-vous faire ?");
        println!("1. Ajouter une tâche");
        println!("2. Afficher les tâches");
        println!("3. Marquer une tâche comme terminée");
        println!("4. Quitter");

        print!("Votre choix : ");
        io::stdout().flush().unwrap();

        // Lecture de l'entrée de l'utilisateur
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Erreur de lecture de l'entrée");

        // Gestion des choix de l'utilisateur
        match choice.trim() {
            "1" => {
                print!("Entrez la description de la tâche : ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Erreur de lecture de l'entrée");
                tasks.push(Task::new(description.trim().to_string()));
            }
            "2" => {
                println!("Tâches :");
                for (index, task) in tasks.iter().enumerate() {
                    println!("{}. [{}] {}", index + 1, if task.completed { "X" } else { " " }, task.description);
                }
            }
            "3" => {
                print!("Entrez le numéro de la tâche terminée : ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Erreur de lecture de l'entrée");
                if let Ok(index) = input.trim().parse::<usize>() {
                    if index > 0 && index <= tasks.len() {
                        tasks[index - 1].complete();
                    } else {
                        println!("Numéro de tâche invalide !");
                    }
                } else {
                    println!("Veuillez entrer un numéro de tâche valide !");
                }
            }
            "4" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide"),
        }
    }
}
