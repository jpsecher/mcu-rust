use minigrep::add;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("{}", add(1, 2));
    println!("Searching for {} in {}", query, file_path);
}
