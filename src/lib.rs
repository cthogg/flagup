use std::{collections::HashMap, env::Args};

pub struct Config {
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

pub fn generate_result(args: Config) {
    let test = args.country_text.clone();
    let tuples = [("Germany", "🇩🇪"), ("France", "🇫🇷")];
    let m: HashMap<&str, &str> = tuples.into_iter().collect();

    let flag = match m.get(&test as &str) {
        Some(&str) => &str,
        None => "🤷‍♂️",
    };

    println!("{}", flag);
}

// TODO: write a test for the generate_result function

// run the rust project on homebrew
// https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/
