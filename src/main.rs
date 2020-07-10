use std::process;
use std::path::PathBuf;
use std::error::Error;
use std::fs;
use csv::Reader;
use greek_vocab_test_maker::{
    Question,
    Record,
    AnswerOption,
};
// import from file
// add manually
// duplicate entry
// delete entry
// edit entry
// print to xml
fn main() {
    if let Err(e) = run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut questions: Vec<Question> = Vec::new();
    loop {
        // clear the screen
        println!("\x1B[2J");
        print_boxed("Greek Vocab Test for Moodle Maker");
        println!("Question count: {}", questions.len());
        println!("");
        print_boxed("a: add    i: import    r: review    q: quit");
        match read_input().as_str() {
            "i" => import(&mut questions),
            "q" => break,
            _ => continue,
        }
    }
    Ok(())
}

pub fn print_boxed(content: &str) {
    println!("{}{}{}", "+", "-".repeat(78), "+");
    for line in content.split("\n") {
        println!("| {:76} |", line);
    }
    println!("{}{}{}", "+", "-".repeat(78), "+");
}

fn read_input() -> String {
    let mut rl = rustyline::Editor::<()>::new();
    let readline = rl.readline(">> ");
    readline.unwrap_or_default()
}

fn import(questions: &mut Vec<Question>) {
    let file: PathBuf = get_file();
    let mut rdr = Reader::from_path(file).unwrap();
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        match record.part_of_speech {
            pos if pos.contains("verb") => build_verb(questions, record.greek, record.english),
            _ => build_non_verb(questions, record.greek, record.english),
        };
    }
}

fn get_file() -> PathBuf {
    print_boxed("Select file:");
    let entries = list_files().unwrap();
    for (num, entry) in entries.iter().enumerate() {
        println!("{}: {:?}", num + 1, entry);
    }
    let choice = get_num_choice("Enter a number: ");
    println!("You chose {}", choice);
    println!("You chose: {:?}", entries[choice]);
    read_input();
    entries[choice].to_owned()
}

fn list_files() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let entries = fs::read_dir(".")?;
    let mut list: Vec<PathBuf> = Vec::new();
    for entry in entries {
        let file = entry?.path();
        if file.is_file() {
            list.push(file);
        }
    }
    Ok(list)
}

fn get_num_choice(prompt: &str) -> usize {
    loop {
        let mut rl = rustyline::Editor::<()>::new();
        let readline = rl.readline(prompt);
        let choice = readline.unwrap_or_default();
        let idx = choice.parse::<usize>();
        match idx {
            Ok(num) => return num - 1,
            Err(_error) => {
                println!("You must enter a number.");
                continue;
            }
        }
    }
}

fn build_non_verb(questions: &mut Vec<Question>, greek: String, english: String) {
    let mut answer_options: Vec<AnswerOption> = Vec::new();
    for answer in english.split(",").collect::<Vec<&str>>() {
        let answer_option = AnswerOption {
            mark: 100,
            answer: answer.trim().to_string(),
            feedback: "Well done!".to_string(),
        };
        answer_options.push(answer_option);
    }
    let question = Question {
        greek: greek,
        answers: answer_options,
    };
    questions.push(question);
}

fn build_verb(questions: &mut Vec<Question>, greek: String, english: String) {
    println!("Greek verb: {}", greek);
    println!("English: {}", english);
    let answers = english.split(",").collect::<Vec<&str>>();
    println!("Answers: {:?}", answers);
    let greek = greek.replace("I ", "");
    let greek_parts = greek.split(",").collect::<Vec<&str>>();
    println!("Parts: {:?}", greek_parts);
    //present tense
    let mut answer_options: Vec<AnswerOption> = Vec::new();
    for answer in &answers {
        println!("Present: {}", answer.trim());
        let answer_option = AnswerOption {
            mark: 100,
            answer: format!("I {}", answer.trim()),
            feedback: "Well done!".to_string(),
        };
        answer_options.push(answer_option);
    }
    let present = Question {
        greek: greek_parts[0].trim().to_string(),
        answers: answer_options,
    };
    questions.push(present);
    // future tense
    if greek_parts.len() > 1 {
        let mut answer_options: Vec<AnswerOption> = Vec::new();
        for answer in &answers {
            let answer_option = AnswerOption {
                mark: 100,
                answer: format!("I*ll {}", answer.trim()),
                feedback: "Well done!".to_string(),
            };
            answer_options.push(answer_option);
        }
        let future = Question {
            greek: greek_parts[1].trim().to_string(),
            answers: answer_options,
        };
        questions.push(future);
    }
    // aorist tense
    if greek_parts.len() > 2 {
        let mut answer_options: Vec<AnswerOption> = Vec::new();
        for answer in &answers {
            let answer_option = AnswerOption {
                mark: 100,
                answer: format!("I {}ed", answer.trim()),
                feedback: "Well done!".to_string(),
            };
            answer_options.push(answer_option);
        }
        let aorist = Question {
            greek: greek_parts[2].trim().to_string(),
            answers: answer_options,
        };
        questions.push(aorist);
    }
    // aorist passive
    if greek_parts.len() > 3 {
        let mut answer_options: Vec<AnswerOption> = Vec::new();
        for answer in &answers {
            let answer_option = AnswerOption {
                mark: 100,
                answer: format!("I was {}ed", answer.trim()),
                feedback: "Well done!".to_string(),
            };
            answer_options.push(answer_option);
        }
        let aorist_passive = Question {
            greek: greek_parts[3].trim().to_string(),
            answers: answer_options,
        };
        questions.push(aorist_passive);
    }
}
