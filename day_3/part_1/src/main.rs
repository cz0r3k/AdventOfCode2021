use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let mut char_counter = vec![0; 12];
    let lines = lines_from_file("./../input.txt");
    let counter = lines.len() / 2;
    for line in lines {
        line.chars().enumerate().for_each(|(i, ch)| {
            if ch == '1' {
                char_counter[i] += 1;
            }
        });
    }
    let gamma_string: String = char_counter
        .iter()
        .map(|&x| (x / counter).to_string())
        .collect();
    let gamma = usize::from_str_radix(&gamma_string, 2).unwrap();
    let mask = 0b1111_1111_1111usize;
    let epsilon = gamma ^ mask;
    println!("gamma = {}", gamma);
    println!("epsilon = {}", epsilon);
    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);
}
