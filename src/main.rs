use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let url = &args[1];
    let response = reqwest::blocking::get(url,)
        .unwrap()
        .text()
        .unwrap();
    
    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("title").unwrap();
    let title = document.select(&title_selector).next().unwrap();
    let text = title.text().collect::<Vec<_>>();

    println!("title: {}", text[0]);
}
