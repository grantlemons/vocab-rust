//! Library for retrieving data from wordreference.com
#![warn(missing_docs)]
#![allow(dead_code)] // Remove after full implementation

use std::str::FromStr;

use reqwest::header::USER_AGENT;
use scraper::{ElementRef, Html, Selector};

#[derive(Debug, PartialEq)]
/// Language options
pub enum Language {
    /// English
    EN,
    /// Spanish
    ES,
}

impl FromStr for Language {
    type Err = ();

    fn from_str(input_string: &str) -> Result<Self, Self::Err> {
        match input_string.to_lowercase().as_str() {
            "english" => Ok(Self::EN),
            "spanish" => Ok(Self::ES),
            "en" => Ok(Self::EN),
            "es" => Ok(Self::ES),
            _ => Err(()),
        }
    }
}

impl Language {
    fn from_element(element: ElementRef) -> Result<Self, ()> {
        Language::from_str(element.text().next().unwrap())
    }
}

#[derive(Debug)]
/// Struct that represents a definition in a language
pub struct LanguageDefinition {
    language: Language,
    word: String,
    part: String,
    definition: String,
    example: String,
}

#[derive(Debug)]
/// Struct that represents a definition in both from and to languages
pub struct Definition {
    from: LanguageDefinition,
    to: LanguageDefinition,
}

#[derive(Debug)]
/// Struct representing a result from wordreference.com
pub struct Response {
    definitions: Vec<Definition>,
}

/// Fetches HTML webpage from wordreference.com
pub async fn get_html(
    word: String,
    from: Option<String>,
    to: Option<String>,
) -> Result<String, reqwest::Error> {
    let from: String = match from {
        Some(value) => value,
        None => "es".to_string(),
    };
    let to: String = match to {
        Some(value) => value,
        None => "en".to_string(),
    };

    let url = format!("https://www.wordreference.com/{from}/{to}/translation.asp?spen={word}");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/56.0.2924.87 Safari/537.36")
        .send()
        .await?;
    response.text().await
}

/// Parse HTML webpage into [`Response`] object
fn parse_html(html: String) -> Result<Response, scraper::error::SelectorErrorKind<'static>> {
    let document = Html::parse_document(html.as_str());
    let table_selector = Selector::parse(r#"table[class="WRD noTapHighlight"]"#)?;

    let tr_selector = Selector::parse("tr")?;
    let td_selector = Selector::parse("td")?;
    let table = document.select(&table_selector).next().unwrap();

    let elements = table
        .select(&td_selector)
        .skip(1)
        .flat_map(|td| {
            td.text()
                .map(|e| e.trim().replace(&['(', ')'][..], ""))
                .filter(|e| !e.is_empty())
        })
        .collect::<Vec<_>>();

    let rows = table
        .select(&tr_selector)
        .skip(2)
        .map(|tr| {
            tr.text()
                .map(|e| e.trim().replace(&['(', ')'][..], ""))
                .filter(|e| !e.is_empty())
                .collect::<Vec<_>>()
        })
        .filter(|a| a.len() > 3 || a.len() == 1)
        .collect::<Vec<Vec<_>>>();
    let definitions = rows
        .chunks(3)
        .map(|e| e.into_iter().flatten().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut response = Response {
        definitions: Vec::new(),
    };

    for def in definitions {
        response.definitions.push(Definition {
            from: LanguageDefinition {
                language: Language::from_str(elements[0].as_str()).unwrap(),
                word: def[0].clone(),
                part: def[1].clone(),
                definition: def[2].clone(),
                example: def[5].clone(),
            },
            to: LanguageDefinition {
                language: Language::from_str(elements[1].as_str()).unwrap(),
                word: def[3].clone(),
                part: def[4].clone(),
                definition: def[4].clone(),
                example: def[6].clone(),
            },
        })
    }

    Ok(response)
}

/// Takes a word and returns a [`Response`] struct
pub async fn get_def(
    word: String,
    from: Option<String>,
    to: Option<String>,
) -> Result<Response, ()> {
    let html = get_html(word, from, to).await.unwrap();
    Ok(parse_html(html).unwrap())
}
