use std::fs;

fn main() {
    let file_path = "test_file";

    let contents = read_file(file_path);
    println!("{}", contents);
}

fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(contents) => contents,
        _ => "".to_string(),

    }
}