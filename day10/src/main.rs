use std::{io::BufRead};

fn read_input() -> Vec<String> {
    std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
}

fn chunk_match(c1: char, c2: char) -> bool {
    match (c1, c2) {
        ('<', '>') | ('[', ']') | ('{', '}') | ('(', ')') => return true,
        _ => return false
    }
}

fn score(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn parse(s: &str) -> Option<i32> {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '<'|'('|'['|'{' => stack.push(c),
            '>'|')'|']'|'}' => {
                let start_c = stack.pop().unwrap();
                if !chunk_match(start_c, c) {
                    return Some(score(c))
                }
            },
            _ => return None
        }
    }
    Some(0)
}

fn part_one(input: &Vec<String>) -> i32 {
    input.iter().map(|s|parse(s).unwrap()).sum()
}

fn parse_complete(s: &str) -> i64 {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        match c {
            '<'|'('|'['|'{' => stack.push(c),
            '>'|')'|']'|'}' => { stack.pop(); },
            _ => return 0,
        }
    }

    stack.iter().rev().map(|c|{
        match c {
            '(' => return 1,
            '[' => return 2,
            '{' => return 3,
            '<' => return 4,
            _ => return 0,
        }
    }).fold(0 as i64, |s, v| s*5 + v)
}

fn part_two(input: &Vec<String>) -> i64 {
    let mut scores = input.iter()
        .filter(|s| parse(s).unwrap() == 0)
        .map(|s| parse_complete(s))
        .collect::<Vec<_>>();
    scores.sort();
    *scores.iter().nth(scores.len()/2).unwrap()
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
