//https://adventofcode.com/2021/day/2
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_data(filename: impl AsRef<Path>) -> Vec<(String, u32)> {
    let file = File::open(filename).expect("file err");
    let buf = BufReader::new(file);
    buf.lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut words = line.split_ascii_whitespace();
            (
                String::from(words.next().unwrap()),
                words.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(String, u32)>>()
}

fn part2(tasks: &[(String, u32)]) -> u32 {
    let mut forward = 0;
    let mut deep = 0;
    let mut aim = 0;
    for (command, number) in tasks {
        if command.eq("forward") {
            forward += number;
            deep += number * aim;
        } else if command.eq("up") {
            aim -= number;
        } else if command.eq("down") {
            aim += number;
        }
    }
    forward * deep
}

fn part1(tasks: &[(String, u32)]) -> u32 {
    let mut forward = 0;
    let mut deep = 0;
    for (command, number) in tasks {
        if command.eq("forward") {
            forward += number;
        } else if command.eq("up") {
            deep -= number;
        } else if command.eq("down") {
            deep += number;
        }
    }
    forward * deep
}

fn main() {
    let tasks = read_data("./input/input2.txt");
    println!("Part 1: {}", part1(&tasks));
    println!("Part 2: {}", part2(&tasks));
}

#[test]
fn test_part1() {
    let tasks = read_data("./input/input2_test.txt");
    assert_eq!(part1(&tasks), 150);
}
#[test]
fn test_part2() {
    let tasks = read_data("./input/input2_test.txt");
    assert_eq!(part2(&tasks), 900);
}
