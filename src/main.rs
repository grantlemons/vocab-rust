//! Retrieves definitions etc. from wordreference api and assembles them into a table
#![warn(missing_docs)]

#[allow(unused_imports)]
use wordreference as wr;

/// Entrypoint for binary program
#[tokio::main]
async fn main() -> Result<(), ()> {
    println!(
        "{:#?}",
        wr::get_def("inutil".to_string(), None, None).await?
    );
    Ok(())
}
