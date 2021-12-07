//https://adventofcode.com/2021/day/3
#![feature(drain_filter)]
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_data(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>()
}

fn count_1(data: &[String], n: usize) -> usize {
    data.iter()
        .filter(|&x| (*x).chars().nth(n).unwrap() == '1')
        .count()
}
fn return_char(one: usize, all: usize) -> char {
    if one * 2 / all > 0 {
        '1'
    } else {
        '0'
    }
}
fn diffrent_char(ch: char) -> char {
    if ch == '1' {
        '0'
    } else {
        '1'
    }
}
fn filter(data: &mut Vec<String>, index: usize, ch: char) -> Vec<String> {
    data.drain_filter(|word| (*word).chars().nth(index) == Some(ch))
        .collect::<Vec<String>>()
}

fn get_string(data: Vec<String>, f: fn(char) -> char) -> String {
    let mut data_copy = data.clone();
    for i in 0..data.len() {
        let ch = f(return_char(count_1(&data_copy, i), data_copy.len()));
        data_copy = filter(&mut data_copy, i, ch);
        if data_copy.len() == 1 {
            break;
        }
    }
    data_copy.first().unwrap().clone()
}

fn part1(data: &[String]) -> usize {
    let len = data[0].len();
    let gamma_string = (0..len)
        .map(|i| return_char(count_1(data, i), data.len()))
        .collect::<String>();
    let gamma = usize::from_str_radix(&gamma_string, 2).unwrap();
    let mask = usize::from_str_radix(&vec!["1"; len].concat(), 2).unwrap();
    gamma * (mask ^ gamma)
}

fn part2(data: Vec<String>) -> usize {
    let oxygen_string = get_string(data.clone(), |ch| ch);
    let carbon_string = get_string(data, diffrent_char);
    let oxygen = usize::from_str_radix(&oxygen_string, 2).unwrap();
    let carbon = usize::from_str_radix(&carbon_string, 2).unwrap();
    carbon * oxygen
}

fn main() {
    let data = read_data("./input/input3.txt");
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(data));
}

#[test]
fn test_part1() {
    let data = read_data("./input/input3_test.txt");
    assert_eq!(part1(&data), 198);
}

#[test]
fn test_part2() {
    let data = read_data("./input/input3_test.txt");
    assert_eq!(part2(data), 230);
}
