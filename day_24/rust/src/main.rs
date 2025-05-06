use aoc_utils::file_read::read_file_as_splitted_by_two_new_lines;

mod wires;

fn main() {

    println!("Hello, world!");
}

fn read_file(path: &str) -> Vec<String> {
    let input  = read_file_as_splitted_by_two_new_lines(path);

    input
}

