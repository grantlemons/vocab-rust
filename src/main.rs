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
    let mut stdout = stdout();
    let mut possible_definitions: Vec<Response> = Vec::new();
    let mut chosen_definitions: Vec<Definition> = Vec::new();

    clear_term();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    loop {
        print!("Word {}: ", possible_definitions.len() + 1);
        let input: String = read!("{}\n");

        if !input.is_empty() {
            possible_definitions.push(wr::get_def(input, None, None).await?)
        } else {
            break;
        }
    }
    clear_term();

    for res in possible_definitions {
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        println!("Definition options for {}", res.definitions[0].from.word);
        println!("========================================================================");
        for (index, def) in res.definitions.iter().enumerate() {
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

        chosen_definitions.push(res.definitions.get(input - 1).unwrap().clone());
        clear_term();
    }

    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
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

    Ok(())
}

fn clear_term() {
    stdout()
        .queue(terminal::Clear(terminal::ClearType::All))
        .unwrap();
}
