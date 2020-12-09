use scrapeq::yhoo::quote;

fn main() {
    match quote() {
        Ok(site) => println!("{}", site),
        Err(_) => eprintln!("Error fetching Hacker News data."),
    }
}
