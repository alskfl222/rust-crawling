use reqwest::blocking::get;
use reqwest::Error;
use scraper::{Html, Selector};

fn main() -> Result<(), Error> {
    let response = get("https://www.google.com")?.text()?;
    let document = Html::parse_document(&response);
    let selector = Selector::parse("title").unwrap();
    let title = document.select(&selector).next().unwrap().text().collect::<String>();
    println!("{}", title);
    Ok(())
}