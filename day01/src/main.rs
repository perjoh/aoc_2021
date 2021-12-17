use std::io::{self, BufRead};

fn read_ints() -> Vec<i32> {
    io::stdin().lock().lines().map(|s| s.unwrap().parse::<i32>().unwrap()).collect()
}

fn part_one(input: &Vec<i32>) -> i32 {
    let mut count:i32 = 0;
    for i in 1..input.len() {
        if input[i-1] < input[i] {
            count = count + 1;
        }
    }
    count
}

fn part_two(input: &Vec<i32>) -> i32 {
    let mut sum = Vec::new();
    for i in 0..input.len()-2 {
        sum.push(input[i] + input[i+1] + input[i+2]);
    }

    part_one(&sum)
}

fn main() {
    let input = read_ints();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}