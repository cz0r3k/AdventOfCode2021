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
        let data = line.split_whitespace().collect::<Vec<&str>>().join(",");
        numbers.push(
            data.split(',')
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>(),
        );
        line.clear();
    }
    numbers
}

fn main() {
    let mut numbers = read_data("./../input.txt");
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    numbers = numbers
        .drain_filter(|x| x[0] == x[2] || x[1] == x[3])
        .collect();
    for num in numbers {
        if num[0] == num[2] {
            let min = cmp::min(num[1], num[3]);
            let max = cmp::max(num[1], num[3]);
            for y in min..=max {
                *map.entry((num[0], y)).or_insert(0) += 1;
            }
        } else {
            let min = cmp::min(num[0], num[2]);
            let max = cmp::max(num[0], num[2]);
            for x in min..=max {
                *map.entry((x, num[1])).or_insert(0) += 1;
            }
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
