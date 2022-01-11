use std::io::BufRead;

fn read_input() -> Vec<String> {
    std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
}

fn to_binary(hex: &String) -> String {
    hex.bytes().map(|v| {
        match v {
            b'0' => "0000",
            b'1' => "0001",
            b'2' => "0010",
            b'3' => "0011",
            b'4' => "0100",
            b'5' => "0101",
            b'6' => "0110",
            b'7' => "0111",
            b'8' => "1000",
            b'9' => "1001",
            b'A' => "1010",
            b'B' => "1011",
            b'C' => "1100",
            b'D' => "1101",
            b'E' => "1110",
            b'F' => "1111",
            _ => panic!(),
        }
    }).collect::<String>()
}

struct Parser {
    version_sum: usize,
}

struct ParseResult {
    value: usize,
    bits_read: usize,
}

impl Parser {
    fn parse_n_packets(&mut self, input: &str, n: usize) -> (Vec<usize>, usize) {
        let mut bits_read = 0;
        let mut values = Vec::new();
        for _ in 0..n {
            let result = self.parse_packet(&input[bits_read..]);
            values.push(result.value);
            bits_read += result.bits_read;
        }
        (values, bits_read)
    }

    fn parse_len_packets(&mut self, input: &str, size: usize) -> (Vec<usize>, usize) {
        let mut values = Vec::new();
        let mut bits_read = 0;
        while bits_read < size {
            let result = self.parse_packet(&input[bits_read..]);
            values.push(result.value);
            bits_read += result.bits_read;
        }
        (values, bits_read)
    }

    fn parse_multiple_packets(&mut self, input: &str) -> (Vec<usize>, usize) {
        if input.bytes().nth(0).unwrap() == b'1' {
            let bits_read: usize = 12;
            let num = usize::from_str_radix(&input[1..bits_read], 2).unwrap();
            let result = self.parse_n_packets(&input[bits_read..], num);
            return (result.0, result.1 + bits_read);
        }

        let bits_read: usize = 16;
        let size = usize::from_str_radix(&input[1..bits_read], 2).unwrap();
        let result = self.parse_len_packets(&input[bits_read..], size);
        (result.0, result.1 + bits_read)
    }

    fn parse_packet(&mut self, input: &str) -> ParseResult {
        let version = usize::from_str_radix(&input[0..3], 2).unwrap();
        let type_id = usize::from_str_radix(&input[3..6], 2).unwrap();
        self.version_sum += version;

        let bits_read = 6;

        match type_id {
            4 => {
                let values = parse_values(&input[bits_read..]);
                let v = values.iter().fold(0 as usize, |s, v| s*16 + v);
                return ParseResult { value: v, bits_read: values.len()*5 + bits_read };
            },
            _ => {
                let result = self.parse_multiple_packets(&input[bits_read..]);
                let result = (result.0, result.1 + bits_read);
                match type_id {
                    0 => return ParseResult { value: result.0.iter().sum(), bits_read: result.1 },
                    1 => return ParseResult { value: result.0.iter().product(), bits_read: result.1 },
                    2 => return ParseResult { value: *result.0.iter().min().unwrap(), bits_read: result.1 },
                    3 => return ParseResult { value: *result.0.iter().max().unwrap(), bits_read: result.1 },
                    5 => return ParseResult { value: (result.0.iter().nth(0).unwrap() > result.0.iter().nth(1).unwrap()) as usize, bits_read: result.1 },
                    6 => return ParseResult { value: (result.0.iter().nth(0).unwrap() < result.0.iter().nth(1).unwrap()) as usize, bits_read: result.1 },
                    7 => return ParseResult { value: (result.0.iter().nth(0).unwrap() == result.0.iter().nth(1).unwrap()) as usize, bits_read: result.1 },
                    _ => panic!(),
                }
            },
        }
    }
}

fn parse_value(input: &str) -> (bool, usize) {
    let value = &input[0..5];
    let more_values = value.bytes().nth(0).unwrap() == b'1';
    let value = usize::from_str_radix(&value[1..], 2).unwrap();
    (more_values, value)
}

fn parse_values(input: &str) -> Vec<usize> {
    let mut values: Vec<usize> = Vec::new();
    let mut ptr: usize = 0;
    loop {
        let result = parse_value(&input[ptr..]);
        ptr += 5;
        values.push(result.1);
        if !result.0 {
            break;
        }
    }
    values
}

fn part_one(input: &Vec<String>) -> usize {
    let mut parser = Parser { version_sum: 0 };
    parser.parse_packet(&to_binary(input.first().unwrap())[0..]);
    parser.version_sum
}

fn part_two(input: &Vec<String>) -> usize {
    let mut parser = Parser { version_sum: 0, };
    parser.parse_packet(&to_binary(input.first().unwrap())[0..]).value
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
