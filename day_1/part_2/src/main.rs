use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn main() {
    let mut counter = 0;
    let mut last_sum = u32::MAX;
    let mut numbers = VecDeque::from([u32::MAX / 3, u32::MAX / 3, u32::MAX / 3]);
    if let Ok(lines) = read_lines("./../input.txt") {
        for line in lines.flatten() {
            let number = line.parse::<u32>().unwrap();
            numbers.pop_front();
            numbers.push_back(number);
            let sum = numbers.iter().sum();
            if sum > last_sum {
                counter += 1;
            }
            last_sum = sum;
        }
    }
    println!("{}", counter);
}
