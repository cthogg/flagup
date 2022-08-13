use clap::Parser;
use flagup_cli::{Args, Config};
use libflagup::generate_flag_from_result;
fn main() {
    let args = Args::parse();
    let args2 = Config::new(args);
    println!("{}", generate_flag_from_result(args2.country_text));
}
