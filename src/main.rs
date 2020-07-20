extern crate reqwest;
extern crate soup;
extern crate textwrap;
#[macro_use]
extern crate prettytable;

use prettytable::Table;
use soup::prelude::*;

/// Line wrap width for quotes
const QUOTE_WRAP_LENGTH: usize = 72;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("http://quotes.toscrape.com/")
        .await?
        .text()
        .await?;
    let soup = Soup::new(&body);
    let quotes = soup.class("text").find_all();
    let authors = soup.class("author").find_all();

    // zip together quotes and authors
    let records = quotes.zip(authors);

    // Create the table
    let mut table = Table::new();
    // Add header row
    table.set_titles(row!["Quote", "Author"]);

    for record in records {
        // wrap quote text
        table.add_row(row![
            textwrap::fill(&record.0.text(), QUOTE_WRAP_LENGTH),
            record.1.text()
        ]);
    }

    // print table
    table.printstd();

    Ok(())
}
