use std::io::BufRead;

fn read_lines() -> Vec<String> {
    std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
}

fn parse_comma_sep_numbers(num_str: &str) -> Vec<usize> {
    num_str.split_terminator(',').into_iter().map(|s| s.parse::<usize>().unwrap()).collect()
}

fn parse_numbers(num_str: &str) -> Vec<usize> {
    num_str.split_whitespace().into_iter().map(|s| s.parse::<usize>().unwrap()).collect()
}

#[derive(Clone)]
struct BingoBoard {
    numbers: Vec<usize>,
}

trait BingoLogic {
    fn check_number(&self, number: usize) -> bool;
    fn mark_number(&mut self, number: usize);
    fn check_bingo(&self) -> bool;
    fn calc_score(&self, number: usize) -> usize;
}

impl BingoLogic for BingoBoard {
    fn check_number(&self, number: usize) -> bool {
        match self.numbers.iter().find(|n| **n == number) {
            Some(_) => true,
            _ => false,
        }
    }

    fn mark_number(&mut self, number: usize) {
        self.numbers = 
            self.numbers.iter().map(|n| {
                if *n == number {
                    return 0
                }
                *n
            } ).collect();
    }

    fn check_bingo(&self) -> bool {
        let bingo = &[0, 0, 0, 0, 0];

        if self.numbers.is_empty() {
            return false
        }

        // check rows
        for i in 0..5 {
            if &self.numbers[i*5..i*5+5] == bingo {
                return true
            }
        }

        // check columns
        for i in 0..5 {
            if  self.numbers[i + 0*5] == 0 &&
                self.numbers[i + 1*5] == 0 &&
                self.numbers[i + 2*5] == 0 &&
                self.numbers[i + 3*5] == 0 &&
                self.numbers[i + 4*5] == 0 {
                return true
            }
        }

        false
    }

    fn calc_score(&self, number: usize) -> usize {
        let num_zeroes = self.numbers.iter().filter(|n| **n == 0).count();
        println!("num zeroes: {}", num_zeroes);
        let s: usize = self.numbers.iter().sum();
        println!("sum: {}", s);
        println!("number: {}", number);
        s*number
    }
}

fn check_number(bb: &mut BingoBoard, number: usize) -> bool {
    bb.mark_number(number);
    bb.check_bingo()
}

fn load_bingo_boards(input: &Vec<String>) -> Vec<BingoBoard> {
    let mut bingo_boards: Vec<BingoBoard> = Vec::new();
    let mut it = input.iter();
    it.next();
    while it.next() != None {
        let mut bingo_board = BingoBoard { numbers: parse_numbers(it.next().unwrap())};
        bingo_board.numbers.append(&mut parse_numbers(it.next().unwrap()));
        bingo_board.numbers.append(&mut parse_numbers(it.next().unwrap()));
        bingo_board.numbers.append(&mut parse_numbers(it.next().unwrap()));
        bingo_board.numbers.append(&mut parse_numbers(it.next().unwrap()));
        bingo_boards.push(bingo_board);
    }
    bingo_boards
}

fn load_random_numbers(input: &Vec<String>) -> Vec<usize> {
    let mut it = input.iter();
    parse_comma_sep_numbers(it.next().unwrap())
}

fn part_one(input: &Vec<String>) -> usize {
    let mut bingo_boards: Vec<BingoBoard> = load_bingo_boards(input);
    let random_numbers = load_random_numbers(input);

    for number in random_numbers {
        for bb in &mut bingo_boards {
            if check_number(bb, number) {
                return bb.calc_score(number)
            }
        }
    }
    0
}

fn part_two(input: &Vec<String>) -> usize {
    let mut bingo_boards = load_bingo_boards(input);
    let random_numbers = load_random_numbers(input);
    let mut bingo_count = bingo_boards.len();
    for number in random_numbers {
        for bb in &mut bingo_boards {
            bb.mark_number(number);
        }

        for bb in &mut bingo_boards {
            if bb.check_bingo() {
                bingo_count -= 1;
                if bingo_count == 0 {
                    return bb.calc_score(number)
                }

                bb.numbers.clear();
            }
        }
    }
    0
}

fn main() {
    let input = read_lines();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}