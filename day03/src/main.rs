use std::{io::BufRead};

fn read_lines() -> Vec<String> {
    std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
}

fn count_zeroes(index:usize, input: &Vec<String>) -> usize {
    input.iter().filter(|s| s.chars().nth(index).unwrap() == '0').count()
}

fn most_common_bit(index:usize, input:&Vec<String>) -> bool {
    input.len()/2 < count_zeroes(index, input)
}

fn part_one(input: &Vec<String>) -> usize {
    let mut gamma:usize = 0;
    let mut eps:usize = 0; 
    for i in 0..input[0].len() {
        gamma <<= 1;
        eps <<= 1;
        if most_common_bit(i, input) {
            gamma += 1;
        } else {
            eps += 1;
        }
    }
    gamma*eps
}

fn get_rating(input: &Vec<String>, invert_filter: bool) -> usize {
    let mut tmp : Vec<String> = input.to_vec();
    for i in 0..input[0].len() {
        let num_zeroes = count_zeroes(i, &tmp);
        let num_ones = tmp.len() - num_zeroes;

        let mut filter_c =
            if num_ones > num_zeroes {
                '1'
            } else if num_ones < num_zeroes {
                '0'
            } else {
                '1'
            };

        if invert_filter {
            if filter_c == '1' {
                filter_c = '0';
            } else {
                filter_c = '1';
            }
        }

        tmp = tmp.iter().filter(|s| s.chars().nth(i).unwrap() == filter_c ).map(|s| s.clone()).collect();

        if tmp.len() == 1 {
            return usize::from_str_radix(&tmp[0], 2).unwrap()
        }
    }

    0

}

fn part_two(input: &Vec<String>) -> usize {
    let oxygen_rating = get_rating(input, false);
    let co2_rating = get_rating(input, true);
    oxygen_rating*co2_rating
}

fn main() {
    let input = read_lines();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
