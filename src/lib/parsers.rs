use crate::{Definition, Language, LanguageDefinition};
use scraper::{ElementRef, Html, Selector};

fn parse_from_language(entry: &ElementRef) -> Language {
    todo!()
}
fn parse_to_language(entry: &ElementRef) -> Language {
    todo!()
}

fn parse_from_word(entry: &ElementRef) -> String {
    todo!()
}
fn parse_to_word(entry: &ElementRef) -> String {
    todo!()
}

fn parse_from_part(entry: &ElementRef) -> String {
    todo!()
}
fn parse_to_part(entry: &ElementRef) -> String {
    todo!()
}

fn parse_from_definition(entry: &ElementRef) -> String {
    todo!()
}
fn parse_to_definition(entry: &ElementRef) -> String {
    todo!()
}

fn parse_from_example(entry: &ElementRef) -> String {
    todo!()
}
fn parse_to_example(entry: &ElementRef) -> String {
    todo!()
}

fn parse_entry(entry: &ElementRef) -> Definition {
    Definition {
        from: LanguageDefinition {
            language: parse_from_language(entry),
            word: parse_from_word(entry),
            part: parse_from_part(entry),
            definition: parse_from_definition(entry),
            example: parse_from_example(entry),
        },
        to: LanguageDefinition {
            language: parse_to_language(entry),
            word: parse_to_word(entry),
            part: parse_to_part(entry),
            definition: parse_to_definition(entry),
            example: parse_to_example(entry),
        },
    }
}
