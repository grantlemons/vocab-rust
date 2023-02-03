//! Library for retrieving data from wordreference.com
#![warn(missing_docs)]

use regex::Regex;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};

mod parsers;
use parsers::*;

#[derive(Debug, Clone)]
/// Struct that represents a definition in a language
pub struct LanguageDefinition {
    /// The word in question
    pub word: String,
    /// Part of speech
    pub part: String,
    /// Word's definition
    pub definition: String,
    /// An example of the word used in a sentence
    pub example: Vec<String>,
}

#[derive(Debug, Clone)]
/// Struct that represents a definition in both from and to languages
pub struct Definition {
    /// The information in the "from" language
    pub from: LanguageDefinition,
    /// The information in the "to" language
    pub to: LanguageDefinition,
}

#[derive(Debug)]
/// Struct representing a result from wordreference.com
pub struct Response {
    /// Vector of all definitions from webpage
    pub definitions: Vec<Definition>,
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
fn parse_html(html: String) -> Result<Response, String> {
    let document: Html = Html::parse_document(html.as_str());
    validate_word(&document)?;
    let table_selector: Selector = Selector::parse(r#"table[class="WRD noTapHighlight"]"#)
        .expect("Creating table selector failed");
    let row_selector: Selector =
        Selector::parse(r#"tr[class="even"],[class="odd"]"#).expect("Creating row selector failed");

    let tables = document.select(&table_selector);

    let mut entries: Vec<Definition> = Vec::new();
    let mut entry: Vec<String> = Vec::new();

    for table in tables.take(2) {
        // first row is always even
        let mut last_row_even: bool = true;
        let is_even_re = Regex::new(r#"class="even""#).unwrap();
        for row in table.select(&row_selector) {
            let text = &row.html();
            let is_even = is_even_re.is_match(text);

            let row_type_matches_last: bool = last_row_even == is_even;
            if row_type_matches_last {
                entry.push(text.to_owned());
            } else {
                entries.push(parse_entry(&entry));
                entry = vec![text.to_owned()];
            }

            // update last var
            last_row_even = is_even;
        }
        if !entry.is_empty() {
            entries.push(parse_entry(&entry));
        }
    }
    Ok(Response {
        definitions: entries,
    })
}

/// Takes a word and returns a [`Response`] struct
pub async fn get_def(
    word: String,
    from: Option<String>,
    to: Option<String>,
) -> Result<Response, String> {
    let html = get_html(word, from, to).await.unwrap();
    parse_html(html)
}

fn validate_word(document: &Html) -> Result<bool, String> {
    let validation_selector =
        Selector::parse(r#"p[id="noEntryFound"]"#).expect("Creating noEntryFound selector failed");
    let is_found: bool = document.select(&validation_selector).next().is_some();
    match is_found {
        true => Err("Word not found".to_string()),
        false => Ok(true),
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_delantal() -> Result<(), String> {
        match crate::get_def("delantal".to_string(), None, None).await {
            Ok(res) => {
                assert_eq!(res.definitions.len(), 3);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    #[tokio::test]
    async fn test_entregar() -> Result<(), String> {
        match crate::get_def("entregar".to_string(), None, None).await {
            Ok(res) => {
                assert_eq!(res.definitions.len(), 14);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    #[tokio::test]
    async fn test_nuevo() -> Result<(), String> {
        match crate::get_def("nuevo".to_string(), None, None).await {
            Ok(res) => {
                assert_eq!(res.definitions.len(), 7);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    #[tokio::test]
    async fn test_palabra() -> Result<(), String> {
        match crate::get_def("palabra".to_string(), None, None).await {
            Ok(res) => {
                assert_eq!(res.definitions.len(), 4);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    #[tokio::test]
    async fn test_invalid_word() -> Result<(), String> {
        match crate::get_def("sjfadohjfahndkllhjra".to_string(), None, None).await {
            Ok(_) => Err("Expected error".to_string()),
            Err(_) => Ok(()),
        }
    }

    #[tokio::test]
    #[ignore]
    async fn english_to_spanish() -> Result<(), String> {
        match crate::get_def("brick".to_string(), None, None).await {
            Ok(res) => {
                assert_eq!(res.definitions.len(), 3);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
