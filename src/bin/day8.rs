//https://adventofcode.com/2021/day/8
#![feature(drain_filter)]
use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
#[derive(Debug, Clone, Copy)]
struct Digit {
    a: char,
    b: char,
    c: char,
    d: char,
    e: char,
    f: char,
    g: char,
}

fn read_data(filename: impl AsRef<Path>) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let file = File::open(filename).expect("file err");
    let buf = BufReader::new(file);
    buf.lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let signals = line.split('|').map(String::from).collect::<Vec<String>>();
            let sig_in = signals
                .get(0)
                .unwrap()
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>();
            let sig_out = signals
                .get(1)
                .unwrap()
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>();
            (sig_in, sig_out)
        })
        .collect::<Vec<(Vec<String>, Vec<String>)>>()
        .iter()
        .cloned()
        .unzip()
}

fn check_easy_digit(signal: &str) -> bool {
    signal.len() == 2 || signal.len() == 3 || signal.len() == 4 || signal.len() == 7
}

fn get_chars(sig: &[String], n: usize) -> HashSet<char> {
    sig.iter()
        .find(|&x| x.len() == n)
        .unwrap()
        .chars()
        .collect::<HashSet<char>>()
}

fn get_all_chars(sig: &[String], n: usize) -> Vec<HashSet<char>> {
    sig.iter()
        .filter(|&x| x.len() == n)
        .map(|x| x.chars().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>()
}

fn get_g(six_size: &[HashSet<char>], abcdf: HashSet<char>) -> HashSet<char> {
    six_size
        .iter()
        .find(|&x| (x - &abcdf).len() == 1)
        .unwrap()
        .clone()
}

fn check_digit(sig_in: Vec<String>) -> (HashSet<char>, HashSet<char>, HashSet<char>) {
    let two_size = get_chars(&sig_in, 2);
    let three_size = get_chars(&sig_in, 3);
    let four_size = get_chars(&sig_in, 4);
    let seven_size = get_chars(&sig_in, 7);
    let six_size = get_all_chars(&sig_in, 6);
    let cf = two_size.clone();
    let a_char = &three_size - &two_size;
    let bd = &four_size - &two_size;
    let abcdf = &(&a_char | &bd) | &cf;
    let g_char = get_g(&six_size, abcdf.clone());
    let abcdfg = &abcdf | &g_char;
    let e_char = &seven_size - &abcdfg;
    (bd, cf, e_char)
}
fn get_number(code: &str, bd_cf_e: &(HashSet<char>, HashSet<char>, HashSet<char>)) -> usize {
    if code.len() == 2 {
        1
    } else if code.len() == 3 {
        7
    } else if code.len() == 4 {
        4
    } else if code.len() == 7 {
        8
    } else if code.len() == 6 {
        if !(&bd_cf_e.1 - &code.chars().collect::<HashSet<char>>()).is_empty() {
            6
        } else if !(&bd_cf_e.2 - &code.chars().collect::<HashSet<char>>()).is_empty() {
            9
        } else {
            0
        }
    } else if (&bd_cf_e.2 - &code.chars().collect::<HashSet<char>>()).is_empty() {
        2
    } else if !(&bd_cf_e.1 - &code.chars().collect::<HashSet<char>>()).is_empty() {
        5
    } else {
        3
    }
}

fn part1(signals: Vec<Vec<String>>) -> usize {
    signals
        .into_iter()
        .map(|line| {
            line.into_iter()
                .filter(|signal| check_easy_digit(signal))
                .count()
        })
        .sum()
}

fn part2((signals_in, signals_out): (Vec<Vec<String>>, Vec<Vec<String>>)) -> usize {
    let mut sum = 0;
    for (index, signals) in signals_in.iter().enumerate() {
        let digit = check_digit(signals.clone());
        let mut counter = 1000;
        let mut partial_sum = 0;
        for out in &signals_out[index] {
            partial_sum += counter * get_number(out, &digit);
            counter /= 10;
        }
        sum += partial_sum;
    }
    sum
}

fn main() {
    let data = read_data("./input/input8.txt");
    println!("Part 1: {}", part1(data.1.clone()));
    println!("Part 2: {}", part2(data));
}

#[test]
fn test_part1() {
    let data = read_data("./input/input8_test.txt");
    assert_eq!(part1(data.1), 26);
}
#[test]
fn test_part2() {
    let data = read_data("./input/input8_test.txt");
    assert_eq!(part2(data), 61229);
}
