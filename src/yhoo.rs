use reqwest::Error;
use scraper::{ElementRef, Html, Selector};
use std::fmt;
use std::result::Result;

#[derive(Debug)]
pub struct Hn {
    name: String,
    stories: Vec<Story>,
}

impl fmt::Display for Hn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        for story in &self.stories {
            writeln!(f, "{}", story)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Story {
    title: String,
    link: Option<String>,
}

impl fmt::Display for Story {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.link {
            Some(link) => write!(f, "\t{}\n\t\t({})", self.title, link),
            None => write!(f, "\t{}", self.title),
        }
    }
}

pub fn quote() -> Result<Hn, Error> {
    let body = get_html("https://finance.yahoo.com/quote/ibm")?;
    println!("{:?}",body);
    let stories = body
        .select(&selector())
        .map(|element| Story {
            title: parse_title(element),
            link: parse_link(element),
        })
        .collect();
    let site = Hn {
        name: "Hacker News".to_string(),
        stories,
    };

    Ok(site)
}

fn get_html(uri: &str) -> Result<Html, Error> {
    Ok(Html::parse_document(&reqwest::blocking::get(uri)?.text()?))
}

fn selector() -> Selector {
    Selector::parse("a.storylink").unwrap()
}

fn parse_link(element: ElementRef) -> Option<String> {
    let mut link: Option<String> = None;
    if let Some(link_str) = element.value().attr("href") {
        let mut link_str = link_str.to_owned();
        if link_str.starts_with("item?") {
            link_str = format!("https://news.ycombinator.com/{}", link_str);
        }
        link = Some(link_str);
    }

    link
}

fn parse_title(element: ElementRef) -> String {
    element.inner_html()
}
