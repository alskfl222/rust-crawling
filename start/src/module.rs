use std::result::Result;
use regex::Regex;
use reqwest::Error;
use scraper::{Html, Selector};

pub fn get_text(document: &Html, selector: &str) -> Result<String, Error> {
  let selector_parse = Selector::parse(&selector).unwrap();
  let elements = document.select(&selector_parse).collect::<Vec<_>>();
  // println!("{} elements", elements.iter().cloned().count());
  let title = elements[0].text().collect::<String>();
  Ok(title)
}

// pub fn get_urls(document: &Html, selector: &str) -> Result<Vec<String>, Error> {
//   let selector_parse = Selector::parse(&selector).unwrap();
//   let elements = document.select(&selector_parse).collect::<Vec<_>>();
//   let mut contents = Vec::new();
//   for element in &elements {
//       let string = element.value().attr("href").unwrap().to_string();
//       if string.chars().next() != Some('#') {
//         contents.push(string);
//       }
//   };
//   Ok(contents)
// }

pub fn parse_temp(string: &String) -> Result<f32, Error> {
  let re = Regex::new(r"\d+\.\d+|\d+").unwrap();
  let captures = re.find_iter(string);
  let numbers: Vec<f32> = captures.map(|capture| capture.as_str().parse().unwrap()).collect();
  Ok(numbers[0])
}