use clap::Parser;
use flagup::{generate_flag_from_result, Args, Config};

fn main() {
    let args = Args::parse();
    let args2 = Config::new(args);
    println!("{}", generate_flag_from_result(args2));
}
