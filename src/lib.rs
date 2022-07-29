use std::collections::HashMap;

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

pub fn generate_result(args: Config) -> &'static str {
    let test = args.country_text.clone();
    let tuples = [("germany", "🇩🇪"), ("france", "🇫🇷"), ("the_gambia", "🇬🇲")];
    let hashmap_of_tuples: HashMap<&str, &str> = tuples.into_iter().collect();

    //FIXME: how to remove this assertion of as below
    let flag = match hashmap_of_tuples.get(&test.to_lowercase() as &str) {
        Some(&str) => &str,
        None => "🤷‍♂️",
    };
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

// run the rust project on homebrew
// https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/
