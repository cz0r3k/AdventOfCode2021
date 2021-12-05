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
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let mut numbers = Vec::new();
    while buf.read_line(&mut line).unwrap() > 0 {
        numbers.push(
            line.split_whitespace()
                .collect::<Vec<&str>>()
                .join(",")
                .split(',')
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>(),
        );
        line.clear();
    }
    numbers
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
        .map(|(i, x)| (x, cmp::min(y1, y2) + i as i32))
        .collect::<Vec<(i32, i32)>>()
}
fn get_diagonal2_cords(x1: i32, x2: i32, y1: i32, y2: i32) -> Vec<(i32, i32)> {
    (cmp::min(x1, x2)..=cmp::max(x1, x2))
        .enumerate()
        .map(|(i, x)| (x, cmp::max(y1, y2) - i as i32))
        .collect::<Vec<(i32, i32)>>()
}

fn main() {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut numbers = read_data("./../input.txt");
    numbers = numbers
        .drain_filter(|num| {
            check_diagonal(num[0], num[2], num[1], num[3])
                || check_horizontal(num[0], num[2], num[1], num[3])
                || check_vertical(num[0], num[2], num[1], num[3])
        })
        .collect();
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
    let sum = map
        .into_values()
        .collect::<Vec<i32>>()
        .iter()
        .filter(|&x| *x > 1)
        .count();
    println!("{}", sum);
}
