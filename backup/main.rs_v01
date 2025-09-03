use serde::Deserialize;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

#[derive(Debug, Deserialize,Clone)]
struct Question {
    question: String,
    options: Vec<String>,
    answer: usize,
}

fn main() -> io::Result<()> {
    // Lade die Fragen aus der YAML-Datei
    let questions = load_questions("questions.yml")?;

    // Mische die Fragen, um die Reihenfolge zu ändern
    let mut rng = rand::thread_rng();
    let mut shuffled_questions = questions.clone();
    shuffled_questions.shuffle(&mut rng);

    let mut score = 0;

    // Gehe jede Frage durch und stelle sie
    for question in shuffled_questions {
        println!("{}", question.question);

        // Drucke die Optionen
        for (i, option) in question.options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        // Frage den Benutzer nach der Antwort
        println!("Bitte geben Sie die Nummer Ihrer Antwort ein:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // Konvertiere die Benutzereingabe in eine Zahl und überprüfe, ob sie gültig ist
        let choice: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ungültige Eingabe. Bitte geben Sie eine gültige Nummer ein.");
                continue;
            }
        };

        // Überprüfe, ob die Antwort richtig ist
        if choice == question.answer {
            println!("Richtig!");
            score += 1;
        } else {
            println!("Falsch. Die richtige Antwort war Option {}.", question.answer);
        }
        println!();
    }

    // Gib die Endpunktzahl aus
    println!("Deine Endpunktzahl ist: {}/{}", score, questions.len());

    Ok(())
}

use std::convert::From;

fn load_questions<P: AsRef<Path>>(path: P) -> io::Result<Vec<Question>> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    serde_yaml::from_reader(reader)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}


