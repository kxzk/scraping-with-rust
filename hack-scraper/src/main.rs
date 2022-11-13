extern crate reqwest;
#[macro_use] extern crate prettytable;
use prettytable::Table;
use scraper::{Html, Selector};

fn get_hacker_news_data() -> Result<String, Box<dyn std::error::Error>> {
    let hn_txt = reqwest::blocking::get("https://news.ycombinator.com/")?
        .text()?;

    Ok(hn_txt)
}

fn main() {
    let hn_txt = get_hacker_news_data().unwrap();

    let document = Html::parse_document(&hn_txt);

    let stories = Selector::parse("td:nth-child(3) > span > a").unwrap();

    let mut table = Table::new();

    for story in document.select(&stories) {
        let story_link = story.value().attr("href").unwrap();
        let story_txt = story.text().collect::<Vec<_>>();

        if story_txt[0] == "login" {
            continue;
        }

        table.add_row(row![FdBybl->story_txt[0]]);
        table.add_row(row![Fy->story_link]);
    }

    table.printstd();
}
