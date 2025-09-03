use serde::Deserialize;
use rand::seq::SliceRandom;
use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Clone)]
struct Question {
    question: String,
    options: Vec<String>,
    answer: usize,
}

#[derive(Debug, Deserialize)]
struct QuestionSet {
    description: String,
    questions: Vec<Question>,
}

fn main() -> io::Result<()> {
    // Schritt 1: Liste der verfügbaren YAML-Dateien mit Beschreibungen anzeigen
    let question_file = select_question_file("Questions")?;
    let question_set = load_question_set(&question_file)?;

    // Schritt 2: Fragen mischen und starten
    let mut rng = rand::thread_rng();
    let mut shuffled_questions = question_set.questions.clone();
    shuffled_questions.shuffle(&mut rng);

    let mut score = 0;

    for question in shuffled_questions {
        println!("{}", question.question);
        for (i, option) in question.options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        println!("Bitte geben Sie die Nummer Ihrer Antwort ein:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let choice: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ungültige Eingabe. Bitte geben Sie eine gültige Nummer ein.");
                continue;
            }
        };

        if choice == question.answer {
            println!("Richtig!");
            score += 1;
        } else {
            println!("Falsch. Die richtige Antwort war Option {}.", question.answer);
        }
        println!();
    }

    println!("Deine Endpunktzahl ist: {}/{}", score, question_set.questions.len());
    Ok(())
}

// Lade alle YAML-Dateien in einem Verzeichnis, zeige die Beschreibung, und lasse den Benutzer wählen
fn select_question_file<P: AsRef<Path>>(dir_path: P) -> io::Result<PathBuf> {
    let entries: Vec<_> = fs::read_dir(&dir_path)?
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map(|ext| ext == "yml" || ext == "yaml").unwrap_or(false))
        .collect();

    if entries.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Keine YAML-Dateien gefunden."));
    }

    println!("Verfügbare Frage-Sets:");

    for (i, entry) in entries.iter().enumerate() {
        if let Ok(set) = load_question_set(entry.path()) {
            println!("{}. {} ({})", i + 1, entry.file_name().to_string_lossy(), set.description);
        } else {
            println!("{}. {} (FEHLER beim Lesen der Beschreibung)", i + 1, entry.file_name().to_string_lossy());
        }
    }

    println!("Bitte wählen Sie eine Datei durch Eingabe der Nummer:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice: usize = input.trim().parse().unwrap_or(0);

    if choice == 0 || choice > entries.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Ungültige Auswahl."));
    }

    Ok(entries[choice - 1].path())
}

// Neue Funktion: Lädt eine YAML-Datei mit Beschreibung und Fragen
fn load_question_set<P: AsRef<Path>>(path: P) -> io::Result<QuestionSet> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    serde_yaml::from_reader(reader).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

