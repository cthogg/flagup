use console::Emoji;
use std::collections::HashMap;
fn main() {
    let mut countries_and_emojis: HashMap<&str, Emoji> = HashMap::new();
    countries_and_emojis.insert("Germany", Emoji::new("🇩🇪", "German flag"));
    countries_and_emojis.insert("France", Emoji::new("🇫🇷", "French flag"));

    let me = match countries_and_emojis.get("Gdermany") {
        Some(&emoji) => emoji,
        None => Emoji::new("🤷‍♂️", "No emoji"),
    };

    println!("{}", me);

    println!("Hello, world!");
}
