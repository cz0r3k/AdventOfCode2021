//https://adventofcode.com/2021/day/8
#![feature(drain_filter)]
use array2d::Array2D;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_data(filename: impl AsRef<Path>) -> Array2D<u32> {
    let file = File::open(filename).expect("file err");
    let buf = BufReader::new(file);
    let v = buf
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.chars()
                .filter_map(|point| point.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    Array2D::from_rows(&v)
}

fn check_low_point(numbers: &Array2D<u32>, i: usize, j: usize) -> bool {
    let point = *numbers.get(i, j).unwrap();
    let up = if i == 0 {
        u32::MAX
    } else {
        *numbers.get(i - 1, j).unwrap_or(&u32::MAX)
    };
    let down = *numbers.get(i + 1, j).unwrap_or(&u32::MAX);
    let left = if j == 0 {
        u32::MAX
    } else {
        *numbers.get(i, j - 1).unwrap_or(&u32::MAX)
    };
    let right = *numbers.get(i, j + 1).unwrap_or(&u32::MAX);
    point < up && point < down && point < left && point < right
}

fn check_point_is_good(
    i: usize,
    j: usize,
    numbers: &Array2D<u32>,
    bool_array: &Array2D<bool>,
) -> bool {
    *numbers.get(i, j).unwrap() != 9 && !*bool_array.get(i, j).unwrap()
}

fn backtracking(
    i: usize,
    j: usize,
    numbers: &Array2D<u32>,
    bool_array: &mut Array2D<bool>,
) -> usize {
    let mut sum = 1;
    bool_array.set(i, j, true).unwrap();
    if i != 0 && check_point_is_good(i - 1, j, numbers, bool_array) {
        sum += backtracking(i - 1, j, numbers, bool_array);
    }
    if j != 0 && check_point_is_good(i, j - 1, numbers, bool_array) {
        sum += backtracking(i, j - 1, numbers, bool_array);
    }
    if i != numbers.column_len() - 1 && check_point_is_good(i + 1, j, numbers, bool_array) {
        sum += backtracking(i + 1, j, numbers, bool_array);
    }
    if j != numbers.row_len() - 1 && check_point_is_good(i, j + 1, numbers, bool_array) {
        sum += backtracking(i, j + 1, numbers, bool_array);
    }
    sum
}

fn part1(numbers: &Array2D<u32>) -> u32 {
    let mut sum = 0;
    for i in 0..numbers.column_len() {
        for j in 0..numbers.row_len() {
            if check_low_point(numbers, i, j) {
                sum += *numbers.get(i, j).unwrap() + 1;
            }
        }
    }
    sum
}
fn part2(numbers: Array2D<u32>) -> usize {
    let mut bool_array = Array2D::filled_with(false, numbers.num_rows(), numbers.num_columns());
    let mut sizes = Vec::new();
    for i in 0..numbers.column_len() {
        for j in 0..numbers.row_len() {
            if check_point_is_good(i, j, &numbers, &bool_array) {
                sizes.push(backtracking(i, j, &numbers, &mut bool_array));
            }
        }
    }
    sizes.sort_unstable();
    sizes[sizes.len() - 1] * sizes[sizes.len() - 2] * sizes[sizes.len() - 3]
}
fn main() {
    let numbers = read_data("./input/input9.txt");
    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(numbers));
}

#[test]
fn test_part1() {
    let numbers = read_data("./input/input9_test.txt");
    assert_eq!(part1(&numbers), 15);
}
#[test]
fn test_part2() {
    let numbers = read_data("./input/input9_test.txt");
    assert_eq!(part2(numbers), 1134);
}
