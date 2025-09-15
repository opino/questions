use serde::Deserialize;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::{self, File};
use std::io::{self, BufReader, BufRead, Write};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use sha2::{Sha256, Digest};

#[derive(Debug, Deserialize, Clone)]
struct Question {
    question: String,
    options: Vec<String>,
    answer: usize,
    hash: Option<String>,
}

#[derive(Debug, Deserialize)]
struct QuestionSet {
    description: String,
    questions: Vec<Question>,
}

// Neue Typdefinition für falsche Antworten mit Frage
type WrongAnswerData = HashMap<String, (i32, String)>;

fn main() -> io::Result<()> {
    let question_file = select_question_file("Questions")?;
    let question_set = load_question_set(&question_file)?;

    println!("Wählen Sie die Methode, wie die Fragen gestellt werden sollen:");
    println!("1. Zufällige Reihenfolge");
    println!("2. Fragen mit Hash - Falsche Antworten zuerst");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let method = input.trim().parse::<u8>().unwrap_or(1);

    let mut questions_to_ask = question_set.questions.clone();

    let mut wrong_answer_counters: WrongAnswerData = if method == 2 {
        load_wrong_answers_with_counts("wrong_answers.txt")?
    } else {
        WrongAnswerData::new()
    };

    if method == 2 {
        let mut priority_questions: Vec<_> = questions_to_ask.iter()
            .filter(|q| wrong_answer_counters.get(&hash_question(q)).map_or(0, |(c, _)| *c) > 0)
            .cloned()
            .collect();

        let mut other_questions: Vec<_> = questions_to_ask.iter()
            .filter(|q| wrong_answer_counters.get(&hash_question(q)).map_or(0, |(c, _)| *c) == 0)
            .cloned()
            .collect();

        let mut rng = thread_rng();
        let mut asked_questions = Vec::new();

        priority_questions.shuffle(&mut rng);
        other_questions.shuffle(&mut rng);

        while !priority_questions.is_empty() || !other_questions.is_empty() {
            if !priority_questions.is_empty() {
                asked_questions.push(priority_questions.pop().unwrap());
            } else if !other_questions.is_empty() {
                asked_questions.push(other_questions.pop().unwrap());
            }
        }

        questions_to_ask = asked_questions;
    } else {
        let mut rng = thread_rng();
        questions_to_ask.shuffle(&mut rng);
    }

    let mut score = 0;
    for question in questions_to_ask {
        println!("{}", question.question);
        for (i, option) in question.options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        println!("Bitte geben Sie die Nummer Ihrer Antwort ein:");
        input.clear();
        io::stdin().read_line(&mut input)?;

        let choice: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ungültige Eingabe. Bitte geben Sie eine gültige Nummer ein.");
                continue;
            }
        };

        let q_hash = hash_question(&question);

        if choice == question.answer {
            println!("Richtig!");
            score += 1;

            let counter_entry = wrong_answer_counters.entry(q_hash.clone())
                .or_insert((0, question.question.clone()));
            if counter_entry.0 > 0 {
                counter_entry.0 -= 1;
            }
        } else {
            println!("Falsch. Die richtige Antwort war Option {}.", question.answer);

            let counter_entry = wrong_answer_counters.entry(q_hash.clone())
                .or_insert((0, question.question.clone()));
            counter_entry.0 += 1;
        }

        save_wrong_answers_with_counts("wrong_answers.txt", &wrong_answer_counters)?;
        println!();
    }

    println!("Deine Endpunktzahl ist: {}/{}", score, question_set.questions.len());
    Ok(())
}

// SHA256-Hash der Frage berechnen
fn hash_question(question: &Question) -> String {
    if let Some(ref h) = question.hash {
        return h.clone();
    }
    let mut hasher = Sha256::new();
    hasher.update(question.question.as_bytes());
    hasher.update(question.options.join("").as_bytes());
    format!("{:x}", hasher.finalize())
}

// Datei mit falschen Antworten laden
fn load_wrong_answers_with_counts<P: AsRef<Path>>(file_path: P) -> io::Result<WrongAnswerData> {
    let mut wrong_answers = WrongAnswerData::new();
    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let mut parts = line.splitn(3, ',');
            if let (Some(hash), Some(count_str), Some(question)) = (parts.next(), parts.next(), parts.next()) {
                if let Ok(count) = count_str.parse::<i32>() {
                    wrong_answers.insert(hash.to_string(), (count, question.trim_matches('"').to_string()));
                }
            }
        }
    }
    Ok(wrong_answers)
}

// Datei mit falschen Antworten speichern
fn save_wrong_answers_with_counts<P: AsRef<Path>>(file_path: P, data: &WrongAnswerData) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    for (hash, (count, question)) in data.iter() {
        if *count > 0 {
            writeln!(file, "{},{},\"{}\"", hash, count, question.replace('"', "\"\""))?;
        }
    }
    Ok(())
}

// Datei mit Fragen auswählen
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

// Fragen aus YAML-Datei laden
fn load_question_set<P: AsRef<Path>>(path: P) -> io::Result<QuestionSet> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    serde_yaml::from_reader(reader).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

