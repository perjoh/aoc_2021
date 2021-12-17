use std::{io::BufRead, collections::HashMap};

fn read_input() -> Vec<String> {
    std::io::stdin().lock().lines().collect::<std::io::Result<Vec<String>>>().unwrap()
}

type Caves = HashMap<String, Vec<String>>;

fn traverse(caves: &Caves, cur: &str, visited: &mut Vec<String>, twice_cave: &str) -> i32 {

    if cur == "end" {
        if twice_cave == "" || visited.iter().filter(|s| *s == twice_cave).count() == 2 {
            return 1
        }
        return 0
    }

    if cur.to_lowercase() == cur {
        visited.push(cur.to_string());
        if cur == twice_cave {
            if visited.iter().filter(|s| *s == cur).count() > 2 {
                return 0
            }
        } else {
            if visited.iter().filter(|s| *s == cur).count() > 1 {
                return 0
            }
        }
    }

    let mut count = 0;
    for c in caves.get(cur).unwrap() {
        let l = visited.len();
        count += traverse(caves, c, visited, twice_cave);
        visited.drain(l..);
    }

    count
}

fn part_one(input: &Vec<String>) -> i32 {
    let mut caves: HashMap<String, Vec<String>> = HashMap::new();

    input.iter().map(|s| s.split_once('-').unwrap() )
        .for_each(|i| { 
            caves.entry(i.0.to_string()).or_insert(Vec::new()).push(i.1.to_string());
            caves.entry(i.1.to_string()).or_insert(Vec::new()).push(i.0.to_string());
        });

    traverse(&caves, "start", &mut Vec::new(), "")
}

fn part_two(input: &Vec<String>) -> i32 {
    let mut caves: HashMap<String, Vec<String>> = HashMap::new();

    input.iter().map(|s| s.split_once('-').unwrap() )
        .for_each(|i| { 
            caves.entry(i.0.to_string()).or_insert(Vec::new()).push(i.1.to_string());
            caves.entry(i.1.to_string()).or_insert(Vec::new()).push(i.0.to_string());
        });

    caves.iter()
        .filter(|(k, _)| k.to_lowercase() == **k && **k != "start" && **k != "end")
        .map(|(k, _)| k)
        .map(|v| traverse(&caves, "start", &mut Vec::new(), v))
        .sum()
}

fn main() {
    let input = read_input();
    let p1 = part_one(&input);
    println!("{}", p1);
    println!("{}", p1 + part_two(&input));
}
