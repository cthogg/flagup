use serde::__private::de::IdentifierDeserializer;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
#[derive(Serialize, Deserialize, Debug)]
struct Data {
    emoji: String,
    name: String,
}

pub struct Config {
    pub country_text: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
        Config {
            country_text: args[1].clone(),
        }
    }
}

fn read_json_file() -> Vec<Data> {
    let current_dir = std::env::current_dir().unwrap().display().to_string();
    let path = format!("{}/data.json", current_dir);
    let data = fs::read_to_string(path).expect("Unable to read file");
    let deserialized: Vec<Data> = serde_json::from_str(data.as_str()).unwrap();
    return deserialized;
}

pub fn generate_result(args: Config) -> &'static str {
    let json = read_json_file();
    println!("{}", serde_json::to_string(&json).unwrap());
    let test = args.country_text.clone();
    let hashmap_of_tuples: Vec<Data> = json.into_iter().filter(|x| x.name == test).collect();

    //FIXME: how to remove this assertion as below
    let flag = hashmap_of_tuples.first().unwrap().emoji.clone().as_str();
    return flag;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_germany_creates_correct_flag() {
        assert_eq!(
            generate_result(Config::new(vec![
                "flagup".to_string(),
                "germany".to_string()
            ])),
            "🇩🇪"
        );
    }
    #[test]
    fn test_france_creates_correct_flag() {
        assert_eq!(
            generate_result(Config::new(vec![
                "flagup".to_string(),
                "france".to_string()
            ])),
            "🇫🇷"
        );
    }
    #[test]
    fn test_not_a_country_returns_a_shrug() {
        assert_eq!(
            generate_result(Config::new(vec![
                "flagup".to_string(),
                "not_a_country".to_string()
            ])),
            "🤷‍♂️"
        );
    }
}
