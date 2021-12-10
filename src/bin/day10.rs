//https://adventofcode.com/2021/day/10
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_data(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("file err");
    let buf = BufReader::new(file);
    buf.lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>()
}

fn check_open_bracket(bracket: char) -> bool {
    bracket == '<' || bracket == '{' || bracket == '(' || bracket == '['
}

fn check_close_bracket(last_braket: char, bracket: char) -> bool {
    (last_braket == '<' && bracket == '>')
        || (last_braket == '{' && bracket == '}')
        || (last_braket == '(' && bracket == ')')
        || (last_braket == '[' && bracket == ']')
}

fn check_points_close_bracket(bracket: char) -> u32 {
    if bracket == ')' {
        3
    } else if bracket == ']' {
        57
    } else if bracket == '}' {
        1197
    } else {
        25137
    }
}

fn check_points_open_bracket(bracket: char) -> u64 {
    if bracket == '(' {
        1
    } else if bracket == '[' {
        2
    } else if bracket == '{' {
        3
    } else {
        4
    }
}

fn increase_sum(sum: &mut u64, stack: &mut Vec<char>) {
    *sum *= 5;
    *sum += check_points_open_bracket(*stack.last().unwrap());
    stack.pop();
}

fn part1(lines: &[String]) -> u32 {
    let mut sum = 0;
    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        for bracket in line.chars() {
            if check_open_bracket(bracket) {
                stack.push(bracket);
            } else if check_close_bracket(*stack.last().unwrap(), bracket) {
                stack.pop();
            } else {
                sum += check_points_close_bracket(bracket);
                break;
            }
        }
    }
    sum
}

fn part2(lines: &[String]) -> u64 {
    let mut points = Vec::new();
    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        let mut incomplete_flag = false;
        for bracket in line.chars() {
            if check_open_bracket(bracket) {
                stack.push(bracket);
            } else if check_close_bracket(*stack.last().unwrap(), bracket) {
                stack.pop();
            } else {
                incomplete_flag = true;
                break;
            }
        }
        if !incomplete_flag {
            let mut sum = 0;
            while !stack.is_empty() {
                increase_sum(&mut sum, &mut stack);
            }
            points.push(sum);
        }
    }
    points.sort_unstable();
    points[points.len() / 2]
}
fn main() {
    let brackets = read_data("./input/input10.txt");
    println!("Part 1: {}", part1(&brackets));
    println!("Part 2: {}", part2(&brackets));
}

#[test]
fn test_part1() {
    let brackets = read_data("./input/input10_test.txt");
    assert_eq!(part1(&brackets), 26397);
}
#[test]
fn test_part2() {
    let brackets = read_data("./input/input10_test.txt");
    assert_eq!(part2(&brackets), 288957);
}
