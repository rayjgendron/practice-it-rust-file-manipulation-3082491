use std::fs;

fn main() {
    let file_path = "file_with_lines";

    let lines = read_file(&file_path).expect(&format!("Unable to read file <{}>", &file_path));

    println!("{:?}", lines);
}

fn read_file(path: &str) -> Result<Vec<Vec<String>>, std::io::Error> {
    let contents: String = fs::read_to_string(path)?;
    let mut outer = Vec::new();
    for line in contents.lines() {
        let mut inner = Vec::new();
        for word in line.split_whitespace() {
            inner.push(word.to_string());
        }
        outer.push(inner);
    }
    Ok(outer)
}