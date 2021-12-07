//https://adventofcode.com/2021/day/4
#![allow(clippy::useless_vec)]
#![feature(drain_filter)]
use array2d::Array2D;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    iter,
    path::Path,
};

fn read_data(filename: impl AsRef<Path>) -> (Vec<i32>, Vec<Array2D<i32>>) {
    let file = File::open(filename).expect("file err");
    let buf = BufReader::new(file);
    let mut lines = buf.lines().filter_map(|x| x.ok());
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let arrays = lines
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>()
        .chunks(5)
        .collect::<Vec<&[String]>>()
        .iter()
        .map(|&chunk| {
            Array2D::from_rows(
                &(chunk
                    .iter()
                    .map(|line| {
                        line.split_whitespace()
                            .filter_map(|num| num.parse::<i32>().ok())
                            .collect::<Vec<i32>>()
                    })
                    .collect::<Vec<Vec<i32>>>()),
            )
        })
        .collect::<Vec<Array2D<i32>>>();
    (numbers, arrays)
}

fn get_empty_arrays(n: usize) -> Vec<Array2D<i32>> {
    iter::repeat(Array2D::from_columns(&vec![vec![-1; 5]; 5]))
        .take(n)
        .collect::<Vec<Array2D<i32>>>()
}
fn sum_array(arr: &Array2D<i32>) -> i32 {
    let mut sum: i32 = 0;
    for row_iter in arr.rows_iter() {
        sum += row_iter.filter(|&x| *x != -1).sum::<i32>();
    }
    sum
}
fn check_win(arr: &Array2D<i32>) -> bool {
    check_column(arr) || check_rows(arr)
}
fn check_rows(arr: &Array2D<i32>) -> bool {
    for mut row_iter in arr.rows_iter() {
        if row_iter.all(|&x| x != -1) {
            return true;
        }
    }
    false
}
fn check_column(arr: &Array2D<i32>) -> bool {
    for mut column_iter in arr.columns_iter() {
        if column_iter.all(|&x| x != -1) {
            return true;
        }
    }
    false
}

fn part1(numbers: Vec<i32>, arrays: Vec<Array2D<i32>>) -> i32 {
    let mut arrays_to_fill = get_empty_arrays(arrays.len());
    for number in numbers {
        for (i, array) in arrays.iter().enumerate() {
            for j in 0..array.num_rows() {
                for k in 0..array.num_columns() {
                    if array.get(j, k) == Some(&number) {
                        *arrays_to_fill.get_mut(i).unwrap().get_mut(j, k).unwrap() = number;
                        if check_win(&arrays_to_fill[i]) {
                            let sum_all = sum_array(array);
                            let sum_marked = sum_array(&arrays_to_fill[i]);
                            return (sum_all - sum_marked) * number;
                        }
                    }
                }
            }
        }
    }
    0
}

fn part2(numbers: Vec<i32>, mut arrays: Vec<Array2D<i32>>) -> i32 {
    let mut arrays_to_fill = get_empty_arrays(arrays.len());
    for number in numbers {
        let mut to_delete = Vec::new();
        for (i, array) in arrays.iter().enumerate() {
            for j in 0..array.num_rows() {
                for k in 0..array.num_columns() {
                    if array.get(j, k) == Some(&number) {
                        *arrays_to_fill.get_mut(i).unwrap().get_mut(j, k).unwrap() = number;
                        if check_win(arrays_to_fill.get(i).unwrap()) {
                            to_delete.push(i);
                        }
                    }
                }
            }
        }
        if arrays.len() == 1 {
            if check_win(&arrays_to_fill[0]) {
                let sum_all = sum_array(&arrays[0]);
                let sum_marked = sum_array(&arrays_to_fill[0]);
                return (sum_all - sum_marked) * number;
            }
        } else {
            for (_, i) in to_delete.iter().rev().enumerate() {
                arrays.remove(*i);
                arrays_to_fill.remove(*i);
            }
        }
    }
    0
}

fn main() {
    let (numbers, arrays) = read_data("./input/input4.txt");
    println!("Part 1: {}", part1(numbers.clone(), arrays.clone()));
    println!("Part 2: {}", part2(numbers, arrays));
}

#[test]
fn test_part1() {
    let (numbers, arrays) = read_data("./input/input4_test.txt");
    assert_eq!(part1(numbers, arrays), 4512);
}
#[test]
fn test_part2() {
    let (numbers, arrays) = read_data("./input/input4_test.txt");
    assert_eq!(part2(numbers, arrays), 1924);
}
