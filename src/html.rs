use std::fs;

pub fn index() -> String {
    let content = fs::read_to_string("src/html/index.html").expect("Should have been able to read the file");

    return content;
}