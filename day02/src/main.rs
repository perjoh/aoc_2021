use core::panic;
use std::io::{self, BufRead};

fn read_lines() -> Vec<String> {
    io::stdin().lock().lines().collect::<io::Result<Vec<String>>>().unwrap()
}

fn part_one(input: &Vec<String>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for s in input {
        let mut it = s.split_whitespace();
        let direction = it.next().unwrap();
        let x = it.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => horizontal = horizontal + x,
            "down" => depth = depth + x,
            "up" => depth = depth - x,
            _ => panic!()
        }
    }

    horizontal*depth
}

fn part_two(input: &Vec<String>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for s in input {
        let mut it = s.split_whitespace();
        let direction = it.next().unwrap();
        let x = it.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => { 
                horizontal = horizontal + x;
                depth = depth + aim*x;
            }
            "down" => {
                aim = aim + x;
            }
            "up" => { 
                aim = aim - x;
            }
            _ => panic!()
        }
    }

    horizontal*depth
}

fn main() {
    let lines = read_lines();
    println!("{}", part_one(&lines));

    assert_eq!(part_two(&vec!["forward 5".to_string(), "down 5".to_string(), "forward 8".to_string(), "up 3".to_string(), "down 8".to_string(), "forward 2".to_string()]), 900);

    println!("{}", part_two(&lines));
}
