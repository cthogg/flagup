use flag_creator::{generate_result, Config};
fn main() {
    let args = Config::new(std::env::args());
    generate_result(args);
}
