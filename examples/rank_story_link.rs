extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name, Predicate};

fn main() {
    hacker_news("https://news.ycombinator.com");
}

fn hacker_news(url: &str) {

    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();

    for node in document.find(Class("athing")) {
        let rank = node.find(Class("rank")).next().unwrap();
        let story = node.find(Class("title").descendant(Name("a")))
            .next()
            .unwrap()
            .text();
        println!("\n | {} | {}\n", rank.text(), story);
        let url = node.find(Class("title").descendant(Name("a"))).next().unwrap();
        println!("{:?}\n", url.attr("href").unwrap());
    }
}
