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
    let mut forward = 0;
    let mut deep = 0;
    if let Ok(lines) = read_lines("./../input.txt") {
        for line in lines.flatten() {
            let mut iter = line.split_whitespace();
            let command = iter.next().unwrap();
            let number = iter.next().unwrap().parse::<u32>().unwrap();
            if command.eq("forward") {
                forward += number;
            } else if command.eq("up") {
                deep -= number;
            } else if command.eq("down") {
                deep += number;
            }
        }
    }
    println!("{}", forward * deep);
}
