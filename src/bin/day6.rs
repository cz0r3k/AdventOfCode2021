//https://adventofcode.com/2021/day/6
#![feature(drain_filter)]
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_data(filename: impl AsRef<Path>) -> Vec<u16> {
    let file = File::open(filename).expect("file err");
    let buf = BufReader::new(file);
    buf.lines()
        .find_map(|line| line.ok())
        .unwrap()
        .split(',')
        .filter_map(|num| num.parse::<u16>().ok())
        .collect::<Vec<u16>>()
}
fn group_numbers(data: &mut Vec<u16>) -> Vec<u64> {
    (0..=8)
        .map(|index| data.drain_filter(|x| *x == index as u16).count() as u64)
        .collect::<Vec<u64>>()
}

fn part(mut numbers: Vec<u16>, days: usize) -> u64 {
    let mut numbers: Vec<u64> = group_numbers(&mut numbers);
    for _ in 0..days {
        numbers.rotate_left(1);
        numbers[6] += numbers[8];
    }
    numbers.iter().sum::<u64>()
}

fn main() {
    let numbers = read_data("./input/input6.txt");
    println!("Part 1: {}", part(numbers.clone(), 80));
    println!("Part 2: {}", part(numbers, 256));
}

#[test]
fn test_18_days() {
    let numbers = read_data("./input/input6_test.txt");
    assert_eq!(part(numbers, 18), 26);
}
#[test]
fn test_80_days() {
    let numbers = read_data("./input/input6_test.txt");
    assert_eq!(part(numbers, 80), 5934);
}
