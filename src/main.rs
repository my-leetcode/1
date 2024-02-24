use std::io::{self, Write};

use crate::solution::Solution;

mod solution;

fn main() {
    print!("Enter target > ");
    let mut line = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();
    let target = line.trim().parse::<i32>().unwrap();

    print!("Enter numbers (separated by space) > ");
    let mut line = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();
    let nums = line
        .trim()
        .split(' ')
        .flat_map(str::parse)
        .collect::<Vec<_>>();

    println!("{:#?}", Solution::two_sum(nums, target))
}
