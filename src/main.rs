use std::env;
use reqwest::blocking;
use scraper::{Html, Selector};

fn main() {
    let args: Vec<String> = env::args().collect();

    let url = &args[1];
    let response = blocking::get(url)
        .unwrap()
        .text()
        .unwrap();

    let document = Html::parse_document(&response);
    let title_selector = Selector::parse("title").unwrap();
    let title = document.select(&title_selector).next().unwrap();
    let text = title.text().collect::<Vec<_>>().join("");

    println!("title: {}", text.trim());
}
