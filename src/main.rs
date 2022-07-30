use flagup::{generate_flag_from_result, Config};
fn main() {
    let args = Config::new(std::env::args().collect());
    println!("{}", generate_flag_from_result(args));
}
