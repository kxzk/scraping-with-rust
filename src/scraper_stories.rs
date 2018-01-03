extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

fn main() {
    hn_headlines();
}

fn hn_headlines() {

   let mut resp = reqwest::get("https://news.ycombinator.com").unwrap(); 
   assert!(resp.status().is_success());
   let body = resp.text().unwrap();
   let fragment = Html::parse_document(&body);
   let stories = Selector::parse(".storylink").unwrap();

   for story in fragment.select(&stories) {
        let story_txt = story.text().collect::<Vec<_>>();
        println!("{:?}", story_txt);
    }
}
