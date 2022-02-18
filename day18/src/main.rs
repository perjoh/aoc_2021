use std::io::BufRead;

fn read_input() -> Vec<String> {
    std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
}

#[derive(Clone)]
struct Node {
    depth: u8,
    value: usize,
}

type SnailfishNumber = Vec<Node>;

fn parse(input: &Vec<String>) -> Vec<SnailfishNumber> {
    input.iter()
        .map(|s| {
            let mut depth: u8 = 0;
            let mut sf_numbers = Vec::new();
            s.bytes().for_each(|c| {
                match c {
                    b'[' => depth += 1,
                    b']' => depth -= 1,
                    b',' => {},
                    b'0'..=b'9' => sf_numbers.push(Node { depth, value: (c - b'0') as usize }),
                    _ => panic!(),
                }
            });
            sf_numbers
        }).collect::<Vec<_>>()
}

fn add(lhs: &SnailfishNumber, rhs: &SnailfishNumber) -> SnailfishNumber {
    lhs.iter().chain(rhs).map(|n| Node { depth: n.depth + 1, value: n.value }).collect()
}

fn split(num: &mut SnailfishNumber) -> bool {
    for i in 0..num.len() {
        if i < num.len() && num[i].value > 9 {
            let l = num[i].value/2;
            let r = (num[i].value+1)/2;
            let depth = num[i].depth+1;
            num[i] = Node { depth, value: l };
            num.insert(i+1, Node { depth, value: r });
            return true;
        }
    }
    false
}

fn magnitude(mut num: Vec<Node>) -> usize {
    for depth in (1..5).rev() {
        let mut i = 0;
        while i < num.len()-1 {
            if num[i].depth == depth {
                if num[i].depth == num[i+1].depth {
                    num[i] = Node { 
                        depth: num[i].depth-1, 
                        value: num[i].value*3 + num[i+1].value*2
                    };
                    num.remove(i+1);
                }
            }
            i+=1;
        }
    }
    num[0].value
}

fn explode(num: &mut SnailfishNumber) -> bool {
    for i in 0..num.len() {
        if num[i].depth == 5 {
            if i > 0 {
                num[i-1].value += num[i].value;
            }

            if i + 2 < num.len() {
                num[i+2].value += num[i+1].value;
            }

            let depth = num[i].depth - 1;

            num.remove(i+1);
            num[i] = Node { depth, value: 0 };

            return true;
        }
    }
    false
}

fn reduce_sf_num(num: &mut SnailfishNumber) {
    loop {
        if !explode(num) && !split(num) {
            break;
        }
    }
}

fn part_one(input: &Vec<String>) -> usize {
    let mut sf = parse(input);
    while sf.len() > 1 {
        sf[0] = add(&sf[0], &sf[1]);
        reduce_sf_num(&mut sf[0]);
        sf.remove(1);
    }
    magnitude(sf[0].clone())
}

fn part_two(input: &Vec<String>) -> usize {
    let mut maxmag = Vec::new();
    let sf = parse(input);
    for y in 0..sf.len()-1 {
        for x in 1..sf.len() {
            let mut num = add(&sf[x], &sf[y]);
            reduce_sf_num(&mut num);
            maxmag.push(magnitude(num));
        }
    }
    *maxmag.iter().max().unwrap()
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}