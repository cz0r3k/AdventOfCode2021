#![feature(drain_filter)]
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
    let mut oxygen = lines.clone();
    let mut carbon = lines.clone();

    for i in 0..lines[0].len() {
        let digit: char = if (oxygen
            .iter()
            .filter(|&line| (*line).chars().nth(i) == Some('1'))
            .count()
            * 2
            / oxygen.len())
            > 0
        {
            '1'
        } else {
            '0'
        };
        oxygen = oxygen
            .drain_filter(|line| (*line).chars().nth(i) == Some(digit))
            .collect::<Vec<String>>();
        if oxygen.len() == 1 {
            break;
        }
    }
    for i in 0..lines[0].len() {
        let digit = if (carbon
            .iter()
            .filter(|&line| (*line).chars().nth(i) == Some('1'))
            .count()
            * 2
            / carbon.len())
            > 0
        {
            '1'
        } else {
            '0'
        };
        carbon = carbon
            .drain_filter(|line| (*line).chars().nth(i) != Some(digit))
            .collect::<Vec<String>>();
        if carbon.len() == 1 {
            break;
        }
    }
    let o2 = usize::from_str_radix(&oxygen[0], 2).unwrap();
    let co2 = usize::from_str_radix(&carbon[0], 2).unwrap();
    println!("{} --> {}", oxygen[0], o2);
    println!("{} --> {}", carbon[0], co2);
    println!("{} * {} = {}", o2, co2, o2 * co2);
}
