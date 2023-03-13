mod module;

use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use chrono::{DateTime, Utc};
use reqwest::{Error, blocking::get};
use scraper::Html;
use module::{get_text, parse_temp};

#[derive(Debug)]
struct Weather {
    time: DateTime<Utc>,
    location: String,
    temperature: f32,
    status: String
}

fn main() -> Result<(), Error> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        let response = get("https://weather.naver.com/today/05110101")?.text()?;
        let document = Html::parse_document(&response);
        let now = Utc::now();
        let location = get_text(&document, "div#now strong.location_name").unwrap();
        let temp_text = get_text(&document, "div#now strong.current").unwrap();
        let temperature = parse_temp(&temp_text).unwrap();
        let status = get_text(&document, "div#now span.weather").unwrap();
        let info = Weather {
            time: now,
            location,
            temperature,
            status
        };
        println!("{:?}", info);
        println!();
        thread::sleep(Duration::from_secs(1));
    }
    
    println!();
    println!("EXIT LOOP");

    Ok(())
}
