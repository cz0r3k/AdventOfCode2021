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
    let lines = lines_from_file("./../input.txt");
    let counter = lines.len() / 2;
    let mut char_counter = vec![0; lines[0].len()];
    char_counter
        .iter_mut()
        .enumerate()
        .for_each(|(i, counter)| {
            *counter = lines
                .iter()
                .filter(|&ch| (*ch).chars().nth(i) == Some('1'))
                .count();
        });
    let gamma_string: String = char_counter
        .iter()
        .map(|&x| (x / counter).to_string())
        .collect();
    let gamma = usize::from_str_radix(&gamma_string, 2).unwrap();
    let mask = 0b1111_1111_1111;
    let epsilon = gamma ^ mask;
    println!("gamma = {}", gamma);
    println!("epsilon = {}", epsilon);
    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);
}
