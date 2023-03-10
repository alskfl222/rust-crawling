mod module;

use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use chrono::prelude::Utc;
use reqwest::{Error, blocking::get};
use scraper::Html;
use module::{get_text, get_urls};

fn main() -> Result<(), Error> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        let now = Utc::now();
        println!("{}", now);
        let response = get("https://weather.naver.com/today/05110101")?.text()?;
        let document = Html::parse_document(&response);
        let title = get_text(&document, "div#now h2").unwrap();
        let location = get_text(&document, "div#now strong.location_name").unwrap();
        let links = get_urls(&document, "a").unwrap();
        println!("{}, {}", title, location);
        println!("{:?}", links);
        println!();
        thread::sleep(Duration::from_secs(1));
    }
    
    println!();
    println!("EXIT LOOP");

    Ok(())
}
