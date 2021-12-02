use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    let mut count = 0;
    for (i, _) in numbers.iter().enumerate() {
        if i > 0 && numbers[i] > numbers[i - 1] {
            count += 1;
        }
    }
    println!("{}", count);
}
