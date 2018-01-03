extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;

fn main() {
    hacker_news();
}

fn hacker_news() {

    let mut resp = reqwest::get("https://news.ycombinator.com").unwrap();
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

}
