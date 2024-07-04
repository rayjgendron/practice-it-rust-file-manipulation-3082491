use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file: File = File::open("file_with_lines").unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines(){
        println!("{}", line.unwrap());
    }
}
