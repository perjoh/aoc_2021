use std::io::BufRead;
use std::collections::HashMap;

fn read_input() -> Vec<String> {
    std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
}

fn calc_polymer(input: &Vec<String>, num: usize) -> u64 {
    let template = 
        input.first().unwrap().as_bytes()
            .windows(2)
            .map(|p| (p[0] as char, p[1] as char))
            .collect::<Vec<(_, _)>>();

    let mut polymer: HashMap<(_, _), u64> = HashMap::new();
    template.iter().for_each(|f| {
        let e= polymer.entry(*f).or_insert(0);
        *e += 1;
    });


    let rules = 
        input.iter()
            .skip(2)
            .map(|s| s.split_once(" -> ").unwrap()).collect::<HashMap<_, _>>();

    let rules: HashMap<(_, _), ((_, _), (_, _))> = 
        rules.iter()
            .map(|v| {
                let a = v.0.chars().nth(0).unwrap();
                let b = v.0.chars().nth(1).unwrap();
                let c = v.1.chars().nth(0).unwrap();
                ((a, b), ((a, c), (c, b)))
            }).collect();

    for _ in 0..num {
        let mut polymer_tmp: HashMap<(char, char), u64> = HashMap::new();

        polymer.iter()
            .for_each(|p| {
                match rules.get(p.0) {
                    Some(v) => {
                        let a = polymer_tmp.entry(v.0).or_insert(0);
                        *a += p.1;
                        let b = polymer_tmp.entry(v.1).or_insert(0);
                        *b += p.1;
                    },
                    None => { 
                        let a = polymer_tmp.entry(*p.0).or_insert(0);
                        *a = *p.1;
                    },
                }
            });

        polymer = polymer_tmp;
    }

    let mut count: HashMap<char, u64> = HashMap::new();

    polymer.iter()
        .for_each(|p| {
            let e = count.entry(p.0.0).or_insert(0);
            *e += *p.1;
        });

    let e = count.entry(template.last().unwrap().1).or_insert(0);
    *e += 1;

    let min = count.values().min().unwrap();
    let max = count.values().max().unwrap();
    max - min

}

fn part_one(input: &Vec<String>) -> u64 {
    calc_polymer(input, 10)
}

fn part_two(input: &Vec<String>) -> u64 {
    calc_polymer(input, 40)
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
