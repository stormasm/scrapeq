mod sites;

fn main() {
    match sites::hacker_news() {
        Ok(site) => println!("{}", site),
        Err(_) => eprintln!("Error fetching Hacker News data."),
    }
}
