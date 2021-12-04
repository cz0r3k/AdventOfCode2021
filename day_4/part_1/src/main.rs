use array2d::Array2D;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_data(filename: impl AsRef<Path>) -> (Vec<u32>, Vec<Array2D<u32>>) {
    let file = File::open(filename).expect("file err");
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    buf.read_line(&mut line).expect("Cant read line");
    let numbers = line
        .split_whitespace()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    line.clear();
    let mut arrays: Vec<Array2D<u32>> = Vec::new();
    while buf.read_line(&mut line).unwrap() > 0 {
        line.clear();
        let mut rows: Vec<Vec<u32>> = Vec::new();
        for _i in 0..5 {
            buf.read_line(&mut line).unwrap();
            rows.push(
                line.split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            );
            line.clear();
        }
        arrays.push(Array2D::from_rows(&rows));
    }

    (numbers, arrays)
}
fn sum_array(arr: &Array2D<Option<u32>>) -> u32 {
    let mut sum: u32 = 0;
    for i in 0..5 {
        sum += arr
            .row_iter(i)
            .filter(|&x| *x != None)
            .map(|&x| x.unwrap())
            .sum::<u32>();
    }
    sum
}
fn sum_all_array(arr: &Array2D<u32>) -> u32 {
    let mut sum: u32 = 0;
    for i in 0..5 {
        sum += arr.row_iter(i).sum::<u32>();
    }
    sum
}
fn check_win(arr: &Array2D<Option<u32>>) -> Option<u32> {
    if check_column(arr) || check_rows(arr) {
        return Some(sum_array(arr));
    }
    None
}
fn check_rows(arr: &Array2D<Option<u32>>) -> bool {
    for i in 0..5 {
        if arr.row_iter(i).all(|&x| x != None) {
            return true;
        }
    }
    false
}
fn check_column(arr: &Array2D<Option<u32>>) -> bool {
    for i in 0..5 {
        if arr.column_iter(i).all(|&x| x != None) {
            return true;
        }
    }
    false
}

fn main() {
    let (numbers, arrays) = read_data("./../input.txt");
    let mut arrays_to_fill: Vec<Array2D<Option<u32>>> = Vec::new();
    for _i in 0..arrays.len() {
        arrays_to_fill.push(Array2D::from_columns(&vec![vec![None; 5]; 5]));
    }
    'outer: for number in numbers {
        for (i, array) in arrays.iter().enumerate() {
            for j in 0..5 {
                for k in 0..5 {
                    if array.get(j, k) == Some(&number) {
                        *arrays_to_fill.get_mut(i).unwrap().get_mut(j, k).unwrap() = Some(number);
                        let sum_marked = check_win(arrays_to_fill.get(i).unwrap());
                        if sum_marked != None {
                            let sum_all = sum_all_array(array);
                            println!(
                                "{} * {} = {}",
                                number,
                                sum_all - sum_marked.unwrap(),
                                number as u32 * (sum_all - sum_marked.unwrap())
                            );
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
}
