use std::fs;

fn main() {
    let file_path = "no_file";
    
    let contents = read_file(&file_path).expect(&format!("unable to read file <{}>", file_path));
    println!("{}", contents);
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path) 
 }
