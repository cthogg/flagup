use console::Emoji;
use std::{collections::HashMap, env::Args};
fn main() {
    let args = Config::new(std::env::args());
    let test = args.country_text.clone();
    let mut countries_and_emojis: HashMap<&str, Emoji> = HashMap::new();
    countries_and_emojis.insert("Germany", Emoji::new("🇩🇪", "German flag"));
    countries_and_emojis.insert("France", Emoji::new("🇫🇷", "French flag"));

    let flag = match countries_and_emojis.get(&test as &str) {
        //is casting allowed? I would assume it is just as bad as in typescript?
        Some(&emoji) => emoji,
        None => Emoji::new("🤷‍♂️", "No emoji"),
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
