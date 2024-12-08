#![allow(unused)]
/// Advent of Code - day 2
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::Path;

fn main() {
    let data = read_data("data.txt");
    let res = parse_lines(&data[..]);
    let mut count = 0;
    for row in &res {
        if !row_is_safe(&row) {
            count += 1;
        }
    }
    assert_eq!(res.len(), 1000); // aoc rows = 1000
    println!("BAD TOTAL = {}", count);
}

fn read_data(input: &str) -> String {
    let data = fs::read_to_string(&input).unwrap();
    data.trim().to_string()
}

fn parse_lines(input: &str) -> Vec<&str> {
    let v: Vec<_> = input.split("\n").collect();
    v
}

fn row_is_safe(input: &str) -> bool {
    let arr: Vec<_> = input
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut start = arr[0];
    for v in 2..arr.len() {
        if arr[v] > arr[v - 1] && arr[v] < arr[v - 1] + 4 {
            println!("{:?}", v);
            return true;
        } else {
            if arr[v] < arr[v - 1] && arr[v] < arr[v - 1] - 4 {
                println!("{:?}", v);
                return true;
            } else {
                println!("{:?}", "unsafe - exit this row");
                return false;
            }
        }
    }
    true
}
