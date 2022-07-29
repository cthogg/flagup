use flagup::{generate_result, Config};
fn main() {
    let args = Config::new(std::env::args().collect());
    println!("{}", generate_result(args));
}
