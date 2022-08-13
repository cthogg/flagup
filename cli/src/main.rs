use clap::Parser;
use flagup_cli::{Args, Config};
use libflagup::generate_flag_from_country;
fn main() {
    let args = Args::parse();
    let args2 = Config::new(args);
    println!("{}", generate_flag_from_country(args2.country_text));
}
