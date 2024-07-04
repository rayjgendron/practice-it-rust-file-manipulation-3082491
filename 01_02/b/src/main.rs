use std::fs;

fn main() {
    let contents: String = fs::read_to_string("test_file").expect("can't open stupid file!");
    println!("{}", contents);
}