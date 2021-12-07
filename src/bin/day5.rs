//https://adventofcode.com/2021/day/5
#![feature(drain_filter)]
use std::{
    cmp,
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_data(filename: impl AsRef<Path>) -> Vec<Vec<i32>> {
    let file = File::open(filename).expect("file err");
    let buf = BufReader::new(file);
    buf.lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .collect::<Vec<&str>>()
                .join(",")
                .split(',')
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}
fn check_horizontal(_x1: i32, _x2: i32, y1: i32, y2: i32) -> bool {
    y1 == y2
}
fn check_vertical(x1: i32, x2: i32, _y1: i32, _y2: i32) -> bool {
    x1 == x2
}
fn check_diagonal(x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
    check_diagonal1(x1, x2, y1, y2) || check_diagonal2(x1, x2, y1, y2)
}
fn check_diagonal1(x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
    x1 - x2 == y1 - y2
}
fn check_diagonal2(x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
    x1 - x2 == -(y1 - y2)
}
fn get_horizontal_cords(x1: i32, x2: i32, y1: i32, _y2: i32) -> Vec<(i32, i32)> {
    (cmp::min(x1, x2)..=cmp::max(x1, x2))
        .map(|x| (x, y1))
        .collect::<Vec<(i32, i32)>>()
}
fn get_vertical_cords(x1: i32, _x2: i32, y1: i32, y2: i32) -> Vec<(i32, i32)> {
    (cmp::min(y1, y2)..=cmp::max(y1, y2))
        .map(|y| (x1, y))
        .collect::<Vec<(i32, i32)>>()
}
fn get_diagonal_cords(x1: i32, x2: i32, y1: i32, y2: i32) -> Vec<(i32, i32)> {
    if check_diagonal1(x1, x2, y1, y2) {
        get_diagonal1_cords(x1, x2, y1, y2)
    } else {
        get_diagonal2_cords(x1, x2, y1, y2)
    }
}
fn get_diagonal1_cords(x1: i32, x2: i32, y1: i32, y2: i32) -> Vec<(i32, i32)> {
    (cmp::min(x1, x2)..=cmp::max(x1, x2))
        .enumerate()
        .map(|(index, x)| (x, cmp::min(y1, y2) + index as i32))
        .collect::<Vec<(i32, i32)>>()
}
fn get_diagonal2_cords(x1: i32, x2: i32, y1: i32, y2: i32) -> Vec<(i32, i32)> {
    (cmp::min(x1, x2)..=cmp::max(x1, x2))
        .enumerate()
        .map(|(i, x)| (x, cmp::max(y1, y2) - i as i32))
        .collect::<Vec<(i32, i32)>>()
}
fn cords_to_map(numbers: Vec<Vec<i32>>) -> HashMap<(i32, i32), i32> {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    for num in numbers {
        let cords;
        if check_vertical(num[0], num[2], num[1], num[3]) {
            cords = get_vertical_cords(num[0], num[2], num[1], num[3]);
        } else if check_horizontal(num[0], num[2], num[1], num[3]) {
            cords = get_horizontal_cords(num[0], num[2], num[1], num[3]);
        } else {
            cords = get_diagonal_cords(num[0], num[2], num[1], num[3]);
        }
        for cord in cords {
            *map.entry(cord).or_insert(0) += 1;
        }
    }
    map
}
fn count_map_fields(map: HashMap<(i32, i32), i32>) -> usize {
    map.into_values()
        .collect::<Vec<i32>>()
        .iter()
        .filter(|&x| *x > 1)
        .count()
}

fn part1(mut numbers: Vec<Vec<i32>>) -> usize {
    numbers = numbers
        .drain_filter(|num| {
            check_horizontal(num[0], num[2], num[1], num[3])
                || check_vertical(num[0], num[2], num[1], num[3])
        })
        .collect();
    let map = cords_to_map(numbers);
    count_map_fields(map)
}

fn part2(mut numbers: Vec<Vec<i32>>) -> usize {
    numbers = numbers
        .drain_filter(|num| {
            check_diagonal(num[0], num[2], num[1], num[3])
                || check_horizontal(num[0], num[2], num[1], num[3])
                || check_vertical(num[0], num[2], num[1], num[3])
        })
        .collect();
    let map = cords_to_map(numbers);
    count_map_fields(map)
}

fn main() {
    let numbers = read_data("./input/input5.txt");
    println!("Part 1: {}", part1(numbers.clone()));
    println!("Part 2: {}", part2(numbers));
}

#[test]
fn test_part1() {
    let data = read_data("./input/input5_test.txt");
    assert_eq!(part1(data), 5);
}
#[test]
fn test_part2() {
    let data = read_data("./input/input5_test.txt");
    assert_eq!(part2(data), 12);
}
