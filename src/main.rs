//! Retrieves definitions etc. from wordreference api and assembles them into a table
#![warn(missing_docs)]

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::io::stdout;
use text_io::read;
use wordreference as wr;
use wordreference::{Definition, Response};

/// Entrypoint for binary program
#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut words: Vec<Response> = Vec::new();
    let mut chosen_definitions: Vec<Definition> = Vec::new();

    clear_term();
    choose_words(&mut words).await?;
    for word in words {
        clear_term();
        stdout().execute(cursor::MoveTo(0, 0)).unwrap();
        choose_definition(&word, &mut chosen_definitions);
    }
    clear_term();
    print_definitions(&chosen_definitions);

    Ok(())
}

fn clear_term() {
    stdout()
        .queue(terminal::Clear(terminal::ClearType::All))
        .unwrap();
}

async fn choose_words(words: &mut Vec<Response>) -> Result<(), ()> {
    stdout().execute(cursor::MoveTo(0, 0)).unwrap();
    loop {
        print!("Word {}: ", words.len() + 1);
        let input: String = read!("{}\n");

        if !input.is_empty() {
            let definitions = wr::get_def(input, None, None).await?;
            words.push(definitions);
        } else {
            break;
        }
    }
    Ok(())
}

fn choose_definition(word: &Response, chosen_definitions: &mut Vec<Definition>) {
    println!("Definition options for {}", word.definitions[0].from.word);
    println!("========================================================================");
    for (index, def) in word.definitions.iter().enumerate() {
        println!(
            "{}: {} ({})  --  {}  --  \"{}\" / \"{}\"",
            index + 1,
            def.from.word,
            def.from.part,
            def.from.definition,
            def.from.example,
            def.to.example
        );
    }
    print!("Index: ");
    let input: usize = read!("{}\n");

    chosen_definitions.push(word.definitions.get(input - 1).unwrap().clone());
}

fn print_definitions(chosen_definitions: &Vec<Definition>) {
    stdout().execute(cursor::MoveTo(0, 0)).unwrap();
    println!(
        "{:<4}{:<25}{:<4}|{:^20}|{:^20}|{:^40}|{:^20}|",
        "", "Palabra", "", "Categoría", "Fuente", "Definición y diccionario", "Contexto"
    );
    println!(
        "{:=<4}{:=<25}{:=<4}|{:=<20}|{:=<20}|{:=<40}|{:=<20}|",
        "", "", "", "", "", "", ""
    );
    for def in chosen_definitions {
        println!(
            "{:<4}{:<25}{:<4}|{:^20}|{:^20}|{:^40}|{:^20}|",
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
