mod module;

use reqwest::{Error, blocking::get};
use scraper::Html;
use module::{get_text, get_urls};

fn main() -> Result<(), Error> {
    let response = get("https://weather.naver.com/today/05110101")?.text()?;
    let document = Html::parse_document(&response);
    let title = get_text(&document, "div#now h2").unwrap();
    let location = get_text(&document, "div#now strong.location_name").unwrap();
    let links = get_urls(&document, "a").unwrap();
    println!("{}, {}", title, location);
    println!("{:?}", links);
    Ok(())
}
