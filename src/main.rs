//! Retrieves definitions etc. from wordreference api and assembles them into a table
#![warn(missing_docs)]

use wordreference as wr;

/// Entrypoint for binary program
#[tokio::main]
async fn main() -> Result<(), ()> {
    let word = "tiene";
    let response = wr::get_def(word.to_string(), None, None).await?;

    println!("{:#?}", response);
    Ok(())
}
