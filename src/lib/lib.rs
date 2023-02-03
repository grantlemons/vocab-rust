//! Library for retrieving data from wordreference.com
#![warn(missing_docs)]

use std::str::FromStr;

use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, Clone)]
/// Struct that represents a definition in a language
pub struct LanguageDefinition {
    /// What language this struct is for
    pub language: Language,
    /// The word in question
    pub word: String,
    /// Part of speech
    pub part: String,
    /// Word's definition
    pub definition: String,
    /// An example of the word used in a sentence
    pub example: String,
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
    validate_word()?;
    let table_selector: Selector =
        Selector::parse(r#"table[class="WRD noTapHighlight"]"#).expect("Parsing 'table' failed");

    let tr_selector: Selector = Selector::parse("tr").expect("Parsing 'tr' failed");
    let td_selector: Selector = Selector::parse("td").expect("Parsing 'td' failed");
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
        .filter(|r| r.len() > 3 || r.len() == 1)
        .collect::<Vec<Vec<_>>>();
    let definitions = rows
        .chunks(3)
        .map(|r| r.iter().flatten().collect::<Vec<_>>())
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
) -> Result<Response, String> {
    let html = get_html(word, from, to).await.unwrap();
    parse_html(html)
}

fn validate_word() -> Result<bool, String> {
    match Selector::parse(r#"p[id="noEntryFound"]"#) {
        Ok(_) => Err("".to_string()),
        Err(_) => Ok(true),
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_delantal() -> Result<(), String> {
        match crate::get_def("delantal".to_string(), None, None).await {
            Ok(res) => {
                assert_eq!(res.definitions.len(), 2);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    #[tokio::test]
    async fn test_entregar() -> Result<(), String> {
        match crate::get_def("entregar".to_string(), None, None).await {
            Ok(res) => {
                assert_eq!(res.definitions.len(), 11);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    #[tokio::test]
    async fn test_nuevo() -> Result<(), String> {
        match crate::get_def("nuevo".to_string(), None, None).await {
            Ok(res) => {
                assert_eq!(res.definitions.len(), 4);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    #[tokio::test]
    async fn test_palabra() -> Result<(), String> {
        match crate::get_def("palabra".to_string(), None, None).await {
            Ok(res) => {
                assert_eq!(res.definitions.len(), 2);
                for def in res.definitions {
                    let en_example = def.to.example;
                    assert_ne!(en_example, "n");
                }
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
