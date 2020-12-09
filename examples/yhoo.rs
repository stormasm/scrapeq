use scrapeq::yhoo::hacker_news;

fn main() {
    match hacker_news() {
        Ok(site) => println!("{}", site),
        Err(_) => eprintln!("Error fetching Hacker News data."),
    }
}
