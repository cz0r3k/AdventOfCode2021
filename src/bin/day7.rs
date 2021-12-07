//https://adventofcode.com/2021/day/7
#![feature(drain_filter)]
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_data(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("file err");
    let buf = BufReader::new(file);
    buf.lines()
        .find_map(|line| line.ok())
        .unwrap()
        .split(',')
        .filter_map(|num| num.parse::<i32>().ok())
        .collect::<Vec<i32>>()
}
fn sum_of_the_sequence(n: i32) -> i32 {
    (1 + n) * n / 2
}

fn part(numbers: &[i32], f: fn(i32) -> i32) -> i32 {
    let max = numbers.iter().copied().max().unwrap();
    let mut last = i32::MAX;
    for i in 0..max {
        let sum = numbers.iter().map(|&x| f(i32::abs(x - i))).sum::<i32>();
        if sum > last {
            break;
        }
        last = sum;
    }
    last
}

fn part1(numbers: &[i32]) -> i32 {
    part(numbers, |x| x)
}
fn part2(numbers: &[i32]) -> i32 {
    part(numbers, sum_of_the_sequence)
}

fn main() {
    let numbers = read_data("./input/input7.txt");
    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
}

#[test]
fn test_part1() {
    let numbers = read_data("./input/input7_test.txt");
    assert_eq!(part1(&numbers), 37);
}
#[test]
fn test_part2() {
    let numbers = read_data("./input/input7_test.txt");
    assert_eq!(part2(&numbers), 168);
}
