use std::io::BufRead;

fn read_input() -> Vec<String> {
    std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
}

fn part_one(input: &Vec<String>) -> usize {
    input.iter().map(|s|{
        s.split_whitespace().skip(11).fold(0, |s,v |{
            match v.len() {
                2|3|4|7 => s+1,
                _ => s
            }
        })
    }).sum()
}

fn in_str(s: &str, sub: &str) -> bool {
    sub.chars().filter(|c| s.contains(&c.to_string())).count() == sub.len()
}

fn minus_str(lhs: &str, rhs: &str) -> String {
    lhs.chars().filter(|c|{ !rhs.contains(&c.to_string())}).collect()
}

fn part_two(input: &Vec<String>) -> i32 {
    input.iter().map(|s|{
        // per line
        let signal_patterns = s.split_whitespace().take(10).collect::<Vec<_>>();
        let one = signal_patterns.iter().filter(|s| s.len() == 2).take(1).last().unwrap();
        let seven = signal_patterns.iter().filter(|s| s.len() == 3).take(1).last().unwrap();
        let four = signal_patterns.iter().filter(|s| s.len() == 4).take(1).last().unwrap();

        let output_values = s.split_whitespace().skip(11).collect::<Vec<_>>();
        output_values.iter().map(|s|{
            match s.len() {
                2 => '1',
                3 => '7',
                4 => '4',
                7 => '8',
                5 => { // two, three or five
                    let pat = minus_str(&four, &one);
                    if in_str(s, &pat) {
                        return '5'
                    } else if in_str(s, &seven) {
                        return '3'
                    }
                    '2'
                },
                6 => { // zero, six or nine
                    if in_str(s, &seven) {
                        let pat = minus_str(&four, &one);
                        if in_str(s, &pat)  {
                            return '9'
                        }
                        return '0'
                    }
                    '6'
                },
                _ => panic!()
            }
        }).collect::<String>()
    }).map(|s|s.parse::<i32>().unwrap()).sum()
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
