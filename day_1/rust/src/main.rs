use std::{collections::HashMap, fs};

use regex::Regex;

fn main() {
    part_one();    
    part_two();
}

fn part_two(){
    let input = read_file();
    let mut first_list: Vec<usize> = vec![];
    let mut second_list: HashMap<usize,usize> = HashMap::new();

    let re = Regex::new(r"(?<first>[0-9]*) *(?<second>[0-9]*)").unwrap();
    
    input.iter().for_each(|line| {
        let caps = re.captures(line).unwrap();
        
        let first = &caps["first"].parse::<usize>().unwrap();
        let second = caps["second"].parse::<usize>().unwrap();

        first_list.push(*first);

        second_list.entry(second).and_modify(|old_value| *old_value += 1).or_insert(1);

    });
 
    let mut sum: usize = 0;

    first_list.iter().for_each(|value| {
        let second_list_value = second_list.get(&value);

        if let Some(second_value) = second_list_value {
            sum += *value * second_value;

        }
    });

    println!("Sum equals to: {}", sum);

}

fn part_one(){
    let input = read_file();
    let mut first_list: Vec<isize> = vec![];
    let mut second_list: Vec<isize> = vec![];

    let re = Regex::new(r"(?<first>[0-9]*) *(?<second>[0-9]*)").unwrap();
    
    input.iter().for_each(|line| {
        let caps = re.captures(line).unwrap();
        
        let first: &isize = &caps["first"].parse::<isize>().unwrap();
        let second: &isize = &caps["second"].parse::<isize>().unwrap();

        first_list.push(*first);
        second_list.push(*second);

    });

    first_list.sort();
    second_list.sort();

    let mut sum: isize = 0;

    first_list.iter().enumerate().for_each(|(index, value)| {
        let second_list_value = second_list[index];

        sum = sum + (*value - second_list_value).abs();

    });

    println!("Sum equals to: {}", sum);
}

fn read_file() -> Vec<String> {
    String::from_utf8(fs::read("input.txt").unwrap())
    .unwrap()
    .split("\n")
    .map(|s| s.to_string())
    .collect()
}
