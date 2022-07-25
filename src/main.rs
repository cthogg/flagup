use std::{collections::HashMap, env::Args};
fn main() {
    let args = Config::new(std::env::args());
    let test = args.country_text.clone();
    let mut countries_and_emojis: HashMap<&str, &str> = HashMap::new();
    // let tuples: [(&str, &str)] = [("🇩🇪", "Germany"), ("🇫🇷", "France")];
    // let m: HashMap<&str, &str> = tuples.into_iter().collect();
    countries_and_emojis.insert("Germany", "🇩🇪");
    countries_and_emojis.insert("France", "🇫🇷");

    let flag = match countries_and_emojis.get(&test as &str) {
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
