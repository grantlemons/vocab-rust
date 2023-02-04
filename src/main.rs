//! Retrieves definitions etc. from wordreference api and assembles them into a table
#![warn(missing_docs)]

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::io::stdout;
use text_io::read;
use wordreference::{Definition, Response};

/// Entrypoint for binary program
#[tokio::main]
async fn main() -> Result<(), String> {
    let mut words: Vec<Response> = Vec::new();
    let mut chosen_definitions: Vec<Definition> = Vec::new();

    clear_term();
    stdout().execute(cursor::MoveTo(0, 0)).unwrap();
    println!("Enter an empty line to continue.\n===============");
    while choose_word(&mut words).await? {}
    for word in words {
        clear_term();
        stdout().execute(cursor::MoveTo(0, 0)).unwrap();
        choose_definition(&word, &mut chosen_definitions);
    }
    clear_term();
    if !chosen_definitions.is_empty() {
        print_definitions(&chosen_definitions);
    }
    Ok(())
}

fn clear_term() {
    stdout()
        .queue(terminal::Clear(terminal::ClearType::All))
        .unwrap();
}

/// Present the user with a menu for selecting a word
async fn choose_word(words: &mut Vec<Response>) -> Result<bool, String> {
    print!("Word {}: ", words.len() + 1);
    let input: String = read_string();

    if !input.is_empty() {
        let definitions = wordreference::get_def(input, None, None).await?;
        words.push(definitions);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Present to the user a menu for selecting the intended definition from the possible ones for a chosen word
fn choose_definition(word: &Response, chosen_definitions: &mut Vec<Definition>) {
    let mut input: usize = 1;
    if word.definitions.len() > 1 {
        println!("Definition options for {}", word.definitions[0].from.word);
        println!("========================================================================");
        for (index, def) in word.definitions.iter().enumerate() {
            println!(
                "{:<3} {:<20} {:<10} {:<40} {:?} / {:?}",
                format!("{:>3}", format!("{}:", index + 1)),
                def.from.word,
                format!("({})", def.from.part),
                def.from.definition,
                def.from.example,
                def.to.example
            );
        }
        print!("Index: ");
        input = read_usize();
    }

    chosen_definitions.push(word.definitions.get(input - 1).unwrap().clone());
}

/// Print a formatted table of words, parts of speech, and definitions
fn print_definitions(chosen_definitions: &Vec<Definition>) {
    stdout().execute(cursor::MoveTo(0, 0)).unwrap();
    println!(
        "|{:<4}{:<25}{:<4}|{:^20}|{:^20}|{:^40}|{:^20}|",
        "", "Palabra", "", "Categoría", "Fuente", "Definición y diccionario", "Contexto"
    );
    println!(
        "|{:=<4}{:=<25}{:=<4}|{:=<20}|{:=<20}|{:=<40}|{:=<20}|",
        "", "", "", "", "", "", ""
    );
    for def in chosen_definitions {
        println!(
            "|{:<4}{:<25}{:<4}|{:^20}|{:^20}|{:^40}|{:^20}|",
            "",
            def.from.word,
            "",
            def.from.part,
            "",
            format!("{} (WR)", def.from.definition),
            ""
        );
    }
}

#[cfg(not(target_os = "windows"))]
fn read_string() -> String {
    read!("{}\n")
}
#[cfg(not(target_os = "windows"))]
fn read_usize() -> usize {
    read!("{}\n")
}

#[cfg(target_os = "windows")]
fn read_string() -> String {
    read!("{}\r\n")
}
#[cfg(target_os = "windows")]
fn read_usize() -> usize {
    read!("{}\r\n")
}
