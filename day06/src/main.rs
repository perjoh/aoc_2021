use std::io::BufRead;

fn read_input() -> Vec<String> {
    std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
}

type MegaValues = Vec<(u8, u64)>;

fn parse_values(input: &str) -> MegaValues {
    input.split(',').map(|s| (s.parse::<u8>().unwrap(), 1)).collect()
}

fn cycle(values: &mut MegaValues) {
    let new_count = values.iter().filter(|v| v.0 == 0).fold(0, |s, v| s + v.1);
    for value in & mut *values {
        if value.0 == 0 {
            value.0 = 6;
        } else {
            value.0 -= 1;
        }
    }
    values.push((8, new_count));
}

fn do_cycles(values: &mut MegaValues, num_cycles: i32) -> u64 {
    for _ in 0..num_cycles {
        cycle(&mut *values);
    }
    values.iter().fold(0, |s, v| s + v.1)
}

fn part_one(input: &Vec<String>) -> u64 {
    let mut values = parse_values(&input[0]);
    do_cycles(&mut values, 80)
}

fn part_two(input: &Vec<String>) -> u64 {
    let mut values = parse_values(&input[0]);
    do_cycles(&mut values, 256)
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
