use std::{collections::HashMap, env::Args};
fn main() {
    let args = Config::new(std::env::args());
    let test = args.country_text.clone();
    let tuples = [("Germany", "🇩🇪"), ("France", "🇫🇷")];
    let m: HashMap<&str, &str> = tuples.into_iter().collect();

    let flag = match m.get(&test as &str) {
        Some(&str) => &str,
        None => "🤷‍♂️",
    };

    println!("{}", flag);
}

struct Config {
    pub country_text: String,
}

impl Config {
    pub fn new(args: Args) -> Self {
        let ar: Vec<String> = args.collect();
        Config {
            country_text: ar[1].clone(),
        }
    }
}
