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
    let mut last_number = u32::MAX;
    if let Ok(lines) = read_lines("./../input.txt") {
        for line in lines.flatten() {
            let number = line.parse::<u32>().unwrap();
            if number > last_number {
                counter += 1;
            }
            last_number = number;
        }
    }
    println!("{}", counter);
}
