pub fn display_char_matrix<A: std::fmt::Display>(matrix: Vec<Vec<A>>) {
    println!("");
    for line in matrix {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }
    println!("");
}

pub fn display_matrix<A: std::fmt::Display>(matrix: &Vec<Vec<A>>) {
    println!("");
    for line in matrix {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }
    println!("");
}
