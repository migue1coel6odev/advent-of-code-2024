use std::fs;

pub fn read_file_as_string(path: &str) -> String {
    String::from_utf8(fs::read(path).unwrap()).unwrap()
}

pub fn read_file_by_line(path: &str) -> Vec<String> {
    read_file_as_string(path)
        .split("\n")
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
}

pub fn read_file_as_char_matrix(path: &str) -> Vec<Vec<char>> {
    read_file_as_string(path)
        .split("\n")
        .map(|f| f.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
