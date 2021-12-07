//https://adventofcode.com/2021/day/1
use std::{
    collections::VecDeque,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_data(filename: impl AsRef<Path>) -> Vec<u32> {
    let file = File::open(filename).expect("file err");
    let buf = BufReader::new(file);
    buf.lines()
        .filter_map(|line| line.ok())
        .filter_map(|num| num.parse::<u32>().ok())
        .collect::<Vec<u32>>()
}
fn count(numbers: &[u32], n: usize) -> usize {
    let mut last_numbers = VecDeque::from(vec![u32::MAX / n as u32; n]);
    let mut last_sum = last_numbers.iter().sum::<u32>();
    let mut counter = 0;
    for number in numbers {
        last_numbers.pop_front();
        last_numbers.push_back(*number);
        let sum = last_numbers.iter().sum();
        if sum > last_sum {
            counter += 1;
        }
        last_sum = sum;
    }
    counter
}

fn part1(numbers: &[u32]) -> usize {
    count(numbers, 1)
}
fn part2(numbers: &[u32]) -> usize {
    count(numbers, 3)
}

fn main() {
    let numbers = read_data("./input/input1.txt");
    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
}

#[test]
fn test_part1() {
    let numbers = read_data("./input/input1_test.txt");
    assert_eq!(part1(&numbers), 7);
}
#[test]
fn test_part2() {
    let numbers = read_data("./input/input1_test.txt");
    assert_eq!(part2(&numbers), 5);
}
