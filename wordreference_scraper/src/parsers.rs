use crate::{Definition, LanguageDefinition};
use scraper::{Html, Selector};

fn parse_from_word(entry: &Html) -> String {
    let selector: Selector = Selector::parse("strong").unwrap();
    entry
        .select(&selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .trim()
        .to_string()
}
fn parse_to_word(entry: &Html) -> String {
    let selector: Selector = Selector::parse("td.ToWrd").unwrap();
    entry
        .select(&selector)
        .map(|e| e.text().next().unwrap().trim())
        .collect::<Vec<_>>()
        .join(", ")
}

fn parse_from_part(entry: &Html) -> String {
    let selector: Selector = Selector::parse("td.FrWrd").unwrap();
    let em_selector: Selector = Selector::parse("em").unwrap();
    match entry
        .select(&selector)
        .next()
        .unwrap()
        .select(&em_selector)
        .next()
    {
        Some(val) => val.text().next().unwrap().trim().to_string(),
        None => "".to_string(),
    }
}
fn parse_to_part(entry: &Html) -> String {
    let selector: Selector = Selector::parse("td.ToWrd").unwrap();
    let em_selector: Selector = Selector::parse("em").unwrap();
    entry
        .select(&selector)
        .next()
        .unwrap()
        .select(&em_selector)
        .map(|e| e.text().next().unwrap().trim())
        .collect::<Vec<_>>()
        .join(", ")
}

fn parse_from_definition(entry: &Html) -> String {
    let selector: Selector = Selector::parse("td").unwrap();
    entry
        .select(&selector)
        .nth(1)
        .unwrap()
        .text()
        .next()
        .unwrap()
        .replace(&['(', ')'][..], "")
        .trim()
        .to_string()
}
fn parse_to_definition(entry: &Html) -> String {
    parse_to_word(entry)
}

fn parse_from_example(entry: &Html) -> Vec<String> {
    let selector: Selector = Selector::parse("td.FrEx").unwrap();
    let span_selector: Selector = Selector::parse("span").unwrap();
    match entry.select(&selector).next() {
        Some(val) => val
            .select(&span_selector)
            .map(|e| e.text().next().unwrap().trim().to_string())
            .collect::<Vec<_>>(),
        None => Vec::new(),
    }
}
fn parse_to_example(entry: &Html) -> Vec<String> {
    let selector: Selector = Selector::parse("td.ToEx").unwrap();
    let span_selector: Selector = Selector::parse("span").unwrap();
    match entry.select(&selector).next() {
        Some(val) => val
            .select(&span_selector)
            .map(|e| e.text().next().unwrap().trim().to_string())
            .collect::<Vec<_>>(),
        None => Vec::new(),
    }
}

pub fn parse_entry(entry: &[String]) -> Definition {
    let fragment: String = format!(
        "<table><tbody>{}</tbody></table>",
        entry
            .join("")
            .replace("<br>", "")
            .replace('⇒', "->")
            .replace('ⓘ', "")
    );
    let html = Html::parse_fragment(&fragment);
    Definition {
        from: LanguageDefinition {
            word: parse_from_word(&html),
            part: parse_from_part(&html),
            definition: parse_from_definition(&html),
            example: parse_from_example(&html),
        },
        to: LanguageDefinition {
            word: parse_to_word(&html),
            part: parse_to_part(&html),
            definition: parse_to_definition(&html),
            example: parse_to_example(&html),
        },
    }
}
