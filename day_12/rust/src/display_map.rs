pub struct DisplayMap {}

impl DisplayMap {
    pub fn display(map: &Vec<Vec<char>>) {
        println!("");
        for line in map {
            for c in line {
                print!("{}", c);
            }
            println!();
        }
        println!("");
    }
}
