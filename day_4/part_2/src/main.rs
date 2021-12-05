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
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    buf.read_line(&mut line).expect("Cant read line");
    let numbers = line
        .split_whitespace()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    line.clear();
    let mut arrays: Vec<Array2D<i32>> = Vec::new();
    while buf.read_line(&mut line).unwrap() > 0 {
        line.clear();
        let mut rows: Vec<Vec<i32>> = Vec::new();
        for _i in 0..5 {
            buf.read_line(&mut line).unwrap();
            rows.push(
                line.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
            line.clear();
        }
        arrays.push(Array2D::from_rows(&rows));
    }

    (numbers, arrays)
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

fn main() {
    let (numbers, mut arrays) = read_data("./../input.txt");
    let mut arrays_to_fill: Vec<Array2D<i32>> = Vec::new();
    let mut arr = iter::repeat(Array2D::from_columns(&vec![vec![-1; 5]; 5]));
    for _i in 0..arrays.len() {
        arrays_to_fill.push(arr.next().unwrap());
    }
    for number in numbers {
        for (i, array) in arrays.iter().enumerate() {
            for j in 0..5 {
                for k in 0..5 {
                    if array.get(j, k) == Some(&number) {
                        *arrays_to_fill.get_mut(i).unwrap().get_mut(j, k).unwrap() = number;
                    }
                }
            }
        }
        if arrays.len() == 1 {
            if check_win(&arrays_to_fill[0]) {
                let all_sum = sum_array(&arrays[0]);
                let sum_marked = sum_array(&arrays_to_fill[0]);
                let sum = all_sum - sum_marked;
                println!("{} * {} = {}", number, sum, number * sum);
                break;
            }
        } else {
            let mut to_delete = Vec::new();
            for (i, array) in arrays_to_fill.iter().enumerate() {
                if check_win(array) {
                    to_delete.push(i);
                }
            }
            for (_, i) in to_delete.iter().rev().enumerate() {
                arrays.remove(*i);
                arrays_to_fill.remove(*i);
            }
        }
    }
}
