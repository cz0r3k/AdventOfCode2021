//https://adventofcode.com/2021/day/8
#![allow(clippy::type_complexity)]
use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_data(filename: impl AsRef<Path>) -> (Vec<Vec<HashSet<char>>>, Vec<Vec<HashSet<char>>>) {
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
                .map(|signal| signal.chars().collect::<HashSet<char>>())
                .collect::<Vec<HashSet<char>>>();
            let sig_out = signals
                .get(1)
                .unwrap()
                .split_whitespace()
                .map(|signal| signal.chars().collect::<HashSet<char>>())
                .collect::<Vec<HashSet<char>>>();
            (sig_in, sig_out)
        })
        .collect::<Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)>>()
        .iter()
        .cloned()
        .unzip()
}

fn check_easy_digit(signal: &HashSet<char>) -> bool {
    signal.len() == 2 || signal.len() == 3 || signal.len() == 4 || signal.len() == 7
}

fn get_chars(sig: &[HashSet<char>], n: usize) -> HashSet<char> {
    sig.iter().find(|&x| x.len() == n).unwrap().clone()
}

fn get_all_chars(sig: &[HashSet<char>], n: usize) -> Vec<HashSet<char>> {
    sig.iter()
        .filter(|&x| x.len() == n)
        .cloned()
        .collect::<Vec<HashSet<char>>>()
}

fn get_g(six_size: &[HashSet<char>], abcdf: HashSet<char>) -> HashSet<char> {
    six_size
        .iter()
        .find(|&x| (x - &abcdf).len() == 1)
        .unwrap()
        .clone()
}

fn check_digit(sig_in: Vec<HashSet<char>>) -> (HashSet<char>, HashSet<char>) {
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
    (cf, e_char)
}
fn get_easy_digit(code: &HashSet<char>) -> usize {
    if code.len() == 2 {
        1
    } else if code.len() == 3 {
        7
    } else if code.len() == 4 {
        4
    } else {
        8
    }
}
fn get_number(code: &HashSet<char>, cf_e: (&HashSet<char>, &HashSet<char>)) -> usize {
    if check_easy_digit(code) {
        get_easy_digit(code)
    } else if code.len() == 6 {
        get_six_size_digit(code, cf_e.0, cf_e.1)
    } else {
        get_five_size_digit(code, cf_e.0, cf_e.1)
    }
}

fn get_six_size_digit(code: &HashSet<char>, cf: &HashSet<char>, e: &HashSet<char>) -> usize {
    if !(cf - code).is_empty() {
        6
    } else if !(e - code).is_empty() {
        9
    } else {
        0
    }
}

fn get_five_size_digit(code: &HashSet<char>, cf: &HashSet<char>, e: &HashSet<char>) -> usize {
    if (e - code).is_empty() {
        2
    } else if !(cf - code).is_empty() {
        5
    } else {
        3
    }
}

fn part1(signals: &[Vec<HashSet<char>>]) -> usize {
    signals
        .iter()
        .map(|line| {
            line.iter()
                .filter(|&signal| check_easy_digit(signal))
                .count()
        })
        .sum()
}

fn part2(signals_in: &[Vec<HashSet<char>>], signals_out: &[Vec<HashSet<char>>]) -> usize {
    let mut sum = 0;
    for (index, signals) in signals_in.iter().enumerate() {
        let digit = check_digit(signals.clone());
        let mut counter = 1000;
        let mut partial_sum = 0;
        for out in &signals_out[index] {
            partial_sum += counter * get_number(out, (&digit.0, &digit.1));
            counter /= 10;
        }
        sum += partial_sum;
    }
    sum
}

fn main() {
    let (sig_in, sig_out) = read_data("./input/input8.txt");
    println!("Part 1: {}", part1(&sig_out));
    println!("Part 2: {}", part2(&sig_in, &sig_out));
}

#[test]
fn test_part1() {
    let (_, sig_out) = read_data("./input/input8_test.txt");
    assert_eq!(part1(&sig_out), 26);
}
#[test]
fn test_part2() {
    let (sig_in, sig_out) = read_data("./input/input8_test.txt");
    assert_eq!(part2(&sig_in, &sig_out), 61229);
}
