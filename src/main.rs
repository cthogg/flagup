use flagup::{generate_result, read_json_file, Config};
fn main() {
    let args = Config::new(std::env::args().collect());
    println!("{}", generate_result(args, read_json_file()));
}
