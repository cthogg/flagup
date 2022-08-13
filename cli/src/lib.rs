use clap::Parser;
/// Simple program to show a flag of a country
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Country to search for
    #[clap(value_parser)]
    country: String,
}

pub struct Config {
    pub country_text: String,
}

impl Config {
    pub fn new(args: Args) -> Self {
        Config {
            country_text: args.country.clone(),
        }
    }
}
